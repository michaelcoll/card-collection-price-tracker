<script setup lang="ts">
import type { CollectionCard } from '~/bindings/CollectionCard';
import type { SortBy } from '~/bindings/SortBy';
import type { SortDir } from '~/bindings/SortDir';

type CardDisplay = {
  name: string;
  qty: number;
  unit: number;
  trend: number;
  rar: string;
  set: string;
  language: string;
  foil: boolean;
};

const GRAPH_DATA = [4132, 4118, 4140, 4096, 4150, 4172, 4160, 4188, 4176, 4205, 4192, 4218];

const { getCollection, getCollectionStats, importCards } = useCardsService();

const q = ref('');
const qDebounced = refDebounced(q, 200);

const params = ref({
  sort_by: 'trend' as SortBy,
  sort_dir: 'desc' as SortDir,
  page: 0,
  page_size: 20,
  q: '',
});

const { data: collectionData, pending, refresh } = await getCollection(params);
const { data: statsData } = await getCollectionStats();

const allCards = ref<CollectionCard[]>([]);

watch(
  collectionData,
  (data) => {
    if (!data) return;
    if (params.value.page === 0) {
      allCards.value = [...data.items];
    } else {
      allCards.value.push(...data.items);
    }
  },
  { immediate: true },
);

watch([() => params.value.sort_by, () => params.value.sort_dir], () => {
  allCards.value = [];
  params.value.page = 0;
  refresh();
});

watch(qDebounced, (val) => {
  allCards.value = [];
  params.value.page = 0;
  params.value.q = val;
  refresh();
});

watch(
  () => params.value.page,
  (page) => {
    if (page > 0) refresh();
  },
);

const hasMore = computed(() =>
  collectionData.value ? allCards.value.length < collectionData.value.total : false,
);

const sentinel = ref<HTMLElement | null>(null);
let io: IntersectionObserver | null = null;

onMounted(() => {
  io = new IntersectionObserver(
    ([entry]) => {
      if (entry?.isIntersecting && hasMore.value && !pending.value) {
        params.value.page += 1;
      }
    },
    { rootMargin: '300px' },
  );
  onUnmounted(() => io?.disconnect());
});

watch(sentinel, (el, oldEl) => {
  if (oldEl) io?.unobserve(oldEl);
  if (el) io?.observe(el);
});

const cards = computed<CardDisplay[]>(() =>
  allCards.value.map((c) => ({
    name: c.name,
    qty: c.quantity,
    unit: c.purchase_price,
    trend: c.price_guide?.trend ?? 0,
    rar: c.rarity_code,
    set: c.set_code,
    language: c.language_code,
    foil: c.foil,
  })),
);

const view = ref<'grid' | 'list'>('grid');
const size = ref<'sm' | 'md' | 'lg'>('md');
const graph = ref<'compact' | 'expanded'>('compact');
const graphRange = ref('30 j');
const sheet = ref(false);
const importOpen = ref(false);
const importStep = ref<'drop'>('drop');
const importLoading = ref(false);
const importError = ref<string | null>(null);
const isDragging = ref(false);
const fileInputRef = ref<HTMLInputElement | null>(null);
const active = ref({ rar: [] as string[], sets: [] as string[] });
const detail = ref<CollectionCard | null>(null);

const detailDelta = (card: CollectionCard) => {
  const u = card.purchase_price;
  const t = card.price_guide?.trend ?? 0;
  if (!u) return 0;
  return Math.round(((t - u) / u) * 100);
};

const toggle = (k: 'rar' | 'sets', v: string) => {
  const arr = active.value[k];
  active.value[k] = arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v];
};

const setList = computed(() => statsData.value?.sets ?? []);

const priceMin = computed(() =>
  statsData.value?.price_trend_min != null ? Math.floor(statsData.value.price_trend_min / 100) : 0,
);
const priceMax = computed(() =>
  statsData.value?.price_trend_max != null ? Math.ceil(statsData.value.price_trend_max / 100) : 150,
);

const graphOptions = [
  {
    value: 'compact',
    label: '',
    icon: 'lucide:bar-chart-2',
    title: 'Graphe compact',
    tone: 'cyan',
  },
  {
    value: 'expanded',
    label: '',
    icon: 'lucide:trending-up',
    title: 'Graphe étendu',
    tone: 'cyan',
  },
];
const sizeOptions = [
  { value: 'sm', label: '', icon: 'lucide:grid-3x3', title: 'Petites cartes', tone: 'cyan' },
  { value: 'md', label: '', icon: 'lucide:grid-2x2', title: 'Cartes moyennes', tone: 'cyan' },
  { value: 'lg', label: '', icon: 'lucide:square', title: 'Grandes cartes', tone: 'cyan' },
];

const openImport = () => {
  importStep.value = 'drop';
  importError.value = null;
  importOpen.value = true;
};

const handleFile = async (file: File) => {
  if (!file.name.endsWith('.csv')) {
    importError.value = 'Le fichier doit être au format .csv';
    return;
  }
  importError.value = null;
  importLoading.value = true;
  try {
    const csv = await file.text();
    await importCards(csv);
    importOpen.value = false;
    allCards.value = [];
    params.value.page = 0;
    refresh();
  } catch (e: unknown) {
    const err = e as { data?: { message?: string } };
    importError.value = err?.data?.message ?? "Erreur lors de l'import";
  } finally {
    importLoading.value = false;
    if (fileInputRef.value) fileInputRef.value.value = '';
  }
};

const onFileInputChange = (e: Event) => {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (file) handleFile(file);
};

const onDrop = (e: DragEvent) => {
  e.preventDefault();
  isDragging.value = false;
  const file = e.dataTransfer?.files[0];
  if (file) handleFile(file);
};

const onDragOver = (e: DragEvent) => {
  e.preventDefault();
  isDragging.value = true;
};

const onDragLeave = () => {
  isDragging.value = false;
};
</script>

<template>
  <div class="max-w-[1180px] mx-auto px-5 pt-7 pb-10 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- ── VALUE BAR ── -->
    <div
      class="bg-cyan-500/10 dark:bg-cyan-400/10 backdrop-blur-md border border-cyan-500/30 dark:border-cyan-400/30 rounded-2xl shadow-lg px-5 py-5 mb-4"
    >
      <div class="flex items-center justify-between flex-wrap gap-4">
        <div class="flex flex-col gap-1">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
            >Valeur totale · CardMarket</span
          >
          <span
            class="font-mono font-bold tracking-tight whitespace-nowrap text-[clamp(30px,5vw,38px)]"
            >€ 4 218,60</span
          >
          <span class="text-cyan-600 dark:text-cyan-400 text-sm font-mono"
            >▴ €86,20 · +2,1 % (30 j)</span
          >
        </div>
        <div class="flex flex-col gap-2 items-end">
          <div class="flex items-center gap-2">
            <span
              class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap max-md:hidden"
              >graphe</span
            >
            <SegToggle v-model="graph" :options="graphOptions" size="sm" />
          </div>
          <Sparkline v-if="graph === 'compact'" :data="[40, 52, 46, 60, 54, 70, 64, 78, 72, 90]" />
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap max-md:hidden flex items-center gap-1.5"
          >
            <Icon name="lucide:refresh-cw" :size="11" /> synchro auto · il y a 2 h
          </span>
        </div>
      </div>

      <div v-if="graph === 'expanded'" class="flex flex-col gap-2.5 mt-4">
        <div class="flex gap-2">
          <button
            v-for="r in ['30 j', '3 m', '1 an', 'Max']"
            :key="r"
            :class="[
              'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border whitespace-nowrap cursor-pointer select-none transition-all duration-150',
              graphRange === r
                ? 'border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 bg-cyan-500/10 dark:bg-cyan-400/10'
                : 'border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800',
            ]"
            @click="graphRange = r"
          >
            {{ r }}
          </button>
        </div>
        <LineGraph
          :data="GRAPH_DATA"
          hi="€4,3k"
          lo="€4,1k"
          :value-fmt="(v) => '€' + (v ?? 0).toLocaleString('fr-FR')"
        />
      </div>
    </div>

    <!-- ── CONTROLS ── -->
    <div class="flex items-center justify-between flex-wrap gap-3 mb-4">
      <button
        class="md:hidden inline-flex items-center gap-2 justify-center px-4 py-2.5 rounded-xl text-sm font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
        @click="openImport"
      >
        <Icon name="lucide:upload" :size="16" />
        Importer Manabox
      </button>
      <button
        class="md:hidden inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 whitespace-nowrap cursor-pointer select-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
        @click="sheet = true"
      >
        <Icon name="lucide:filter" :size="13" />
        Filtres
      </button>
    </div>

    <!-- ── BODY ── -->
    <div class="flex gap-6 items-start">
      <!-- Sidebar filters (desktop) -->
      <div
        class="max-md:hidden flex-none w-[210px] sticky top-[86px] flex flex-col gap-3.5 h-[calc(100vh-106px)]"
      >
        <button
          class="inline-flex items-center gap-2 justify-center px-4 py-2.5 rounded-xl text-sm font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none w-full hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
          @click="openImport"
        >
          <Icon name="lucide:upload" :size="16" />
          Importer Manabox
        </button>
        <aside
          class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 flex-1 overflow-y-auto"
        >
          <CollectionFilters
            v-model:q="q"
            :active="active"
            :set-list="setList"
            :price-min="priceMin"
            :price-max="priceMax"
            @toggle="toggle"
          />
        </aside>
      </div>

      <!-- Main content -->
      <div class="flex-1 min-w-0">
        <!-- Header row -->
        <div class="flex items-center justify-between mb-3.5 min-h-[22px]">
          <span v-if="statsData" class="text-slate-400 dark:text-slate-500 text-sm">{{
            `${statsData.total_cards.toLocaleString('fr-FR')} cartes · ${statsData.unique_cards} uniques`
          }}</span>
          <div class="flex items-center gap-2.5">
            <SegToggle
              v-if="view === 'grid'"
              v-model="size"
              :options="sizeOptions"
              size="sm"
              class="max-md:hidden"
            />
            <button
              :class="[
                'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border transition-all duration-150 whitespace-nowrap cursor-pointer select-none',
                params.sort_dir === 'asc'
                  ? 'border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 bg-cyan-500/10 dark:bg-cyan-400/10'
                  : 'border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800',
              ]"
              @click="params.sort_dir = params.sort_dir === 'desc' ? 'asc' : 'desc'"
            >
              {{ params.sort_dir === 'asc' ? 'Prix croissant' : 'Prix décroissant' }}
              <Icon
                :name="params.sort_dir === 'asc' ? 'lucide:chevron-up' : 'lucide:chevron-down'"
                :size="13"
              />
            </button>
          </div>
        </div>

        <!-- Loading state (initial) -->
        <div
          v-if="pending && allCards.length === 0"
          class="flex items-center justify-center py-16 text-slate-400 dark:text-slate-500 text-sm font-mono"
        >
          <Icon name="lucide:loader-circle" :size="18" class="animate-spin mr-2.5" />
          Chargement…
        </div>

        <!-- Empty state -->
        <div
          v-else-if="!pending && allCards.length === 0"
          class="flex flex-col items-center justify-center py-20 gap-4 text-slate-400 dark:text-slate-500"
        >
          <Icon name="lucide:inbox" :size="48" class="opacity-40" />
          <p class="text-base font-mono text-center">Aucune carte dans la collection.</p>
          <button
            class="mt-1 flex items-center gap-2 px-4 py-2 rounded-xl bg-slate-100 dark:bg-zinc-800 hover:bg-slate-200 dark:hover:bg-zinc-700 text-sm font-mono transition-colors cursor-pointer"
            @click="openImport()"
          >
            <Icon name="lucide:upload" :size="14" />
            Importer depuis Manabox
          </button>
        </div>

        <!-- Grid view -->
        <template v-else-if="view === 'grid'">
          <div
            :class="[
              'grid max-md:[grid-template-columns:repeat(auto-fill,minmax(150px,1fr))] max-md:gap-3.5',
              size === 'sm'
                ? 'gap-3 [grid-template-columns:repeat(auto-fill,minmax(130px,1fr))]'
                : '',
              size === 'md'
                ? 'gap-4 [grid-template-columns:repeat(auto-fill,minmax(185px,1fr))]'
                : '',
              size === 'lg'
                ? 'gap-6 [grid-template-columns:repeat(auto-fill,minmax(340px,1fr))]'
                : '',
            ]"
          >
            <CardCell
              v-for="c in allCards"
              :key="c.scryfall_id"
              :scryfall-id="c.scryfall_id"
              :name="c.name"
              :qty="c.quantity"
              :purchased="c.purchase_price"
              :trend="c.price_guide?.trend ?? 0"
              deal="compare"
              :foil="c.foil"
              :size="size"
              @click="detail = c"
            />
          </div>

          <!-- Infinite scroll sentinel -->
          <div ref="sentinel" class="h-px" />
          <div
            v-if="pending && allCards.length > 0"
            class="flex items-center justify-center py-8 text-slate-400 dark:text-slate-500 text-sm font-mono"
          >
            <Icon name="lucide:loader-circle" :size="16" class="animate-spin mr-2" />
            Chargement…
          </div>
        </template>
      </div>
    </div>

    <!-- ── MOBILE FILTER SHEET ── -->
    <div
      v-if="sheet"
      class="fixed inset-0 z-[80] bg-black/60 backdrop-blur-sm animate-[fade_0.2s_ease]"
      @click="sheet = false"
    >
      <div
        class="fixed left-0 right-0 bottom-0 z-[81] max-h-[84vh] overflow-auto px-4 pt-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] bg-white dark:bg-zinc-900 border-t border-slate-300 dark:border-white/15 rounded-t-3xl shadow-2xl animate-[slideup_0.3s_cubic-bezier(0.3,1,0.4,1)]"
        @click.stop
      >
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-display font-semibold text-base tracking-tight">Filtres</h3>
          <button
            class="w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            @click="sheet = false"
          >
            <Icon name="lucide:x" :size="16" />
          </button>
        </div>
        <CollectionFilters
          :active="active"
          :set-list="setList"
          :price-min="priceMin"
          :price-max="priceMax"
          @toggle="toggle"
        />
        <button
          class="inline-flex items-center gap-2 justify-center px-4 py-2.5 rounded-xl text-sm font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none w-full mt-4 hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
          @click="sheet = false"
        >
          Voir les résultats
        </button>
      </div>
    </div>

    <!-- ── CARD DETAIL MODAL ── -->
    <div
      v-if="detail"
      class="fixed inset-0 z-[80] bg-black/60 backdrop-blur-sm animate-[fade_0.2s_ease] grid place-items-center p-5"
      @click="detail = null"
    >
      <div
        class="relative w-full max-w-[840px] p-0 overflow-hidden max-h-[calc(100dvh-40px)] bg-white dark:bg-zinc-900 border border-slate-300 dark:border-white/15 rounded-3xl shadow-2xl animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)] max-[720px]:max-w-[440px]"
        @click.stop
      >
        <!-- close -->
        <button
          class="absolute top-3.5 right-3.5 z-[5] w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
          @click="detail = null"
        >
          <Icon name="lucide:x" :size="16" />
        </button>

        <!-- body grid -->
        <div
          class="grid [grid-template-columns:minmax(300px,360px)_1fr] max-h-[calc(100dvh-40px)] overflow-y-auto max-[720px]:[grid-template-columns:1fr]"
        >
          <!-- art -->
          <div
            class="relative p-7 flex items-center justify-center border-r border-slate-200 dark:border-white/10 bg-slate-100 dark:bg-zinc-800 max-[720px]:border-r-0 max-[720px]:border-b max-[720px]:p-6"
          >
            <MtgCard
              :scryfall-id="detail?.scryfall_id"
              :name="detail?.name"
              class="w-full max-w-[300px] drop-shadow-2xl max-[720px]:max-w-[260px]"
            />
          </div>

          <!-- info -->
          <div class="px-6 py-7 flex flex-col gap-4 min-w-0">
            <!-- header -->
            <div>
              <h3 class="font-display font-semibold text-xl tracking-tight mb-1.5">
                {{ detail.name }}
              </h3>
              <span
                class="text-slate-400 dark:text-slate-500 text-sm inline-flex items-center gap-2 flex-wrap"
              >
                {{ detail.set_code.toUpperCase() }} · {{ detail.rarity_code }}
                <span
                  v-if="detail?.foil"
                  class="inline-flex items-center ml-2 text-2xs font-bold tracking-wide px-1.5 py-px rounded-full text-zinc-900 [background:linear-gradient(110deg,#ffd84d,#4dffd0,#4db4ff,#b85dff,#ff5db8)] [background-size:200%_100%] [animation:foilSlide_4s_linear_infinite]"
                >
                  ✦ Foil
                </span>
              </span>
            </div>

            <!-- stats -->
            <div class="grid grid-cols-3 gap-2.5">
              <div
                class="flex flex-col gap-1 px-3 py-3 border border-slate-200 dark:border-white/10 rounded-xl bg-white dark:bg-zinc-900"
              >
                <span
                  class="font-mono text-2xs tracking-widest uppercase text-slate-400 dark:text-slate-500"
                  >Quantité</span
                >
                <span class="font-mono text-lg font-bold tracking-tight"
                  >×{{ detail.quantity }}</span
                >
              </div>
              <div
                class="flex flex-col gap-1 px-3 py-3 border border-slate-200 dark:border-white/10 rounded-xl bg-white dark:bg-zinc-900"
              >
                <span
                  class="font-mono text-2xs tracking-widest uppercase text-slate-400 dark:text-slate-500"
                  >Prix unit.</span
                >
                <span class="font-mono text-lg font-bold tracking-tight">{{
                  formatPrice(detail.purchase_price)
                }}</span>
              </div>
              <div
                class="flex flex-col gap-1 px-3 py-3 border border-slate-200 dark:border-white/10 rounded-xl bg-white dark:bg-zinc-900"
              >
                <span
                  class="font-mono text-2xs tracking-widest uppercase text-slate-400 dark:text-slate-500"
                  >Total</span
                >
                <span
                  class="font-mono text-lg font-bold tracking-tight text-cyan-600 dark:text-cyan-400"
                  >{{ formatPrice(detail.quantity * detail.purchase_price) }}</span
                >
              </div>
            </div>

            <!-- market -->
            <div
              class="bg-black/20 border border-slate-200 dark:border-white/10 rounded-xl px-3.5 py-3"
            >
              <div class="flex items-center justify-between">
                <span
                  class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
                  >Marché · CardMarket</span
                >
                <span
                  :class="[
                    'font-mono text-xs',
                    detailDelta(detail) >= 0
                      ? 'text-cyan-600 dark:text-cyan-400'
                      : 'text-red-500 dark:text-red-400',
                  ]"
                >
                  {{ detailDelta(detail) >= 0 ? '▴' : '▾' }}
                  {{ Math.abs(detailDelta(detail)) }} %
                </span>
              </div>
              <div class="flex items-center gap-2.5 mt-2">
                <span class="font-mono text-xl font-bold">{{
                  formatPrice(detail.price_guide?.trend ?? 0)
                }}</span>
                <Sparkline
                  :data="
                    detailDelta(detail) >= 0
                      ? [40, 46, 42, 55, 52, 64, 60, 78]
                      : [78, 66, 70, 58, 60, 50, 52, 40]
                  "
                />
              </div>
            </div>

            <!-- actions -->
            <div class="flex flex-col gap-2 mt-auto">
              <button
                class="inline-flex items-center gap-2 justify-center px-4 py-2.5 rounded-xl text-sm font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none w-full hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
                @click="navigateTo('/find')"
              >
                <Icon name="lucide:user" :size="16" />
                Voir qui la possède
              </button>
              <button
                class="inline-flex items-center gap-2 justify-center py-2.5 px-4 rounded-xl text-sm font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none w-full hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
                @click="navigateTo('/trade')"
              >
                Proposer en échange
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- ── IMPORT MODAL ── -->
    <div
      v-if="importOpen"
      class="fixed inset-0 z-[80] bg-black/60 backdrop-blur-sm animate-[fade_0.2s_ease] grid place-items-center p-5"
      @click="importOpen = false"
    >
      <div
        class="w-full max-w-[480px] p-6 bg-white dark:bg-zinc-900 border border-slate-300 dark:border-white/15 rounded-3xl shadow-2xl animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)]"
        @click.stop
      >
        <div class="flex items-center justify-between mb-1">
          <h3 class="font-display font-semibold text-xl tracking-tight">Importer depuis Manabox</h3>
          <button
            class="w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            @click="importOpen = false"
          >
            <Icon name="lucide:x" :size="16" />
          </button>
        </div>
        <p class="text-slate-600 dark:text-slate-300 text-sm mt-0">
          Exporte ta collection en
          <span class="font-mono tracking-tight">.csv</span>
          depuis Manabox, puis dépose-la ici.
        </p>

        <!-- Step: drop zone -->
        <template v-if="importStep === 'drop'">
          <input
            ref="fileInputRef"
            type="file"
            accept=".csv"
            class="hidden"
            @change="onFileInputChange"
          />
          <div
            class="border-[1.5px] border-dashed rounded-xl transition-all duration-200 flex flex-col items-center gap-3 p-8 mt-2 cursor-pointer"
            :class="
              isDragging
                ? 'border-cyan-500/40 dark:border-cyan-400/40 bg-cyan-500/10 dark:bg-cyan-400/10'
                : 'border-slate-300 dark:border-white/15 bg-black/10 hover:border-cyan-500/40 dark:hover:border-cyan-400/40 hover:bg-cyan-500/10 dark:hover:bg-cyan-400/10'
            "
            @click="fileInputRef?.click()"
            @drop="onDrop"
            @dragover="onDragOver"
            @dragleave="onDragLeave"
          >
            <Icon
              :name="importLoading ? 'lucide:loader-2' : 'lucide:upload'"
              :size="30"
              class="text-cyan-600 dark:text-cyan-400"
              :class="{ 'animate-spin': importLoading }"
            />
            <div class="flex flex-col items-center gap-1">
              <span class="font-semibold">{{
                importLoading ? 'Import en cours…' : 'Glisse ton fichier .csv ici'
              }}</span>
              <span v-if="!importLoading" class="text-slate-400 dark:text-slate-500 text-xs"
                >ou clique pour parcourir</span
              >
            </div>
          </div>
          <p v-if="importError" class="text-red-400 text-sm mt-2.5 mb-0">
            {{ importError }}
          </p>
        </template>
      </div>
    </div>
  </div>
</template>

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
  <div class="mx-auto max-w-[1180px] px-5 pt-7 pb-10 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- ── VALUE BAR ── -->
    <div
      class="mb-4 rounded-2xl border border-cyan-500/30 bg-cyan-500/10 px-5 py-5 shadow-lg backdrop-blur-md dark:border-cyan-400/30 dark:bg-cyan-400/10"
    >
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div class="flex flex-col gap-1">
          <span
            class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
            >Valeur totale · CardMarket</span
          >
          <span
            class="font-mono text-[clamp(30px,5vw,38px)] font-bold tracking-tight whitespace-nowrap"
            >€ 4 218,60</span
          >
          <span class="font-mono text-sm text-cyan-600 dark:text-cyan-400"
            >▴ €86,20 · +2,1 % (30 j)</span
          >
        </div>
        <div class="flex flex-col items-end gap-2">
          <div class="flex items-center gap-2">
            <span
              class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase max-md:hidden dark:text-slate-500"
              >graphe</span
            >
            <SegToggle v-model="graph" :options="graphOptions" size="sm" />
          </div>
          <Sparkline v-if="graph === 'compact'" :data="[40, 52, 46, 60, 54, 70, 64, 78, 72, 90]" />
          <span
            class="text-2xs flex items-center gap-1.5 font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase max-md:hidden dark:text-slate-500"
          >
            <Icon name="lucide:refresh-cw" :size="11" /> synchro auto · il y a 2 h
          </span>
        </div>
      </div>

      <div v-if="graph === 'expanded'" class="mt-4 flex flex-col gap-2.5">
        <div class="flex gap-2">
          <button
            v-for="r in ['30 j', '3 m', '1 an', 'Max']"
            :key="r"
            :class="[
              'inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-all duration-150 select-none',
              graphRange === r
                ? 'border-cyan-500/30 bg-cyan-500/10 text-cyan-700 dark:border-cyan-400/30 dark:bg-cyan-400/10 dark:text-cyan-300'
                : 'border-slate-200 bg-slate-100 text-slate-600 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100',
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
    <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
      <button
        class="inline-flex items-center justify-center gap-2 rounded-xl border border-transparent bg-cyan-500 px-4 py-2.5 text-sm leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 md:hidden dark:bg-cyan-400 dark:hover:bg-cyan-300"
        @click="openImport"
      >
        <Icon name="lucide:upload" :size="16" />
        Importer Manabox
      </button>
      <button
        class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border border-slate-200 bg-slate-100 px-3 py-1.5 text-xs font-medium whitespace-nowrap text-slate-600 transition-all duration-150 select-none hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 md:hidden dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100"
        @click="sheet = true"
      >
        <Icon name="lucide:filter" :size="13" />
        Filtres
      </button>
    </div>

    <!-- ── BODY ── -->
    <div class="flex items-start gap-6">
      <!-- Sidebar filters (desktop) -->
      <div class="sticky top-[86px] flex w-[210px] flex-none flex-col gap-3.5 max-md:hidden">
        <button
          class="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-transparent bg-cyan-500 px-4 py-2.5 text-sm leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
          @click="openImport"
        >
          <Icon name="lucide:upload" :size="16" />
          Importer Manabox
        </button>
        <aside
          class="rounded-2xl border border-slate-200 bg-white/60 p-4 shadow-lg backdrop-blur-md dark:border-white/10 dark:bg-zinc-900/60"
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
      <div class="min-w-0 flex-1">
        <!-- Header row -->
        <div class="mb-3.5 flex min-h-[22px] items-center justify-between">
          <span v-if="statsData" class="text-sm text-slate-400 dark:text-slate-500">{{
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
                'inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-all duration-150 select-none',
                params.sort_dir === 'asc'
                  ? 'border-cyan-500/30 bg-cyan-500/10 text-cyan-700 dark:border-cyan-400/30 dark:bg-cyan-400/10 dark:text-cyan-300'
                  : 'border-slate-200 bg-slate-100 text-slate-600 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100',
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
          class="flex items-center justify-center py-16 font-mono text-sm text-slate-400 dark:text-slate-500"
        >
          <Icon name="lucide:loader-circle" :size="18" class="mr-2.5 animate-spin" />
          Chargement…
        </div>

        <!-- Empty state -->
        <div
          v-else-if="!pending && allCards.length === 0"
          class="flex flex-col items-center justify-center gap-4 py-20 text-slate-400 dark:text-slate-500"
        >
          <Icon name="lucide:inbox" :size="48" class="opacity-40" />
          <p class="text-center font-mono text-base">Aucune carte dans la collection.</p>
          <button
            class="mt-1 flex cursor-pointer items-center gap-2 rounded-xl bg-slate-100 px-4 py-2 font-mono text-sm transition-colors hover:bg-slate-200 dark:bg-zinc-800 dark:hover:bg-zinc-700"
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
                ? '[grid-template-columns:repeat(auto-fill,minmax(130px,1fr))] gap-3'
                : '',
              size === 'md'
                ? '[grid-template-columns:repeat(auto-fill,minmax(185px,1fr))] gap-4'
                : '',
              size === 'lg'
                ? '[grid-template-columns:repeat(auto-fill,minmax(340px,1fr))] gap-6'
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
            class="flex items-center justify-center py-8 font-mono text-sm text-slate-400 dark:text-slate-500"
          >
            <Icon name="lucide:loader-circle" :size="16" class="mr-2 animate-spin" />
            Chargement…
          </div>
        </template>
      </div>
    </div>

    <!-- ── MOBILE FILTER SHEET ── -->
    <div
      v-if="sheet"
      class="fixed inset-0 z-[80] animate-[fade_0.2s_ease] bg-black/60 backdrop-blur-sm"
      @click="sheet = false"
    >
      <div
        class="fixed right-0 bottom-0 left-0 z-[81] max-h-[84vh] animate-[slideup_0.3s_cubic-bezier(0.3,1,0.4,1)] overflow-auto rounded-t-3xl border-t border-slate-300 bg-white px-4 pt-5 pb-[calc(1.25rem+env(safe-area-inset-bottom))] shadow-2xl dark:border-white/15 dark:bg-zinc-900"
        @click.stop
      >
        <div class="mb-4 flex items-center justify-between">
          <h3 class="font-display text-base font-semibold tracking-tight">Filtres</h3>
          <button
            class="grid h-9 w-9 place-items-center rounded-lg border border-slate-200 bg-slate-100 text-slate-600 transition-all duration-150 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100"
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
          class="mt-4 inline-flex w-full items-center justify-center gap-2 rounded-xl border border-transparent bg-cyan-500 px-4 py-2.5 text-sm leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
          @click="sheet = false"
        >
          Voir les résultats
        </button>
      </div>
    </div>

    <!-- ── CARD DETAIL MODAL ── -->
    <div
      v-if="detail"
      class="fixed inset-0 z-[80] grid animate-[fade_0.2s_ease] place-items-center bg-black/60 p-5 backdrop-blur-sm"
      @click="detail = null"
    >
      <div
        class="relative max-h-[calc(100dvh-40px)] w-full max-w-[840px] animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)] overflow-hidden rounded-3xl border border-slate-300 p-0 shadow-2xl max-[720px]:max-w-[440px] dark:border-white/15"
        @click.stop
      >
        <!-- close -->
        <button
          class="absolute top-3.5 right-3.5 z-[5] grid h-9 w-9 place-items-center rounded-lg border border-slate-200 bg-slate-100 text-slate-600 transition-all duration-150 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100"
          @click="detail = null"
        >
          <Icon name="lucide:x" :size="16" />
        </button>

        <!-- body grid -->
        <div
          class="grid max-h-[calc(100dvh-40px)] [grid-template-columns:minmax(300px,360px)_1fr] overflow-y-auto max-[720px]:[grid-template-columns:1fr]"
        >
          <!-- art -->
          <div
            class="relative flex items-center justify-center border-r border-slate-200/60 bg-white/40 p-7 backdrop-blur-md max-[720px]:border-r-0 max-[720px]:border-b max-[720px]:p-6 dark:border-white/10 dark:bg-zinc-900/40"
          >
            <MtgCard
              :scryfall-id="detail?.scryfall_id"
              :name="detail?.name"
              class="w-full max-w-[300px] drop-shadow-2xl max-[720px]:max-w-[260px]"
            />
          </div>

          <!-- info -->
          <div class="flex min-w-0 flex-col gap-4 bg-white px-6 py-7 dark:bg-zinc-900">
            <!-- header -->
            <div>
              <h3 class="font-display mb-1.5 text-xl font-semibold tracking-tight">
                {{ detail.name }}
              </h3>
              <span
                class="inline-flex flex-wrap items-center gap-2 text-sm text-slate-400 dark:text-slate-500"
              >
                {{ detail.set_code.toUpperCase() }} · {{ detail.rarity_code }}
                <span
                  v-if="detail?.foil"
                  class="text-2xs ml-2 inline-flex [animation:foilSlide_4s_linear_infinite] items-center rounded-full [background-size:200%_100%] px-1.5 py-px font-bold tracking-wide text-zinc-900 [background:linear-gradient(110deg,#ffd84d,#4dffd0,#4db4ff,#b85dff,#ff5db8)]"
                >
                  ✦ Foil
                </span>
              </span>
            </div>

            <!-- stats -->
            <div class="grid grid-cols-3 gap-2.5">
              <div
                class="flex flex-col gap-1 rounded-xl border border-slate-200 bg-white px-3 py-3 dark:border-white/10 dark:bg-zinc-900"
              >
                <span
                  class="text-2xs font-mono tracking-widest text-slate-400 uppercase dark:text-slate-500"
                  >Quantité</span
                >
                <span class="font-mono text-lg font-bold tracking-tight"
                  >×{{ detail.quantity }}</span
                >
              </div>
              <div
                class="flex flex-col gap-1 rounded-xl border border-slate-200 bg-white px-3 py-3 dark:border-white/10 dark:bg-zinc-900"
              >
                <span
                  class="text-2xs font-mono tracking-widest text-slate-400 uppercase dark:text-slate-500"
                  >Prix unit.</span
                >
                <span class="font-mono text-lg font-bold tracking-tight">{{
                  formatPrice(detail.purchase_price)
                }}</span>
              </div>
              <div
                class="flex flex-col gap-1 rounded-xl border border-slate-200 bg-white px-3 py-3 dark:border-white/10 dark:bg-zinc-900"
              >
                <span
                  class="text-2xs font-mono tracking-widest text-slate-400 uppercase dark:text-slate-500"
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
              class="rounded-xl border border-slate-200 bg-black/20 px-3.5 py-3 dark:border-white/10"
            >
              <div class="flex items-center justify-between">
                <span
                  class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
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
              <div class="mt-2 flex items-center gap-2.5">
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
            <div class="mt-auto flex flex-col gap-2">
              <button
                class="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-transparent bg-cyan-500 px-4 py-2.5 text-sm leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
                @click="navigateTo('/find')"
              >
                <Icon name="lucide:user" :size="16" />
                Voir qui la possède
              </button>
              <button
                class="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-slate-200 bg-transparent px-4 py-2.5 text-sm leading-none font-semibold whitespace-nowrap text-slate-600 transition-all duration-150 hover:-translate-y-px hover:border-slate-300 hover:bg-slate-100 hover:text-slate-800 active:translate-y-0 dark:border-white/10 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-white/5 dark:hover:text-slate-100"
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
      class="fixed inset-0 z-[80] grid animate-[fade_0.2s_ease] place-items-center bg-black/60 p-5 backdrop-blur-sm"
      @click="importOpen = false"
    >
      <div
        class="w-full max-w-[480px] animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)] rounded-3xl border border-slate-300 bg-white p-6 shadow-2xl dark:border-white/15 dark:bg-zinc-900"
        @click.stop
      >
        <div class="mb-1 flex items-center justify-between">
          <h3 class="font-display text-xl font-semibold tracking-tight">Importer depuis Manabox</h3>
          <button
            class="grid h-9 w-9 place-items-center rounded-lg border border-slate-200 bg-slate-100 text-slate-600 transition-all duration-150 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 hover:ring-4 hover:ring-cyan-500/10 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100"
            @click="importOpen = false"
          >
            <Icon name="lucide:x" :size="16" />
          </button>
        </div>
        <p class="mt-0 text-sm text-slate-600 dark:text-slate-300">
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
            class="mt-2 flex cursor-pointer flex-col items-center gap-3 rounded-xl border-[1.5px] border-dashed p-8 transition-all duration-200"
            :class="
              isDragging
                ? 'border-cyan-500/40 bg-cyan-500/10 dark:border-cyan-400/40 dark:bg-cyan-400/10'
                : 'border-slate-300 bg-black/10 hover:border-cyan-500/40 hover:bg-cyan-500/10 dark:border-white/15 dark:hover:border-cyan-400/40 dark:hover:bg-cyan-400/10'
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
              <span v-if="!importLoading" class="text-xs text-slate-400 dark:text-slate-500"
                >ou clique pour parcourir</span
              >
            </div>
          </div>
          <p v-if="importError" class="mt-2.5 mb-0 text-sm text-red-400">
            {{ importError }}
          </p>
        </template>
      </div>
    </div>
  </div>
</template>

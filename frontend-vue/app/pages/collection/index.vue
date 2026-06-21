<script setup lang="ts">
import type { SortBy } from '~/bindings/SortBy';
import type { SortDir } from '~/bindings/SortDir';
import type { CollectionCard } from '~/bindings/CollectionCard';

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

const { getCollection, importCards } = useCardsService();

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

const toggle = (k: 'rar' | 'sets', v: string) => {
  const arr = active.value[k];
  active.value[k] = arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v];
};

const setCounts = computed(() => {
  const c: Record<string, number> = {};
  cards.value.forEach((card) => {
    c[card.set] = (c[card.set] || 0) + 1;
  });
  return c;
});
const setList = computed(() =>
  Object.keys(setCounts.value).sort(
    (a, b) => (setCounts.value[b] ?? 0) - (setCounts.value[a] ?? 0),
  ),
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
  <div
    class="max-w-[var(--maxw)] mx-auto px-[22px] pt-[28px] pb-[40px] max-[860px]:px-[16px] max-[860px]:pt-[20px] max-[860px]:pb-[30px]"
  >
    <!-- ── VALUE BAR ── -->
    <div
      class="bg-[var(--cyan-fill)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--cyan-line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] px-[22px] py-[20px] mb-[18px]"
    >
      <div class="flex items-center justify-between flex-wrap gap-[18px]">
        <div class="flex flex-col gap-[3px]">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
            >Valeur totale · CardMarket</span
          >
          <span
            class="[font-family:var(--font-mono)] font-bold tracking-[-0.02em] whitespace-nowrap text-[clamp(30px,5vw,38px)]"
            >€ 4 218,60</span
          >
          <span class="text-[var(--cyan)] text-[13.5px] [font-family:var(--font-mono)]"
            >▴ €86,20 · +2,1 % (30 j)</span
          >
        </div>
        <div class="flex flex-col gap-[8px] items-end">
          <div class="flex items-center gap-[8px]">
            <span
              class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap max-[860px]:hidden"
              >graphe</span
            >
            <SegToggle v-model="graph" :options="graphOptions" size="sm" />
          </div>
          <Sparkline v-if="graph === 'compact'" :data="[40, 52, 46, 60, 54, 70, 64, 78, 72, 90]" />
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap max-[860px]:hidden flex items-center gap-[5px]"
          >
            <Icon name="lucide:refresh-cw" :size="11" /> synchro auto · il y a 2 h
          </span>
        </div>
      </div>

      <div v-if="graph === 'expanded'" class="flex flex-col gap-[10px] mt-[16px]">
        <div class="flex gap-[7px]">
          <button
            v-for="r in ['30 j', '3 m', '1 an', 'Max']"
            :key="r"
            :class="[
              'inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid whitespace-nowrap cursor-pointer select-none transition-all duration-[150ms] ease',
              graphRange === r
                ? 'border-[var(--cyan-line)] text-[var(--cyan-ink)] bg-[var(--cyan-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)]'
                : 'border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]',
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
          :value-fmt="(v) => '€' + v.toLocaleString('fr-FR')"
        />
      </div>
    </div>

    <!-- ── CONTROLS ── -->
    <div class="flex items-center justify-between flex-wrap gap-[12px] mb-[18px]">
      <button
        class="min-[861px]:hidden inline-flex items-center gap-[8px] justify-center px-[15px] py-[9px] rounded-[10px] text-[13.5px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
        @click="openImport"
      >
        <Icon name="lucide:upload" :size="16" />
        Importer Manabox
      </button>
      <button
        class="min-[861px]:hidden inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[150ms] ease whitespace-nowrap cursor-pointer select-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
        @click="sheet = true"
      >
        <Icon name="lucide:filter" :size="13" />
        Filtres
      </button>
    </div>

    <!-- ── BODY ── -->
    <div class="flex gap-[22px] items-start">
      <!-- Sidebar filters (desktop) -->
      <div
        class="max-[860px]:hidden flex-none w-[210px] sticky top-[86px] flex flex-col gap-[14px] h-[calc(100vh-106px)]"
      >
        <button
          class="inline-flex items-center gap-[8px] justify-center px-[15px] py-[9px] rounded-[10px] text-[13.5px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none w-full hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
          @click="openImport"
        >
          <Icon name="lucide:upload" :size="16" />
          Importer Manabox
        </button>
        <aside
          class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] flex-1 overflow-y-auto"
        >
          <CollectionFilters
            v-model:q="q"
            :active="active"
            :set-counts="setCounts"
            :set-list="setList"
            @toggle="toggle"
          />
        </aside>
      </div>

      <!-- Main content -->
      <div class="flex-1 min-w-0">
        <!-- Header row -->
        <div class="flex items-center justify-between mb-[14px]">
          <span v-if="collectionData" class="text-[var(--ink-3)] text-[13px]">{{
            `1 248 cartes · ${collectionData.total} uniques`
          }}</span>
          <div class="flex items-center gap-[10px]">
            <SegToggle
              v-if="view === 'grid'"
              v-model="size"
              :options="sizeOptions"
              size="sm"
              class="max-[860px]:hidden"
            />
            <button
              :class="[
                'inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid transition-all duration-[150ms] ease whitespace-nowrap cursor-pointer select-none',
                params.sort_dir === 'asc'
                  ? 'border-[var(--cyan-line)] text-[var(--cyan-ink)] bg-[var(--cyan-fill)]'
                  : 'border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]',
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
          class="flex items-center justify-center py-[60px] text-[var(--ink-3)] text-[13px] [font-family:var(--font-mono)]"
        >
          <Icon name="lucide:loader-circle" :size="18" class="animate-spin mr-[10px]" />
          Chargement…
        </div>

        <!-- Empty state -->
        <div
          v-else-if="!pending && allCards.length === 0"
          class="flex flex-col items-center justify-center py-[80px] gap-[16px] text-[var(--ink-3)]"
        >
          <Icon name="lucide:inbox" :size="48" class="opacity-40" />
          <p class="text-[15px] [font-family:var(--font-mono)] text-center">
            Aucune carte dans la collection.
          </p>
          <button
            class="mt-[4px] flex items-center gap-[8px] px-[16px] py-[8px] rounded-[var(--r-md)] bg-[var(--surface-2)] hover:bg-[var(--surface-3)] text-[13px] [font-family:var(--font-mono)] transition-colors cursor-pointer"
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
              'grid max-[860px]:[grid-template-columns:repeat(auto-fill,minmax(150px,1fr))] max-[860px]:gap-[14px]',
              size === 'sm'
                ? 'gap-[13px] [grid-template-columns:repeat(auto-fill,minmax(130px,1fr))]'
                : '',
              size === 'md'
                ? 'gap-[18px] [grid-template-columns:repeat(auto-fill,minmax(185px,1fr))]'
                : '',
              size === 'lg'
                ? 'gap-[24px] [grid-template-columns:repeat(auto-fill,minmax(340px,1fr))]'
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
              :size="size"
            />
          </div>

          <!-- Infinite scroll sentinel -->
          <div ref="sentinel" class="h-px" />
          <div
            v-if="pending && allCards.length > 0"
            class="flex items-center justify-center py-[32px] text-[var(--ink-3)] text-[13px] [font-family:var(--font-mono)]"
          >
            <Icon name="lucide:loader-circle" :size="16" class="animate-spin mr-[8px]" />
            Chargement…
          </div>
        </template>
      </div>
    </div>

    <!-- ── MOBILE FILTER SHEET ── -->
    <div
      v-if="sheet"
      class="fixed inset-0 z-[80] bg-[color-mix(in_srgb,black_58%,transparent)] [backdrop-filter:blur(4px)] [-webkit-backdrop-filter:blur(4px)] animate-[fade_0.2s_ease]"
      @click="sheet = false"
    >
      <div
        class="fixed left-0 right-0 bottom-0 z-[81] max-h-[84vh] overflow-auto px-[18px] pt-[20px] pb-[calc(20px+env(safe-area-inset-bottom))] bg-[var(--surface)] border-t border-solid border-[var(--line-2)] rounded-t-[var(--r-xl)] shadow-[0_-20px_60px_-20px_rgba(0,0,0,1)] animate-[slideup_0.3s_cubic-bezier(0.3,1,0.4,1)]"
        @click.stop
      >
        <div class="flex items-center justify-between mb-[16px]">
          <h3
            class="[font-family:var(--font-display)] font-semibold text-[16px] tracking-[-0.01em]"
          >
            Filtres
          </h3>
          <button
            class="w-[34px] h-[34px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            @click="sheet = false"
          >
            <Icon name="lucide:x" :size="16" />
          </button>
        </div>
        <CollectionFilters
          :active="active"
          :set-counts="setCounts"
          :set-list="setList"
          @toggle="toggle"
        />
        <button
          class="inline-flex items-center gap-[8px] justify-center px-[15px] py-[9px] rounded-[10px] text-[13.5px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none w-full mt-[18px] hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
          @click="sheet = false"
        >
          Voir les résultats
        </button>
      </div>
    </div>

    <!-- ── IMPORT MODAL ── -->
    <div
      v-if="importOpen"
      class="fixed inset-0 z-[80] bg-[color-mix(in_srgb,black_58%,transparent)] [backdrop-filter:blur(4px)] [-webkit-backdrop-filter:blur(4px)] animate-[fade_0.2s_ease] grid place-items-center p-[20px]"
      @click="importOpen = false"
    >
      <div
        class="w-full max-w-[480px] p-[22px] bg-[var(--surface)] border border-solid border-[var(--line-2)] rounded-[var(--r-xl)] shadow-[0_30px_70px_-30px_rgba(0,0,0,1)] animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)]"
        @click.stop
      >
        <div class="flex items-center justify-between mb-[4px]">
          <h3
            class="[font-family:var(--font-display)] font-semibold text-[20px] tracking-[-0.015em]"
          >
            Importer depuis Manabox
          </h3>
          <button
            class="w-[34px] h-[34px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            @click="importOpen = false"
          >
            <Icon name="lucide:x" :size="16" />
          </button>
        </div>
        <p class="text-[var(--ink-2)] text-[13.5px] mt-0">
          Exporte ta collection en
          <span
            class="[font-family:var(--font-mono)] [font-feature-settings:'tnum'_1,'ss01'_1] tracking-[-0.01em]"
            >.csv</span
          >
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
            class="[border-width:1.5px] border-dashed rounded-[14px] transition-all duration-200 flex flex-col items-center gap-[12px] p-[34px] mt-[8px] cursor-pointer"
            :class="
              isDragging
                ? 'border-[var(--cyan-line)] bg-[var(--cyan-fill)]'
                : 'border-[var(--line-2)] bg-[color-mix(in_srgb,black_16%,transparent)] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]'
            "
            @click="fileInputRef?.click()"
            @drop="onDrop"
            @dragover="onDragOver"
            @dragleave="onDragLeave"
          >
            <Icon
              :name="importLoading ? 'lucide:loader-2' : 'lucide:upload'"
              :size="30"
              class="text-[var(--cyan)]"
              :class="{ 'animate-spin': importLoading }"
            />
            <div class="flex flex-col items-center gap-[3px]">
              <span class="font-semibold">{{
                importLoading ? 'Import en cours…' : 'Glisse ton fichier .csv ici'
              }}</span>
              <span v-if="!importLoading" class="text-[var(--ink-3)] text-[12.5px]"
                >ou clique pour parcourir</span
              >
            </div>
          </div>
          <p v-if="importError" class="text-[var(--red,#f87171)] text-[13px] mt-[10px] mb-0">
            {{ importError }}
          </p>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CollectionCard } from '~/bindings/CollectionCard';
import type { PriceHistoryEntry } from '~/bindings/PriceHistoryEntry';
import type { SortBy } from '~/bindings/SortBy';
import type { SortDir } from '~/bindings/SortDir';

const { getCollection, getCollectionStats, importCards, getPriceHistory, getCardPriceHistory } =
  useCardsService();

const RARITY_CODES: Record<string, string> = {
  Mythique: 'M',
  Rare: 'R',
  Unco: 'U',
  Commune: 'C',
};

const q = ref('');
const qDebounced = refDebounced(q, 200);

const params = ref({
  sort_by: 'trend' as SortBy,
  sort_dir: 'desc' as SortDir,
  page: 0,
  page_size: 20,
  q: '',
  rarity: undefined as string | undefined,
  sets: undefined as string | undefined,
  price_min: undefined as number | undefined,
  price_max: undefined as number | undefined,
  owned: true,
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

const view = ref<'grid' | 'list'>('grid');
const size = ref<'sm' | 'md' | 'lg'>('md');
const graph = ref<'compact' | 'expanded'>('compact');
// Detail view is desktop-only — the extra KPIs/axis/range chips don't fit a small screen.
const isDesktop = useMediaQuery('(min-width: 768px)');
const showDetail = computed(() => graph.value === 'expanded' && isDesktop.value);
const graphRange = ref('30 j');

const dayLabelFormatter = new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short' });

const toIsoDate = (d: Date) =>
  `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;

const dateRangeFor = (range: string) => {
  const end = new Date();
  const start = new Date(end);
  if (range === '3 m') start.setMonth(start.getMonth() - 3);
  else if (range === '1 an') start.setFullYear(start.getFullYear() - 1);
  else if (range === 'Max') start.setFullYear(start.getFullYear() - 10);
  else start.setDate(start.getDate() - 29);
  return { start_date: toIsoDate(start), end_date: toIsoDate(end) };
};

const historyParams = ref(dateRangeFor(graphRange.value));
const {
  data: priceHistoryData,
  pending: priceHistoryPending,
  refresh: refreshPriceHistory,
} = await getPriceHistory(historyParams);

watch(graphRange, (range) => {
  historyParams.value = dateRangeFor(range);
  refreshPriceHistory();
});

const toEnvelopeData = (entries: PriceHistoryEntry[]) =>
  entries.map((e) => {
    const [year, month, day] = e.date.split('-').map(Number);
    return {
      low: e.low / 100,
      avg: e.avg / 100,
      trend: e.trend / 100,
      label: dayLabelFormatter.format(new Date(year!, month! - 1, day)),
    };
  });

const computeVariation = (entries: PriceHistoryEntry[]) => {
  if (entries.length < 2) return { pct: 0, positive: true };
  const first = entries[0]!.trend;
  const last = entries[entries.length - 1]!.trend;
  const pct = first !== 0 ? ((last - first) / first) * 100 : 0;
  return { pct, positive: pct >= 0 };
};

const envelopeData = computed(() => toEnvelopeData(priceHistoryData.value ?? []));
const hasEnoughHistory = computed(() => envelopeData.value.length >= 2);

const totalValueCents = computed(() => {
  const entries = priceHistoryData.value;
  return entries && entries.length > 0 ? entries[entries.length - 1]!.trend : 0;
});

const variation = computed(() => computeVariation(priceHistoryData.value ?? []));

const sheet = ref(false);
const importOpen = ref(false);
const importStep = ref<'drop'>('drop');
const importLoading = ref(false);
const importError = ref<string | null>(null);
const isDragging = ref(false);
const fileInputRef = ref<HTMLInputElement | null>(null);
const active = ref({ rar: [] as string[], sets: [] as string[] });
const detail = ref<CollectionCard | null>(null);

const cardHistoryData = ref<PriceHistoryEntry[]>([]);
const cardHistoryPending = ref(false);

watch(detail, async (card) => {
  cardHistoryData.value = [];
  if (!card) return;
  cardHistoryPending.value = true;
  try {
    cardHistoryData.value = await getCardPriceHistory(card.scryfall_id);
  } finally {
    cardHistoryPending.value = false;
  }
});

const cardEnvelopeData = computed(() => toEnvelopeData(cardHistoryData.value));
const cardHasEnoughHistory = computed(() => cardEnvelopeData.value.length >= 2);
const cardVariation = computed(() => computeVariation(cardHistoryData.value));

const toggle = (k: 'rar' | 'sets', v: string) => {
  const arr = active.value[k];
  active.value[k] = arr.includes(v) ? arr.filter((x) => x !== v) : [...arr, v];
};

watch(
  () => [active.value.rar, active.value.sets],
  () => {
    params.value.rarity = active.value.rar.length
      ? active.value.rar.map((r) => RARITY_CODES[r]).join(',')
      : undefined;
    params.value.sets = active.value.sets.length ? active.value.sets.join(',') : undefined;
    allCards.value = [];
    params.value.page = 0;
    refresh();
  },
  { deep: true },
);

const onPriceChange = useDebounceFn((lo: number, hi: number) => {
  params.value.price_min = lo > 0 ? lo * 100 : undefined;
  params.value.price_max = hi < priceMax.value ? hi * 100 : undefined;
  allCards.value = [];
  params.value.page = 0;
  refresh();
}, 300);

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
    const err = e as { data?: { error?: string } };
    importError.value = err?.data?.error ?? "Erreur lors de l'import";
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
    <!-- ── VALUE BAR — immersive (compact) ⇄ framed + scales (detail) ── -->
    <div
      :class="[
        'relative mb-4 overflow-hidden rounded-2xl border border-slate-200 bg-white/60 shadow-lg backdrop-blur-md transition-[height] duration-[450ms] ease-[cubic-bezier(0.4,0,0.1,1)] dark:border-white/10 dark:bg-zinc-900/60',
        showDetail ? 'h-[220px] md:h-[396px]' : 'h-[188px] max-md:h-[220px]',
      ]"
    >
      <!-- graph background -->
      <div
        :class="[
          'absolute transition-[inset] duration-[450ms] ease-[cubic-bezier(0.4,0,0.1,1)]',
          showDetail ? 'inset-0 md:inset-x-4 md:top-[100px] md:bottom-[52px]' : 'inset-0',
        ]"
      >
        <EnvelopeGraph v-if="hasEnoughHistory" :data="envelopeData" :detail="showDetail" />
        <div
          v-else
          class="text-2xs flex h-full items-center justify-center font-mono tracking-wide text-slate-400 uppercase dark:text-slate-500"
        >
          {{ priceHistoryPending ? 'Chargement…' : "Pas encore assez d'historique" }}
        </div>
      </div>

      <!-- scrim (fades out in detail mode) -->
      <div
        :class="[
          'pointer-events-none absolute inset-0 z-[1] bg-gradient-to-b from-white/95 via-white/55 to-transparent transition-opacity duration-300 dark:from-zinc-900/95 dark:via-zinc-900/55',
          showDetail ? 'opacity-0' : 'opacity-100',
        ]"
      />

      <!-- top: KPIs + controls -->
      <div
        class="absolute inset-x-0 top-0 z-[3] flex flex-col items-start justify-between gap-3 px-5 py-5 max-md:gap-2.5 md:flex-row md:flex-wrap"
      >
        <div class="flex items-stretch">
          <div class="flex flex-col gap-1">
            <span
              class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
              >Valeur totale · CardMarket</span
            >
            <span
              class="font-mono text-[clamp(28px,3.4vw,36px)] font-bold tracking-tight whitespace-nowrap"
              >{{ formatPrice(totalValueCents) }}</span
            >
          </div>
          <div
            class="ml-5 hidden flex-col justify-center gap-1 border-l border-slate-200 pl-5 md:flex dark:border-white/10"
          >
            <span
              class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
              >Variation · {{ graphRange }}</span
            >
            <span
              :class="[
                'font-mono text-lg font-semibold',
                variation.positive
                  ? 'text-cyan-600 dark:text-cyan-400'
                  : 'text-red-500 dark:text-red-400',
              ]"
              >{{ variation.positive ? '▴' : '▾' }} {{ variation.positive ? '+' : '−'
              }}{{
                Math.abs(variation.pct).toLocaleString('fr-FR', {
                  minimumFractionDigits: 1,
                  maximumFractionDigits: 1,
                })
              }}
              %</span
            >
          </div>
          <div
            v-if="statsData"
            class="ml-5 hidden flex-col justify-center gap-1 border-l border-slate-200 pl-5 md:flex dark:border-white/10"
          >
            <span
              class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
              >Cartes</span
            >
            <span class="font-mono text-lg font-semibold">{{
              statsData.total_cards.toLocaleString('fr-FR')
            }}</span>
          </div>
          <div
            v-if="statsData"
            class="ml-5 hidden flex-col justify-center gap-1 border-l border-slate-200 pl-5 md:flex dark:border-white/10"
          >
            <span
              class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
              >Uniques</span
            >
            <span class="font-mono text-lg font-semibold">{{ statsData.unique_cards }}</span>
          </div>
        </div>

        <div class="hidden items-center gap-2.5 md:flex">
          <div v-if="showDetail" class="flex gap-1.5">
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
          <SegToggle v-model="graph" :options="graphOptions" size="sm" />
        </div>
      </div>

      <!-- bottom-left: sync status (hidden in detail, hidden on mobile — no room next to legend) -->
      <span
        :class="[
          'text-2xs absolute bottom-3.5 left-5 z-[3] hidden items-center gap-1.5 font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase transition-opacity duration-300 sm:flex dark:text-slate-500',
          showDetail ? 'pointer-events-none opacity-0' : 'opacity-100',
        ]"
      >
        <Icon name="lucide:refresh-cw" :size="11" /> synchro auto · il y a 2 h
      </span>

      <!-- bottom-right: legend -->
      <div class="absolute right-5 bottom-3.5 z-[3] flex gap-4">
        <span
          class="text-2xs flex items-center gap-1.5 font-mono whitespace-nowrap text-slate-500 dark:text-slate-300"
        >
          <span
            class="h-2.5 w-[15px] rounded-[3px] border border-cyan-500/40 bg-cyan-500/25 dark:border-cyan-400/40 dark:bg-cyan-400/25"
          />
          low → avg
        </span>
        <span
          class="text-2xs flex items-center gap-1.5 font-mono whitespace-nowrap text-slate-500 dark:text-slate-300"
        >
          <span class="h-0 w-[17px] border-t-[2.5px] border-cyan-500 dark:border-cyan-400" />
          trend
        </span>
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
            @price-change="onPriceChange"
          />
        </aside>
      </div>

      <!-- Main content -->
      <div class="min-w-0 flex-1">
        <!-- Header row -->
        <div class="mb-3.5 flex min-h-[22px] items-center justify-between">
          <span v-if="statsData" class="text-sm text-slate-400 dark:text-slate-500" />
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
              :the-gatherer-id="c.the_gatherer_id ?? undefined"
              :name="c.name"
              :qty="c.collection_entry?.quantity ?? 0"
              :purchased="c.collection_entry?.purchase_price ?? 0"
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
          @price-change="onPriceChange"
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
      class="fixed inset-0 z-[80] grid animate-[fade_0.2s_ease] place-items-center bg-black/60 px-5 pt-[calc(1.25rem+env(safe-area-inset-top))] pb-5 backdrop-blur-sm"
      @click="detail = null"
    >
      <div
        class="relative max-h-[calc(100dvh-40px-env(safe-area-inset-top))] w-full max-w-[840px] animate-[pop_0.26s_cubic-bezier(0.3,1.2,0.4,1)] overflow-hidden rounded-3xl border border-slate-300 p-0 shadow-2xl max-[720px]:max-w-[440px] dark:border-white/15"
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
          class="grid max-h-[calc(100dvh-40px-env(safe-area-inset-top))] [grid-template-columns:minmax(300px,360px)_1fr] overflow-y-auto max-[720px]:[grid-template-columns:1fr]"
        >
          <!-- art -->
          <div
            class="relative flex items-center justify-center border-r border-slate-200/60 bg-white/40 p-7 backdrop-blur-md max-[720px]:border-r-0 max-[720px]:border-b max-[720px]:p-6 dark:border-white/10 dark:bg-zinc-900/40"
          >
            <MtgCard
              :scryfall-id="detail?.scryfall_id"
              :the-gatherer-id="detail?.the_gatherer_id ?? undefined"
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
                  >×{{ detail.collection_entry?.quantity ?? 0 }}</span
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
                  formatPrice(detail.collection_entry?.purchase_price ?? 0)
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
                  >{{
                    formatPrice(
                      (detail.collection_entry?.quantity ?? 0) *
                        (detail.collection_entry?.purchase_price ?? 0),
                    )
                  }}</span
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
                  >Marché · CardMarket · 30 j</span
                >
                <span
                  :class="[
                    'font-mono text-xs',
                    cardVariation.positive
                      ? 'text-cyan-600 dark:text-cyan-400'
                      : 'text-red-500 dark:text-red-400',
                  ]"
                >
                  {{ cardVariation.positive ? '▴' : '▾' }}
                  {{ Math.abs(cardVariation.pct).toFixed(0) }} %
                </span>
              </div>
              <div class="mt-2 flex items-center gap-2.5">
                <span class="font-mono text-xl font-bold">{{
                  formatPrice(detail.price_guide?.trend ?? 0)
                }}</span>
              </div>
              <div class="mt-2 h-[140px]">
                <EnvelopeGraph v-if="cardHasEnoughHistory" :data="cardEnvelopeData" detail />
                <div
                  v-else
                  class="text-2xs flex h-full items-center justify-center font-mono tracking-wide text-slate-400 uppercase dark:text-slate-500"
                >
                  {{ cardHistoryPending ? 'Chargement…' : "Pas encore assez d'historique" }}
                </div>
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

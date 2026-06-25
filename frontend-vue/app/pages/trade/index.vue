<script setup lang="ts">
const mode = ref<'eur' | 'edh'>('eur');

const give = [
  { name: 'Black Market Connections', color: 'b', eur: 13, edh: 18 },
  { name: 'Reprieve', color: 'w', eur: 4, edh: 7 },
  { name: 'Persistent Constrictor', color: 'b', eur: 2, edh: 12 },
];
const get = [
  { name: 'Sire of Seven Deaths', color: 'm', eur: 31, edh: 21 },
  { name: 'The Soul Stone', color: 'b', eur: 9, edh: 5 },
];

const giveTotal = computed(() =>
  give.reduce((s, c) => s + (mode.value === 'eur' ? c.eur : c.edh), 0),
);
const getTotal = computed(() =>
  get.reduce((s, c) => s + (mode.value === 'eur' ? c.eur : c.edh), 0),
);
const diff = computed(() => getTotal.value - giveTotal.value);

const even = computed(() => Math.abs(diff.value) < (mode.value === 'eur' ? 3 : 2));
const lean = computed(() => (even.value ? 'even' : diff.value > 0 ? 'lean-me' : 'lean-them'));
const verdict = computed(() => {
  if (even.value) return mode.value === 'eur' ? 'Équilibré' : '≈ équilibré';
  const abs = Math.abs(diff.value);
  if (diff.value > 0) return mode.value === 'eur' ? `+€${abs} pour toi` : `+${abs} pts pour toi`;
  return mode.value === 'eur' ? `+€${abs} pour lui` : `+${abs} pts pour lui`;
});
const getShare = computed(() => {
  const total = giveTotal.value + getTotal.value || 1;
  return (getTotal.value / total) * 100;
});

const fmt = (v: number) => (mode.value === 'eur' ? `€${v}` : `${v}%`);

const giveEurSum = give.reduce((s, c) => s + c.eur, 0);
const getEurSum = get.reduce((s, c) => s + c.eur, 0);

const modeOptions = [
  { value: 'eur', label: 'Prix €', tone: 'cyan' },
  { value: 'edh', label: 'EDHREC %', tone: 'vio' },
];
</script>

<template>
  <div class="max-w-[1180px] mx-auto px-5 pb-10 pt-7 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- HEADER -->
    <div class="flex items-center justify-between flex-wrap gap-3.5 mb-5">
      <div class="flex items-center gap-3">
        <button
          class="w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
        >
          <Icon name="lucide:chevron-left" size="16" />
        </button>
        <div class="flex items-center gap-2.5">
          <PlayerAvatar initials="M4" :online="true" />
          <div class="flex flex-col gap-px">
            <h2 class="font-display font-semibold text-base tracking-tight">
              Échange avec <span class="text-cyan-600 dark:text-cyan-400">@mizzix_42</span>
            </h2>
            <span class="text-xs text-slate-400 dark:text-slate-500">
              <Icon
                name="mdi:star"
                size="11"
                class="text-violet-500 dark:text-violet-300 align-[-1px]"
              />
              4,8 · 2,3 km · en ligne
            </span>
          </div>
        </div>
      </div>
      <SegToggle v-model="mode" :options="modeOptions" />
    </div>

    <!-- EDHREC INFO BANNER -->
    <div
      v-if="mode === 'edh'"
      class="bg-violet-500/10 dark:bg-violet-400/10 backdrop-blur-md border border-violet-500/30 dark:border-violet-400/30 rounded-2xl shadow-lg px-4 py-3 mb-4 flex gap-2.5 items-start"
    >
      <span class="text-violet-500 dark:text-violet-300 mt-px"
        ><Icon name="lucide:info" size="16"
      /></span>
      <span class="text-sm text-slate-600 dark:text-slate-300">
        Le <span class="text-violet-500 dark:text-violet-300">% EDHREC</span> = part des decks
        référencés qui jouent la carte. On compare la désirabilité plutôt que l'euro.
      </span>
    </div>

    <!-- TRADE GRID -->
    <div
      class="grid gap-4 items-stretch [grid-template-columns:1fr_auto_1fr] max-md:[grid-template-columns:1fr]"
    >
      <!-- LEFT: Je donne -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 flex flex-col gap-3"
      >
        <div class="flex items-center justify-between gap-4">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
            >Je donne</span
          >
          <span class="text-xs text-slate-400 dark:text-slate-500">{{ give.length }} cartes</span>
        </div>
        <div class="flex flex-col gap-2">
          <div
            v-for="(c, i) in give"
            :key="i"
            class="flex items-center gap-3 px-3 py-2 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 transition-all duration-150 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
          >
            <MtgCard :name="c.name" :mini="true" class="w-7 flex-none" />
            <div class="flex-1 min-w-0">
              <div
                class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
              >
                {{ c.name }}
              </div>
            </div>
            <span
              :class="[
                'font-mono text-sm',
                mode === 'eur'
                  ? 'text-slate-600 dark:text-slate-300'
                  : 'text-violet-500 dark:text-violet-300',
              ]"
            >
              {{ mode === 'eur' ? `€${c.eur}` : `${c.edh}%` }}
            </span>
            <button
              class="w-7 h-7 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            >
              <Icon name="lucide:x" size="13" />
            </button>
          </div>
        </div>
        <button
          class="flex items-center justify-center gap-2 p-3 text-slate-600 dark:text-slate-300 text-sm font-semibold border-[1.5px] border-dashed border-slate-300 dark:border-white/15 rounded-xl bg-black/10 transition-all duration-200 hover:border-cyan-500/40 dark:hover:border-cyan-400/40 hover:bg-cyan-500/10 dark:hover:bg-cyan-400/10"
        >
          <Icon name="lucide:plus" size="16" /> Ajouter une de mes cartes
        </button>
        <div class="h-px bg-slate-200 dark:bg-white/10 mt-auto" />
        <div class="flex items-center justify-between gap-4">
          <span class="text-sm text-slate-400 dark:text-slate-500">{{
            mode === 'eur' ? 'Total' : 'Cumul inclusion'
          }}</span>
          <span
            :class="[
              'font-mono font-bold tracking-tight whitespace-nowrap text-xl',
              mode === 'edh' ? 'text-violet-500 dark:text-violet-300' : '',
            ]"
          >
            {{ fmt(giveTotal) }}
          </span>
        </div>
      </div>

      <!-- CENTER: Balance split -->
      <div class="flex flex-col items-center justify-center gap-3.5 min-w-[168px]">
        <div class="flex flex-col items-center w-full max-w-[200px] gap-2">
          <div class="w-full flex justify-between items-baseline">
            <span
              class="text-2xs tracking-wide uppercase text-slate-400 dark:text-slate-500 whitespace-nowrap"
              >Donne</span
            >
            <span
              class="text-2xs tracking-wide uppercase text-slate-400 dark:text-slate-500 whitespace-nowrap"
              >Reçois</span
            >
          </div>
          <div
            class="relative w-full h-3 rounded-full overflow-hidden flex bg-slate-100 dark:bg-zinc-800 border border-slate-200 dark:border-white/10"
          >
            <span
              class="h-full transition-[width] duration-500 ease-out bg-violet-500 dark:bg-violet-400"
              :style="{ width: 100 - getShare + '%' }"
            />
            <span
              class="h-full transition-[width] duration-500 ease-out bg-cyan-500 dark:bg-cyan-400"
              :style="{ width: getShare + '%' }"
            />
            <span
              class="absolute left-1/2 -top-0.5 -bottom-0.5 w-0.5 bg-slate-100 dark:bg-zinc-950 -translate-x-1/2 shadow-[0_0_0_1px_rgba(120,120,120,0.3)]"
            />
          </div>
          <div class="w-full flex justify-between items-baseline">
            <span class="font-mono text-sm font-semibold text-violet-500 dark:text-violet-300">{{
              fmt(giveTotal)
            }}</span>
            <span class="font-mono text-sm font-semibold text-cyan-600 dark:text-cyan-400">{{
              fmt(getTotal)
            }}</span>
          </div>
          <div
            :class="[
              'font-mono text-sm font-semibold text-center px-3 py-2 rounded-xl border',
              lean === 'even'
                ? 'text-cyan-700 dark:text-cyan-300 border-cyan-500/30 dark:border-cyan-400/30 bg-cyan-500/10 dark:bg-cyan-400/10'
                : 'text-violet-700 dark:text-violet-300 border-violet-500/30 dark:border-violet-400/30 bg-violet-500/10 dark:bg-violet-400/10',
            ]"
          >
            {{ verdict }}
          </div>
        </div>
        <button
          v-if="mode === 'edh'"
          class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-violet-500/30 dark:border-violet-400/30 text-violet-700 dark:text-violet-300 bg-violet-500/10 dark:bg-violet-400/10 transition-all duration-150 whitespace-nowrap cursor-pointer select-none"
        >
          <Icon name="lucide:refresh-cw" size="12" />
          Voir aussi en € · €{{ giveEurSum }} ↔ €{{ getEurSum }}
        </button>
      </div>

      <!-- RIGHT: Je reçois -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 flex flex-col gap-3"
      >
        <div class="flex items-center justify-between gap-4">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
            >Je reçois</span
          >
          <span class="text-xs text-slate-400 dark:text-slate-500">{{ get.length }} cartes</span>
        </div>
        <div class="flex flex-col gap-2">
          <div
            v-for="(c, i) in get"
            :key="i"
            class="flex items-center gap-3 px-3 py-2 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 transition-all duration-150 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
          >
            <MtgCard :name="c.name" :mini="true" class="w-7 flex-none" />
            <div class="flex-1 min-w-0">
              <div
                class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
              >
                {{ c.name }}
              </div>
            </div>
            <span
              :class="[
                'font-mono text-sm',
                mode === 'eur'
                  ? 'text-cyan-600 dark:text-cyan-400'
                  : 'text-violet-500 dark:text-violet-300',
              ]"
            >
              {{ mode === 'eur' ? `€${c.eur}` : `${c.edh}%` }}
            </span>
            <button
              class="w-7 h-7 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            >
              <Icon name="lucide:x" size="13" />
            </button>
          </div>
        </div>
        <button
          class="flex items-center justify-center gap-2 p-3 text-slate-600 dark:text-slate-300 text-sm font-semibold border-[1.5px] border-dashed border-slate-300 dark:border-white/15 rounded-xl bg-black/10 transition-all duration-200 hover:border-cyan-500/40 dark:hover:border-cyan-400/40 hover:bg-cyan-500/10 dark:hover:bg-cyan-400/10"
        >
          <Icon name="lucide:plus" size="16" /> Chercher dans sa collection
        </button>
        <div class="h-px bg-slate-200 dark:bg-white/10 mt-auto" />
        <div class="flex items-center justify-between gap-4">
          <span class="text-sm text-slate-400 dark:text-slate-500">{{
            mode === 'eur' ? 'Total' : 'Cumul inclusion'
          }}</span>
          <span
            class="font-mono font-bold tracking-tight whitespace-nowrap text-xl text-cyan-600 dark:text-cyan-400"
            >{{ fmt(getTotal) }}</span
          >
        </div>
      </div>
    </div>

    <!-- ACTIONS -->
    <div
      class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 mt-5"
    >
      <div class="flex items-center justify-between flex-wrap gap-3.5">
        <div class="flex flex-col gap-0.5">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
            >Contact via le canal du joueur</span
          >
          <span class="text-xs text-slate-400 dark:text-slate-500"
            >Aucun paiement ni messagerie internes · redirection externe</span
          >
        </div>
        <div class="flex items-center flex-wrap gap-2.5">
          <button
            class="inline-flex items-center gap-2 justify-center py-2.5 px-4 rounded-xl text-sm font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
          >
            Enregistrer le brouillon
          </button>
          <button
            class="inline-flex items-center gap-2 justify-center py-2.5 px-4 rounded-xl text-sm font-semibold border border-slate-300 dark:border-white/15 text-slate-800 dark:text-slate-100 bg-slate-100 dark:bg-zinc-800 transition-all duration-150 whitespace-nowrap leading-none hover:bg-slate-200 dark:hover:bg-zinc-700 hover:border-slate-300 dark:hover:border-white/15 hover:-translate-y-px active:translate-y-0"
          >
            Proposer l'échange
          </button>
          <button
            class="inline-flex items-center gap-2 justify-center py-2.5 px-4 rounded-xl text-sm font-bold border border-transparent text-zinc-950 bg-cyan-500 dark:bg-cyan-400 shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
          >
            <Icon name="simple-icons:discord" size="16" /> Contacter sur Discord
            <Icon name="lucide:arrow-up-right" size="14" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

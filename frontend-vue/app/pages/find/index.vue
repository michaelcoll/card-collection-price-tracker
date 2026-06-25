<script setup lang="ts">
const mode = ref<'name' | 'decklist'>('name');
const sort = ref<'prix' | 'distance' | 'reputation'>('prix');

const owners = [
  { u: '@mizzix_42', init: 'M4', rep: 4.8, dist: '2,3 km', qty: 2, price: 29, online: true },
  { u: '@kaalia_dt', init: 'KA', rep: 4.9, dist: 'en ligne', qty: 1, price: 34, online: true },
  { u: '@golgari_jo', init: 'GO', rep: 4.6, dist: '11 km', qty: 1, price: 31, online: false },
  { u: '@urza_main', init: 'UR', rep: 4.7, dist: '5,1 km', qty: 3, price: 28, online: false },
];

const sorted = computed(() =>
  [...owners].sort((a, b) => {
    if (sort.value === 'prix') return a.price - b.price;
    if (sort.value === 'distance') return parseFloat(a.dist) - parseFloat(b.dist);
    return b.rep - a.rep;
  }),
);

const coverers = [
  { u: '@mizzix_42', init: 'M4', pct: 81, n: 80, val: 240, online: true },
  { u: '@kaalia_dt', init: 'KA', pct: 63, n: 62, val: 188, online: true },
  { u: '@urza_main', init: 'UR', pct: 47, n: 46, val: 142, online: false },
  { u: '@simic_ramp', init: 'SI', pct: 31, n: 31, val: 96, online: false },
];

const modeOptions = [
  { value: 'name', label: 'Par nom', tone: 'cyan' },
  { value: 'decklist', label: 'Par decklist', tone: 'cyan' },
];

const sortChips = [
  { key: 'prix', label: 'Prix' },
  { key: 'distance', label: 'Distance' },
  { key: 'reputation', label: 'Réputation' },
];

const decklist = ref(
  '1x Vampiric Tutor\n1x Black Market Connections\n1x Reprieve\n1x Chronicle of Victory\n1x The Soul Stone\n1x Sire of Seven Deaths\n1x Emeritus of Woe',
);
</script>

<template>
  <div class="mx-auto max-w-[1180px] px-5 pt-7 pb-10 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- HEADER -->
    <div class="mb-4 flex flex-wrap items-center justify-between gap-3.5">
      <h2 class="font-display text-xl font-semibold tracking-tight">
        Cartes chez les autres joueurs
      </h2>
      <SegToggle v-model="mode" :options="modeOptions" />
    </div>

    <!-- MODE: PAR NOM -->
    <div v-if="mode === 'name'">
      <!-- Search field -->
      <div
        class="mb-5 flex items-center gap-2.5 rounded-2xl border border-slate-300 bg-black/20 py-2 pr-2 pl-4 transition-all duration-200 focus-within:border-cyan-500/40 focus-within:bg-black/10 focus-within:ring-4 focus-within:ring-cyan-500/10 dark:border-white/15 dark:focus-within:border-cyan-400/40"
      >
        <Icon name="lucide:search" size="20" class="flex-none text-slate-400 dark:text-slate-500" />
        <input
          value="Sire of Seven Deaths"
          placeholder="Nom de la carte…"
          class="min-w-0 flex-1 border-0 bg-transparent text-base text-slate-800 outline-none placeholder:text-slate-400 dark:text-slate-100 dark:placeholder:text-slate-500"
        />
      </div>

      <!-- Card header -->
      <div
        class="mb-4 rounded-2xl border border-slate-200 bg-white/60 p-4 shadow-lg backdrop-blur-md dark:border-white/10 dark:bg-zinc-900/60"
      >
        <div class="flex gap-4">
          <MtgCard color="m" name="Sire of Seven Deaths" class="w-24 flex-none" />
          <div class="flex min-w-0 flex-1 flex-col gap-2.5">
            <div>
              <h3 class="font-display mb-1 text-xl font-semibold tracking-tight">
                Sire of Seven Deaths
              </h3>
              <span class="text-sm text-slate-400 dark:text-slate-500"
                >Creature — Eldrazi · Foundations</span
              >
            </div>
            <div class="flex flex-wrap gap-2">
              <span
                class="inline-flex cursor-default items-center gap-1.5 rounded-full border border-slate-200 bg-slate-100 px-3 py-1.5 text-xs font-medium whitespace-nowrap text-slate-600 select-none dark:border-white/10 dark:bg-white/5 dark:text-slate-300"
              >
                Prix réf.
                <span class="ml-1 font-mono tracking-tight text-cyan-600 dark:text-cyan-400"
                  >€31</span
                >
              </span>
              <span
                class="inline-flex cursor-default items-center gap-1.5 rounded-full border border-violet-500/30 bg-violet-500/10 px-3 py-1.5 text-xs font-medium whitespace-nowrap text-violet-700 select-none dark:border-violet-400/30 dark:bg-violet-400/10 dark:text-violet-300"
              >
                EDHREC
                <span class="ml-1 font-mono tracking-tight">41 %</span>
              </span>
            </div>
            <span class="text-sm text-slate-400 dark:text-slate-500"
              >14 joueurs possèdent cette carte</span
            >
          </div>
        </div>
      </div>

      <!-- Sort row -->
      <div class="mb-3.5 flex flex-wrap items-center justify-between gap-2.5">
        <span class="text-sm text-slate-400 dark:text-slate-500"
          >{{ owners.length }} résultats à proximité</span
        >
        <div class="flex items-center gap-2">
          <span
            class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
            >trier</span
          >
          <div class="flex gap-1.5">
            <button
              v-for="chip in sortChips"
              :key="chip.key"
              :class="[
                'inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-all duration-150 select-none',
                sort === chip.key
                  ? 'border-cyan-500/30 bg-cyan-500/10 text-cyan-700 dark:border-cyan-400/30 dark:bg-cyan-400/10 dark:text-cyan-300'
                  : 'border-slate-200 bg-slate-100 text-slate-600 hover:border-slate-300 hover:bg-slate-50 hover:text-slate-800 dark:border-white/10 dark:bg-white/5 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-zinc-800 dark:hover:text-slate-100',
              ]"
              @click="sort = chip.key as typeof sort"
            >
              {{ chip.label }}
            </button>
          </div>
        </div>
      </div>

      <!-- Owner rows -->
      <div class="flex flex-col gap-2">
        <div
          v-for="o in sorted"
          :key="o.u"
          class="flex items-center gap-3 rounded-xl border border-slate-200 bg-white px-3.5 py-3 transition-all duration-150 hover:border-slate-300 hover:bg-slate-50 dark:border-white/10 dark:bg-zinc-900 dark:hover:border-white/15 dark:hover:bg-zinc-800"
        >
          <PlayerAvatar :initials="o.init" :online="o.online" />
          <div class="min-w-0 flex-1">
            <div
              class="overflow-hidden text-sm font-semibold text-ellipsis whitespace-nowrap text-slate-800 dark:text-slate-100"
            >
              {{ o.u }}
            </div>
            <div
              class="flex flex-wrap items-center gap-2 text-xs text-slate-400 dark:text-slate-500"
            >
              <span>
                <Icon
                  name="mdi:star"
                  size="11"
                  class="align-[-1px] text-violet-500 dark:text-violet-300"
                />
                {{ o.rep.toLocaleString('fr-FR') }}
              </span>
              <span>
                <Icon name="lucide:map-pin" size="11" class="align-[-1px]" />
                {{ o.dist }}
              </span>
              <span>×{{ o.qty }} dispo</span>
            </div>
          </div>
          <span class="font-mono text-sm font-semibold text-cyan-600 dark:text-cyan-400"
            >€{{ o.price }}</span
          >
          <NuxtLink
            to="/trade"
            class="inline-flex items-center justify-center gap-2 rounded-lg border border-transparent bg-cyan-500 px-3 py-1.5 text-xs leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
            >Échanger</NuxtLink
          >
        </div>
      </div>
    </div>

    <!-- MODE: PAR DECKLIST -->
    <div
      v-else
      class="grid [grid-template-columns:minmax(240px,320px)_1fr] items-start gap-6 max-md:[grid-template-columns:1fr]"
    >
      <!-- Left: paste zone -->
      <div
        class="flex flex-col gap-3 self-start rounded-2xl border border-slate-200 bg-white/60 p-4 shadow-lg backdrop-blur-md dark:border-white/10 dark:bg-zinc-900/60"
      >
        <span
          class="text-2xs font-mono font-medium tracking-widest whitespace-nowrap text-slate-400 uppercase dark:text-slate-500"
          >Coller ma decklist</span
        >
        <textarea
          v-model="decklist"
          rows="9"
          class="w-full resize-y p-3 font-mono text-xs leading-relaxed text-slate-800 dark:text-slate-100"
        />
        <div class="flex items-center justify-between">
          <span class="text-xs text-slate-400 dark:text-slate-500">99 cartes détectées</span>
          <span
            class="inline-flex cursor-default items-center gap-1.5 rounded-full border border-slate-200 bg-slate-100 px-3 py-1.5 text-xs font-medium whitespace-nowrap text-slate-600 select-none dark:border-white/10 dark:bg-white/5 dark:text-slate-300"
          >
            <span class="h-2 w-2 rounded-full bg-violet-500 dark:bg-violet-400" /> 2 non reconnues
          </span>
        </div>
        <button
          class="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-transparent bg-cyan-500 px-4 py-2.5 text-sm leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
        >
          <Icon name="lucide:search" size="15" /> Trouver les joueurs
        </button>
      </div>

      <!-- Right: coverage results -->
      <div class="flex min-w-0 flex-1 flex-col gap-3.5">
        <div class="flex items-center justify-between">
          <h3 class="font-display text-base font-semibold tracking-tight">
            12 joueurs couvrent ta liste
          </h3>
          <button
            class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border border-cyan-500/30 bg-cyan-500/10 px-3 py-1.5 text-xs font-medium whitespace-nowrap text-cyan-700 transition-all duration-150 select-none dark:border-cyan-400/30 dark:bg-cyan-400/10 dark:text-cyan-300"
          >
            % couverture <Icon name="lucide:chevron-down" size="13" />
          </button>
        </div>

        <div
          v-for="(c, i) in coverers"
          :key="c.u"
          :class="[
            'rounded-2xl p-4 shadow-lg backdrop-blur-md',
            i === 0
              ? 'border border-cyan-500/30 bg-cyan-500/10 dark:border-cyan-400/30 dark:bg-cyan-400/10'
              : 'border border-slate-200 bg-white/60 dark:border-white/10 dark:bg-zinc-900/60',
          ]"
        >
          <div class="mb-2.5 flex items-center justify-between">
            <div class="flex items-center gap-2.5">
              <PlayerAvatar :initials="c.init" :online="c.online" />
              <span
                class="overflow-hidden text-sm font-semibold text-ellipsis whitespace-nowrap text-slate-800 dark:text-slate-100"
                >{{ c.u }}</span
              >
            </div>
            <span
              :class="[
                'font-mono font-bold tracking-tight whitespace-nowrap text-cyan-600 dark:text-cyan-400',
                i === 0 ? 'text-2xl' : 'text-xl',
              ]"
              >{{ c.pct }}%</span
            >
          </div>
          <div
            class="h-2 overflow-hidden rounded-full border border-slate-200 bg-black/30 dark:border-white/5"
          >
            <i
              class="block h-full rounded-full bg-cyan-500 transition-[width] duration-700 ease-out dark:bg-cyan-400"
              :style="{ width: c.pct + '%' }"
            />
          </div>
          <div class="mt-2.5 flex items-center justify-between">
            <span class="text-xs text-slate-400 dark:text-slate-500">
              couvre {{ c.n }}/99 cartes · valeur ≈
              <span class="font-mono tracking-tight">€{{ c.val }}</span>
            </span>
            <div v-if="i === 0" class="flex items-center gap-2">
              <button
                class="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-200 bg-transparent px-3 py-1.5 text-xs leading-none font-semibold whitespace-nowrap text-slate-600 transition-all duration-150 hover:-translate-y-px hover:border-slate-300 hover:bg-slate-100 hover:text-slate-800 active:translate-y-0 max-md:hidden dark:border-white/10 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-white/5 dark:hover:text-slate-100"
              >
                Voir les {{ c.n }}
              </button>
              <NuxtLink
                to="/trade"
                class="inline-flex items-center justify-center gap-2 rounded-lg border border-transparent bg-cyan-500 px-3 py-1.5 text-xs leading-none font-bold whitespace-nowrap text-zinc-950 shadow-lg transition-all duration-150 hover:-translate-y-px hover:bg-cyan-400 active:translate-y-0 dark:bg-cyan-400 dark:hover:bg-cyan-300"
                >Composer l'échange</NuxtLink
              >
            </div>
            <NuxtLink
              v-else
              to="/trade"
              class="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-200 bg-transparent px-3 py-1.5 text-xs leading-none font-semibold whitespace-nowrap text-slate-600 transition-all duration-150 hover:-translate-y-px hover:border-slate-300 hover:bg-slate-100 hover:text-slate-800 active:translate-y-0 dark:border-white/10 dark:text-slate-300 dark:hover:border-white/15 dark:hover:bg-white/5 dark:hover:text-slate-100"
              >Composer</NuxtLink
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

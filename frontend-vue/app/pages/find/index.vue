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
  <div class="max-w-[1180px] mx-auto px-5 pt-7 pb-10 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- HEADER -->
    <div class="flex items-center justify-between flex-wrap gap-3.5 mb-4">
      <h2 class="font-display font-semibold text-xl tracking-tight">
        Cartes chez les autres joueurs
      </h2>
      <SegToggle v-model="mode" :options="modeOptions" />
    </div>

    <!-- MODE: PAR NOM -->
    <div v-if="mode === 'name'">
      <!-- Search field -->
      <div
        class="flex items-center gap-2.5 pl-4 pr-2 py-2 rounded-2xl bg-black/20 border border-slate-300 dark:border-white/15 transition-all duration-200 focus-within:border-cyan-500/40 dark:focus-within:border-cyan-400/40 focus-within:ring-4 focus-within:ring-cyan-500/10 focus-within:bg-black/10 mb-5"
      >
        <Icon name="lucide:search" size="20" class="text-slate-400 dark:text-slate-500 flex-none" />
        <input
          value="Sire of Seven Deaths"
          placeholder="Nom de la carte…"
          class="flex-1 border-0 bg-transparent outline-none text-base min-w-0 text-slate-800 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-500"
        />
      </div>

      <!-- Card header -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 mb-4"
      >
        <div class="flex gap-4">
          <MtgCard color="m" name="Sire of Seven Deaths" class="w-24 flex-none" />
          <div class="flex flex-col flex-1 min-w-0 gap-2.5">
            <div>
              <h3 class="font-display font-semibold text-xl tracking-tight mb-1">
                Sire of Seven Deaths
              </h3>
              <span class="text-slate-400 dark:text-slate-500 text-sm"
                >Creature — Eldrazi · Foundations</span
              >
            </div>
            <div class="flex flex-wrap gap-2">
              <span
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 whitespace-nowrap cursor-default select-none"
              >
                Prix réf.
                <span class="font-mono tracking-tight text-cyan-600 dark:text-cyan-400 ml-1"
                  >€31</span
                >
              </span>
              <span
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-violet-500/30 dark:border-violet-400/30 text-violet-700 dark:text-violet-300 bg-violet-500/10 dark:bg-violet-400/10 whitespace-nowrap cursor-default select-none"
              >
                EDHREC
                <span class="font-mono tracking-tight ml-1">41 %</span>
              </span>
            </div>
            <span class="text-slate-400 dark:text-slate-500 text-sm"
              >14 joueurs possèdent cette carte</span
            >
          </div>
        </div>
      </div>

      <!-- Sort row -->
      <div class="flex items-center justify-between flex-wrap gap-2.5 mb-3.5">
        <span class="text-slate-400 dark:text-slate-500 text-sm"
          >{{ owners.length }} résultats à proximité</span
        >
        <div class="flex items-center gap-2">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
            >trier</span
          >
          <div class="flex gap-1.5">
            <button
              v-for="chip in sortChips"
              :key="chip.key"
              :class="[
                'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border whitespace-nowrap cursor-pointer select-none transition-all duration-150',
                sort === chip.key
                  ? 'border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 bg-cyan-500/10 dark:bg-cyan-400/10'
                  : 'border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800',
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
          class="flex items-center gap-3 px-3.5 py-3 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 transition-all duration-150 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
        >
          <PlayerAvatar :initials="o.init" :online="o.online" />
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
            >
              {{ o.u }}
            </div>
            <div
              class="text-xs text-slate-400 dark:text-slate-500 flex items-center gap-2 flex-wrap"
            >
              <span>
                <Icon
                  name="mdi:star"
                  size="11"
                  class="text-violet-500 dark:text-violet-300 align-[-1px]"
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
          <span class="font-mono text-sm text-cyan-600 dark:text-cyan-400 font-semibold"
            >€{{ o.price }}</span
          >
          <NuxtLink
            to="/trade"
            class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
            >Échanger</NuxtLink
          >
        </div>
      </div>
    </div>

    <!-- MODE: PAR DECKLIST -->
    <div
      v-else
      class="grid gap-6 items-start [grid-template-columns:minmax(240px,320px)_1fr] max-md:[grid-template-columns:1fr]"
    >
      <!-- Left: paste zone -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 flex flex-col gap-3 self-start"
      >
        <span
          class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
          >Coller ma decklist</span
        >
        <textarea
          v-model="decklist"
          rows="9"
          class="w-full resize-y font-mono text-xs p-3 leading-relaxed text-slate-800 dark:text-slate-100"
        />
        <div class="flex items-center justify-between">
          <span class="text-slate-400 dark:text-slate-500 text-xs">99 cartes détectées</span>
          <span
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 whitespace-nowrap cursor-default select-none"
          >
            <span class="w-2 h-2 rounded-full bg-violet-500 dark:bg-violet-400" /> 2 non reconnues
          </span>
        </div>
        <button
          class="inline-flex items-center gap-2 justify-center px-4 py-2.5 rounded-xl text-sm font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none w-full hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
        >
          <Icon name="lucide:search" size="15" /> Trouver les joueurs
        </button>
      </div>

      <!-- Right: coverage results -->
      <div class="flex-1 min-w-0 flex flex-col gap-3.5">
        <div class="flex items-center justify-between">
          <h3 class="font-display font-semibold text-base tracking-tight">
            12 joueurs couvrent ta liste
          </h3>
          <button
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 bg-cyan-500/10 dark:bg-cyan-400/10 whitespace-nowrap cursor-pointer select-none transition-all duration-150"
          >
            % couverture <Icon name="lucide:chevron-down" size="13" />
          </button>
        </div>

        <div
          v-for="(c, i) in coverers"
          :key="c.u"
          :class="[
            'backdrop-blur-md rounded-2xl shadow-lg p-4',
            i === 0
              ? 'bg-cyan-500/10 dark:bg-cyan-400/10 border border-cyan-500/30 dark:border-cyan-400/30'
              : 'bg-white/60 dark:bg-zinc-900/60 border border-slate-200 dark:border-white/10',
          ]"
        >
          <div class="flex items-center justify-between mb-2.5">
            <div class="flex items-center gap-2.5">
              <PlayerAvatar :initials="c.init" :online="c.online" />
              <span
                class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
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
            class="h-2 rounded-full bg-black/30 overflow-hidden border border-slate-200 dark:border-white/5"
          >
            <i
              class="block h-full rounded-full bg-cyan-500 dark:bg-cyan-400 transition-[width] duration-700 ease-out"
              :style="{ width: c.pct + '%' }"
            />
          </div>
          <div class="flex items-center justify-between mt-2.5">
            <span class="text-slate-400 dark:text-slate-500 text-xs">
              couvre {{ c.n }}/99 cartes · valeur ≈
              <span class="font-mono tracking-tight">€{{ c.val }}</span>
            </span>
            <div v-if="i === 0" class="flex items-center gap-2">
              <button
                class="max-md:hidden inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
              >
                Voir les {{ c.n }}
              </button>
              <NuxtLink
                to="/trade"
                class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
                >Composer l'échange</NuxtLink
              >
            </div>
            <NuxtLink
              v-else
              to="/trade"
              class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
              >Composer</NuxtLink
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

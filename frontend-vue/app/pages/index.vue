<script setup lang="ts">
const mode = ref<'name' | 'decklist'>('name');
const q = ref('');

const recents = ['Vampiric Tutor', 'Black Market Connections', 'Emeritus of Woe', 'Reprieve'];

const trends = [
  {
    name: 'Sire of Seven Deaths',
    color: 'm',
    price: 3100,
    id: '8d8432a7-1c8a-4cfb-947c-ecf9791063eb',
  },
  { name: 'Vampiric Tutor', color: 'b', price: 2800, id: '7a79190f-de60-4eb6-b925-594eb76ca8c3' },
  {
    name: 'Chronicle of Victory',
    color: 'w',
    price: 2400,
    id: 'b3c2d68d-690b-41e7-99ed-2d20c7e0a9b4',
  },
  { name: 'The Soul Stone', color: 'b', price: 900, id: '1982f910-a9bd-4e94-a187-84381b22aacc' },
  {
    name: 'Black Market Connections',
    color: 'b',
    price: 1300,
    id: '318f8ec3-0613-448d-87d2-0bcc9e95da64',
  },
];

const trades = [
  { u: '@mizzix_42', s: 'En attente de réponse', t: 'them' },
  { u: '@golgari_jo', s: 'À toi de jouer', t: 'me' },
];

const searchOptions = [
  { value: 'name', label: 'Nom de carte', tone: 'cyan' },
  { value: 'decklist', label: 'Decklist', tone: 'cyan' },
];

const selectRecent = (name: string) => {
  q.value = name;
};
</script>

<template>
  <div class="max-w-[1180px] mx-auto px-5 pb-10 pt-7 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <!-- ── HERO ── -->
    <div class="flex flex-col items-center text-center gap-6 pt-10 pb-8">
      <!-- Status badge -->
      <span
        class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 bg-cyan-500/10 dark:bg-cyan-400/10 whitespace-nowrap cursor-default select-none"
      >
        <span class="w-2 h-2 rounded-full bg-current" />
        2 418 joueurs · 1,2 M cartes indexées
      </span>

      <!-- Hero logo lockup -->
      <div class="flex flex-col items-center gap-2.5">
        <div class="flex items-center gap-[clamp(12px,2.4vw,20px)]">
          <span
            class="font-display font-semibold leading-none text-[clamp(30px,5.2vw,46px)] tracking-tight text-slate-800 dark:text-slate-100"
            >Arcane</span
          >
          <span class="flex-none grid place-items-center" aria-hidden="true">
            <svg
              viewBox="0 0 28 28"
              fill="none"
              stroke-width="2.2"
              stroke-linejoin="round"
              class="w-[clamp(36px,5.4vw,50px)] h-[clamp(36px,5.4vw,50px)]"
            >
              <rect
                x="5"
                y="8.5"
                width="11"
                height="11"
                rx="3"
                transform="rotate(45 10.5 14)"
                stroke="var(--cyan)"
              />
              <rect
                x="12"
                y="8.5"
                width="11"
                height="11"
                rx="3"
                transform="rotate(45 17.5 14)"
                stroke="var(--violet)"
              />
            </svg>
          </span>
          <span
            class="font-display font-semibold leading-none text-[clamp(30px,5.2vw,46px)] tracking-tight text-cyan-600 dark:text-cyan-400"
            >Exchange</span
          >
        </div>
        <span
          class="font-mono text-[clamp(10px,1.3vw,11.5px)] tracking-[0.26em] uppercase text-slate-400 dark:text-slate-500"
          >Le marché des joueurs</span
        >
      </div>

      <!-- Subtitle -->
      <p
        class="text-slate-600 dark:text-slate-300 max-w-[460px] text-base -mt-2 mb-4 leading-relaxed"
      >
        Recherche une carte ou colle une decklist. On te montre qui la possède, à quel prix, et tu
        composes l'échange.
      </p>

      <!-- Search area -->
      <div class="flex flex-col w-full max-w-[540px] gap-3.5 items-center">
        <SegToggle v-model="mode" :options="searchOptions" />

        <!-- search-hero: glow via real child div + group-focus-within -->
        <div class="relative w-full group">
          <div
            class="absolute -inset-x-2.5 -inset-y-8 -z-10 rounded-[40px] bg-cyan-500/20 dark:bg-cyan-400/20 opacity-50 blur-xl transition-opacity duration-300 pointer-events-none group-focus-within:opacity-90"
          />

          <!-- Name search -->
          <div
            v-if="mode === 'name'"
            class="flex items-center gap-2.5 pl-4 pr-2 py-2 rounded-2xl min-h-[62px] bg-black/20 border border-slate-300 dark:border-white/15 transition-all duration-200 focus-within:border-cyan-500/40 dark:focus-within:border-cyan-400/40 focus-within:ring-4 focus-within:ring-cyan-500/10 focus-within:bg-black/10"
          >
            <AppIcon
              name="search"
              :size="20"
              class="text-slate-400 dark:text-slate-500 flex-none"
            />
            <input
              v-model="q"
              class="flex-1 border-0 bg-transparent outline-none text-base min-w-0 text-slate-800 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-500"
              placeholder="Vampiric Tutor, Sire of Seven Deaths…"
              @keydown.enter="navigateTo('/find')"
            />
            <button
              class="inline-flex items-center gap-2 justify-center px-6 self-stretch rounded-xl text-base font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
              @click="navigateTo('/find')"
            >
              Chercher
            </button>
          </div>

          <!-- Decklist search -->
          <div
            v-else
            class="flex flex-col items-start gap-3 pl-4 pr-2 py-2 rounded-2xl bg-black/20 border border-slate-300 dark:border-white/15 transition-all duration-200 focus-within:border-cyan-500/40 dark:focus-within:border-cyan-400/40 focus-within:ring-4 focus-within:ring-cyan-500/10 focus-within:bg-black/10"
          >
            <div class="flex items-center w-full gap-2.5">
              <AppIcon
                name="layers"
                :size="20"
                class="text-slate-400 dark:text-slate-500 flex-none"
              />
              <span class="text-slate-400 dark:text-slate-500 text-sm"
                >Colle ta decklist (Moxfield · Archidekt · texte)</span
              >
            </div>
            <textarea
              :rows="4"
              class="w-full resize-y font-mono text-sm text-slate-800 dark:text-slate-100"
              placeholder="1x Vampiric Tutor&#10;1x Black Market Connections&#10;1x The Soul Stone…"
            />
            <button
              class="inline-flex items-center gap-2 justify-center py-1.5 px-3 self-end rounded-lg text-xs font-bold text-zinc-950 bg-cyan-500 dark:bg-cyan-400 border border-transparent shadow-lg transition-all duration-150 whitespace-nowrap leading-none hover:bg-cyan-400 dark:hover:bg-cyan-300 hover:-translate-y-px active:translate-y-0"
              @click="navigateTo('/find')"
            >
              Trouver les joueurs
            </button>
          </div>
        </div>

        <!-- Recent searches -->
        <div v-if="mode === 'name'" class="flex items-center flex-wrap gap-2 justify-center">
          <span
            class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap mr-0.5"
            >récents</span
          >
          <button
            v-for="r in recents"
            :key="r"
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 whitespace-nowrap cursor-pointer select-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            @click="selectRecent(r)"
          >
            <AppIcon name="clock" :size="12" />
            {{ r }}
          </button>
        </div>
      </div>
    </div>

    <!-- ── TRENDS ── -->
    <div class="mt-6">
      <div class="flex items-center justify-between gap-3 mb-3.5">
        <div class="flex items-center gap-2">
          <span class="text-cyan-600 dark:text-cyan-400"
            ><AppIcon name="trending" :size="18"
          /></span>
          <h2 class="font-display font-semibold text-base tracking-tight m-0">
            Tendances cette semaine
          </h2>
        </div>
        <a
          class="text-sm text-slate-600 dark:text-slate-300 inline-flex items-center gap-1 transition-colors duration-150 hover:text-cyan-600 dark:hover:text-cyan-400"
          href="#"
        >
          voir tout
          <AppIcon name="chevron" :size="14" class="-rotate-90" />
        </a>
      </div>

      <div
        class="grid gap-4 [grid-template-columns:repeat(auto-fill,minmax(118px,1fr))] max-md:[grid-template-columns:repeat(auto-fill,minmax(96px,1fr))] max-md:gap-3.5"
      >
        <CardCell
          v-for="t in trends"
          :key="t.name"
          :scryfall-id="t.id"
          :name="t.name"
          :price="t.price"
          @click="navigateTo('/find')"
        />
      </div>
    </div>

    <!-- ── SECONDARY DISCOVERY ── -->
    <div class="flex flex-wrap gap-4 mt-7">
      <!-- Collection panel -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg flex-1 p-5 min-w-[260px]"
      >
        <div class="flex items-center justify-between gap-4">
          <div class="flex flex-col gap-0.5">
            <span
              class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
              >Ta collection</span
            >
            <span class="font-mono font-bold tracking-tight whitespace-nowrap text-2xl"
              >€ 4 218,60</span
            >
            <span class="text-cyan-600 dark:text-cyan-400 text-sm font-mono">▴ €86,20 (30 j)</span>
          </div>
          <Sparkline />
        </div>
        <button
          class="inline-flex items-center gap-2 justify-center py-2.5 px-4 rounded-xl text-sm font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none w-full mt-4 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
          @click="navigateTo('/collection')"
        >
          Ouvrir ma collection
          <AppIcon name="arrowUR" :size="15" />
        </button>
      </div>

      <!-- Trades panel -->
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg flex-1 p-5 min-w-[260px]"
      >
        <span
          class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
          >Échanges en cours</span
        >
        <div class="flex flex-col gap-2 mt-3">
          <button
            v-for="e in trades"
            :key="e.u"
            class="flex items-center gap-3 px-3.5 py-3 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 transition-all duration-150 w-full text-left hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
            @click="navigateTo('/trade')"
          >
            <PlayerAvatar :initials="e.u.slice(1, 3).toUpperCase()" :online="e.t === 'me'" />
            <span class="flex-1 min-w-0 flex flex-col">
              <span
                class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
                >{{ e.u }}</span
              >
              <span class="text-xs text-slate-400 dark:text-slate-500">{{ e.s }}</span>
            </span>
            <AppIcon
              name="chevron"
              :size="16"
              class="-rotate-90 text-slate-400 dark:text-slate-500 flex-none"
            />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

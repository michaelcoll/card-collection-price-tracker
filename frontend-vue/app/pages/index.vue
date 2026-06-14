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
  <div
    class="max-w-[var(--maxw)] mx-auto px-[22px] pb-[40px] pt-[28px] max-[860px]:px-[16px] max-[860px]:pt-[20px] max-[860px]:pb-[30px]"
  >
    <!-- ── HERO ── -->
    <div class="flex flex-col items-center text-center gap-[22px] pt-[38px] pb-[30px]">
      <!-- Status badge -->
      <span
        class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--cyan-line)] text-[var(--cyan-ink)] bg-[var(--cyan-fill)] whitespace-nowrap cursor-default select-none"
      >
        <span class="w-[7px] h-[7px] rounded-full bg-current" />
        2 418 joueurs · 1,2 M cartes indexées
      </span>

      <!-- Title -->
      <h1
        class="[font-family:var(--font-display)] font-semibold text-[clamp(28px,4vw,42px)] max-[860px]:text-[30px] tracking-[-0.025em] leading-[1.04] max-w-[620px] mt-[0.67em] mb-[0.67em]"
      >
        Trouve la carte.<br />
        <span class="text-[var(--cyan)]">Trouve le joueur.</span>
      </h1>

      <!-- Subtitle -->
      <p
        class="text-[var(--ink-2)] max-w-[460px] text-[15.5px] mt-[-8px] mb-[15.5px] leading-[1.6]"
      >
        Recherche une carte ou colle une decklist. On te montre qui la possède, à quel prix, et tu
        composes l'échange.
      </p>

      <!-- Search area -->
      <div class="flex flex-col w-full max-w-[540px] gap-[14px] items-center">
        <SegToggle v-model="mode" :options="searchOptions" />

        <!-- search-hero: glow via real child div + group-focus-within -->
        <div class="relative w-full group">
          <div
            class="absolute -inset-x-[10px] -inset-y-[30px] -z-[1] rounded-[40px] bg-[radial-gradient(60%_80%_at_50%_50%,var(--cyan-glow),transparent_70%)] opacity-50 blur-[20px] transition-opacity duration-300 pointer-events-none group-focus-within:opacity-90"
          />

          <!-- Name search -->
          <div
            v-if="mode === 'name'"
            class="flex items-center gap-[10px] pl-[18px] pr-[8px] py-[8px] rounded-[15px] min-h-[62px] bg-[color-mix(in_srgb,black_22%,transparent)] border border-solid border-[var(--line-2)] transition-all duration-[180ms] ease focus-within:border-[var(--cyan-line)] focus-within:shadow-[0_0_0_4px_var(--cyan-fill),0_0_28px_-8px_var(--cyan-glow)] focus-within:bg-[color-mix(in_srgb,black_14%,transparent)]"
          >
            <AppIcon name="search" :size="20" class="text-[var(--ink-3)] flex-none" />
            <input
              v-model="q"
              class="flex-1 border-0 bg-transparent outline-none text-[16px] min-w-0 placeholder:text-[var(--ink-3)]"
              placeholder="Vampiric Tutor, Sire of Seven Deaths…"
              @keydown.enter="navigateTo('/find')"
            />
            <button
              class="inline-flex items-center gap-[8px] justify-center px-[22px] self-stretch rounded-[11px] text-[16px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
              @click="navigateTo('/find')"
            >
              Chercher
            </button>
          </div>

          <!-- Decklist search -->
          <div
            v-else
            class="flex flex-col items-start gap-[12px] pl-[18px] pr-[8px] py-[8px] rounded-[15px] bg-[color-mix(in_srgb,black_22%,transparent)] border border-solid border-[var(--line-2)] transition-all duration-[180ms] ease focus-within:border-[var(--cyan-line)] focus-within:shadow-[0_0_0_4px_var(--cyan-fill),0_0_28px_-8px_var(--cyan-glow)] focus-within:bg-[color-mix(in_srgb,black_14%,transparent)]"
          >
            <div class="flex items-center w-full gap-[10px]">
              <AppIcon name="layers" :size="20" class="text-[var(--ink-3)] flex-none" />
              <span class="text-[var(--ink-3)] text-[14.5px]"
                >Colle ta decklist (Moxfield · Archidekt · texte)</span
              >
            </div>
            <textarea
              :rows="4"
              class="w-full resize-y [font-family:var(--font-mono)] text-[13px]"
              placeholder="1x Vampiric Tutor&#10;1x Black Market Connections&#10;1x The Soul Stone…"
            />
            <button
              class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] self-end rounded-[8px] text-[12px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
              @click="navigateTo('/find')"
            >
              Trouver les joueurs
            </button>
          </div>
        </div>

        <!-- Recent searches -->
        <div v-if="mode === 'name'" class="flex items-center flex-wrap gap-[8px] justify-center">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap mr-[2px]"
            >récents</span
          >
          <button
            v-for="r in recents"
            :key="r"
            class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[150ms] ease whitespace-nowrap cursor-pointer select-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            @click="selectRecent(r)"
          >
            <AppIcon name="clock" :size="12" />
            {{ r }}
          </button>
        </div>
      </div>
    </div>

    <!-- ── TRENDS ── -->
    <div class="mt-[26px]">
      <div class="flex items-center justify-between gap-[12px] mb-[14px]">
        <div class="flex items-center gap-[9px]">
          <span class="text-[var(--cyan)]"><AppIcon name="trending" :size="18" /></span>
          <h2
            class="[font-family:var(--font-display)] font-semibold text-[16px] tracking-[-0.01em] m-0"
          >
            Tendances cette semaine
          </h2>
        </div>
        <a
          class="text-[13px] text-[var(--ink-2)] inline-flex items-center gap-[4px] transition-colors duration-[150ms] hover:text-[var(--cyan)]"
          href="#"
        >
          voir tout
          <AppIcon name="chevron" :size="14" class="-rotate-90" />
        </a>
      </div>

      <div
        class="grid gap-[18px] [grid-template-columns:repeat(auto-fill,minmax(118px,1fr))] max-[860px]:[grid-template-columns:repeat(auto-fill,minmax(96px,1fr))] max-[860px]:gap-[14px]"
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
    <div class="flex flex-wrap gap-[var(--d-gap)] mt-[28px]">
      <!-- Collection panel -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] flex-1 p-[20px] min-w-[260px]"
      >
        <div class="flex items-center justify-between gap-[var(--d-gap)]">
          <div class="flex flex-col gap-[2px]">
            <span
              class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
              >Ta collection</span
            >
            <span
              class="[font-family:var(--font-mono)] font-bold tracking-[-0.02em] whitespace-nowrap text-[26px]"
              >€ 4 218,60</span
            >
            <span class="text-[var(--cyan)] text-[13px] [font-family:var(--font-mono)]"
              >▴ €86,20 (30 j)</span
            >
          </div>
          <Sparkline />
        </div>
        <button
          class="inline-flex items-center gap-[8px] justify-center py-[9px] px-[15px] rounded-[10px] text-[13.5px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none w-full mt-[16px] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
          @click="navigateTo('/collection')"
        >
          Ouvrir ma collection
          <AppIcon name="arrowUR" :size="15" />
        </button>
      </div>

      <!-- Trades panel -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] flex-1 p-[20px] min-w-[260px]"
      >
        <span
          class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
          >Échanges en cours</span
        >
        <div class="flex flex-col gap-[9px] mt-[12px]">
          <button
            v-for="e in trades"
            :key="e.u"
            class="flex items-center gap-[13px] px-[14px] py-[11px] rounded-[12px] border border-solid border-[var(--line)] bg-[var(--surface)] transition-all duration-[150ms] ease w-full text-left hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            @click="navigateTo('/trade')"
          >
            <PlayerAvatar :initials="e.u.slice(1, 3).toUpperCase()" :online="e.t === 'me'" />
            <span class="flex-1 min-w-0 flex flex-col">
              <span
                class="text-[14px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
                >{{ e.u }}</span
              >
              <span class="text-[12px] text-[var(--ink-3)]">{{ e.s }}</span>
            </span>
            <AppIcon name="chevron" :size="16" class="-rotate-90 text-[var(--ink-3)] flex-none" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

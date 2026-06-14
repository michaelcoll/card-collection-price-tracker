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
  <div
    class="max-w-[var(--maxw)] mx-auto px-[22px] pb-[40px] pt-[28px] max-[860px]:px-[16px] max-[860px]:pt-[20px] max-[860px]:pb-[30px]"
  >
    <!-- HEADER -->
    <div class="flex items-center justify-between flex-wrap gap-[14px] mb-[20px]">
      <div class="flex items-center gap-[12px]">
        <button
          class="w-[34px] h-[34px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
        >
          <Icon name="lucide:chevron-left" size="16" />
        </button>
        <div class="flex items-center gap-[10px]">
          <PlayerAvatar initials="M4" :online="true" />
          <div class="flex flex-col gap-[1px]">
            <h2
              class="[font-family:var(--font-display)] font-semibold text-[16px] tracking-[-0.01em]"
            >
              Échange avec <span class="text-[var(--cyan)]">@mizzix_42</span>
            </h2>
            <span class="text-[12px] text-[var(--ink-3)]">
              <Icon name="mdi:star" size="11" class="text-[var(--violet)] align-[-1px]" />
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
      class="bg-[var(--violet-fill)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--violet-line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] px-[16px] py-[12px] mb-[16px] flex gap-[10px] items-start"
    >
      <span class="text-[var(--violet)] mt-[1px]"><Icon name="lucide:info" size="16" /></span>
      <span class="text-[13px] text-[var(--ink-2)]">
        Le <span class="text-[var(--violet)]">% EDHREC</span> = part des decks référencés qui jouent
        la carte. On compare la désirabilité plutôt que l'euro.
      </span>
    </div>

    <!-- TRADE GRID -->
    <div
      class="grid gap-[18px] items-stretch [grid-template-columns:1fr_auto_1fr] max-[860px]:[grid-template-columns:1fr]"
    >
      <!-- LEFT: Je donne -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] flex flex-col gap-[12px]"
      >
        <div class="flex items-center justify-between gap-[16px]">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
            >Je donne</span
          >
          <span class="text-[12px] text-[var(--ink-3)]">{{ give.length }} cartes</span>
        </div>
        <div class="flex flex-col gap-[8px]">
          <div
            v-for="(c, i) in give"
            :key="i"
            class="flex items-center gap-[13px] px-[11px] py-[8px] rounded-[12px] border border-solid border-[var(--line)] bg-[var(--surface)] transition-all duration-[150ms] ease hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
          >
            <MtgCard :name="c.name" :mini="true" class="w-[26px] flex-none" />
            <div class="flex-1 min-w-0">
              <div
                class="text-[13.5px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
              >
                {{ c.name }}
              </div>
            </div>
            <span
              :class="[
                '[font-family:var(--font-mono)] text-[13.5px]',
                mode === 'eur' ? 'text-[var(--ink-2)]' : 'text-[var(--violet)]',
              ]"
            >
              {{ mode === 'eur' ? `€${c.eur}` : `${c.edh}%` }}
            </span>
            <button
              class="w-[26px] h-[26px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            >
              <Icon name="lucide:x" size="13" />
            </button>
          </div>
        </div>
        <button
          class="flex items-center justify-center gap-[8px] p-[12px] text-[var(--ink-2)] text-[13px] font-semibold [border-width:1.5px] border-dashed border-[var(--line-2)] rounded-[14px] bg-[color-mix(in_srgb,black_16%,transparent)] transition-all duration-[200ms] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]"
        >
          <Icon name="lucide:plus" size="16" /> Ajouter une de mes cartes
        </button>
        <div class="h-[1px] bg-[var(--line)] mt-auto" />
        <div class="flex items-center justify-between gap-[16px]">
          <span class="text-[13px] text-[var(--ink-3)]">{{
            mode === 'eur' ? 'Total' : 'Cumul inclusion'
          }}</span>
          <span
            :class="[
              '[font-family:var(--font-mono)] font-bold tracking-[-0.02em] whitespace-nowrap text-[19px]',
              mode === 'edh' ? 'text-[var(--violet)]' : '',
            ]"
          >
            {{ fmt(giveTotal) }}
          </span>
        </div>
      </div>

      <!-- CENTER: Balance split -->
      <div class="flex flex-col items-center justify-center gap-[14px] min-w-[168px]">
        <div class="flex flex-col items-center w-full max-w-[200px] gap-[7px]">
          <div class="w-full flex justify-between items-baseline">
            <span
              class="text-[9.5px] tracking-[0.06em] uppercase text-[var(--ink-3)] whitespace-nowrap"
              >Donne</span
            >
            <span
              class="text-[9.5px] tracking-[0.06em] uppercase text-[var(--ink-3)] whitespace-nowrap"
              >Reçois</span
            >
          </div>
          <div
            class="relative w-full h-[12px] rounded-full overflow-hidden flex bg-[var(--surface-3)] border border-solid border-[var(--line)]"
          >
            <span
              class="h-full transition-[width] duration-[500ms] [transition-timing-function:cubic-bezier(0.5,1.2,0.4,1)] bg-[linear-gradient(90deg,var(--violet-dim),var(--violet))]"
              :style="{ width: 100 - getShare + '%' }"
            />
            <span
              class="h-full transition-[width] duration-[500ms] [transition-timing-function:cubic-bezier(0.5,1.2,0.4,1)] bg-[linear-gradient(90deg,var(--cyan-soft),var(--cyan))]"
              :style="{ width: getShare + '%' }"
            />
            <span
              class="absolute left-1/2 -top-[2px] -bottom-[2px] w-[2px] bg-[var(--bg)] -translate-x-1/2 shadow-[0_0_0_1px_var(--line-2)]"
            />
          </div>
          <div class="w-full flex justify-between items-baseline">
            <span
              class="[font-family:var(--font-mono)] text-[15px] font-semibold text-[var(--violet)]"
              >{{ fmt(giveTotal) }}</span
            >
            <span
              class="[font-family:var(--font-mono)] text-[15px] font-semibold text-[var(--cyan)]"
              >{{ fmt(getTotal) }}</span
            >
          </div>
          <div
            :class="[
              '[font-family:var(--font-mono)] text-[13px] font-semibold text-center px-[13px] py-[7px] rounded-[10px] border border-solid',
              lean === 'even'
                ? 'text-[var(--cyan-ink)] border-[var(--cyan-line)] bg-[var(--cyan-fill)]'
                : 'text-[var(--violet-ink)] border-[var(--violet-line)] bg-[var(--violet-fill)]',
            ]"
          >
            {{ verdict }}
          </div>
        </div>
        <button
          v-if="mode === 'edh'"
          class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--violet-line)] text-[var(--violet-ink)] bg-[var(--violet-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)] transition-all duration-[150ms] ease whitespace-nowrap cursor-pointer select-none"
        >
          <Icon name="lucide:refresh-cw" size="12" />
          Voir aussi en € · €{{ giveEurSum }} ↔ €{{ getEurSum }}
        </button>
      </div>

      <!-- RIGHT: Je reçois -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] flex flex-col gap-[12px]"
      >
        <div class="flex items-center justify-between gap-[16px]">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
            >Je reçois</span
          >
          <span class="text-[12px] text-[var(--ink-3)]">{{ get.length }} cartes</span>
        </div>
        <div class="flex flex-col gap-[8px]">
          <div
            v-for="(c, i) in get"
            :key="i"
            class="flex items-center gap-[13px] px-[11px] py-[8px] rounded-[12px] border border-solid border-[var(--line)] bg-[var(--surface)] transition-all duration-[150ms] ease hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
          >
            <MtgCard :name="c.name" :mini="true" class="w-[26px] flex-none" />
            <div class="flex-1 min-w-0">
              <div
                class="text-[13.5px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
              >
                {{ c.name }}
              </div>
            </div>
            <span
              :class="[
                '[font-family:var(--font-mono)] text-[13.5px]',
                mode === 'eur' ? 'text-[var(--cyan)]' : 'text-[var(--violet)]',
              ]"
            >
              {{ mode === 'eur' ? `€${c.eur}` : `${c.edh}%` }}
            </span>
            <button
              class="w-[26px] h-[26px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
            >
              <Icon name="lucide:x" size="13" />
            </button>
          </div>
        </div>
        <button
          class="flex items-center justify-center gap-[8px] p-[12px] text-[var(--ink-2)] text-[13px] font-semibold [border-width:1.5px] border-dashed border-[var(--line-2)] rounded-[14px] bg-[color-mix(in_srgb,black_16%,transparent)] transition-all duration-[200ms] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]"
        >
          <Icon name="lucide:plus" size="16" /> Chercher dans sa collection
        </button>
        <div class="h-[1px] bg-[var(--line)] mt-auto" />
        <div class="flex items-center justify-between gap-[16px]">
          <span class="text-[13px] text-[var(--ink-3)]">{{
            mode === 'eur' ? 'Total' : 'Cumul inclusion'
          }}</span>
          <span
            class="[font-family:var(--font-mono)] font-bold tracking-[-0.02em] whitespace-nowrap text-[19px] text-[var(--cyan)]"
            >{{ fmt(getTotal) }}</span
          >
        </div>
      </div>
    </div>

    <!-- ACTIONS -->
    <div
      class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[16px] mt-[20px]"
    >
      <div class="flex items-center justify-between flex-wrap gap-[14px]">
        <div class="flex flex-col gap-[2px]">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
            >Contact via le canal du joueur</span
          >
          <span class="text-[12.5px] text-[var(--ink-3)]"
            >Aucun paiement ni messagerie internes · redirection externe</span
          >
        </div>
        <div class="flex items-center flex-wrap gap-[10px]">
          <button
            class="inline-flex items-center gap-[8px] justify-center py-[9px] px-[15px] rounded-[10px] text-[13.5px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
          >
            Enregistrer le brouillon
          </button>
          <button
            class="inline-flex items-center gap-[8px] justify-center py-[9px] px-[15px] rounded-[10px] text-[13.5px] font-semibold border border-solid border-[var(--line-2)] text-[var(--ink)] bg-[var(--surface-2)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--surface-3)] hover:border-[var(--line-2)] hover:-translate-y-px active:translate-y-0"
          >
            Proposer l'échange
          </button>
          <button
            class="inline-flex items-center gap-[8px] justify-center py-[9px] px-[15px] rounded-[10px] text-[13.5px] font-bold border border-solid border-transparent text-[var(--on-accent)] bg-[var(--cyan)] shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
          >
            <Icon name="simple-icons:discord" size="16" /> Contacter sur Discord
            <Icon name="lucide:arrow-up-right" size="14" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

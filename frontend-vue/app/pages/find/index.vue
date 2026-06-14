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
  <div
    class="max-w-[var(--maxw)] mx-auto px-[22px] pt-[28px] pb-[40px] max-[860px]:px-[16px] max-[860px]:pt-[20px] max-[860px]:pb-[30px]"
  >
    <!-- HEADER -->
    <div class="flex items-center justify-between flex-wrap gap-[14px] mb-[18px]">
      <h2 class="[font-family:var(--font-display)] font-semibold text-[20px] tracking-[-0.015em]">
        Cartes chez les autres joueurs
      </h2>
      <SegToggle v-model="mode" :options="modeOptions" />
    </div>

    <!-- MODE: PAR NOM -->
    <div v-if="mode === 'name'">
      <!-- Search field -->
      <div
        class="flex items-center gap-[10px] pl-[16px] pr-[8px] py-[8px] rounded-[15px] bg-[color-mix(in_srgb,black_22%,transparent)] border border-solid border-[var(--line-2)] transition-all duration-[180ms] ease focus-within:border-[var(--cyan-line)] focus-within:shadow-[0_0_0_4px_var(--cyan-fill)] focus-within:bg-[color-mix(in_srgb,black_14%,transparent)] mb-[20px]"
      >
        <Icon name="lucide:search" size="20" class="text-[var(--ink-3)] flex-none" />
        <input
          value="Sire of Seven Deaths"
          placeholder="Nom de la carte…"
          class="flex-1 border-0 bg-transparent outline-none text-[16px] min-w-0 placeholder:text-[var(--ink-3)]"
        />
      </div>

      <!-- Card header -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] mb-[18px]"
      >
        <div class="flex gap-[18px]">
          <MtgCard color="m" name="Sire of Seven Deaths" class="w-[96px] flex-none" />
          <div class="flex flex-col flex-1 min-w-0 gap-[10px]">
            <div>
              <h3
                class="[font-family:var(--font-display)] font-semibold text-[20px] tracking-[-0.015em] mb-[4px]"
              >
                Sire of Seven Deaths
              </h3>
              <span class="text-[var(--ink-3)] text-[13px]">Creature — Eldrazi · Foundations</span>
            </div>
            <div class="flex flex-wrap gap-[8px]">
              <span
                class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] whitespace-nowrap cursor-default select-none"
              >
                Prix réf.
                <span
                  class="[font-family:var(--font-mono)] [font-feature-settings:'tnum'_1,'ss01'_1] tracking-[-0.01em] text-[var(--cyan)] ml-[4px]"
                  >€31</span
                >
              </span>
              <span
                class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--violet-line)] text-[var(--violet-ink)] bg-[var(--violet-fill)] whitespace-nowrap cursor-default select-none"
              >
                EDHREC
                <span
                  class="[font-family:var(--font-mono)] [font-feature-settings:'tnum'_1,'ss01'_1] tracking-[-0.01em] ml-[4px]"
                  >41 %</span
                >
              </span>
            </div>
            <span class="text-[var(--ink-3)] text-[13px]">14 joueurs possèdent cette carte</span>
          </div>
        </div>
      </div>

      <!-- Sort row -->
      <div class="flex items-center justify-between flex-wrap gap-[10px] mb-[14px]">
        <span class="text-[var(--ink-3)] text-[13px]"
          >{{ owners.length }} résultats à proximité</span
        >
        <div class="flex items-center gap-[8px]">
          <span
            class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
            >trier</span
          >
          <div class="flex gap-[6px]">
            <button
              v-for="chip in sortChips"
              :key="chip.key"
              :class="[
                'inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid whitespace-nowrap cursor-pointer select-none transition-all duration-[150ms] ease',
                sort === chip.key
                  ? 'border-[var(--cyan-line)] text-[var(--cyan-ink)] bg-[var(--cyan-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)]'
                  : 'border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]',
              ]"
              @click="sort = chip.key as typeof sort"
            >
              {{ chip.label }}
            </button>
          </div>
        </div>
      </div>

      <!-- Owner rows -->
      <div class="flex flex-col gap-[8px]">
        <div
          v-for="o in sorted"
          :key="o.u"
          class="flex items-center gap-[13px] px-[14px] py-[11px] rounded-[12px] border border-solid border-[var(--line)] bg-[var(--surface)] transition-all duration-[150ms] ease hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
        >
          <PlayerAvatar :initials="o.init" :online="o.online" />
          <div class="flex-1 min-w-0">
            <div
              class="text-[14px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
            >
              {{ o.u }}
            </div>
            <div class="text-[12px] text-[var(--ink-3)] flex items-center gap-[8px] flex-wrap">
              <span>
                <Icon name="mdi:star" size="11" class="text-[var(--violet)] align-[-1px]" />
                {{ o.rep.toLocaleString('fr-FR') }}
              </span>
              <span>
                <Icon name="lucide:map-pin" size="11" class="align-[-1px]" />
                {{ o.dist }}
              </span>
              <span>×{{ o.qty }} dispo</span>
            </div>
          </div>
          <span
            class="[font-family:var(--font-mono)] text-[14.5px] text-[var(--cyan)] font-semibold"
            >€{{ o.price }}</span
          >
          <NuxtLink
            to="/trade"
            class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
            >Échanger</NuxtLink
          >
        </div>
      </div>
    </div>

    <!-- MODE: PAR DECKLIST -->
    <div
      v-else
      class="grid gap-[22px] items-start [grid-template-columns:minmax(240px,320px)_1fr] max-[860px]:[grid-template-columns:1fr]"
    >
      <!-- Left: paste zone -->
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] flex flex-col gap-[12px] self-start"
      >
        <span
          class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
          >Coller ma decklist</span
        >
        <textarea
          v-model="decklist"
          rows="9"
          class="w-full resize-y [font-family:var(--font-mono)] text-[12.5px] p-[13px] leading-[1.7]"
        />
        <div class="flex items-center justify-between">
          <span class="text-[var(--ink-3)] text-[12px]">99 cartes détectées</span>
          <span
            class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] whitespace-nowrap cursor-default select-none"
          >
            <span class="w-[7px] h-[7px] rounded-full bg-[var(--violet)]" /> 2 non reconnues
          </span>
        </div>
        <button
          class="inline-flex items-center gap-[8px] justify-center px-[15px] py-[9px] rounded-[10px] text-[13.5px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none w-full hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
        >
          <Icon name="lucide:search" size="15" /> Trouver les joueurs
        </button>
      </div>

      <!-- Right: coverage results -->
      <div class="flex-1 min-w-0 flex flex-col gap-[14px]">
        <div class="flex items-center justify-between">
          <h3
            class="[font-family:var(--font-display)] font-semibold text-[16px] tracking-[-0.01em]"
          >
            12 joueurs couvrent ta liste
          </h3>
          <button
            class="inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid border-[var(--cyan-line)] text-[var(--cyan-ink)] bg-[var(--cyan-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)] whitespace-nowrap cursor-pointer select-none transition-all duration-[150ms] ease"
          >
            % couverture <Icon name="lucide:chevron-down" size="13" />
          </button>
        </div>

        <div
          v-for="(c, i) in coverers"
          :key="c.u"
          :class="[
            '[backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[15px]',
            i === 0
              ? 'bg-[var(--cyan-fill)] border border-solid border-[var(--cyan-line)]'
              : 'bg-[var(--glass-bg)] border border-solid border-[var(--line)]',
          ]"
        >
          <div class="flex items-center justify-between mb-[10px]">
            <div class="flex items-center gap-[10px]">
              <PlayerAvatar :initials="c.init" :online="c.online" />
              <span
                class="text-[14.5px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
                >{{ c.u }}</span
              >
            </div>
            <span
              :class="[
                '[font-family:var(--font-mono)] font-bold tracking-[-0.02em] whitespace-nowrap text-[var(--cyan)]',
                i === 0 ? 'text-[24px]' : 'text-[19px]',
              ]"
              >{{ c.pct }}%</span
            >
          </div>
          <div
            class="h-[8px] rounded-full bg-[color-mix(in_srgb,black_30%,transparent)] overflow-hidden border border-solid border-[var(--line-3)]"
          >
            <i
              class="block h-full rounded-full bg-[linear-gradient(90deg,var(--cyan-dim),var(--cyan))] shadow-[0_0_12px_-2px_var(--cyan-glow)] transition-[width] duration-[600ms] [transition-timing-function:cubic-bezier(0.4,0,0.1,1)]"
              :style="{ width: c.pct + '%' }"
            />
          </div>
          <div class="flex items-center justify-between mt-[10px]">
            <span class="text-[var(--ink-3)] text-[12.5px]">
              couvre {{ c.n }}/99 cartes · valeur ≈
              <span
                class="[font-family:var(--font-mono)] [font-feature-settings:'tnum'_1,'ss01'_1] tracking-[-0.01em]"
                >€{{ c.val }}</span
              >
            </span>
            <div v-if="i === 0" class="flex items-center gap-[8px]">
              <button
                class="max-[860px]:hidden inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
              >
                Voir les {{ c.n }}
              </button>
              <NuxtLink
                to="/trade"
                class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-bold text-[var(--on-accent)] bg-[var(--cyan)] border border-solid border-transparent shadow-[0_8px_20px_-14px_var(--cyan-glow)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--cyan-soft)] hover:shadow-[0_0_0_3px_var(--cyan-fill),0_10px_22px_-14px_var(--cyan-glow)] hover:-translate-y-px active:translate-y-0"
                >Composer l'échange</NuxtLink
              >
            </div>
            <NuxtLink
              v-else
              to="/trade"
              class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
              >Composer</NuxtLink
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

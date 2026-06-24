<script setup lang="ts">
import type { SetInfo } from '~/bindings/SetInfo';

const props = defineProps<{
  active: { rar: string[]; sets: string[] };
  setList: SetInfo[];
  priceMin?: number;
  priceMax?: number;
}>();

const emit = defineEmits<{
  toggle: [k: 'rar' | 'sets', v: string];
}>();

const RARITIES = ['Mythique', 'Rare', 'Unco', 'Commune'];

const q = defineModel<string>('q', { default: '' });

const sliderMax = computed(() => props.priceMax ?? 150);

const lo = ref(props.priceMin ?? 0);
const hi = ref(props.priceMax ?? 150);

watch(
  () => props.priceMin,
  (v) => {
    if (v != null) lo.value = v;
  },
  { immediate: true },
);
watch(
  () => props.priceMax,
  (v) => {
    if (v != null) hi.value = v;
  },
  { immediate: true },
);

const pricePct = (v: number) => (v / sliderMax.value) * 100;

const onLoInput = (e: Event) => {
  const val = Number((e.target as HTMLInputElement).value);
  lo.value = Math.min(val, hi.value - 1);
};
const onHiInput = (e: Event) => {
  const val = Number((e.target as HTMLInputElement).value);
  hi.value = Math.max(val, lo.value + 1);
};

const chipClass = (r: string) =>
  props.active.rar.includes(r)
    ? 'text-[var(--cyan-ink)] border-[var(--cyan-line)] bg-[var(--cyan-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)] hover:text-[var(--cyan-ink)] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]'
    : 'text-[var(--ink-2)] bg-[var(--line-3)] border-[var(--line)] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]';

/* ── Combobox set selector ── */
const cbxOpen = ref(false);
const cbxQuery = ref('');
const cbxEl = ref<HTMLElement | null>(null);

onClickOutside(cbxEl, () => {
  cbxOpen.value = false;
});

const filteredSets = computed(() => {
  const qLower = cbxQuery.value.toLowerCase();
  if (!qLower) return props.setList;
  return props.setList.filter(
    (s) => s.name.toLowerCase().includes(qLower) || s.code.toLowerCase().includes(qLower),
  );
});

const setNameByCode = (code: string) =>
  props.setList.find((s) => s.code === code)?.name ?? code.toUpperCase();

const toggleCbx = () => {
  cbxOpen.value = !cbxOpen.value;
  if (!cbxOpen.value) cbxQuery.value = '';
};

const clearSets = () => {
  [...props.active.sets].forEach((code) => emit('toggle', 'sets', code));
};
</script>

<template>
  <div class="flex flex-col gap-[18px] h-full">
    <!-- Search -->
    <div
      class="flex items-center gap-[10px] px-3 py-[9px] rounded-[12px] bg-[color-mix(in_srgb,black_4%,transparent)] border border-[var(--line-2)] transition-all duration-[180ms] ease focus-within:border-[var(--cyan-line)] focus-within:shadow-[0_0_0_4px_var(--cyan-fill),0_0_28px_-8px_var(--cyan-glow)] focus-within:bg-[color-mix(in_srgb,black_14%,transparent)]"
    >
      <Icon name="lucide:search" :size="16" class="text-[var(--ink-3)] shrink-0" />
      <input
        v-model="q"
        placeholder="Filtrer ma collection…"
        class="flex-1 border-0 bg-transparent outline-none text-[14.5px] text-[var(--ink)] min-w-0 placeholder:text-[var(--ink-3)]"
      />
    </div>

    <!-- Rarity -->
    <div class="flex flex-col gap-[9px]">
      <span
        class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
        >Rareté</span
      >
      <div class="flex flex-wrap gap-[7px]">
        <button
          v-for="r in RARITIES"
          :key="r"
          :class="[
            'inline-flex items-center gap-[6px] px-[11px] py-[5px] rounded-full text-[12.5px] font-medium border border-solid transition-all duration-[150ms] ease whitespace-nowrap cursor-pointer select-none',
            chipClass(r),
          ]"
          @click="emit('toggle', 'rar', r)"
        >
          {{ r }}
        </button>
      </div>
    </div>

    <!-- Price range -->
    <div class="flex flex-col gap-[9px]">
      <span
        class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
        >Plage de prix</span
      >
      <div
        class="bg-[color-mix(in_srgb,black_22%,transparent)] border border-[var(--line-3)] rounded-[12px] pt-[15px] px-[14px] pb-3"
      >
        <div class="relative h-[20px]">
          <div
            class="absolute left-0 right-0 top-1/2 -translate-y-1/2 h-[4px] rounded-full bg-[color-mix(in_srgb,black_36%,var(--line-2))]"
          />
          <div
            class="absolute top-1/2 -translate-y-1/2 h-[4px] rounded-full bg-[var(--cyan)] shadow-[0_0_10px_-2px_var(--cyan-glow)]"
            :style="{ left: pricePct(lo) + '%', right: 100 - pricePct(hi) + '%' }"
          />
          <input
            type="range"
            min="0"
            :max="sliderMax"
            :value="lo"
            :style="{ zIndex: lo > sliderMax * 0.92 ? 5 : 3 }"
            class="z-[3]"
            aria-label="Prix minimum"
            @input="onLoInput"
          />
          <input
            type="range"
            min="0"
            :max="sliderMax"
            :value="hi"
            class="z-[4]"
            aria-label="Prix maximum"
            @input="onHiInput"
          />
        </div>
        <div class="flex justify-between mt-[13px] gap-2">
          <span
            class="text-[12px] px-[9px] py-[3px] rounded-[7px] bg-[var(--cyan-fill)] border border-[var(--cyan-line)] text-[var(--cyan-ink)]"
            >€{{ lo }}</span
          >
          <span class="[font-family:var(--font-mono)] text-[var(--ink-3)] text-[11px]">—</span>
          <span
            class="text-[12px] px-[9px] py-[3px] rounded-[7px] bg-[var(--cyan-fill)] border border-[var(--cyan-line)] text-[var(--cyan-ink)]"
            >€{{ hi }}{{ hi >= sliderMax ? '+' : '' }}</span
          >
        </div>
      </div>
    </div>

    <!-- Set / Extension — combobox -->
    <div class="flex flex-col gap-[9px]">
      <span
        class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
        >Set / Extension</span
      >
      <div ref="cbxEl" class="relative">
        <!-- Trigger -->
        <button
          type="button"
          :class="[
            'w-full min-h-[42px] flex items-center gap-2 py-[7px] pr-[10px] pl-3 rounded-[12px] text-left cursor-pointer border border-[var(--line-2)] bg-[var(--line-3)] hover:bg-[var(--surface-2)] transition-[border-color,box-shadow,background] duration-[160ms] ease',
            cbxOpen
              ? 'border-[var(--cyan-line)] shadow-[0_0_0_4px_var(--cyan-fill),0_0_28px_-8px_var(--cyan-glow)]'
              : '',
          ]"
          aria-haspopup="listbox"
          :aria-expanded="cbxOpen"
          @click="toggleCbx"
        >
          <span v-if="active.sets.length === 0" class="flex-1 text-[var(--ink-3)] text-[13.5px]"
            >Toutes les extensions</span
          >
          <span v-else class="flex-1 flex flex-wrap gap-[6px] min-w-0">
            <span
              v-for="code in active.sets"
              :key="code"
              class="inline-flex items-center gap-[6px] py-[3px] pr-1 pl-2 rounded-[8px] bg-[var(--cyan-fill)] border border-[var(--cyan-line)] text-[var(--cyan-ink)] text-[12px] font-medium max-w-full"
            >
              <i :class="['ss', 'text-[14px]', `ss-${code.toLowerCase()}`]" />
              <span class="overflow-hidden text-ellipsis whitespace-nowrap max-w-[120px]">{{
                setNameByCode(code)
              }}</span>
              <i
                class="grid place-items-center w-4 h-4 rounded-[5px] text-[var(--cyan-ink)] opacity-65 flex-none cursor-pointer hover:opacity-100 hover:bg-[color-mix(in_srgb,var(--accent)_22%,transparent)]"
                role="button"
                :aria-label="`Retirer ${setNameByCode(code)}`"
                @click.stop="emit('toggle', 'sets', code)"
              >
                <Icon name="lucide:x" :size="11" />
              </i>
            </span>
          </span>
          <span
            :class="[
              'inline-flex text-[var(--ink-3)] flex-none transition-transform duration-200 ease',
              cbxOpen ? 'rotate-180' : '',
            ]"
          >
            <Icon name="lucide:chevron-down" :size="16" />
          </span>
        </button>

        <!-- Popover -->
        <Transition name="cbx-pop">
          <div
            v-if="cbxOpen"
            class="absolute left-0 right-0 top-[calc(100%+8px)] z-40 rounded-[13px] border border-[var(--line-2)] bg-[var(--surface-2)] shadow-[0_22px_48px_-26px_rgba(0,0,0,0.92)] p-[9px]"
            role="listbox"
            aria-multiselectable="true"
          >
            <!-- Search inside popup -->
            <div
              class="flex items-center gap-2 px-[11px] py-2 rounded-[10px] bg-[color-mix(in_srgb,black_14%,transparent)] border border-[var(--line-2)]"
            >
              <Icon name="lucide:search" :size="15" class="text-[var(--ink-3)] shrink-0" />
              <input
                v-model="cbxQuery"
                placeholder="Filtrer une extension…"
                class="flex-1 border-0 bg-transparent outline-none text-[13.5px] text-[var(--ink)] min-w-0 placeholder:text-[var(--ink-3)]"
              />
            </div>

            <!-- Options list -->
            <div
              class="flex flex-col gap-[2px] max-h-[240px] overflow-y-auto overflow-x-hidden mt-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
            >
              <div
                v-if="filteredSets.length === 0"
                class="py-4 px-1 text-center text-[var(--ink-4)] text-[12.5px]"
              >
                Aucune extension
              </div>
              <button
                v-for="set in filteredSets"
                :key="set.code"
                type="button"
                :class="[
                  'group grid grid-cols-[24px_1fr] items-center gap-[10px] w-full text-left py-2 px-[9px] rounded-[8px] cursor-pointer transition-colors duration-[120ms] ease',
                  active.sets.includes(set.code)
                    ? 'on bg-[var(--cyan-fill)]'
                    : 'hover:bg-[var(--surface-3)]',
                ]"
                role="option"
                :aria-selected="active.sets.includes(set.code)"
                @click="emit('toggle', 'sets', set.code)"
              >
                <span
                  class="grid place-items-center text-[var(--ink-2)] group-[.on]:text-[var(--cyan-ink)]"
                >
                  <i :class="['ss', 'text-[18px]', `ss-${set.code.toLowerCase()}`]" />
                </span>
                <span
                  class="text-[13.5px] text-[var(--ink)] min-w-0 overflow-hidden text-ellipsis whitespace-nowrap group-[.on]:text-[var(--cyan-ink)] group-[.on]:font-medium"
                  >{{ set.name }}</span
                >
              </button>
            </div>

            <!-- Clear all -->
            <button
              v-if="active.sets.length > 0"
              type="button"
              class="w-full mt-2 p-2 rounded-[9px] border border-[var(--line)] [font-family:var(--font-mono)] text-[11px] font-semibold tracking-[0.03em] text-[var(--ink-3)] bg-[color-mix(in_srgb,black_14%,transparent)] transition-all duration-[140ms] ease cursor-pointer hover:text-[var(--cyan-ink)] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]"
              @click="clearSets"
            >
              Tout effacer ({{ active.sets.length }})
            </button>
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Range inputs */
input[type='range'] {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  width: 100%;
  height: 20px;
  margin: 0;
  -webkit-appearance: none;
  appearance: none;
  background: none;
  pointer-events: none;
}

input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  pointer-events: auto;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  cursor: grab;
  background: radial-gradient(circle at 35% 30%, #fff, var(--cyan));
  border: 1px solid var(--cyan);
  box-shadow:
    0 0 0 3px var(--cyan-fill),
    0 2px 6px rgba(0, 0, 0, 0.5);
  transition: box-shadow 0.15s;
}

input[type='range']::-webkit-slider-thumb:active {
  cursor: grabbing;
  box-shadow:
    0 0 0 5px var(--cyan-fill),
    0 2px 6px rgba(0, 0, 0, 0.5);
}

input[type='range']::-moz-range-thumb {
  pointer-events: auto;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  cursor: grab;
  background: radial-gradient(circle at 35% 30%, #fff, var(--cyan));
  border: 1px solid var(--cyan);
  box-shadow:
    0 0 0 3px var(--cyan-fill),
    0 2px 6px rgba(0, 0, 0, 0.5);
}

/* Popover transition */
.cbx-pop-enter-active,
.cbx-pop-leave-active {
  transition:
    opacity 0.14s ease,
    transform 0.14s ease;
}

.cbx-pop-enter-from,
.cbx-pop-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>

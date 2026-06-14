<script setup lang="ts">
const props = defineProps<{
  active: { rar: string[]; sets: string[] };
  setCounts: Record<string, number>;
  setList: string[];
}>();

const emit = defineEmits<{
  toggle: [k: 'rar' | 'sets', v: string];
}>();

const RARITIES = ['Mythique', 'Rare', 'Unco', 'Commune'];

const q = defineModel<string>('q', { default: '' });

const setScrollEl = ref<HTMLElement | null>(null);
const { arrivedState: setArrived } = useScroll(setScrollEl);

const lo = ref(0);
const hi = ref(150);

const pricePct = (v: number) => (v / 150) * 100;

const onLoInput = (e: Event) => {
  const val = Number((e.target as HTMLInputElement).value);
  lo.value = Math.min(val, hi.value - 1);
};
const onHiInput = (e: Event) => {
  const val = Number((e.target as HTMLInputElement).value);
  hi.value = Math.max(val, lo.value + 1);
};

const setTitle = (code: string) => {
  const name = code.toUpperCase();
  const count = props.setCounts[code] || 0;
  return `${name} · ${count} carte${count > 1 ? 's' : ''}`;
};

const chipClass = (r: string) =>
  props.active.rar.includes(r)
    ? 'text-[var(--cyan-ink)] border-[var(--cyan-line)] bg-[var(--cyan-fill)] shadow-[inset_0_0_0_1px_var(--cyan-fill)] hover:text-[var(--cyan-ink)] hover:border-[var(--cyan-line)] hover:bg-[var(--cyan-fill)]'
    : 'text-[var(--ink-2)] bg-[var(--line-3)] border-[var(--line)] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]';

const setPipClass = (code: string) =>
  props.active.sets.includes(code)
    ? 'text-[var(--accent)] border-[var(--accent)] bg-[var(--cyan-fill)] shadow-[0_2px_10px_-3px_var(--accent)] hover:text-[var(--accent)] hover:border-[var(--accent)]'
    : 'text-[var(--ink-2)] border-[var(--line)] hover:text-[var(--ink)] hover:border-[var(--line-2)]';
</script>

<template>
  <div class="flex flex-col gap-[18px] h-full">
    <!-- Search -->
    <div
      class="flex items-center gap-[10px] px-3 py-[9px] rounded-[12px] bg-[color-mix(in_srgb,black_22%,transparent)] border border-[var(--line-2)] transition-all duration-[180ms] ease focus-within:border-[var(--cyan-line)] focus-within:shadow-[0_0_0_4px_var(--cyan-fill),0_0_28px_-8px_var(--cyan-glow)] focus-within:bg-[color-mix(in_srgb,black_14%,transparent)]"
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
            max="150"
            :value="lo"
            :style="{ zIndex: lo > 138 ? 5 : 3 }"
            class="z-[3]"
            aria-label="Prix minimum"
            @input="onLoInput"
          />
          <input
            type="range"
            min="0"
            max="150"
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
            >€{{ hi }}{{ hi >= 150 ? '+' : '' }}</span
          >
        </div>
      </div>
    </div>

    <!-- Set / Extension -->
    <div class="flex flex-col gap-[9px] flex-1 min-h-0">
      <span
        class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
        >Set / Extension</span
      >
      <div class="relative flex-1 min-h-0">
        <Transition name="fade-grad">
          <div
            v-if="!setArrived.top"
            class="absolute top-0 inset-x-0 h-[22px] z-10 pointer-events-none"
            style="background: linear-gradient(to bottom, var(--glass-bg), transparent)"
          />
        </Transition>
        <div ref="setScrollEl" class="set-scroll h-full overflow-y-auto">
          <div class="grid grid-cols-4 gap-[6px] justify-items-center pb-[2px]">
            <button
              v-for="code in setList"
              :key="code"
              :class="[
                'relative grid place-items-center w-[34px] h-[34px] shrink-0 border border-solid bg-[var(--surface-2)] rounded-[9px] cursor-pointer transition-[color,border-color,background,transform,box-shadow] duration-[140ms] ease hover:-translate-y-px',
                setPipClass(code),
              ]"
              :title="setTitle(code)"
              :aria-label="code.toUpperCase()"
              :aria-pressed="active.sets.includes(code)"
              @click="emit('toggle', 'sets', code)"
            >
              <i :class="['ss text-[19px]', `ss-${code.toLowerCase()}`]" />
            </button>
          </div>
        </div>
        <Transition name="fade-grad">
          <div
            v-if="!setArrived.bottom"
            class="absolute bottom-0 inset-x-0 h-[22px] z-10 pointer-events-none"
            style="background: linear-gradient(to top, var(--glass-bg), transparent)"
          />
        </Transition>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Range inputs — pseudo-element styling can't be done with Tailwind */
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

.set-scroll {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.set-scroll::-webkit-scrollbar {
  display: none;
}

.fade-grad-enter-active,
.fade-grad-leave-active {
  transition: opacity 0.15s ease;
}
.fade-grad-enter-from,
.fade-grad-leave-to {
  opacity: 0;
}
</style>

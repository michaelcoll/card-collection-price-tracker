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
    ? 'text-cyan-700 dark:text-cyan-300 border-cyan-500/30 dark:border-cyan-400/30 bg-cyan-500/10 dark:bg-cyan-400/10'
    : 'text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 border-slate-200 dark:border-white/10 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800';

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
  <div class="flex flex-col gap-4 h-full">
    <!-- Search -->
    <div
      class="flex items-center gap-2.5 px-3 py-2 rounded-xl bg-slate-100 dark:bg-black/20 border border-slate-300 dark:border-white/15 transition-all duration-200 focus-within:border-cyan-500/40 dark:focus-within:border-cyan-400/40 focus-within:ring-4 focus-within:ring-cyan-500/10 focus-within:bg-slate-200 dark:focus-within:bg-black/30"
    >
      <Icon name="lucide:search" :size="16" class="text-slate-400 dark:text-slate-500 shrink-0" />
      <input
        v-model="q"
        placeholder="Filtrer ma collection…"
        class="flex-1 border-0 bg-transparent outline-none text-sm text-slate-800 dark:text-slate-100 min-w-0 placeholder:text-slate-400 dark:placeholder:text-slate-500"
      />
    </div>

    <!-- Rarity -->
    <div class="flex flex-col gap-2">
      <span
        class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
        >Rareté</span
      >
      <div class="flex flex-wrap gap-2">
        <button
          v-for="r in RARITIES"
          :key="r"
          :class="[
            'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium border transition-all duration-150 whitespace-nowrap cursor-pointer select-none',
            chipClass(r),
          ]"
          @click="emit('toggle', 'rar', r)"
        >
          {{ r }}
        </button>
      </div>
    </div>

    <!-- Price range -->
    <div class="flex flex-col gap-2">
      <span
        class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
        >Plage de prix</span
      >
      <div
        class="bg-slate-100 dark:bg-black/20 border border-slate-200 dark:border-white/5 rounded-xl pt-4 px-3.5 pb-3"
      >
        <div class="relative h-5">
          <div
            class="absolute left-0 right-0 top-1/2 -translate-y-1/2 h-1 rounded-full bg-slate-300 dark:bg-zinc-700"
          />
          <div
            class="absolute top-1/2 -translate-y-1/2 h-1 rounded-full bg-cyan-500 dark:bg-cyan-400"
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
        <div class="flex justify-between mt-3 gap-2">
          <span
            class="text-xs px-2.5 py-0.5 rounded-md bg-cyan-500/10 dark:bg-cyan-400/10 border border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300"
            >€{{ lo }}</span
          >
          <span class="font-mono text-slate-400 dark:text-slate-500 text-xs">—</span>
          <span
            class="text-xs px-2.5 py-0.5 rounded-md bg-cyan-500/10 dark:bg-cyan-400/10 border border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300"
            >€{{ hi }}{{ hi >= sliderMax ? '+' : '' }}</span
          >
        </div>
      </div>
    </div>

    <!-- Set / Extension — combobox -->
    <div class="flex flex-col gap-2">
      <span
        class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
        >Set / Extension</span
      >
      <div ref="cbxEl" class="relative">
        <!-- Trigger -->
        <button
          type="button"
          :class="[
            'w-full min-h-[42px] flex items-center gap-2 py-2 pr-2.5 pl-3 rounded-xl text-left cursor-pointer border border-slate-300 dark:border-white/15 bg-slate-100 dark:bg-black/20 hover:bg-slate-200 dark:hover:bg-black/30 transition-[border-color,box-shadow,background] duration-150',
            cbxOpen ? 'border-cyan-500/40 dark:border-cyan-400/40 ring-4 ring-cyan-500/10' : '',
          ]"
          aria-haspopup="listbox"
          :aria-expanded="cbxOpen"
          @click="toggleCbx"
        >
          <span
            v-if="active.sets.length === 0"
            class="flex-1 text-slate-400 dark:text-slate-500 text-sm"
            >Toutes les extensions</span
          >
          <span v-else class="flex-1 flex flex-wrap gap-1.5 min-w-0">
            <span
              v-for="code in active.sets"
              :key="code"
              class="inline-flex items-center gap-1.5 py-0.5 pr-1 pl-2 rounded-lg bg-cyan-500/10 dark:bg-cyan-400/10 border border-cyan-500/30 dark:border-cyan-400/30 text-cyan-700 dark:text-cyan-300 text-xs font-medium max-w-full"
            >
              <i :class="['ss', 'text-sm', `ss-${code.toLowerCase()}`]" />
              <span class="overflow-hidden text-ellipsis whitespace-nowrap max-w-[120px]">{{
                setNameByCode(code)
              }}</span>
              <i
                class="grid place-items-center w-4 h-4 rounded text-cyan-700 dark:text-cyan-300 opacity-70 flex-none cursor-pointer hover:opacity-100 hover:bg-cyan-500/20"
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
              'inline-flex text-slate-400 dark:text-slate-500 flex-none transition-transform duration-200',
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
            class="absolute left-0 right-0 top-[calc(100%+8px)] z-40 rounded-xl border border-slate-300 dark:border-white/15 bg-white dark:bg-zinc-800 shadow-2xl p-2.5"
            role="listbox"
            aria-multiselectable="true"
          >
            <!-- Search inside popup -->
            <div
              class="flex items-center gap-2 px-3 py-2 rounded-lg bg-slate-100 dark:bg-black/20 border border-slate-200 dark:border-white/10"
            >
              <Icon
                name="lucide:search"
                :size="15"
                class="text-slate-400 dark:text-slate-500 shrink-0"
              />
              <input
                v-model="cbxQuery"
                placeholder="Filtrer une extension…"
                class="flex-1 border-0 bg-transparent outline-none text-sm text-slate-800 dark:text-slate-100 min-w-0 placeholder:text-slate-400 dark:placeholder:text-slate-500"
              />
            </div>

            <!-- Options list -->
            <div
              class="flex flex-col gap-0.5 max-h-60 overflow-y-auto overflow-x-hidden mt-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
            >
              <div
                v-if="filteredSets.length === 0"
                class="py-4 px-1 text-center text-slate-300 dark:text-slate-600 text-xs"
              >
                Aucune extension
              </div>
              <button
                v-for="set in filteredSets"
                :key="set.code"
                type="button"
                :class="[
                  'group grid grid-cols-[24px_1fr] items-center gap-2.5 w-full text-left py-2 px-2.5 rounded-lg cursor-pointer transition-colors duration-100',
                  active.sets.includes(set.code)
                    ? 'on bg-cyan-500/10 dark:bg-cyan-400/10'
                    : 'hover:bg-slate-100 dark:hover:bg-zinc-700',
                ]"
                role="option"
                :aria-selected="active.sets.includes(set.code)"
                @click="emit('toggle', 'sets', set.code)"
              >
                <span
                  class="grid place-items-center text-slate-600 dark:text-slate-300 group-[.on]:text-cyan-700 dark:group-[.on]:text-cyan-300"
                >
                  <i :class="['ss', 'text-lg', `ss-${set.code.toLowerCase()}`]" />
                </span>
                <span
                  class="text-sm text-slate-800 dark:text-slate-100 min-w-0 overflow-hidden text-ellipsis whitespace-nowrap group-[.on]:text-cyan-700 dark:group-[.on]:text-cyan-300 group-[.on]:font-medium"
                  >{{ set.name }}</span
                >
              </button>
            </div>

            <!-- Clear all -->
            <button
              v-if="active.sets.length > 0"
              type="button"
              class="w-full mt-2 p-2 rounded-lg border border-slate-200 dark:border-white/10 font-mono text-xs font-semibold tracking-wide text-slate-400 dark:text-slate-500 bg-black/10 transition-all duration-150 cursor-pointer hover:text-cyan-700 dark:hover:text-cyan-300 hover:border-cyan-500/30 dark:hover:border-cyan-400/30 hover:bg-cyan-500/10 dark:hover:bg-cyan-400/10"
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
    0 0 0 3px rgba(34, 211, 238, 0.18),
    0 2px 6px rgba(0, 0, 0, 0.5);
  transition: box-shadow 0.15s;
}

input[type='range']::-webkit-slider-thumb:active {
  cursor: grabbing;
  box-shadow:
    0 0 0 5px rgba(34, 211, 238, 0.18),
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
    0 0 0 3px rgba(34, 211, 238, 0.18),
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

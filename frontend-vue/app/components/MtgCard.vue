<script setup lang="ts">
const props = defineProps<{
  scryfallId?: string;
  qty?: number;
  clickable?: boolean;
  mini?: boolean;
  foil?: boolean;
  name?: string;
  size?: 'sm' | 'md' | 'lg';
}>();

const emit = defineEmits(['click']);

const src = computed(
  () =>
    (props.scryfallId &&
      `https://api.scryfall.com/cards/${props.scryfallId}?format=image&version=normal`) ||
    '',
);

const cardRef = ref<HTMLElement | null>(null);

let raf: number | null = null;

const updateFoil = () => {
  const el = cardRef.value;
  if (!el || !props.foil) return;
  const r = el.getBoundingClientRect();
  const vh = window.innerHeight || 1;
  const p = (r.top + r.height / 2) / vh;
  el.style.setProperty('--foil', Math.max(-0.5, Math.min(1.5, p)).toFixed(4));
};

const onScroll = () => {
  if (!raf)
    raf = requestAnimationFrame(() => {
      raf = null;
      updateFoil();
    });
};

onMounted(() => {
  if (!props.foil) return;
  updateFoil();
  window.addEventListener('scroll', onScroll, { passive: true, capture: true });
  window.addEventListener('resize', onScroll);
});

onUnmounted(() => {
  window.removeEventListener('scroll', onScroll, { capture: true });
  window.removeEventListener('resize', onScroll);
  if (raf) cancelAnimationFrame(raf);
});
</script>

<template>
  <div
    ref="cardRef"
    :class="[
      'relative aspect-[5/7] flex flex-col gap-0 overflow-hidden',
      'transition-[transform,box-shadow,border-color] duration-200 ease',
      'border',
      'rounded-[4%]',
      src
        ? 'p-0 bg-zinc-950 border-black/55'
        : [
            mini ? 'p-0.5' : 'p-[5%]',
            'border-slate-300 dark:border-white/15 bg-slate-100 dark:bg-zinc-800 shadow-lg',
          ],
      clickable
        ? 'cursor-pointer hover:-translate-y-1 hover:border-cyan-500/30 dark:hover:border-cyan-400/30 hover:shadow-xl'
        : '',
      foil ? 'foil' : '',
    ]"
    :title="name"
    @click="emit('click')"
  >
    <!-- qty badge -->
    <span
      v-if="qty != null"
      class="absolute top-1.5 right-1.5 z-[5] font-mono text-xs font-semibold px-1.5 py-0.5 rounded-full text-zinc-100 bg-black/60 border border-white/20 backdrop-blur-sm"
      >×{{ qty }}</span
    >

    <!-- inner vignette overlay — replaces ::after pseudo-element -->
    <div
      v-if="!src"
      :class="[
        'absolute inset-0 pointer-events-none z-[1]',
        mini
          ? 'rounded shadow-[inset_0_0_0_2px_rgba(0,0,0,0.55)]'
          : 'rounded-md shadow-[inset_0_0_0_4px_rgba(0,0,0,0.55)]',
      ]"
    />

    <!-- real card scan -->
    <template v-if="src">
      <img
        class="absolute inset-0 z-0 w-full h-full object-cover block select-none"
        style="border-radius: inherit"
        :src="src"
        :alt="name ?? ''"
        loading="lazy"
        draggable="false"
      />
    </template>

    <!-- mini placeholder -->
    <template v-else-if="mini">
      <div
        class="flex-1 rounded relative z-[2] border border-slate-200 dark:border-white/10 bg-slate-100 dark:bg-zinc-800 grid place-items-center overflow-hidden"
      >
        <AppIcon
          name="mountain"
          class="text-slate-300 dark:text-slate-600 opacity-50"
          style="width: 46%; height: 46%"
        />
      </div>
    </template>

    <!-- full placeholder -->
    <template v-else>
      <!-- title bar -->
      <div
        class="flex items-center justify-between gap-1.5 px-[6%] py-[5%] bg-slate-200 dark:bg-zinc-800 border border-slate-200 dark:border-white/10 rounded relative z-[2]"
      >
        <span class="h-1.5 rounded-sm bg-slate-300 dark:bg-slate-600 w-[60%]" />
        <span
          class="w-3 h-3 rounded-full shrink-0 bg-zinc-700 shadow-[inset_0_1px_1px_rgba(255,255,255,0.25)]"
        />
      </div>
      <!-- art -->
      <div
        class="flex-1 mt-[5%] rounded relative z-[2] border border-slate-200 dark:border-white/10 bg-slate-100 dark:bg-zinc-800 grid place-items-center overflow-hidden"
      >
        <AppIcon
          name="mountain"
          :size="28"
          class="text-slate-300 dark:text-slate-600 opacity-50"
          style="width: 34%; height: 34%"
        />
      </div>
      <!-- type bar -->
      <div
        class="mt-[5%] flex items-center gap-1.5 px-[6%] py-[5%] bg-slate-200 dark:bg-zinc-800 border border-slate-200 dark:border-white/10 rounded relative z-[2]"
      >
        <span class="h-1.5 w-[50%] rounded-sm bg-slate-300 dark:bg-slate-600 opacity-70" />
        <span
          class="w-2 h-2 rounded-full shrink-0 bg-zinc-700 shadow-[inset_0_1px_1px_rgba(255,255,255,0.25)]"
        />
      </div>
      <!-- text box -->
      <div
        class="mt-[5%] shrink-0 basis-[22%] rounded relative z-[2] bg-slate-100 dark:bg-zinc-800 border border-slate-200 dark:border-white/10 p-[6%] flex flex-col gap-[14%]"
      >
        <span
          class="h-[3.5px] rounded-sm bg-slate-300 dark:bg-slate-600 opacity-50 block w-[92%]"
        />
        <span
          class="h-[3.5px] rounded-sm bg-slate-300 dark:bg-slate-600 opacity-50 block w-[78%]"
        />
        <span
          class="h-[3.5px] rounded-sm bg-slate-300 dark:bg-slate-600 opacity-50 block w-[85%]"
        />
      </div>
    </template>

    <!-- foil holographic overlay -->
    <span v-if="foil" class="foil-fx" aria-hidden="true" />
  </div>
</template>

<style scoped>
.foil-fx {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  z-index: 4;
  background:
    linear-gradient(115deg, transparent 30%, rgba(255, 255, 255, 0.45) 44%, transparent 52%),
    linear-gradient(
      60deg,
      hsla(190, 95%, 68%, 0.55),
      hsla(265, 95%, 72%, 0.55) 22%,
      hsla(325, 95%, 70%, 0.55) 42%,
      hsla(45, 95%, 68%, 0.55) 62%,
      hsla(140, 90%, 66%, 0.55) 82%,
      hsla(190, 95%, 68%, 0.55)
    );
  background-size:
    230% 230%,
    260% 260%;
  background-position:
    calc(var(--foil, 0.5) * 100%) calc(var(--foil, 0.5) * 100%),
    calc((1 - var(--foil, 0.5)) * 100%) calc(var(--foil, 0.5) * 100%);
  mix-blend-mode: soft-light;
  opacity: 0.85;
  transition: background-position 0.12s linear;
}

.foil::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: inherit;
  z-index: 4;
  pointer-events: none;
  background: linear-gradient(
    115deg,
    transparent 38%,
    rgba(255, 255, 255, 0.55) 47%,
    transparent 56%
  );
  background-size: 300% 300%;
  background-position: calc(140% - var(--foil, 0.5) * 240%) 0;
  mix-blend-mode: screen;
  opacity: 0.5;
  transition: background-position 0.12s linear;
}

@media (prefers-reduced-motion: reduce) {
  .foil-fx,
  .foil::before {
    transition: none;
  }
}
</style>

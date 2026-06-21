<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    data: number[];
    height?: number;
    hi?: string;
    lo?: string;
    valueFmt?: (v?: number) => string;
  }>(),
  {
    height: 200,
    hi: '€4,3k',
    lo: '€4,1k',
    valueFmt: (v?: number) => '€' + (v ?? 0).toLocaleString('fr-FR'),
  },
);

const W = 600;
const H = 200;
const graphRef = ref<HTMLElement | null>(null);
const hover = ref<{ i: number; px: number; py: number; v: number } | null>(null);

const minVal = computed(() => Math.min(...props.data));
const maxVal = computed(() => Math.max(...props.data));
const pad = computed(() => (maxVal.value - minVal.value) * 0.15 || 1);

const xs = (i: number) => (i / (props.data.length - 1)) * W;
const ys = (v: number) => {
  const lo = minVal.value - pad.value;
  const hi = maxVal.value + pad.value;
  return H - ((v - lo) / (hi - lo)) * H;
};

const linePts = computed(() => props.data.map((v, i) => `${xs(i)},${ys(v)}`).join(' '));
const fillPts = computed(() => `0,${H} ${linePts.value} ${W},${H}`);

const onMove = (e: MouseEvent) => {
  const r = graphRef.value?.getBoundingClientRect();
  if (!r) return;
  const x = ((e.clientX - r.left) / r.width) * W;
  let i = Math.round((x / W) * (props.data.length - 1));
  i = Math.max(0, Math.min(props.data.length - 1, i));
  hover.value = {
    i,
    px: (xs(i) / W) * 100,
    py: (ys(props.data[i] ?? 0) / H) * 100,
    v: props.data[i] ?? 0,
  };
};
</script>

<template>
  <div
    ref="graphRef"
    class="relative w-full rounded-[12px] bg-[color-mix(in_srgb,black_18%,transparent)] border border-solid border-[var(--line-3)] overflow-hidden p-[2px]"
    :style="{ height: height + 'px' }"
    @mousemove="onMove"
    @mouseleave="hover = null"
  >
    <svg
      :viewBox="`0 0 ${W} ${H}`"
      preserveAspectRatio="none"
      class="w-full h-full block overflow-visible"
    >
      <defs>
        <linearGradient id="gfill" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="var(--cyan)" stop-opacity="0.34" />
          <stop offset="100%" stop-color="var(--cyan)" stop-opacity="0" />
        </linearGradient>
      </defs>
      <polygon class="[fill:url(#gfill)] [stroke:none]" :points="fillPts" />
      <polyline
        class="[fill:none] [stroke:var(--cyan)] [stroke-width:2.5] [filter:drop-shadow(0_0_6px_var(--cyan-glow))]"
        :points="linePts"
      />
      <line
        v-if="hover"
        class="[stroke:var(--line-2)] [stroke-width:1] [stroke-dasharray:3_3]"
        :x1="xs(hover.i)"
        y1="0"
        :x2="xs(hover.i)"
        :y2="H"
      />
      <circle
        v-if="hover"
        class="[fill:var(--cyan)] [stroke:var(--bg)] [stroke-width:2]"
        :cx="xs(hover.i)"
        :cy="ys(hover.v)"
        r="4.5"
        vector-effect="non-scaling-stroke"
      />
    </svg>
    <span
      class="absolute left-[6px] top-[5px] [font-family:var(--font-mono)] text-[10px] text-[var(--ink-3)]"
      >{{ hi }}</span
    >
    <span
      class="absolute left-[6px] bottom-[5px] [font-family:var(--font-mono)] text-[10px] text-[var(--ink-3)]"
      >{{ lo }}</span
    >
    <div
      v-if="hover"
      class="absolute pointer-events-none bg-[var(--surface-2)] border border-solid border-[var(--line-2)] rounded-[8px] px-[9px] py-[5px] [font-family:var(--font-mono)] text-[11px] whitespace-nowrap shadow-[0_8px_20px_-8px_rgba(0,0,0,0.8)] [transform:translate(-50%,-130%)]"
      :style="{ left: hover.px + '%', top: hover.py + '%' }"
    >
      {{ valueFmt?.(hover.v) }}
    </div>
  </div>
</template>

<script setup lang="ts">
interface Option {
  value: string;
  label?: string;
  icon?: string;
  tone?: string;
  title?: string;
}

const props = defineProps<{
  modelValue: string;
  options: Option[];
  size?: 'sm';
}>();

const emit = defineEmits<{ 'update:modelValue': [value: string] }>();

const containerRef = ref<HTMLElement | null>(null);
const thumb = ref({ left: 3, width: 0, opacity: 0 });

const currentIndex = computed(() => props.options.findIndex((o) => o.value === props.modelValue));

const updateThumb = () => {
  const el = containerRef.value;
  if (!el) return;
  const idx = currentIndex.value < 0 ? 0 : currentIndex.value;
  const btn = el.querySelectorAll('button')[idx] as HTMLElement | undefined;
  if (btn) {
    thumb.value = { left: btn.offsetLeft, width: btn.offsetWidth, opacity: 1 };
  }
};

let ro: ResizeObserver | null = null;

onMounted(() => {
  nextTick(updateThumb);
  ro = new ResizeObserver(() => nextTick(updateThumb));
  if (containerRef.value) ro.observe(containerRef.value);
});

onUnmounted(() => ro?.disconnect());

watch(
  () => [props.modelValue, props.options.length],
  () => nextTick(updateThumb),
);

const btnActiveColor = (o: Option) => {
  if (o.value !== props.modelValue) return 'text-slate-600 dark:text-slate-300';
  if (o.tone === 'cyan') return 'text-cyan-600 dark:text-cyan-400';
  if (o.tone === 'vio') return 'text-violet-500 dark:text-violet-300';
  return 'text-slate-800 dark:text-slate-100';
};
</script>

<template>
  <div
    ref="containerRef"
    class="relative inline-flex gap-0.5 rounded-xl border border-slate-300 bg-slate-200 p-1 dark:border-white/10 dark:bg-black/20"
  >
    <span
      class="absolute top-0.5 bottom-0.5 z-0 rounded-lg border border-slate-300 bg-white shadow-md [transition:left_0.26s_cubic-bezier(0.5,1.3,0.5,1),width_0.26s_cubic-bezier(0.5,1.3,0.5,1)] dark:border-white/15 dark:bg-zinc-800"
      :style="{ left: thumb.left + 'px', width: thumb.width + 'px', opacity: thumb.opacity }"
    />
    <button
      v-for="o in options"
      :key="o.value"
      :title="o.title"
      :aria-label="o.title"
      :class="[
        'relative z-10 inline-flex items-center justify-center rounded-lg font-semibold whitespace-nowrap transition-colors duration-200',
        size === 'sm' ? 'px-2.5 py-1.5 text-xs' : 'px-3.5 py-2 text-xs',
        btnActiveColor(o),
      ]"
      @click="emit('update:modelValue', o.value)"
    >
      <Icon v-if="o.icon" :name="o.icon" size="15" />
      <template v-else>{{ o.label }}</template>
    </button>
  </div>
</template>

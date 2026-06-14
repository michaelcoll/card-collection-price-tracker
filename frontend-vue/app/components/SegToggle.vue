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
  if (o.value !== props.modelValue) return 'text-[var(--ink-2)]';
  if (o.tone === 'cyan') return 'text-[var(--cyan)]';
  if (o.tone === 'vio') return 'text-[var(--violet)]';
  return 'text-[var(--ink)]';
};
</script>

<template>
  <div
    ref="containerRef"
    class="inline-flex p-[3px] gap-[2px] rounded-[11px] bg-[color-mix(in_srgb,black_26%,transparent)] border border-solid border-[var(--line)] relative"
  >
    <span
      class="absolute top-[3px] bottom-[3px] z-0 rounded-[8px] bg-[var(--surface-2)] border border-solid border-[var(--line-2)] shadow-[0_4px_12px_-6px_rgba(0,0,0,0.7)] [transition:left_0.26s_cubic-bezier(0.5,1.3,0.5,1),width_0.26s_cubic-bezier(0.5,1.3,0.5,1)]"
      :style="{ left: thumb.left + 'px', width: thumb.width + 'px', opacity: thumb.opacity }"
    />
    <button
      v-for="o in options"
      :key="o.value"
      :title="o.title"
      :aria-label="o.title"
      :class="[
        'relative z-[1] rounded-[8px] font-semibold whitespace-nowrap transition-colors duration-200',
        size === 'sm' ? 'px-[10px] py-[5px] text-[11.5px]' : 'px-[14px] py-[7px] text-[12.5px]',
        btnActiveColor(o),
      ]"
      @click="emit('update:modelValue', o.value)"
    >
      <Icon v-if="o.icon" :name="o.icon" size="15" />
      <template v-else>{{ o.label }}</template>
    </button>
  </div>
</template>

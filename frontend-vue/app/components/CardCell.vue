<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    scryfallId?: string;
    name: string;
    qty?: number;
    price?: number;
    purchased?: number;
    trend?: number;
    deal?: 'none' | 'compare';
    clickable?: boolean;
    size?: 'sm' | 'md' | 'lg';
  }>(),
  {
    clickable: true,
  },
);

const emit = defineEmits(['click']);

const dealInfo = computed(() => {
  if (!props.deal || props.deal === 'none') return null;
  const u = props.purchased;
  const t = props.trend;
  if (u == null || t == null) return null;
  const pct = Math.round(((t - u) / u) * 100);
  const kind = pct >= 3 ? 'good' : pct <= -3 ? 'bad' : 'par';
  return { pct, kind, abs: Math.abs(pct), sign: pct <= 0 ? '−' : '+' };
});

const dealTagClass = computed(() => {
  if (!dealInfo.value) return '';
  const kinds: Record<string, string> = {
    good: 'text-[var(--good)] bg-[var(--good-fill)]',
    bad: 'text-[var(--down)] bg-[var(--down-fill)]',
    par: 'text-[var(--ink-3)]',
  };
  return `[font-family:var(--font-mono)] text-[10.5px] font-bold px-[5px] py-[1px] rounded-[5px] ${kinds[dealInfo.value.kind] ?? ''}`;
});
</script>

<template>
  <div class="flex flex-col gap-2">
    <MtgCard
      :qty="qty"
      :scryfall-id="scryfallId"
      :name="name"
      :clickable="clickable ?? true"
      :size="size"
      @click="emit('click')"
    />
    <div class="flex flex-col gap-[3px]">
      <span
        class="text-[12.5px] font-semibold text-[var(--ink)] leading-[1.25] overflow-hidden text-ellipsis whitespace-nowrap"
        >{{ name }}</span
      >

      <!-- no deal -->
      <span
        v-if="!deal || deal === 'none'"
        class="[font-family:var(--font-mono)] text-[12px] text-[var(--ink)] font-semibold"
      >
        <template v-if="price != null">{{ formatPrice(price) }}</template>
      </span>

      <!-- compare: trend price + unit crossed out + % badge -->
      <template v-else-if="deal === 'compare' && dealInfo">
        <span
          v-if="dealInfo.kind === 'par'"
          class="[font-family:var(--font-mono)] text-[12px] text-[var(--ink)] font-semibold"
          >{{ formatPrice(trend) }}</span
        >
        <span v-else class="flex items-center gap-1.5 flex-wrap">
          <span
            class="[font-family:var(--font-mono)] text-[12px] text-[var(--ink)] font-semibold"
            >{{ formatPrice(trend) }}</span
          >
          <span
            v-if="size !== 'sm'"
            class="[font-family:var(--font-mono)] text-[11px] text-[var(--ink-3)] line-through"
            >{{ formatPrice(purchased) }}</span
          >
          <span :class="dealTagClass">{{ dealInfo.sign }}{{ dealInfo.abs }}%</span>
        </span>
      </template>
      <span
        v-else-if="deal === 'compare'"
        class="[font-family:var(--font-mono)] text-[12px] text-[var(--ink)] font-semibold"
      >
        <template v-if="purchased != null">{{ formatPrice(purchased) }}</template>
        <template v-else-if="price != null">{{ formatPrice(price) }}</template>
      </span>
    </div>
  </div>
</template>

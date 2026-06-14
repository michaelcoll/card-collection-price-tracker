<script setup lang="ts">
defineProps<{
  data?: number[];
}>();
</script>

<template>
  <div class="flex items-end gap-[3px] h-[46px]">
    <i
      v-for="(h, i) in data ?? [35, 50, 42, 65, 58, 80, 72, 95]"
      :key="i"
      class="w-[7px] rounded-[3px_3px_1px_1px] bg-[var(--cyan-dim)] [transition:height_0.5s_cubic-bezier(0.4,0,0.1,1)] last:bg-[var(--cyan)] last:shadow-[0_0_10px_var(--cyan-glow)]"
      :style="{ height: h + '%' }"
    />
  </div>
</template>

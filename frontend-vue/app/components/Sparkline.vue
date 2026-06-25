<script setup lang="ts">
defineProps<{
  data?: number[];
}>();
</script>

<template>
  <div class="flex h-12 items-end gap-1">
    <i
      v-for="(h, i) in data ?? [35, 50, 42, 65, 58, 80, 72, 95]"
      :key="i"
      class="w-2 rounded-t-sm bg-cyan-600 transition-[height] duration-500 ease-out last:bg-cyan-500 dark:bg-cyan-500 dark:last:bg-cyan-400"
      :style="{ height: h + '%' }"
    />
  </div>
</template>

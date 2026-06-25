<script setup lang="ts">
defineProps<{
  initials: string;
  online?: boolean;
}>();
</script>

<template>
  <div
    class="relative grid h-8 w-8 shrink-0 place-items-center rounded-full border border-slate-300 bg-slate-100 font-mono text-xs text-slate-600 dark:border-white/15 dark:bg-zinc-800 dark:text-slate-300"
  >
    {{ initials }}
    <span
      v-if="online"
      class="absolute -right-px -bottom-px h-2.5 w-2.5 rounded-full border-2 border-slate-100 bg-cyan-500 dark:border-zinc-950 dark:bg-cyan-400"
    />
  </div>
</template>

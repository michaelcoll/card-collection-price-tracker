<script setup lang="ts">
defineProps<{
  initials: string;
  online?: boolean;
}>();
</script>

<template>
  <div
    class="w-8 h-8 rounded-full shrink-0 relative bg-slate-100 dark:bg-zinc-800 border border-slate-300 dark:border-white/15 grid place-items-center font-mono text-xs text-slate-600 dark:text-slate-300"
  >
    {{ initials }}
    <span
      v-if="online"
      class="absolute -right-px -bottom-px w-2.5 h-2.5 rounded-full bg-cyan-500 dark:bg-cyan-400 border-2 border-slate-100 dark:border-zinc-950"
    />
  </div>
</template>

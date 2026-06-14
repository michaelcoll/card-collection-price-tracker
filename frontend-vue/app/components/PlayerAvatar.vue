<script setup lang="ts">
defineProps<{
  initials: string;
  online?: boolean;
}>();
</script>

<template>
  <div
    class="w-[30px] h-[30px] rounded-full shrink-0 relative bg-[radial-gradient(circle_at_35%_28%,var(--surface-3),var(--surface))] border border-solid border-[var(--line-2)] grid place-items-center [font-family:var(--font-mono)] text-[11px] text-[var(--ink-2)]"
  >
    {{ initials }}
    <span
      v-if="online"
      class="absolute right-[-1px] bottom-[-1px] w-[9px] h-[9px] rounded-full bg-[var(--cyan)] border-2 border-solid border-[var(--bg)] shadow-[0_0_6px_var(--cyan-glow)]"
    />
  </div>
</template>

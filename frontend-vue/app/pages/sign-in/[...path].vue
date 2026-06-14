<script setup lang="ts">
const { clerkAppearance } = useAppTheme();
</script>

<template>
  <div class="max-w-[var(--maxw)] mx-auto px-[22px] pb-[40px] pt-[60px] flex justify-center">
    <SignIn :appearance="clerkAppearance" />
  </div>
</template>

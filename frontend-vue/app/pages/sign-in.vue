<script setup lang="ts">
const { clerkAppearance } = useAppTheme();
</script>

<template>
  <div class="mx-auto flex max-w-[1180px] justify-center px-5 pt-16 pb-10">
    <SignIn :appearance="clerkAppearance" />
  </div>
</template>

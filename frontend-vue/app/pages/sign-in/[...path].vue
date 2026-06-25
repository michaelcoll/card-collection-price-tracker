<script setup lang="ts">
const { clerkAppearance } = useAppTheme();
</script>

<template>
  <div class="max-w-[1180px] mx-auto px-5 pb-10 pt-16 flex justify-center">
    <SignIn :appearance="clerkAppearance" />
  </div>
</template>

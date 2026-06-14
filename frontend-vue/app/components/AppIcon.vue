<script setup lang="ts">
const PATHS: Record<string, string> = {
  search: 'M11 4a7 7 0 1 0 0 14 7 7 0 0 0 0-14ZM20 20l-3.5-3.5',
  swap: 'M7 7h11M7 7l3-3M7 7l3 3M17 17H6m11 0-3 3m3-3-3-3',
  plus: 'M12 5v14M5 12h14',
  sliders: 'M4 6h10M18 6h2M4 12h2M10 12h10M4 18h7M15 18h5M14 4v4M6 10v4M11 16v4',
  upload: 'M12 16V4m0 0L8 8m4-4 4 4M5 20h14',
  star: 'M12 3.5l2.6 5.3 5.9.9-4.3 4.1 1 5.8-5.2-2.7-5.2 2.7 1-5.8L3.5 9.7l5.9-.9z',
  pin: 'M12 21s7-5.5 7-11a7 7 0 1 0-14 0c0 5.5 7 11 7 11Zm0-8.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z',
  chevron: 'M6 9l6 6 6-6',
  arrowUR: 'M7 17 17 7M9 7h8v8',
  home: 'M4 11.5 12 4l8 7.5M6 10v10h12V10',
  grid: 'M4 4h7v7H4zM13 4h7v7h-7zM4 13h7v7H4zM13 13h7v7h-7z',
  list: 'M8 6h12M8 12h12M8 18h12M4 6h.01M4 12h.01M4 18h.01',
  collection: 'M5 4h10l4 4v12H5zM15 4v4h4',
  trade: 'M4 8h13M4 8l3-3M4 8l3 3M20 16H7m13 0-3-3m3 3-3 3',
  user: 'M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8ZM5 20c0-3.3 3.1-6 7-6s7 2.7 7 6',
  sparkle: 'M12 3l1.8 4.7L18 9.5l-4.2 1.8L12 16l-1.8-4.7L6 9.5l4.2-1.8z',
  check: 'M5 12.5l4.5 4.5L19 7',
  x: 'M6 6l12 12M18 6 6 18',
  info: 'M12 11v6M12 7.5v.5M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Z',
  clock: 'M12 7v5l3.5 2M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Z',
  eye: 'M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7-10-7-10-7Zm10 3a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z',
  lock: 'M6 11V8a6 6 0 1 1 12 0v3M5 11h14v10H5z',
  shield: 'M12 3l7 3v6c0 4.5-3 7.5-7 9-4-1.5-7-4.5-7-9V6z',
  mountain: 'M3 18l6-9 4 5 3-4 5 8z',
  filter: 'M3 5h18l-7 8v6l-4-2v-4z',
  trending: 'M3 17l6-6 4 4 8-8M21 7v5m0-5h-5',
  layers: 'M12 3l9 5-9 5-9-5zM3 13l9 5 9-5',
  refresh: 'M4 12a8 8 0 0 1 14-5.3L20 8M20 12a8 8 0 0 1-14 5.3L4 16m0-8V4m0 4h4m12 8v4m0-4h-4',
  heart: 'M12 20s-7-4.5-7-9.5A3.5 3.5 0 0 1 12 7a3.5 3.5 0 0 1 7 3.5C19 15.5 12 20 12 20Z',
  bars: 'M5 20v-6M10 20V8M15 20v-9M20 20v-4',
  sizeSm:
    'M4 4h4v4H4zM10 4h4v4h-4zM16 4h4v4h-4zM4 10h4v4H4zM10 10h4v4h-4zM16 10h4v4h-4zM4 16h4v4H4zM10 16h4v4h-4zM16 16h4v4h-4z',
  sizeMd: 'M4 4h7v7H4zM13 4h7v7h-7zM4 13h7v7H4zM13 13h7v7h-7z',
  sizeLg: 'M5 4h14v16H5z',
  copy: 'M9 9h10v10H9zM5 15V5h10',
  discord:
    'M8.5 8.5C10 8 14 8 15.5 8.5M7.5 16c-1-2-1.3-5 0-8.5C9 6.8 11 6.5 12 6.5s3 .3 4.5 1c1.3 3.5 1 6.5 0 8.5M9.5 13.5h.01M14.5 13.5h.01M7.5 16l-1 2 3-1.2M16.5 16l1 2-3-1.2',
};

defineProps<{
  name: string;
  size?: number;
  fill?: boolean;
  strokeWidth?: number;
}>();
</script>

<template>
  <svg
    viewBox="0 0 24 24"
    :width="size ?? 18"
    :height="size ?? 18"
    :fill="fill ? 'currentColor' : 'none'"
    :stroke="fill ? 'none' : 'currentColor'"
    :stroke-width="strokeWidth ?? 1.7"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path :d="PATHS[name] ?? ''" />
  </svg>
</template>

<script setup lang="ts">
const props = defineProps<{
  scryfallId?: string;
  qty?: number;
  clickable?: boolean;
  mini?: boolean;
  name?: string;
  size?: 'sm' | 'md' | 'lg';
}>();

const emit = defineEmits(['click']);

const src = computed(
  () =>
    (props.scryfallId &&
      `https://api.scryfall.com/cards/${props.scryfallId}?format=image&version=normal`) ||
    '',
);
</script>

<template>
  <div
    :class="[
      'relative aspect-[5/7] flex flex-col gap-0 overflow-hidden',
      'transition-[transform,box-shadow,border-color] duration-200 ease',
      'border border-solid',
      mini ? 'rounded-[2px]' : size === 'lg' ? 'rounded-[20px]' : 'rounded-[8px]',
      src
        ? 'p-0 bg-[#0a0a0a] border-[color-mix(in_srgb,black_55%,transparent)]'
        : [
            mini ? 'p-[3px]' : 'p-[5%]',
            'border-[var(--line-2)] bg-[linear-gradient(160deg,var(--surface-3),var(--surface)_60%)] shadow-[0_1px_0_rgba(255,255,255,0.04)_inset,0_14px_30px_-20px_rgba(0,0,0,0.9)]',
          ],
      clickable
        ? 'cursor-pointer hover:-translate-y-1 hover:border-[var(--cyan-line)] hover:shadow-[0_0_0_1px_var(--cyan-fill),0_22px_40px_-20px_rgba(0,0,0,1),0_0_30px_-14px_var(--cyan-glow)]'
        : '',
    ]"
    :title="name"
    @click="emit('click')"
  >
    <!-- qty badge -->
    <span
      v-if="qty != null"
      class="absolute top-[7px] right-[7px] z-[5] [font-family:var(--font-mono)] text-[11px] font-semibold px-[7px] py-[2px] rounded-full text-[#f4f3f2] bg-[color-mix(in_srgb,black_60%,transparent)] border border-solid border-[color-mix(in_srgb,white_22%,transparent)] backdrop-blur-[6px]"
      >×{{ qty }}</span
    >

    <!-- inner vignette overlay — replaces ::after pseudo-element -->
    <div
      v-if="!src"
      :class="[
        'absolute inset-0 pointer-events-none z-[1]',
        mini
          ? 'rounded-[4px] shadow-[inset_0_0_0_2px_color-mix(in_srgb,black_55%,transparent)]'
          : 'rounded-[6px] shadow-[inset_0_0_0_4px_color-mix(in_srgb,black_55%,transparent)]',
      ]"
    />

    <!-- real card scan -->
    <template v-if="src">
      <img
        class="absolute inset-0 z-0 w-full h-full object-cover block select-none"
        style="border-radius: inherit"
        :src="src"
        :alt="name ?? ''"
        loading="lazy"
        draggable="false"
      />
    </template>

    <!-- mini placeholder -->
    <template v-else-if="mini">
      <div
        class="flex-1 rounded-[4px] relative z-[2] border border-solid border-[var(--line)] [background:repeating-linear-gradient(135deg,color-mix(in_srgb,black_18%,transparent)_0_7px,transparent_7px_14px),linear-gradient(180deg,var(--surface-2),color-mix(in_srgb,black_30%,var(--surface)))] grid place-items-center overflow-hidden"
      >
        <AppIcon
          name="mountain"
          style="width: 46%; height: 46%; color: var(--ink-4); opacity: 0.55"
        />
      </div>
    </template>

    <!-- full placeholder -->
    <template v-else>
      <!-- title bar -->
      <div
        class="flex items-center justify-between gap-[5px] px-[6%] py-[5%] bg-[color-mix(in_srgb,black_30%,var(--surface-3))] border border-solid border-[var(--line)] rounded-[5px] relative z-[2]"
      >
        <span class="h-[6px] rounded-[3px] bg-[var(--ink-4)] w-[60%]" />
        <span
          class="w-[11px] h-[11px] rounded-full shrink-0 bg-[#3a3540] shadow-[inset_0_1px_1px_rgba(255,255,255,0.25)]"
        />
      </div>
      <!-- art -->
      <div
        class="flex-1 mt-[5%] rounded-[4px] relative z-[2] border border-solid border-[var(--line)] [background:repeating-linear-gradient(135deg,color-mix(in_srgb,black_18%,transparent)_0_7px,transparent_7px_14px),linear-gradient(180deg,var(--surface-2),color-mix(in_srgb,black_30%,var(--surface)))] grid place-items-center overflow-hidden"
      >
        <AppIcon
          name="mountain"
          :size="28"
          style="width: 34%; height: 34%; color: var(--ink-4); opacity: 0.55"
        />
      </div>
      <!-- type bar -->
      <div
        class="mt-[5%] flex items-center gap-[5px] px-[6%] py-[5%] bg-[color-mix(in_srgb,black_30%,var(--surface-3))] border border-solid border-[var(--line)] rounded-[5px] relative z-[2]"
      >
        <span class="h-[5px] w-[50%] rounded-[3px] bg-[var(--ink-4)] opacity-70" />
        <span
          class="w-[8px] h-[8px] rounded-full shrink-0 bg-[#3a3540] shadow-[inset_0_1px_1px_rgba(255,255,255,0.25)]"
        />
      </div>
      <!-- text box -->
      <div
        class="mt-[5%] shrink-0 basis-[22%] rounded-[4px] relative z-[2] bg-[color-mix(in_srgb,black_24%,var(--surface-2))] border border-solid border-[var(--line)] p-[6%] flex flex-col gap-[14%]"
      >
        <span class="h-[3.5px] rounded-[2px] bg-[var(--ink-4)] opacity-50 block w-[92%]" />
        <span class="h-[3.5px] rounded-[2px] bg-[var(--ink-4)] opacity-50 block w-[78%]" />
        <span class="h-[3.5px] rounded-[2px] bg-[var(--ink-4)] opacity-50 block w-[85%]" />
      </div>
    </template>
  </div>
</template>

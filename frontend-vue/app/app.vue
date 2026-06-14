<script setup lang="ts">
const { theme, clerkAppearance } = useAppTheme();

if (import.meta.client) {
  const saved = localStorage.getItem('tae_theme');
  if (saved === 'light' || saved === 'dark') theme.value = saved;
}

useHead({
  htmlAttrs: { 'data-theme': theme },
  bodyAttrs: { 'data-bg': 'aurora' },
});

watch(theme, (val) => {
  if (import.meta.client) localStorage.setItem('tae_theme', val);
});

const toggleTheme = () => {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
};

const { isSignedIn, isLoaded } = useAuth();

const route = useRoute();
const isActive = (path: string) => route.path === path;

const navLinkClass = (path: string) => [
  'relative flex items-center px-[16px] text-[13.5px] font-medium transition-[color,background] duration-[180ms] whitespace-nowrap',
  isActive(path)
    ? 'text-[var(--cyan)] bg-[linear-gradient(180deg,color-mix(in_oklch,var(--cyan)_22%,transparent),color-mix(in_oklch,var(--cyan)_4%,transparent))] shadow-[inset_0_-2px_0_var(--cyan),inset_0_1px_0_color-mix(in_oklch,var(--cyan)_30%,transparent),0_8px_22px_-16px_var(--cyan-glow)]'
    : 'text-[var(--ink-2)] hover:text-[var(--ink)] hover:bg-[var(--line-3)]',
];

const bottomNavLinkClass = (path: string) => [
  'flex-1 flex flex-col items-center gap-[3px] text-[10px] p-[4px] font-semibold [font-family:var(--font-mono)] tracking-[0.02em] transition-colors duration-[150ms]',
  isActive(path) ? 'text-[var(--cyan)]' : 'text-[var(--ink-3)]',
];
</script>

<template>
  <div class="min-h-screen pb-[96px]">
    <NuxtRouteAnnouncer />

    <!-- HEADER -->
    <header
      class="sticky top-0 z-[40] [backdrop-filter:blur(var(--glass-blur))_saturate(140%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(140%)] bg-[color-mix(in_srgb,var(--bg)_62%,transparent)] border-b border-solid border-[var(--line)]"
    >
      <div
        class="max-w-[var(--maxw)] mx-auto px-[22px] max-[860px]:px-[16px] h-[62px] max-[860px]:h-[56px] flex items-center gap-[18px]"
      >
        <NuxtLink
          to="/"
          class="flex items-center gap-[10px] [font-family:var(--font-display)] cursor-pointer"
        >
          <span
            class="w-[30px] h-[30px] rounded-[9px] shrink-0 grid place-items-center overflow-hidden bg-[linear-gradient(150deg,color-mix(in_oklch,var(--cyan)_30%,var(--surface)),var(--surface-2))] border border-solid border-[var(--cyan-line)] shadow-[0_0_18px_-6px_var(--cyan-glow)]"
          >
            <svg
              viewBox="0 0 28 28"
              class="w-[17px] h-[17px]"
              fill="none"
              stroke-width="2.2"
              stroke-linejoin="round"
              aria-hidden="true"
            >
              <rect
                x="5"
                y="8.5"
                width="11"
                height="11"
                rx="3"
                transform="rotate(45 10.5 14)"
                stroke="var(--cyan)"
              />
              <rect
                x="12"
                y="8.5"
                width="11"
                height="11"
                rx="3"
                transform="rotate(45 17.5 14)"
                stroke="var(--violet)"
              />
            </svg>
          </span>
          <span class="font-semibold text-[16px] tracking-[-0.01em]"
            >Arcane <b class="text-[var(--cyan)] font-semibold">Exchange</b></span
          >
        </NuxtLink>

        <nav class="flex gap-[2px] ml-[8px] self-stretch max-[860px]:hidden">
          <NuxtLink to="/collection" :class="navLinkClass('/collection')">Collection</NuxtLink>
          <NuxtLink to="/trade" :class="navLinkClass('/trade')">Échanges</NuxtLink>
          <NuxtLink to="/find" :class="navLinkClass('/find')">Rechercher</NuxtLink>
          <NuxtLink to="/prefs" :class="navLinkClass('/prefs')">Profil</NuxtLink>
        </nav>

        <span class="flex-1" />

        <template v-if="isLoaded">
          <NuxtLink
            v-if="!isSignedIn"
            to="/sign-in"
            class="inline-flex items-center gap-[8px] justify-center px-[11px] py-[6px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
            >Connexion
          </NuxtLink>
        </template>

        <button
          class="w-[34px] h-[34px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
          @click="toggleTheme"
          :aria-label="theme === 'dark' ? 'Passer en mode clair' : 'Passer en mode sombre'"
          :title="theme === 'dark' ? 'Mode clair' : 'Mode sombre'"
        >
          <Icon :name="theme === 'dark' ? 'lucide:sun' : 'lucide:moon'" size="17" />
        </button>

        <button
          class="w-[34px] h-[34px] rounded-[9px] grid place-items-center border border-solid border-[var(--line)] text-[var(--ink-2)] bg-[var(--line-3)] transition-all duration-[180ms] hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--surface-2)] max-[860px]:hidden"
          aria-label="Notifications"
        >
          <Icon name="lucide:bell" size="17" />
        </button>

        <div v-if="isLoaded && isSignedIn" class="user-btn-wrap flex items-center h-[34px]">
          <UserButton :appearance="clerkAppearance" />
        </div>
      </div>
    </header>

    <!-- PAGE CONTENT -->
    <main>
      <NuxtPage />
    </main>

    <!-- MOBILE BOTTOM NAV -->
    <nav
      class="hidden max-[860px]:flex max-[860px]:fixed max-[860px]:left-0 max-[860px]:right-0 max-[860px]:bottom-0 max-[860px]:z-[50] max-[860px]:pt-[9px] max-[860px]:px-[8px] max-[860px]:pb-[calc(9px_+_env(safe-area-inset-bottom))] max-[860px]:bg-[color-mix(in_srgb,var(--bg)_80%,transparent)] max-[860px]:[backdrop-filter:blur(var(--glass-blur))_saturate(140%)] max-[860px]:[-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(140%)] max-[860px]:border-t max-[860px]:border-solid max-[860px]:border-[var(--line)]"
    >
      <NuxtLink to="/collection" :class="bottomNavLinkClass('/collection')">
        <Icon name="lucide:layers" size="21" />
        Collection
      </NuxtLink>
      <NuxtLink to="/trade" :class="bottomNavLinkClass('/trade')">
        <Icon name="lucide:arrow-left-right" size="21" />
        Échanges
      </NuxtLink>
      <NuxtLink to="/find" :class="bottomNavLinkClass('/find')">
        <Icon name="lucide:search" size="21" />
        Rechercher
      </NuxtLink>
      <NuxtLink to="/prefs" :class="bottomNavLinkClass('/prefs')">
        <Icon name="lucide:user" size="21" />
        Profil
      </NuxtLink>
    </nav>
  </div>
</template>

<style>
/* Clerk UserButton internal overrides */
.user-btn-wrap .cl-rootBox,
.user-btn-wrap .cl-userButtonBox {
  display: flex;
  align-items: center;
  height: 34px;
}

.user-btn-wrap .cl-userButtonTrigger {
  width: 34px;
  height: 34px;
  padding: 0;
  border-radius: 50%;
}

[data-theme='dark'] .cl-userButtonTrigger.cl-open,
[data-theme='dark'] .cl-userButtonTrigger:focus-visible {
  box-shadow:
    0 0 0 2px var(--bg),
    0 0 0 4px var(--cyan) !important;
}

[data-theme='light'] .cl-userButtonTrigger.cl-open,
[data-theme='light'] .cl-userButtonTrigger:focus-visible {
  box-shadow:
    0 0 0 2px var(--bg),
    0 0 0 4px var(--cyan) !important;
}
</style>

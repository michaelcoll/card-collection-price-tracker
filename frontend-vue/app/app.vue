<script setup lang="ts">
const { theme, clerkAppearance } = useAppTheme();

if (import.meta.client) {
  const saved = localStorage.getItem('tae_theme');
  if (saved === 'light' || saved === 'dark') theme.value = saved;
}

const htmlClass = computed(() => (theme.value === 'dark' ? 'dark' : ''));

useHead({
  htmlAttrs: { class: htmlClass },
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
  'relative flex items-center px-4 text-sm font-medium transition-[color,background] duration-200 whitespace-nowrap',
  isActive(path)
    ? 'text-cyan-600 dark:text-cyan-400 bg-cyan-500/10 dark:bg-cyan-400/10 shadow-[inset_0_-2px_0_currentColor]'
    : 'text-slate-600 dark:text-slate-300 hover:text-slate-800 dark:hover:text-slate-100 hover:bg-slate-100 dark:hover:bg-white/5',
];

const bottomNavLinkClass = (path: string) => [
  'flex-1 flex flex-col items-center gap-1 text-2xs p-1 font-semibold font-mono tracking-wide transition-colors duration-150',
  isActive(path) ? 'text-cyan-600 dark:text-cyan-400' : 'text-slate-400 dark:text-slate-500',
];
</script>

<template>
  <div class="min-h-screen pb-24">
    <NuxtRouteAnnouncer />

    <!-- HEADER -->
    <header
      class="sticky top-0 z-40 backdrop-blur-md bg-slate-100/60 dark:bg-zinc-950/60 border-b border-slate-200 dark:border-white/10"
    >
      <div class="max-w-[1180px] mx-auto px-5 max-md:px-4 h-16 max-md:h-14 flex items-center gap-4">
        <NuxtLink to="/" class="flex items-center gap-2.5 font-display cursor-pointer">
          <span
            class="w-8 h-8 rounded-lg shrink-0 grid place-items-center overflow-hidden bg-cyan-500/15 dark:bg-cyan-400/15 border border-cyan-500/30 dark:border-cyan-400/30"
          >
            <svg
              viewBox="0 0 28 28"
              class="w-4 h-4"
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
          <span class="font-semibold text-base tracking-tight"
            >Arcane <b class="text-cyan-600 dark:text-cyan-400 font-semibold">Exchange</b></span
          >
        </NuxtLink>

        <nav class="flex gap-0.5 ml-2 self-stretch max-md:hidden">
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
            class="inline-flex items-center gap-2 justify-center px-3 py-1.5 rounded-lg text-xs font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
            >Connexion
          </NuxtLink>
        </template>

        <button
          class="w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
          @click="toggleTheme"
          :aria-label="theme === 'dark' ? 'Passer en mode clair' : 'Passer en mode sombre'"
          :title="theme === 'dark' ? 'Mode clair' : 'Mode sombre'"
        >
          <Icon :name="theme === 'dark' ? 'lucide:sun' : 'lucide:moon'" size="17" />
        </button>

        <button
          class="w-9 h-9 rounded-lg grid place-items-center border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-slate-100 dark:bg-white/5 transition-all duration-150 hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800 max-md:hidden"
          aria-label="Notifications"
        >
          <Icon name="lucide:bell" size="17" />
        </button>

        <div v-if="isLoaded && isSignedIn" class="user-btn-wrap flex items-center h-9">
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
      class="hidden max-md:flex max-md:fixed max-md:left-0 max-md:right-0 max-md:bottom-0 max-md:z-50 max-md:pt-2 max-md:px-2 max-md:pb-[calc(0.5rem+env(safe-area-inset-bottom))] max-md:bg-slate-100/80 dark:max-md:bg-zinc-950/80 max-md:backdrop-blur-md max-md:border-t max-md:border-slate-200 dark:max-md:border-white/10"
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

.cl-userButtonTrigger.cl-open,
.cl-userButtonTrigger:focus-visible {
  box-shadow:
    0 0 0 2px var(--bg),
    0 0 0 4px var(--cyan) !important;
}
</style>

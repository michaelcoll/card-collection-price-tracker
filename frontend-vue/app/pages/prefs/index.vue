<script setup lang="ts">
const { user } = useUser();

const initials = computed(() => {
  if (!user.value) return '?';
  const fn = user.value.firstName?.[0] ?? '';
  const ln = user.value.lastName?.[0] ?? '';
  return (fn + ln).toUpperCase() || user.value.username?.[0]?.toUpperCase() || '?';
});

const vis = ref<'public' | 'trade' | 'private'>('trade');
const showVal = ref<'me' | 'all'>('me');
const proximity = ref(true);
const accent = ref('#00daf3');
const manageOpen = ref(false);

const THEMES = [
  { c: '#00daf3', name: 'Cyan néon', sub: 'Défaut' },
  { c: '#00f0c8', name: 'Turquoise', sub: '#00F0C8' },
  { c: '#3aa0ff', name: 'Azur', sub: '#3AA0FF' },
  { c: '#19c4e6', name: 'Cyan profond', sub: '#19C4E6' },
];

const setAccent = (c: string) => {
  accent.value = c;
  if (import.meta.client) {
    document.documentElement.style.setProperty('--accent', c);
    localStorage.setItem('tae_accent', c);
  }
};

onMounted(() => {
  if (import.meta.client) {
    const saved = localStorage.getItem('tae_accent');
    if (saved) {
      accent.value = saved;
      document.documentElement.style.setProperty('--accent', saved);
    }
  }
});

const channels = [
  { icon: 'simple-icons:discord', name: 'Discord', value: 'theo_arcaniste#4821', connected: true },
  { icon: 'simple-icons:telegram', name: 'Telegram', value: '@theo_mtg', connected: true },
  { icon: 'simple-icons:whatsapp', name: 'WhatsApp', value: '', connected: false },
];

const visOptions = [
  { value: 'public', label: 'Publique', tone: 'cyan' },
  { value: 'trade', label: 'Pour échange', tone: 'cyan' },
  { value: 'private', label: 'Privée', tone: 'cyan' },
];
const showValOptions = [
  { value: 'me', label: 'Moi seul', tone: 'cyan' },
  { value: 'all', label: 'Tout le monde', tone: 'cyan' },
];
</script>

<template>
  <div class="max-w-[680px] mx-auto px-5 pb-10 pt-7 max-md:px-4 max-md:pt-5 max-md:pb-8">
    <h2 class="font-display font-semibold text-xl tracking-tight mb-4">Préférences</h2>

    <!-- COMPTE -->
    <div
      class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-4 mb-6"
    >
      <div class="flex items-center justify-between gap-3.5">
        <div class="flex items-center gap-3">
          <div
            class="w-8 h-8 rounded-full shrink-0 relative bg-slate-100 dark:bg-zinc-800 border border-slate-300 dark:border-white/15 grid place-items-center font-mono text-xs text-slate-600 dark:text-slate-300 overflow-hidden"
          >
            <img
              v-if="user?.imageUrl"
              :src="user.imageUrl"
              :alt="user.fullName ?? ''"
              class="w-full h-full object-cover rounded-full"
            />
            <template v-else>{{ initials }}</template>
          </div>
          <div class="flex flex-col gap-0.5">
            <span
              class="text-base font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
              >{{ user?.fullName ?? user?.username ?? '—' }}</span
            >
            <span class="text-xs text-slate-400 dark:text-slate-500 flex items-center gap-1.5">
              {{ user?.primaryEmailAddress?.emailAddress ?? '' }} ·
              <span class="text-violet-500 dark:text-violet-300 inline-flex items-center gap-1">
                <Icon name="lucide:shield" size="12" /> géré par Clerk
              </span>
            </span>
          </div>
        </div>
        <button
          class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
          @click="manageOpen = true"
        >
          Gérer le compte <Icon name="lucide:arrow-up-right" size="14" />
        </button>
      </div>
    </div>

    <!-- MODAL CLERK USER PROFILE -->
    <div
      v-if="manageOpen"
      class="fixed inset-0 z-[80] bg-black/60 backdrop-blur-sm animate-[fade_0.2s_ease] grid place-items-center p-5"
      @click.self="manageOpen = false"
    >
      <div class="p-0 overflow-hidden rounded-3xl" @click.stop>
        <UserProfile />
      </div>
    </div>

    <!-- APPARENCE -->
    <section class="mb-6">
      <div class="flex flex-col gap-1.5 mb-3">
        <span
          class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
          >Apparence · thème d'accent</span
        >
        <span class="text-xs text-slate-400 dark:text-slate-500"
          >La couleur d'action de toute l'app — boutons, valeurs en hausse, états actifs.</span
        >
      </div>
      <div class="grid [grid-template-columns:repeat(auto-fit,minmax(146px,1fr))] gap-2.5">
        <button
          v-for="th in THEMES"
          :key="th.c"
          :data-on="accent === th.c"
          :style="{ '--tc': th.c } as any"
          class="flex items-center gap-3 px-3 py-3 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 text-left relative transition-all duration-150 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800 hover:-translate-y-px data-[on=true]:border-[var(--tc)] data-[on=true]:shadow-[inset_0_0_0_1px_var(--tc)]"
          @click="setAccent(th.c)"
        >
          <span class="w-8 h-8 rounded-lg shrink-0 bg-[var(--tc)] shadow-inner" />
          <div class="flex flex-col gap-px">
            <span class="text-sm font-semibold text-slate-800 dark:text-slate-100">{{
              th.name
            }}</span>
            <span class="text-xs text-slate-400 dark:text-slate-500 font-mono">{{ th.sub }}</span>
          </div>
          <span v-if="accent === th.c" class="ml-auto text-[var(--tc)] flex">
            <Icon name="lucide:check" size="17" />
          </span>
        </button>
      </div>
    </section>

    <!-- CANAUX DE CONTACT -->
    <section class="mb-6">
      <div class="flex flex-col gap-1.5 mb-3">
        <span
          class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap"
          >Canaux de contact pour les échanges</span
        >
        <span class="text-xs text-slate-400 dark:text-slate-500"
          >Ces canaux alimentent le bouton « Contacter » de tes transactions.</span
        >
      </div>
      <div class="flex flex-col gap-2">
        <div
          v-for="ch in channels"
          :key="ch.name"
          class="flex items-center gap-3 px-3.5 py-3 rounded-xl border border-slate-200 dark:border-white/10 bg-white dark:bg-zinc-900 transition-all duration-150 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-50 dark:hover:bg-zinc-800"
        >
          <span
            :class="
              ch.connected
                ? 'text-cyan-600 dark:text-cyan-400'
                : 'text-slate-400 dark:text-slate-500'
            "
            class="grid place-items-center w-8 shrink-0"
          >
            <Icon :name="ch.icon" size="19" />
          </span>
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-semibold text-slate-800 dark:text-slate-100 overflow-hidden text-ellipsis whitespace-nowrap"
            >
              {{ ch.name }}
            </div>
            <div
              v-if="ch.connected"
              class="text-xs text-slate-600 dark:text-slate-300 flex items-center gap-2 flex-wrap font-mono tracking-tight"
            >
              {{ ch.value }}
            </div>
            <div
              v-else
              class="text-xs text-slate-400 dark:text-slate-500 flex items-center gap-2 flex-wrap"
            >
              Non connecté
            </div>
          </div>
          <button
            v-if="ch.connected"
            class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-semibold border border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 bg-transparent transition-all duration-150 whitespace-nowrap leading-none hover:text-slate-800 dark:hover:text-slate-100 hover:border-slate-300 dark:hover:border-white/15 hover:bg-slate-100 dark:hover:bg-white/5 hover:-translate-y-px active:translate-y-0"
          >
            Modifier
          </button>
          <button
            v-else
            class="inline-flex items-center gap-2 justify-center py-1.5 px-3 rounded-lg text-xs font-semibold border border-slate-300 dark:border-white/15 text-slate-800 dark:text-slate-100 bg-slate-100 dark:bg-zinc-800 transition-all duration-150 whitespace-nowrap leading-none hover:bg-slate-200 dark:hover:bg-zinc-700 hover:border-slate-300 dark:hover:border-white/15 hover:-translate-y-px active:translate-y-0"
          >
            <Icon name="lucide:plus" size="14" /> Ajouter
          </button>
        </div>
      </div>
    </section>

    <!-- CONFIDENTIALITÉ -->
    <section>
      <span
        class="font-mono text-2xs font-medium uppercase tracking-widest text-slate-400 dark:text-slate-500 whitespace-nowrap block mb-3"
        >Confidentialité de la collection</span
      >
      <div
        class="bg-white/60 dark:bg-zinc-900/60 backdrop-blur-md border border-slate-200 dark:border-white/10 rounded-2xl shadow-lg p-1.5"
      >
        <div class="flex flex-col gap-0">
          <div class="flex items-center justify-between gap-4 px-3.5 py-3.5">
            <div class="flex flex-col gap-0.5">
              <span class="font-semibold text-sm">Visibilité de la collection</span>
              <span class="text-xs text-slate-400 dark:text-slate-500"
                >Qui peut voir tes cartes pour proposer un échange</span
              >
            </div>
            <SegToggle v-model="vis" :options="visOptions" size="sm" />
          </div>

          <div class="h-px bg-slate-200 dark:bg-white/10" />

          <div class="flex items-center justify-between gap-4 px-3.5 py-3.5">
            <div class="flex flex-col gap-0.5">
              <span class="font-semibold text-sm">Afficher la valeur estimée</span>
              <span class="text-xs text-slate-400 dark:text-slate-500"
                >Montant total visible sur ton profil public</span
              >
            </div>
            <SegToggle v-model="showVal" :options="showValOptions" size="sm" />
          </div>

          <div class="h-px bg-slate-200 dark:bg-white/10" />

          <div class="flex items-center justify-between gap-4 px-3.5 py-3.5">
            <div class="flex flex-col gap-0.5">
              <span class="font-semibold text-sm">Apparaître dans les recherches de proximité</span>
              <span class="text-xs text-slate-400 dark:text-slate-500"
                >Les joueurs proches peuvent te trouver par carte ou decklist</span
              >
            </div>
            <button
              :aria-pressed="proximity"
              :class="[
                'w-12 h-7 rounded-full shrink-0 relative transition-all duration-200 cursor-pointer',
                proximity
                  ? 'bg-cyan-500 dark:bg-cyan-400 border border-transparent'
                  : 'bg-slate-200 dark:bg-zinc-800 border border-slate-300 dark:border-white/15',
              ]"
              @click="proximity = !proximity"
            >
              <span
                :class="[
                  'absolute top-1 w-5 h-5 rounded-full transition-[left] duration-200 ease-out',
                  proximity ? 'left-6 bg-zinc-950' : 'left-1 bg-slate-500 dark:bg-slate-400',
                ]"
              />
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

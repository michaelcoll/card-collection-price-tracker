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
  <div
    class="max-w-[680px] mx-auto px-[22px] pb-[40px] pt-[28px] max-[860px]:px-[16px] max-[860px]:pt-[20px] max-[860px]:pb-[30px]"
  >
    <h2
      class="[font-family:var(--font-display)] font-semibold text-[20px] tracking-[-0.015em] mb-[18px]"
    >
      Préférences
    </h2>

    <!-- COMPTE -->
    <div
      class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[18px] mb-[22px]"
    >
      <div class="flex items-center justify-between gap-[14px]">
        <div class="flex items-center gap-[13px]">
          <div
            class="w-[30px] h-[30px] rounded-full shrink-0 relative bg-[radial-gradient(circle_at_35%_28%,var(--surface-3),var(--surface))] border border-solid border-[var(--line-2)] grid place-items-center [font-family:var(--font-mono)] text-[11px] text-[var(--ink-2)] overflow-hidden"
          >
            <img
              v-if="user?.imageUrl"
              :src="user.imageUrl"
              :alt="user.fullName ?? ''"
              class="w-full h-full object-cover rounded-full"
            />
            <template v-else>{{ initials }}</template>
          </div>
          <div class="flex flex-col gap-[2px]">
            <span
              class="text-[15px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
              >{{ user?.fullName ?? user?.username ?? '—' }}</span
            >
            <span class="text-[12px] text-[var(--ink-3)] flex items-center gap-[5px]">
              {{ user?.primaryEmailAddress?.emailAddress ?? '' }} ·
              <span class="text-[var(--violet)] inline-flex items-center gap-[3px]">
                <Icon name="lucide:shield" size="12" /> géré par Clerk
              </span>
            </span>
          </div>
        </div>
        <button
          class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
          @click="manageOpen = true"
        >
          Gérer le compte <Icon name="lucide:arrow-up-right" size="14" />
        </button>
      </div>
    </div>

    <!-- MODAL CLERK USER PROFILE -->
    <div
      v-if="manageOpen"
      class="fixed inset-0 z-[80] bg-[color-mix(in_srgb,black_58%,transparent)] [backdrop-filter:blur(4px)] [-webkit-backdrop-filter:blur(4px)] animate-[fade_0.2s_ease] grid place-items-center p-[20px]"
      @click.self="manageOpen = false"
    >
      <div class="p-0 overflow-hidden rounded-[var(--r-xl)]" @click.stop>
        <UserProfile />
      </div>
    </div>

    <!-- APPARENCE -->
    <section class="mb-[26px]">
      <div class="flex flex-col gap-[5px] mb-[12px]">
        <span
          class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
          >Apparence · thème d'accent</span
        >
        <span class="text-[12.5px] text-[var(--ink-3)]"
          >La couleur d'action de toute l'app — boutons, valeurs en hausse, états actifs.</span
        >
      </div>
      <div class="grid [grid-template-columns:repeat(auto-fit,minmax(146px,1fr))] gap-[10px]">
        <button
          v-for="th in THEMES"
          :key="th.c"
          :data-on="accent === th.c"
          :style="{ '--tc': th.c } as any"
          class="flex items-center gap-[12px] px-[13px] py-[12px] rounded-[13px] border border-solid border-[var(--line)] bg-[var(--surface)] text-left relative transition-all duration-[160ms] ease hover:border-[var(--line-2)] hover:bg-[var(--surface-2)] hover:-translate-y-px data-[on=true]:border-[var(--tc)] data-[on=true]:bg-[color-mix(in_srgb,var(--tc)_11%,var(--surface))] data-[on=true]:shadow-[inset_0_0_0_1px_var(--tc),0_0_24px_-12px_var(--tc)]"
          @click="setAccent(th.c)"
        >
          <span
            class="w-[30px] h-[30px] rounded-[9px] shrink-0 bg-[linear-gradient(140deg,var(--tc),color-mix(in_oklch,var(--tc)_50%,#131313))] shadow-[inset_0_1px_1px_rgba(255,255,255,0.35),0_0_16px_-5px_var(--tc)]"
          />
          <div class="flex flex-col gap-[1px]">
            <span class="text-[13.5px] font-semibold text-[var(--ink)]">{{ th.name }}</span>
            <span class="text-[11px] text-[var(--ink-3)] [font-family:var(--font-mono)]">{{
              th.sub
            }}</span>
          </div>
          <span v-if="accent === th.c" class="ml-auto text-[var(--tc)] flex">
            <Icon name="lucide:check" size="17" />
          </span>
        </button>
      </div>
    </section>

    <!-- CANAUX DE CONTACT -->
    <section class="mb-[26px]">
      <div class="flex flex-col gap-[5px] mb-[12px]">
        <span
          class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap"
          >Canaux de contact pour les échanges</span
        >
        <span class="text-[12.5px] text-[var(--ink-3)]"
          >Ces canaux alimentent le bouton « Contacter » de tes transactions.</span
        >
      </div>
      <div class="flex flex-col gap-[8px]">
        <div
          v-for="ch in channels"
          :key="ch.name"
          class="flex items-center gap-[13px] px-[14px] py-[11px] rounded-[12px] border border-solid border-[var(--line)] bg-[var(--surface)] transition-all duration-[150ms] ease hover:border-[var(--line-2)] hover:bg-[var(--surface-2)]"
        >
          <span
            :class="ch.connected ? 'text-[var(--cyan)]' : 'text-[var(--ink-3)]'"
            class="grid place-items-center w-[30px] shrink-0"
          >
            <Icon :name="ch.icon" size="19" />
          </span>
          <div class="flex-1 min-w-0">
            <div
              class="text-[14px] font-semibold text-[var(--ink)] overflow-hidden text-ellipsis whitespace-nowrap"
            >
              {{ ch.name }}
            </div>
            <div
              v-if="ch.connected"
              class="text-[12px] text-[var(--ink-2)] flex items-center gap-[8px] flex-wrap [font-family:var(--font-mono)] [font-feature-settings:'tnum'_1,'ss01'_1] tracking-[-0.01em]"
            >
              {{ ch.value }}
            </div>
            <div
              v-else
              class="text-[12px] text-[var(--ink-3)] flex items-center gap-[8px] flex-wrap"
            >
              Non connecté
            </div>
          </div>
          <button
            v-if="ch.connected"
            class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line)] text-[var(--ink-2)] bg-transparent transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:text-[var(--ink)] hover:border-[var(--line-2)] hover:bg-[var(--line-3)] hover:-translate-y-px active:translate-y-0"
          >
            Modifier
          </button>
          <button
            v-else
            class="inline-flex items-center gap-[8px] justify-center py-[6px] px-[11px] rounded-[8px] text-[12px] font-semibold border border-solid border-[var(--line-2)] text-[var(--ink)] bg-[var(--surface-2)] transition-all duration-[160ms] ease whitespace-nowrap leading-none hover:bg-[var(--surface-3)] hover:border-[var(--line-2)] hover:-translate-y-px active:translate-y-0"
          >
            <Icon name="lucide:plus" size="14" /> Ajouter
          </button>
        </div>
      </div>
    </section>

    <!-- CONFIDENTIALITÉ -->
    <section>
      <span
        class="[font-family:var(--font-mono)] text-[10.5px] font-medium uppercase tracking-[0.13em] text-[var(--ink-3)] whitespace-nowrap block mb-[12px]"
        >Confidentialité de la collection</span
      >
      <div
        class="bg-[var(--glass-bg)] [backdrop-filter:blur(var(--glass-blur))_saturate(130%)] [-webkit-backdrop-filter:blur(var(--glass-blur))_saturate(130%)] border border-solid border-[var(--line)] rounded-[var(--r-lg)] shadow-[var(--shadow)] p-[6px]"
      >
        <div class="flex flex-col gap-0">
          <div class="flex items-center justify-between gap-[16px] px-[14px] py-[14px]">
            <div class="flex flex-col gap-[2px]">
              <span class="font-semibold text-[14px]">Visibilité de la collection</span>
              <span class="text-[12.5px] text-[var(--ink-3)]"
                >Qui peut voir tes cartes pour proposer un échange</span
              >
            </div>
            <SegToggle v-model="vis" :options="visOptions" size="sm" />
          </div>

          <div class="h-[1px] bg-[var(--line)]" />

          <div class="flex items-center justify-between gap-[16px] px-[14px] py-[14px]">
            <div class="flex flex-col gap-[2px]">
              <span class="font-semibold text-[14px]">Afficher la valeur estimée</span>
              <span class="text-[12.5px] text-[var(--ink-3)]"
                >Montant total visible sur ton profil public</span
              >
            </div>
            <SegToggle v-model="showVal" :options="showValOptions" size="sm" />
          </div>

          <div class="h-[1px] bg-[var(--line)]" />

          <div class="flex items-center justify-between gap-[16px] px-[14px] py-[14px]">
            <div class="flex flex-col gap-[2px]">
              <span class="font-semibold text-[14px]"
                >Apparaître dans les recherches de proximité</span
              >
              <span class="text-[12.5px] text-[var(--ink-3)]"
                >Les joueurs proches peuvent te trouver par carte ou decklist</span
              >
            </div>
            <button
              :aria-pressed="proximity"
              :class="[
                'w-[46px] h-[27px] rounded-full shrink-0 relative transition-all duration-[200ms] cursor-pointer',
                proximity
                  ? 'bg-[var(--cyan)] border border-solid border-transparent shadow-[0_0_14px_-3px_var(--cyan-glow)]'
                  : 'bg-[var(--surface-3)] border border-solid border-[var(--line-2)]',
              ]"
              @click="proximity = !proximity"
            >
              <span
                :class="[
                  'absolute top-[2px] w-[21px] h-[21px] rounded-full transition-[left] duration-[220ms] [transition-timing-function:cubic-bezier(0.5,1.3,0.5,1)]',
                  proximity ? 'left-[21px] bg-[var(--on-accent)]' : 'left-[2px] bg-[var(--ink-2)]',
                ]"
              />
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  ssr: false,
  devtools: { enabled: true },
  sourcemap: { client: 'hidden' },
  app: {
    head: {
      title: 'Arcane Exchange',
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/icon.svg' },
        {
          rel: 'stylesheet',
          type: 'text/css',
          href: '//cdn.jsdelivr.net/npm/keyrune@latest/css/keyrune.css',
        },
      ],
    },
  },
  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxt/icon',
    '@nuxt/eslint',
    '@sentry/nuxt/module',
    '@nuxt/fonts',
    '@clerk/nuxt',
    '@vueuse/nuxt',
  ],
  runtimeConfig: {
    public: {
      apiBase: '/api/v1',
    },
  },
  css: ['~/assets/css/main.css'],
  fonts: {
    families: [
      { name: 'Space Grotesk', provider: 'google', weights: [400, 500, 600, 700] },
      { name: 'Hanken Grotesk', provider: 'google', weights: [400, 500, 600, 700, 800] },
      { name: 'JetBrains Mono', provider: 'google', weights: [400, 500, 600, 700] },
    ],
  },
  tailwindcss: {
    config: {
      corePlugins: {
        preflight: false,
      },
    },
  },
  sentry: {
    org: process.env.SENTRY_ORG,
    project: process.env.SENTRY_PROJECT_FRONT,
    authToken: process.env.SENTRY_AUTH_TOKEN,
    telemetry: false,
    sourcemaps: {
      // As you're enabling client source maps, you probably want to delete them after they're uploaded to Sentry.
      // Set the appropriate glob pattern for your output folder - some glob examples below:
      filesToDeleteAfterUpload: [
        './**/*.map',
        '.*/**/public/**/*.map',
        '.output/**/public/**/*.map',
      ],
    },
  },
  nitro: {
    routeRules: {
      '/api/v1/**': {
        proxy: {
          to: `http://${process.env.NUXT_BACKEND_HOSTNAME}:8080/**`,
        },
      },
    },
  },
});

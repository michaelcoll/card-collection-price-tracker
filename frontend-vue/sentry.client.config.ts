import * as Sentry from '@sentry/nuxt';

Sentry.init({
  dsn: process.env.NUXT_PUBLIC_SENTRY_DSN,

  // Tracing
  tracesSampleRate: 1.0,

  // Logs
  enableLogs: true,
});

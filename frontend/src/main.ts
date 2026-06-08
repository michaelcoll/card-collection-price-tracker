import * as Sentry from '@sentry/angular';
import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';

Sentry.init({
  dsn: 'https://a7b75e32e922daa86acf51998f1613cc@o4511529669033984.ingest.de.sentry.io/4511540932706384',
  integrations: [Sentry.browserTracingIntegration()],
  tracesSampleRate: 1,
  enableLogs: true,
  sendDefaultPii: true,
});

bootstrapApplication(App, appConfig).catch((err) => console.error(err));

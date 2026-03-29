import {
  APP_INITIALIZER,
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter } from '@angular/router';
import { register } from '@teamhanko/hanko-elements';
import { en } from '@teamhanko/hanko-elements/i18n/en';
import { fr } from '@teamhanko/hanko-elements/i18n/fr';

import { routes } from './app.routes';
import { environment } from '../environments/environment';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    {
      provide: APP_INITIALIZER,
      // Enregistre les web components Hanko avant le rendu de l'application
      useFactory: () => () =>
        register(environment.hankoApiUrl, {
          shadow: false,
          fallbackLanguage: 'fr',
          translations: { en, fr },
        }),
      multi: true,
    },
  ],
};

import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideHttpClient, withFetch } from '@angular/common/http';
import { provideRouter } from '@angular/router';
import { Clerk } from '@clerk/clerk-js';

import { routes } from './app.routes';
import { environment } from '../environments/environment';
import { CLERK } from './core/clerk.token';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(withFetch()),
    // Provider de l'instance Clerk
    {
      provide: CLERK,
      useFactory: () => new Clerk(environment.clerkPublishableKey),
    },
    // Initialisation asynchrone au démarrage
    provideAppInitializer(() => {
      const clerk = inject(CLERK);
      return clerk.load();
    }),
  ],
};

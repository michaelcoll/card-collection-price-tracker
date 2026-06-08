import * as Sentry from '@sentry/angular';
import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
  ErrorHandler,
} from '@angular/core';
import { provideHttpClient, withFetch } from '@angular/common/http';
import { provideRouter, Router } from '@angular/router';
import { Clerk } from '@clerk/clerk-js';
import type { ClerkUIConstructor } from '@clerk/shared/types';

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
    // Clerk v6 : le bundle UI est séparé, il faut le charger avant clerk.load()
    provideAppInitializer(async () => {
      const clerk = inject(CLERK);

      // Dériver le domaine Clerk à partir de la publishable key
      const clerkDomain = atob(environment.clerkPublishableKey.split('_')[2]).slice(0, -1);

      // Charger le bundle UI Clerk (@clerk/ui) depuis le CDN
      await new Promise<void>((resolve, reject) => {
        const script = document.createElement('script');
        script.src = `https://${clerkDomain}/npm/@clerk/ui@1/dist/ui.browser.js`;
        script.async = true;
        script.crossOrigin = 'anonymous';
        script.onload = () => resolve();
        script.onerror = () => reject(new Error('Échec du chargement du bundle @clerk/ui'));
        document.head.appendChild(script);
      });

      // Initialiser Clerk avec le constructeur UI chargé dynamiquement
      return clerk.load({
        ui: {
          ClerkUI: (window as unknown as { __internal_ClerkUICtor: ClerkUIConstructor })
            .__internal_ClerkUICtor,
        },
      });
    }),
    {
      provide: ErrorHandler,
      useValue: Sentry.createErrorHandler(),
    },
    {
      provide: Sentry.TraceService,
      deps: [Router],
    },
    provideAppInitializer(() => {
      inject(Sentry.TraceService);
    }),
  ],
};

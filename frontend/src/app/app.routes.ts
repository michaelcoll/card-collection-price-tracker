import { Routes } from '@angular/router';
import { authGuard } from './core/auth.guard';

export const routes: Routes = [
  {
    path: 'collection',
    canActivate: [authGuard],
    loadComponent: () =>
      import('./collection/collection.component').then((m) => m.CollectionComponent),
  },
  { path: '', redirectTo: 'collection', pathMatch: 'full' },
];

import { Routes } from '@angular/router';

/**
 * Chaque route utilise loadComponent() pour que Angular (esbuild) génère
 * un chunk JS séparé par page — c'est l'équivalent du "vendor split" dans
 * le nouveau builder. Le fichier main.js ne contiendra que le bootstrap.
 *
 * Exemple :
 *   {
 *     path: 'collection',
 *     loadComponent: () =>
 *       import('./collection/collection.component').then(m => m.CollectionComponent),
 *   },
 */
export const routes: Routes = [];

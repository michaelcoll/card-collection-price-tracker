import { Injectable, signal, computed } from '@angular/core';

export type Theme = 'dark' | 'light';

@Injectable({ providedIn: 'root' })
export class ThemeService {
  private _theme = signal<Theme>('dark');

  readonly current = this._theme.asReadonly();

  readonly isDark = computed(() => this._theme() === 'dark');

  readonly toggleIcon = computed(() => (this._theme() === 'dark' ? 'dark_mode' : 'light_mode'));

  readonly toggleLabel = computed(() =>
    this._theme() === 'dark' ? 'Passer en mode clair' : 'Passer en mode sombre',
  );

  constructor() {
    // Respect la préférence système au démarrage
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const initial: Theme = prefersDark ? 'dark' : 'light';
    this._apply(initial);
  }

  toggle(): void {
    const next: Theme = this._theme() === 'dark' ? 'light' : 'dark';
    this._apply(next);
  }

  private _apply(theme: Theme): void {
    this._theme.set(theme);
    document.documentElement.setAttribute('data-theme', theme);
  }
}

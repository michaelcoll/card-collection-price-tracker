import { computed, inject, Injectable, signal } from '@angular/core';
import type { Resources } from '@clerk/shared/types';
import { CLERK } from './clerk.token';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly clerk = inject(CLERK);

  private readonly _userId = signal<string | null>(null);
  private readonly _userEmail = signal<string | null>(null);
  private readonly _userImageUrl = signal<string | null>(null);

  readonly isLoggedIn = computed(() => this._userId() !== null);

  /** Email de l'utilisateur, ou son ID Clerk si l'email n'est pas dans le JWT. */
  readonly displayName = computed(() => this._userEmail() ?? '');

  /** URL de l'image de profil de l'utilisateur, ou null si indisponible. */
  readonly avatarUrl = computed(() => this._userImageUrl());

  /** Initiale pour l'avatar (première lettre de l'email ou de l'ID). */
  readonly initial = computed(() => {
    const name = this._userEmail() ?? this._userId() ?? '';
    return name.charAt(0).toUpperCase();
  });

  constructor() {
    // Hydratation initiale (clerk.load() résolu avant la construction via app.config.ts)
    if (this.clerk.session && this.clerk.user) {
      this.setFromUser(this.clerk.user);
    }

    // Réactivité : mise à jour lors des changements de session
    this.clerk.addListener(({ session, user }: Resources) => {
      if (session && user) {
        this.setFromUser(user);
      } else {
        this.clearSession();
      }
    });
  }

  private setFromUser(user: NonNullable<typeof this.clerk.user>): void {
    this._userId.set(user.id);
    this._userEmail.set(user.primaryEmailAddress?.emailAddress ?? user.id);
    this._userImageUrl.set(user.imageUrl ?? null);
  }

  private clearSession(): void {
    this._userId.set(null);
    this._userEmail.set(null);
    this._userImageUrl.set(null);
  }

  /** Retourne les headers Authorization à inclure dans les requêtes API. */
  async getAuthHeaders(): Promise<Record<string, string>> {
    const token = await this.clerk.session?.getToken();
    if (!token) return {};
    return { Authorization: `Bearer ${token}` };
  }

  async logout(): Promise<void> {
    await this.clerk.signOut();
    this.clearSession();
  }
}

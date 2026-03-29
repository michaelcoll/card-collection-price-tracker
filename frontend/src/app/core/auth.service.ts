import { computed, Injectable, signal } from '@angular/core';
import { Hanko } from '@teamhanko/hanko-elements';
import { environment } from '../../environments/environment';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly hanko = new Hanko(environment.hankoApiUrl);

  private readonly _userId = signal<string | null>(null);
  private readonly _userEmail = signal<string | null>(null);

  readonly isLoggedIn = computed(() => this._userId() !== null);

  /** Email de l'utilisateur, ou son ID Hanko si l'email n'est pas dans le JWT. */
  readonly displayName = computed(() => this._userEmail() ?? '');

  /** Initiale pour l'avatar (première lettre de l'email ou de l'ID). */
  readonly initial = computed(() => {
    const name = this._userEmail() ?? this._userId() ?? '';
    return name.charAt(0).toUpperCase();
  });

  constructor() {
    // Vérification de session au démarrage (appel réseau unique)
    this.hanko
      .validateSession()
      .then(({ is_valid, claims, user_id }) => {
        if (is_valid) {
          this._userId.set(claims?.subject ?? user_id ?? null);
          this._userEmail.set(claims?.email?.address ?? claims?.subject ?? null);
        }
      })
      .catch(() => {
        // Pas de session valide — état déconnecté par défaut
      });

    // Mise à jour réactive lors des changements d'état de session
    this.hanko.onSessionCreated(({ claims }) => {
      this._userId.set(claims.subject);
      this._userEmail.set(claims.email?.address ?? claims.subject);
    });

    this.hanko.onSessionExpired(() => this.clearSession());
    this.hanko.onUserLoggedOut(() => this.clearSession());
    this.hanko.onUserDeleted(() => this.clearSession());
  }

  private clearSession(): void {
    this._userId.set(null);
    this._userEmail.set(null);
  }

  /** Retourne les headers Authorization à inclure dans les requêtes API. */
  getAuthHeaders(): Record<string, string> {
    const token = this.hanko.getSessionToken();
    if (!token) return {};
    return { Authorization: `Bearer ${token}` };
  }

  async logout(): Promise<void> {
    await this.hanko.logout();
    this.clearSession();
  }
}

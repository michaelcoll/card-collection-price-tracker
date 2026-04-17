import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { from, Observable, switchMap } from 'rxjs';
import { AuthService } from '../core/auth.service';

@Injectable({ providedIn: 'root' })
export class CardService {
  private readonly http = inject(HttpClient);
  private readonly auth = inject(AuthService);

  importCollection(file: File): Observable<void> {
    return from(this.auth.getAuthHeaders()).pipe(
      switchMap((authHeaders) => {
        const headers = new HttpHeaders({
          ...authHeaders,
          'Content-Type': 'text/csv',
        });
        return this.http.post<void>('/cards/import', file, { headers });
      }),
    );
  }
}

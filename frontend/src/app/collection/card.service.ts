import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { from, Observable, switchMap } from 'rxjs';
import { AuthService } from '../core/auth.service';
import { PaginatedCollection } from '../api/bindings/PaginatedCollection';
import { SortDir } from '../api/bindings/SortDir';
import { SortBy } from '../api/bindings/SortBy';

@Injectable({ providedIn: 'root' })
export class CardService {
  private readonly http = inject(HttpClient);
  private readonly auth = inject(AuthService);

  getCollection(
    page: number,
    pageSize: number,
    sortBy: SortBy,
    sortDir: SortDir,
  ): Observable<PaginatedCollection> {
    return from(this.auth.getAuthHeaders()).pipe(
      switchMap((authHeaders) => {
        const headers = new HttpHeaders(authHeaders);
        const params = new HttpParams()
          .set('page', page)
          .set('page_size', pageSize)
          .set('sort_by', sortBy)
          .set('sort_dir', sortDir);
        return this.http.get<PaginatedCollection>('/cards', { headers, params });
      }),
    );
  }

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

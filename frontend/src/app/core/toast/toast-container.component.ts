import { Component, inject } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';
import { ToastService } from '../toast.service';

@Component({
  selector: 'app-toast-container',
  standalone: true,
  imports: [MatIconModule],
  template: `
    <div class="toast-container" aria-live="polite" aria-atomic="false">
      @for (toast of toastService.toasts(); track toast.id) {
        <div
          class="toast"
          [class.toast--success]="toast.type === 'success'"
          [class.toast--error]="toast.type === 'error'"
          role="status"
        >
          <mat-icon fontSet="material-symbols-rounded" class="toast__icon">
            {{ toast.type === 'success' ? 'check_circle' : 'error' }}
          </mat-icon>
          <span class="toast__message">{{ toast.message }}</span>
          <button class="toast__close" (click)="toastService.dismiss(toast.id)" aria-label="Fermer">
            <mat-icon fontSet="material-symbols-rounded">close</mat-icon>
          </button>
        </div>
      }
    </div>
  `,
  styleUrl: './toast-container.component.css',
})
export class ToastContainerComponent {
  protected readonly toastService = inject(ToastService);
}

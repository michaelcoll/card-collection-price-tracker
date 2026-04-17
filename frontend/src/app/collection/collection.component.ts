import { Component, ElementRef, inject, signal, ViewChild } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';
import { CardService } from './card.service';
import { ToastService } from '../core/toast.service';

@Component({
  selector: 'app-collection',
  standalone: true,
  imports: [MatIconModule],
  templateUrl: './collection.component.html',
  styleUrl: './collection.component.css',
})
export class CollectionComponent {
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;

  private readonly cardService = inject(CardService);
  private readonly toastService = inject(ToastService);

  protected readonly isLoading = signal(false);

  triggerFileInput(): void {
    this.fileInput.nativeElement.click();
  }

  onFileSelected(event: Event): void {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    this.isLoading.set(true);

    this.cardService.importCollection(file).subscribe({
      next: () => {
        this.isLoading.set(false);
        this.toastService.success('Collection importée avec succès !');
        input.value = '';
      },
      error: (err) => {
        this.isLoading.set(false);
        console.log(err);
        this.toastService.error(`Erreur lors de l'import : ${err.message ?? 'Inconnue'}`);
        input.value = '';
      },
    });
  }
}

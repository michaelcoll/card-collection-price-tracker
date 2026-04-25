import {
  AfterViewInit,
  Component,
  ElementRef,
  inject,
  OnDestroy,
  OnInit,
  signal,
  ViewChild,
} from '@angular/core';
import { MatIconModule } from '@angular/material/icon';
import { ToastService } from '../core/toast.service';
import { CardItemComponent } from './card-item/card-item.component';
import { CardService } from './card.service';
import { CollectionCard } from '../api/bindings/CollectionCard';
import { SortDir } from '../api/bindings/SortDir';
import { SortBy } from '../api/bindings/SortBy';

@Component({
  selector: 'app-collection',
  standalone: true,
  imports: [MatIconModule, CardItemComponent],
  templateUrl: './collection.component.html',
})
export class CollectionComponent implements OnInit, AfterViewInit, OnDestroy {
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;
  @ViewChild('sentinel') sentinel!: ElementRef<HTMLDivElement>;

  private readonly cardService = inject(CardService);
  private readonly toastService = inject(ToastService);

  protected readonly isLoading = signal(false);
  protected readonly isLoadingMore = signal(false);
  protected readonly cards = signal<CollectionCard[]>([]);
  protected readonly total = signal(0);
  protected readonly sortDir = signal<SortDir>('desc');

  private currentPage = 0;
  private readonly pageSize = 60;
  private hasMore = true;
  private observer?: IntersectionObserver;

  ngOnInit(): void {
    this.loadPage(0, true);
  }

  ngAfterViewInit(): void {
    this.observer = new IntersectionObserver(
      (entries) => {
        if (
          entries[0].isIntersecting &&
          !this.isLoading() &&
          !this.isLoadingMore() &&
          this.hasMore
        ) {
          this.loadPage(this.currentPage + 1, false);
        }
      },
      { threshold: 0.1 },
    );
    if (this.sentinel) {
      this.observer.observe(this.sentinel.nativeElement);
    }
  }

  ngOnDestroy(): void {
    this.observer?.disconnect();
  }

  toggleSort(): void {
    this.sortDir.update((d) => (d === 'desc' ? 'asc' : 'desc'));
    this.loadPage(0, true);
  }

  private loadPage(page: number, reset: boolean): void {
    if (reset) {
      this.hasMore = true;
      this.isLoading.set(true);
    } else {
      this.isLoadingMore.set(true);
    }

    this.cardService.getCollection(page, this.pageSize, 'trend', this.sortDir()).subscribe({
      next: (result) => {
        this.total.set(result.total);
        if (reset) {
          this.cards.set(result.items);
        } else {
          this.cards.update((prev) => [...prev, ...result.items]);
        }
        this.currentPage = page;
        this.hasMore = result.items.length === this.pageSize;
        this.isLoading.set(false);
        this.isLoadingMore.set(false);
      },
      error: (err) => {
        this.isLoading.set(false);
        this.isLoadingMore.set(false);
        this.toastService.error(`Erreur lors du chargement : ${err.message ?? 'Inconnue'}`);
      },
    });
  }

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
        this.toastService.success('Collection importée avec succès !');
        input.value = '';
        this.loadPage(0, true);
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

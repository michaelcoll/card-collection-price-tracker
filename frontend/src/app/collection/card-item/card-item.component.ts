import { Component, input } from '@angular/core';
import { CollectionCard } from '../../api/bindings/CollectionCard';
import { MatIcon } from '@angular/material/icon';

@Component({
  selector: 'app-card-item',
  standalone: true,
  templateUrl: './card-item.component.html',
  imports: [MatIcon],
})
export class CardItemComponent {
  card = input.required<CollectionCard>();

  get imageUrl(): string {
    return `https://api.scryfall.com/cards/${this.card().scryfall_id}?format=image&version=normal`;
  }

  get trendPrice(): string | null {
    const trend = this.card().price_guide?.trend;
    if (trend == null) return null;
    return (trend / 100).toLocaleString(navigator.language, {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  }

  get trendPriceDiff(): string | null {
    const trend = this.card().price_guide?.trend;
    const purchase = this.card().purchase_price;
    if (trend == null) return null;
    return ((trend - purchase) / 100).toLocaleString(navigator.language, {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });
  }

  get isPriceTrendingDown(): boolean {
    const trend = this.card().price_guide?.trend;
    if (trend == null) return false;
    return this.card().purchase_price > trend;
  }

  get stackAngles(): number[] {
    const count = Math.min(this.card().quantity - 1, 6);
    if (count <= 0) return [];
    if (count === 1) return [-4];
    const maxAngle = 4 + (count - 1) * 1.5;
    return Array.from({ length: count }, (_, i) =>
      Math.round(-maxAngle + (i / (count - 1)) * 2 * maxAngle),
    );
  }

  get description(): string {
    const foil = this.card().foil ? '⭑' : '·';
    return `${this.card().collector_number} ${this.card().set_code.toUpperCase()} ${foil} ${this.card().language_code}`;
  }
}

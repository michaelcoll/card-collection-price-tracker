import type { CollectionStats } from '~/bindings/CollectionStats';
import type { Message } from '~/bindings/Message';
import type { PaginatedCollection } from '~/bindings/PaginatedCollection';
import type { PriceHistoryEntry } from '~/bindings/PriceHistoryEntry';
import type { SortBy } from '~/bindings/SortBy';
import type { SortDir } from '~/bindings/SortDir';

type GetPriceHistoryParams = {
  start_date: string;
  end_date: string;
};

type GetCardPriceHistoryParams = Partial<GetPriceHistoryParams>;

type GetCollectionParams = {
  page?: number;
  page_size?: number;
  sort_by?: SortBy;
  sort_dir?: SortDir;
  q?: string;
  rarity?: string;
  sets?: string;
  price_min?: number;
  price_max?: number;
  owned?: boolean;
};

export const useCardsService = () => {
  const { apiCall } = useApi();

  const getCollection = (params?: MaybeRefOrGetter<GetCollectionParams>) =>
    useAsyncData(
      'cards-collection',
      () => apiCall<PaginatedCollection>('/cards', { query: toValue(params) }),
      { lazy: true },
    );

  const getCollection2 = (params?: MaybeRefOrGetter<GetCollectionParams>) =>
    apiCall<PaginatedCollection>('/cards', { query: toValue(params) });

  const importCards = (csv: string) =>
    apiCall<Message>('/cards/import', {
      method: 'POST',
      body: csv,
      headers: { 'Content-Type': 'text/plain' },
    });

  const getCardInfo = () => apiCall('/cards/card-info', { method: 'POST' });

  const getCollectionStats = () =>
    useAsyncData('cards-collection-stats', () => apiCall<CollectionStats>('/cards/stats'), {
      lazy: true,
    });

  const getPriceHistory = (params: MaybeRefOrGetter<GetPriceHistoryParams>) =>
    useAsyncData(
      'cards-price-history',
      () => apiCall<PriceHistoryEntry[]>('/cards/price-history', { query: toValue(params) }),
      { lazy: true },
    );

  const getCardPriceHistory = (scryfallId: string, params?: GetCardPriceHistoryParams) =>
    apiCall<PriceHistoryEntry[]>(`/cards/${scryfallId}/price-history`, { query: params });

  return {
    getCollection,
    getCollection2,
    importCards,
    getCardInfo,
    getCollectionStats,
    getPriceHistory,
    getCardPriceHistory,
  };
};

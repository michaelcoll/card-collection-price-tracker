import type { CollectionParams } from '~/bindings/CollectionParams';
import type { CollectionStats } from '~/bindings/CollectionStats';
import type { Message } from '~/bindings/Message';
import type { PaginatedCollection } from '~/bindings/PaginatedCollection';
import type { PriceHistoryEntry } from '~/bindings/PriceHistoryEntry';
import type { PriceHistoryParams } from '~/bindings/PriceHistoryParams';

export const useCollectionService = () => {
  const { apiCall } = useApi();

  const getCollection = (params?: MaybeRefOrGetter<CollectionParams>) =>
    useAsyncData(
      'collection',
      () => apiCall<PaginatedCollection>('/collection', { query: toValue(params) }),
      { lazy: true },
    );

  const importCards = (csv: string) =>
    apiCall<Message>('/collection/import', {
      method: 'POST',
      body: csv,
      headers: { 'Content-Type': 'text/plain' },
    });

  const getCollectionStats = () =>
    useAsyncData('collection-stats', () => apiCall<CollectionStats>('/collection/stats'), {
      lazy: true,
    });

  const getPriceHistory = (params: MaybeRefOrGetter<PriceHistoryParams>) =>
    useAsyncData(
      'collection-price-history',
      () => apiCall<PriceHistoryEntry[]>('/collection/price-history', { query: toValue(params) }),
      { lazy: true },
    );

  return {
    getCollection,
    importCards,
    getCollectionStats,
    getPriceHistory,
  };
};

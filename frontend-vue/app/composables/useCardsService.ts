import type { PriceHistoryEntry } from '~/bindings/PriceHistoryEntry';
import type { PriceHistoryParams } from '~/bindings/PriceHistoryParams';

export const useCardsService = () => {
  const { apiCall } = useApi();

  const getCardInfo = () => apiCall('/cards/card-info', { method: 'POST' });

  const getCardPriceHistory = (scryfallId: string, params?: PriceHistoryParams) =>
    apiCall<PriceHistoryEntry[]>(`/cards/${scryfallId}/price-history`, { query: params });

  return {
    getCardInfo,
    getCardPriceHistory,
  };
};

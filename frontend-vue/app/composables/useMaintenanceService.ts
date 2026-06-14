type StatsResponse = {
  card_number: number;
  card_price_number: number;
  db_size_mb: number;
};

type EnqueueResponse = {
  enqueued: number;
};

export const useMaintenanceService = () => {
  const config = useRuntimeConfig();
  const base = config.public.apiBase;

  const getStats = () =>
    useAsyncData('maintenance-stats', () => $fetch<StatsResponse>(`${base}/maintenance/stats`));

  const triggerPriceUpdate = () =>
    $fetch(`${base}/maintenance/trigger-price-update`, { method: 'POST' });

  const updateCardmarketIds = () =>
    $fetch<EnqueueResponse>(`${base}/maintenance/update-cardmarket-ids`, {
      method: 'POST',
    });

  return { getStats, triggerPriceUpdate, updateCardmarketIds };
};

export default function (value?: number) {
  return Intl.NumberFormat('fr-FR', {
    style: 'currency',
    maximumFractionDigits: 2,
    minimumFractionDigits: 0,
    currency: 'EUR',
  }).format((value ?? 0) / 100);
}

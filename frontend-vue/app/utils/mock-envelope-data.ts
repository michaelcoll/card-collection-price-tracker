export type EnvelopePoint = { low: number; avg: number; trend: number; label: string };

// Mock daily envelope (low/avg/trend) — replaced by a real backend series in a later phase.
export default function (): EnvelopePoint[] {
  const MONTHS = [
    'jan',
    'fév',
    'mar',
    'avr',
    'mai',
    'juin',
    'juil',
    'aoû',
    'sep',
    'oct',
    'nov',
    'déc',
  ];
  const days = 30;
  const out: EnvelopePoint[] = [];
  const today = new Date();
  for (let i = 0; i < days; i++) {
    const t = i / (days - 1);
    const trendBase = 4130 + t * 88;
    const wave = Math.sin(i / 3.1) * 10 + Math.sin(i / 6.5) * 6;
    const avg = trendBase + wave;
    const low = avg - (26 + ((i * 53) % 34));
    const trend = avg + (30 + ((i * 29) % 42));
    const dt = new Date(today);
    dt.setDate(today.getDate() - (days - 1 - i));
    out.push({ low, avg, trend, label: dt.getDate() + ' ' + MONTHS[dt.getMonth()]! });
  }
  return out;
}

export function getRelativeTime(
  date: Date | number,
  lang = navigator.language
): string {
  const timeMs = typeof date === 'number' ? date : date.getTime();
  const timeDiff = Math.round((timeMs - Date.now()) / 1000);
  const cutoffs = [
    60,
    3600,
    86400,
    86400 * 7,
    86400 * 30,
    86400 * 365,
    Infinity,
  ];
  const units: Intl.RelativeTimeFormatUnit[] = [
    'second',
    'minute',
    'hour',
    'day',
    'week',
    'month',
    'year',
  ];
  const unitIndex = cutoffs.findIndex((cutoff) => cutoff > Math.abs(timeDiff));
  const divisor = unitIndex ? cutoffs[unitIndex - 1] : 1;

  const rtf = new Intl.RelativeTimeFormat(lang, { numeric: 'always' });
  return rtf.format(Math.floor(timeDiff / divisor), units[unitIndex]);
}

export function getFormattedTime(
  date: Date | number,
  lang = navigator.language
): string {
  return new Intl.DateTimeFormat(lang, {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
    second: '2-digit',
    timeZoneName: 'short',
  }).format(date);
}

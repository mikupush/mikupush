export function formatRate(rateBytes: number): string {
  const maxBytes = 1024
  const maxKb = 1024 * 1024
  const maxMb = 1024 * 1024 * 1024

  if (rateBytes < maxBytes) {
    return `${rateBytes} B/s`;
  }

  if (rateBytes < maxKb) {
    return `${Math.round(rateBytes / 1024)} KB/s`;
  }

  if (rateBytes < maxMb) {
    return `${(rateBytes / (1024 * 1024)).toFixed(1)} MB/s`;
  }

  return `${(rateBytes / (1024 * 1024 * 1024)).toFixed(1)} GB/s`;
}

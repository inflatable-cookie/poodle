export function missingNoticeMarkers(
  source: string,
  markers: readonly string[],
): string[] {
  return markers.filter((marker) => !source.includes(marker));
}

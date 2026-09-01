/**
 * Adapter-owned nearest-scroll-container auto-scroll.
 *
 * Spec 069: one frame loop, nearest eligible container that can still scroll
 * in the requested direction, acceleration near the edge. The controller owns
 * the loop; this module is the pure choice + delta.
 */

export interface AutoScrollRect {
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly left: number;
}

export interface AutoScrollMetrics {
  readonly scrollTop: number;
  readonly scrollLeft: number;
  readonly scrollHeight: number;
  readonly scrollWidth: number;
  readonly clientHeight: number;
  readonly clientWidth: number;
  readonly rect: AutoScrollRect;
  /** When false, this box must not be auto-scrolled on X. Default true. */
  readonly overflowX?: boolean;
  /** When false, this box must not be auto-scrolled on Y. Default true. */
  readonly overflowY?: boolean;
}

export interface AutoScrollCandidate {
  readonly id: string;
  /** Higher wins when several containers contain the pointer. */
  readonly depth: number;
  readonly metrics: AutoScrollMetrics;
}

export interface AutoScrollPointer {
  readonly x: number;
  readonly y: number;
}

export interface AutoScrollEdgeConfig {
  readonly size: number;
  readonly minPxPerSecond: number;
  readonly maxPxPerSecond: number;
}

export interface AutoScrollDelta {
  readonly id: string;
  readonly dx: number;
  readonly dy: number;
}

export const DEFAULT_AUTO_SCROLL_EDGE: AutoScrollEdgeConfig = Object.freeze({
  size: 28,
  minPxPerSecond: 40,
  maxPxPerSecond: 1200,
});

const SCROLL_EPSILON = 0.5;

export function canScrollVertical(metrics: AutoScrollMetrics, direction: -1 | 1): boolean {
  if (metrics.overflowY === false) return false;
  if (metrics.scrollHeight - metrics.clientHeight <= SCROLL_EPSILON) return false;
  if (direction < 0) return metrics.scrollTop > SCROLL_EPSILON;
  return metrics.scrollTop + metrics.clientHeight < metrics.scrollHeight - SCROLL_EPSILON;
}

export function canScrollHorizontal(metrics: AutoScrollMetrics, direction: -1 | 1): boolean {
  if (metrics.overflowX === false) return false;
  if (metrics.scrollWidth - metrics.clientWidth <= SCROLL_EPSILON) return false;
  if (direction < 0) return metrics.scrollLeft > SCROLL_EPSILON;
  return metrics.scrollLeft + metrics.clientWidth < metrics.scrollWidth - SCROLL_EPSILON;
}

function containsPoint(rect: AutoScrollRect, pointer: AutoScrollPointer): boolean {
  return pointer.x >= rect.left && pointer.x <= rect.right && pointer.y >= rect.top && pointer.y <= rect.bottom;
}

function edgeSpeed(distanceFromEdge: number, size: number, config: AutoScrollEdgeConfig): number {
  if (size <= 0) return 0;
  const t = Math.min(1, Math.max(0, 1 - distanceFromEdge / size));
  if (t <= 0) return 0;
  return config.minPxPerSecond + t * (config.maxPxPerSecond - config.minPxPerSecond);
}

function axisDelta(
  distanceFromStart: number,
  distanceFromEnd: number,
  size: number,
  canStart: boolean,
  canEnd: boolean,
  dtMs: number,
  config: AutoScrollEdgeConfig,
): number {
  const towardStart = canStart ? edgeSpeed(distanceFromStart, size, config) : 0;
  const towardEnd = canEnd ? edgeSpeed(distanceFromEnd, size, config) : 0;
  if (towardStart <= 0 && towardEnd <= 0) return 0;
  const px = ((towardStart > towardEnd ? -towardStart : towardEnd) * dtMs) / 1000;
  return px;
}

function edgeVector(
  metrics: AutoScrollMetrics,
  pointer: AutoScrollPointer,
  config: AutoScrollEdgeConfig,
): { dx: number; dy: number } | null {
  if (!containsPoint(metrics.rect, pointer)) return null;
  const size = Math.max(0, config.size);
  const dy = axisDelta(
    pointer.y - metrics.rect.top,
    metrics.rect.bottom - pointer.y,
    size,
    true,
    true,
    1000,
    config,
  );
  const dx = axisDelta(
    pointer.x - metrics.rect.left,
    metrics.rect.right - pointer.x,
    size,
    true,
    true,
    1000,
    config,
  );
  if (dx === 0 && dy === 0) return null;
  return { dx, dy };
}

/**
 * Direction comes from the deepest container's edge. Ownership walks up to the
 * nearest ancestor that can still scroll in that direction.
 */
export function resolveAutoScroll(
  candidates: readonly AutoScrollCandidate[],
  pointer: AutoScrollPointer,
  dtMs: number,
  config: AutoScrollEdgeConfig = DEFAULT_AUTO_SCROLL_EDGE,
): AutoScrollDelta | null {
  if (dtMs <= 0) return null;

  const containing = candidates
    .filter((candidate) => containsPoint(candidate.metrics.rect, pointer))
    .sort((left, right) => right.depth - left.depth || (left.id < right.id ? -1 : left.id > right.id ? 1 : 0));

  const origin = containing[0];
  if (!origin) return null;
  const direction = edgeVector(origin.metrics, pointer, config);
  if (!direction) return null;

  const wantY: -1 | 1 | 0 = direction.dy < 0 ? -1 : direction.dy > 0 ? 1 : 0;
  const wantX: -1 | 1 | 0 = direction.dx < 0 ? -1 : direction.dx > 0 ? 1 : 0;
  const scale = dtMs / 1000;

  for (const candidate of containing) {
    const dy = wantY !== 0 && canScrollVertical(candidate.metrics, wantY) ? direction.dy * scale : 0;
    const dx = wantX !== 0 && canScrollHorizontal(candidate.metrics, wantX) ? direction.dx * scale : 0;
    if (dx === 0 && dy === 0) continue;
    return { id: candidate.id, dx, dy };
  }

  return null;
}

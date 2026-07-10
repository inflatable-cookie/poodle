/**
 * Anchor-positioning machinery (Tooltip, Menu, IconButton tooltips;
 * HoverCard pending; Popover keeps CSS anchoring by contract delta).
 *
 * Pure collision-aware placement resolver promoted from the Svelte
 * implementation's overlay-position module. Candidate placements are scored
 * by viewport overflow then anchor overlap; the first fully-clean candidate
 * wins, otherwise the best-scoring one, clamped into the viewport.
 *
 * Decision record: this replaces the planned Floating UI dependency — the
 * in-house resolver is pure, dependency-free, pixel-identical for existing
 * consumers, and ports to the Rust runtimes. See spec 062.
 */

export type OverlaySide = "top" | "right" | "bottom" | "left";

export type OverlayPlacement =
  | OverlaySide
  | `${OverlaySide}-start`
  | `${OverlaySide}-end`;

export interface RectLike {
  top: number;
  right: number;
  bottom: number;
  left: number;
  width: number;
  height: number;
}

export interface ViewportSize {
  width: number;
  height: number;
}

export interface OverlayPosition {
  placement: OverlayPlacement;
  top: number;
  left: number;
}

const VIEWPORT_PADDING = 8;

function computePosition(
  anchorRect: RectLike,
  overlayRect: RectLike,
  placement: OverlayPlacement,
  offset: number,
): OverlayPosition {
  const [side, align = "center"] = placement.split("-") as [OverlaySide, "start" | "end" | "center" | undefined];

  let top = 0;
  let left = 0;

  if (side === "top" || side === "bottom") {
    top = side === "top" ? anchorRect.top - overlayRect.height - offset : anchorRect.bottom + offset;
    left =
      align === "start"
        ? anchorRect.left
        : align === "end"
          ? anchorRect.right - overlayRect.width
          : anchorRect.left + (anchorRect.width - overlayRect.width) / 2;
  } else {
    left = side === "left" ? anchorRect.left - overlayRect.width - offset : anchorRect.right + offset;
    top =
      align === "start"
        ? anchorRect.top
        : align === "end"
          ? anchorRect.bottom - overlayRect.height
          : anchorRect.top + (anchorRect.height - overlayRect.height) / 2;
  }

  return { placement, top, left };
}

function oppositePlacement(placement: OverlayPlacement): OverlayPlacement {
  if (placement.startsWith("top")) return placement.replace("top", "bottom") as OverlayPlacement;
  if (placement.startsWith("bottom")) return placement.replace("bottom", "top") as OverlayPlacement;
  if (placement.startsWith("left")) return placement.replace("left", "right") as OverlayPlacement;
  return placement.replace("right", "left") as OverlayPlacement;
}

function relatedPlacements(placement: OverlayPlacement): OverlayPlacement[] {
  const [side, align] = placement.split("-") as [OverlaySide, "start" | "end" | undefined];

  if (!align) {
    return [`${side}-start`, `${side}-end`] as OverlayPlacement[];
  }

  return [side];
}

function axisFallbacks(placement: OverlayPlacement): OverlayPlacement[] {
  if (placement.startsWith("top") || placement.startsWith("bottom")) {
    return ["right", "left", "right-start", "right-end", "left-start", "left-end"];
  }

  return ["bottom", "top", "bottom-start", "bottom-end", "top-start", "top-end"];
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

function normalizeWithinViewport(
  position: OverlayPosition,
  overlayRect: RectLike,
  viewport: ViewportSize,
): OverlayPosition {
  const maxLeft = Math.max(VIEWPORT_PADDING, viewport.width - overlayRect.width - VIEWPORT_PADDING);
  const maxTop = Math.max(VIEWPORT_PADDING, viewport.height - overlayRect.height - VIEWPORT_PADDING);

  return {
    ...position,
    left: clamp(position.left, VIEWPORT_PADDING, maxLeft),
    top: clamp(position.top, VIEWPORT_PADDING, maxTop),
  };
}

function overflowAmount(position: OverlayPosition, overlayRect: RectLike, viewport: ViewportSize): number {
  const overflowLeft = Math.max(0, VIEWPORT_PADDING - position.left);
  const overflowTop = Math.max(0, VIEWPORT_PADDING - position.top);
  const overflowRight = Math.max(0, position.left + overlayRect.width - (viewport.width - VIEWPORT_PADDING));
  const overflowBottom = Math.max(0, position.top + overlayRect.height - (viewport.height - VIEWPORT_PADDING));

  return overflowLeft + overflowTop + overflowRight + overflowBottom;
}

function overlapAmount(position: OverlayPosition, anchorRect: RectLike, overlayRect: RectLike): number {
  const overlayRight = position.left + overlayRect.width;
  const overlayBottom = position.top + overlayRect.height;

  const overlapWidth = Math.max(0, Math.min(overlayRight, anchorRect.right) - Math.max(position.left, anchorRect.left));
  const overlapHeight = Math.max(0, Math.min(overlayBottom, anchorRect.bottom) - Math.max(position.top, anchorRect.top));

  return overlapWidth * overlapHeight;
}

function candidatePlacements(preferredPlacement: OverlayPlacement): OverlayPlacement[] {
  const ordered = [
    preferredPlacement,
    oppositePlacement(preferredPlacement),
    ...relatedPlacements(preferredPlacement),
    ...relatedPlacements(oppositePlacement(preferredPlacement)),
    ...axisFallbacks(preferredPlacement),
  ];

  return [...new Set(ordered)];
}

export function resolveOverlayPosition(
  anchorRect: RectLike,
  overlayRect: RectLike,
  preferredPlacement: OverlayPlacement,
  viewport: ViewportSize,
  offset = 6,
): OverlayPosition {
  let bestRawPosition = computePosition(anchorRect, overlayRect, preferredPlacement, offset);
  let bestPosition = normalizeWithinViewport(bestRawPosition, overlayRect, viewport);
  let lowestOverflow = overflowAmount(bestRawPosition, overlayRect, viewport);
  let lowestOverlap = overlapAmount(bestPosition, anchorRect, overlayRect);

  for (const placement of candidatePlacements(preferredPlacement)) {
    const rawCandidate = computePosition(anchorRect, overlayRect, placement, offset);
    const candidate = normalizeWithinViewport(rawCandidate, overlayRect, viewport);
    const overflow = overflowAmount(rawCandidate, overlayRect, viewport);
    const overlap = overlapAmount(candidate, anchorRect, overlayRect);

    if (overflow < lowestOverflow || (overflow === lowestOverflow && overlap < lowestOverlap)) {
      bestRawPosition = rawCandidate;
      bestPosition = candidate;
      lowestOverflow = overflow;
      lowestOverlap = overlap;
    }

    if (overflow === 0 && overlap === 0) {
      return candidate;
    }
  }

  return normalizeWithinViewport(bestRawPosition, overlayRect, viewport);
}

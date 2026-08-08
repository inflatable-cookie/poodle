import { resolveOverlayPosition as resolveCore, type RectLike } from "@poodle/headless";

import type { OverlayPlacement } from "./types";

type OverlayPosition = {
  placement: OverlayPlacement;
  top: number;
  left: number;
};

/**
 * Window-bound wrapper over the core anchor-positioning machinery: same
 * signature the Svelte components always used, viewport read from `window`.
 */
export function resolveOverlayPosition(
  anchorRect: RectLike,
  overlayRect: RectLike,
  preferredPlacement: OverlayPlacement,
  offset = 6,
): OverlayPosition {
  return resolveCore(
    anchorRect,
    overlayRect,
    preferredPlacement,
    { width: window.innerWidth, height: window.innerHeight },
    offset,
  ) as OverlayPosition;
}

/**
 * Accepted Svelte<->React pixel deltas.
 *
 * Every entry needs a written reason. An allowlist without reasons is
 * suppression, not documentation — if a delta cannot be explained, it is a bug
 * in one of the shells and belongs in a fix, not here.
 *
 * `maxDiffRatio` is the fraction of differing pixels tolerated for that slug
 * (all axes). Absent slugs use DEFAULT_MAX_DIFF_RATIO.
 */

export type AllowlistEntry = {
  maxDiffRatio: number;
  reason: string;
};

/**
 * Same browser, same stylesheet, same tokens — matching shells should render
 * identically, so the default floor is near zero (sub-pixel text rasterisation
 * only).
 */
export const DEFAULT_MAX_DIFF_RATIO = 0.0002;

export const ALLOWLIST: Record<string, AllowlistEntry> = {
  "scroll-shell": {
    maxDiffRatio: 0.001,
    reason:
      "Sub-pixel text antialiasing inside the composited scroll viewport. " +
      "Verified identical: DOM, computed styles, box geometry (viewport and " +
      "row offsets match to the sub-pixel), scrollTop, and glyph shapes at 8x " +
      "zoom — only edge-pixel coverage differs, because the two previews place " +
      "the specimen at a different absolute Y and the scroller's layer lands on " +
      "a different device-pixel phase. Hiding all preview chrome moves the " +
      "number (0.044% -> 0.036%) without removing it. ~250 glyph-edge pixels.",
  },
};

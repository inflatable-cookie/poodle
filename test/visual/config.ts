import { allComponents } from "../../packages/svelte/preview/src/component-registry";

/**
 * Cross-framework visual gate configuration (g12.009).
 *
 * The Svelte and React previews serve the same `#components/<slug>` routes from
 * the same stylesheet, so a pixel diff between them is self-verifying evidence:
 * any real difference is a bug in one shell, not an expected delta.
 */

export type Axis = {
  id: string;
  theme: string;
  density: string;
  controlSize: string;
  /** `--poodle-contrast` override; 0.5 is the neutral default. */
  contrast: number;
};

export type Framework = "svelte" | "react";

export const SERVERS: Record<Framework, { cwd: string; port: number }> = {
  svelte: { cwd: "packages/svelte/preview", port: 4174 },
  react: { cwd: "packages/react/preview", port: 4180 },
};

/** First `__section` on a component page is the specimen; both apps use it. */
export const SPECIMEN_SELECTOR = ".poodle-component-page__section";

export const VIEWPORT = { width: 1280, height: 900 };

/** Frozen wall clock — TimeAgo, Calendar "today", LogList all read it. */
export const FIXED_TIME = new Date("2026-06-15T12:00:00.000Z");

/**
 * Components whose rendering is inherently non-deterministic (animation loops,
 * media surfaces, randomised placeholder content). Skipped rather than silently
 * passed; the run summary lists every one.
 */
export const SKIPPED: Record<string, string> = {
  spinner: "continuous rotation animation",
  skeleton: "shimmer animation loop",
  progress: "indeterminate variant animates",
  "page-loading": "composes Spinner",
  "video-player": "media element paint timing",
  "audio-player": "media element paint timing",
  "media-thumbnail": "remote/media-decode dependent paint",
};

/**
 * Axis tier: the components where size/density/contrast bugs actually surface.
 * This is the tier that would have caught the ListCard `data-size` split.
 */
export const AXIS_TIER_SLUGS = [
  "button",
  "icon-button",
  "text-input",
  "select",
  "checkbox",
  "switch",
  "tabs",
  "pill",
  "list-card",
  "data-table",
  "menu",
  "dialog",
  "toolbar",
  "card",
  "slider",
];

const SIZES = ["xs", "sm", "md", "lg", "xl"];
const DENSITIES = ["compact", "comfortable"];

function axis(
  theme: string,
  density: string,
  controlSize: string,
  contrast = 0.5,
): Axis {
  const contrastId = contrast === 0.5 ? "" : `-c${contrast}`;
  return {
    id: `${theme}-${density}-${controlSize}${contrastId}`,
    theme,
    density,
    controlSize,
    contrast,
  };
}

/** Wave 1 proving axis: one theme, one size, one density. */
export const SMOKE_AXES: Axis[] = [axis("eclipse", "compact", "md")];

/** Wave 2: every size stop, both densities, plus the contrast extremes. */
export const AXIS_TIER_AXES: Axis[] = [
  ...SIZES.flatMap((size) =>
    DENSITIES.map((density) => axis("eclipse", density, size)),
  ),
  axis("eclipse", "compact", "md", 0.9),
  axis("iceberg", "compact", "md", 0.1),
];

/** Wave 3: every slug, both reference themes, default size/density. */
export const SWEEP_AXES: Axis[] = [
  axis("eclipse", "compact", "md"),
  axis("iceberg", "compact", "md"),
];

export function sweepSlugs(): string[] {
  return allComponents
    .filter((entry) => entry.hasSpecimen)
    .map((entry) => entry.slug);
}

export type Tier = "smoke" | "axis" | "sweep";

export function tierPlan(tier: Tier): { slugs: string[]; axes: Axis[] } {
  if (tier === "smoke") return { slugs: AXIS_TIER_SLUGS, axes: SMOKE_AXES };
  if (tier === "axis") return { slugs: AXIS_TIER_SLUGS, axes: AXIS_TIER_AXES };
  return { slugs: sweepSlugs(), axes: SWEEP_AXES };
}

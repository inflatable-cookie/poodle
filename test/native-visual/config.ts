/**
 * Native visual gate configuration (g12.014).
 *
 * The web gate (`test/visual/`) diffs Svelte against React, and can do that
 * because both emit the same DOM from the same stylesheet — any difference is a
 * bug by construction, so it needs no committed baselines.
 *
 * The native targets have no such twin. GPUI is a different renderer with its
 * own shell, font stack and compositor; diffing it against Svelte would be all
 * noise. So this gate is a **baseline** gate: capture, commit, and diff against
 * the committed image on the next change. It answers "did this edit move native
 * rendering?", which is the question the structural gates cannot ask.
 *
 * On determinism, after several wrong answers: a single capture is NOT
 * reliable. The preview waits a fixed 1.5s for its first render and captures
 * whatever is on screen, which sometimes catches an incomplete frame — a
 * `segmented-control` baseline written that way had the selected segment barely
 * painted. Symptoms were a rotating set of failures, ratios that drifted for
 * the same component between runs, and bad frames frozen into baselines.
 *
 * So `captureSlugStable` accepts an image only when two consecutive captures
 * produce identical bytes. With that, captures ARE deterministic, and the
 * tolerance can stay near zero instead of being raised until bad frames pass.
 */

import { readFileSync } from "node:fs";
import path from "node:path";

export const repoRoot = path.resolve(import.meta.dir, "../..");

export type Axis = {
  id: string;
  theme: string;
  density: string;
  controlSize: string;
};

/**
 * The default axis. One is enough for a baseline gate — unlike the web gate,
 * which sweeps axes to catch Svelte/React divergence, here a second axis costs
 * a full capture run and catches only what the first already would.
 */
export const DEFAULT_AXIS: Axis = {
  id: "eclipse-compact-sm",
  theme: "eclipse",
  density: "compact",
  controlSize: "sm",
};

export const GPUI_PREVIEW_CWD = "packages/gpui/preview";
export const BASELINE_DIR = "packages/gpui/preview/baselines";
export const OUT_DIR = "test/native-visual/out";

/**
 * Seconds to wait for the window to render and capture itself.
 *
 * Most components capture in about five. `order-by` needed roughly 55, and at a
 * 40s limit it failed three runs in a row while working fine by hand — the
 * limit was the bug, not the component.
 */
export const CAPTURE_TIMEOUT_S = 90;

/**
 * Fraction of differing pixels tolerated.
 *
 * Sits between two measured numbers rather than being picked by feel:
 *
 *   - the antialiasing floor is 14 pixels (0.0002%), the worst case across a
 *     full sweep of components that had not changed
 *   - a real single-component change — 3px added to the button height — moved
 *     1.72% of pixels
 *
 * Four orders of magnitude apart, so 0.002% is an order of magnitude above the
 * noise and three below anything worth catching.
 */
export const MAX_DIFF_RATIO = 0.00002;

/**
 * Components skipped, with the reason. Anything inherently non-deterministic
 * (animation, media surfaces, wall-clock copy) belongs here rather than being
 * quietly tolerated by a raised threshold.
 */
export const SKIPPED: Record<string, string> = {
  spinner: "continuous rotation — the frame captured depends on render timing",
  "page-loading": "indeterminate progress animation",
  "time-ago": "renders relative wall-clock copy",
  "audio-player": "media surface; playhead position is timing-dependent",
  "video-player": "media surface; frame decode is timing-dependent",
  // Measured, not assumed: three capture pairs in a row disagreed, so the bar
  // is genuinely still moving. The others above are reasoned; this one was
  // caught by the two-agreeing-captures rule.
  progress: "indeterminate bar — no two consecutive captures agree",
};

/** Every component slug the GPUI preview can route to. */
export function gpuiSlugs(): string[] {
  const src = readFileSync(
    path.join(repoRoot, "packages/gpui/preview/src/component_registry.rs"),
    "utf8",
  );
  const slugs = [...src.matchAll(/slug: "([a-z0-9-]+)"/g)].map((m) => m[1]);
  return [...new Set(slugs)].sort();
}

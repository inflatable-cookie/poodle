/**
 * Committed parity-debt inventory for the sweep-tier visual report.
 *
 * `effigy visual:report` diffs every specimen slug against its React pair and
 * classifies each failing pair against this file: a pair recorded here is
 * known debt and reports as such; any other failing pair is a new regression
 * and fails the report, and any recorded entry that now passes fails the
 * report as stale debt. The strict tiers (`test:visual*`) do not read this
 * file — a gate failure is always a failure.
 *
 * Every entry names its failure class and a written reason. An entry without
 * a reason is suppression, not bookkeeping — and the end state is an empty
 * file: entries leave when the pair is fixed, not by widening tolerances.
 * Axes are listed explicitly, so an axis added to the sweep fails fresh
 * instead of inheriting debt.
 *
 * Measured 2026-11-28 (after poodle#212 aligned the specimen layouts;
 * sweep tier, 344 comparisons across 172 active slugs): 21 recorded pairs
 * across 12 slugs — 0 size, 21 pixels (including the 4 intermittent flipper
 * pairs for `color-picker` and `text-input`, plus `audio-switch` and
 * `validation-summary` pixel diffs previously masked by their size debt,
 * and `icon-provider` on iceberg), 0 capture. `avatar` and `pill` went
 * green with the SceneSpecimen row-gap alignment.
 */

export type VisualDebtFailureKind = "capture" | "size" | "pixels";

export type VisualDebtEntry = {
  /** Specimen slug (the `#components/<slug>` route both previews serve). */
  slug: string;
  /** Axis ids this entry covers, e.g. `eclipse-compact-md`. */
  axes: readonly string[];
  kind: VisualDebtFailureKind;
  /** True only for non-deterministic flippers; never flagged as stale when passing. */
  intermittent?: boolean;
  reason: string;
};

export const DEBT: VisualDebtEntry[] = [
  {
    slug: "audio-meter",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.37-0.43% of pixels differ on both themes — segment/LED meter rasterization drift; untriaged.",
  },
  {
    slug: "audio-switch",
    axes: ["eclipse-compact-md"],
    kind: "pixels",
    reason:
      "0.057% on eclipse only (previously masked by size debt) — illuminated lamp box-shadow uses currentColor on <button>, differing via color-scheme: dark on React .app-shell vs Svelte .poodle-app-shell; the iceberg axis matches; untriaged.",
  },
  {
    slug: "code-editor",
    axes: ["eclipse-compact-md"],
    kind: "pixels",
    reason:
      "0.41% on eclipse only — CodeMirror line/glyph rasterization; the iceberg axis matches; untriaged.",
  },
  {
    slug: "color-picker",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    intermittent: true,
    reason:
      "0.081-0.092% — intermittent 1px shift on the Default open, RGB mode anchored popover (0.081% on eclipse-compact-md, 0.092% on iceberg-compact-md; passes 0% on clean runs); both sweep axes are recorded; untriaged.",
  },
  {
    slug: "dock-region",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "1.13-1.23% — dock stack/splitter geometry drift; untriaged.",
  },
  {
    slug: "fader",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.115% — fader track/handle antialiasing; untriaged.",
  },
  {
    slug: "gain-reduction-meter",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.05-0.10% of pixels differ — just above the 0.02% default floor, glyph/canvas-edge antialiasing scale; untriaged.",
  },
  {
    slug: "icon",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.024% in Eclipse and 0.025% in Iceberg. Both framework captures and their diff match the former opt-in style at fc9b7ad24 byte for byte.",
  },
  {
    slug: "icon-provider",
    axes: ["iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.023% on iceberg only — just above the 0.02% default floor, hint-line glyph antialiasing drift; the eclipse axis matches; untriaged.",
  },
  {
    slug: "model-connection-setup",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.03% of pixels differ — just above the 0.02% default floor, antialiasing scale; untriaged.",
  },
  {
    slug: "text-input",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    intermittent: true,
    reason:
      "0.068% — flips between 0% and 0.068% across runs and themes at the third TextInput row; both sweep axes are recorded; untriaged.",
  },
  {
    slug: "validation-summary",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.269-0.287% of pixels differ on both themes (previously masked by size debt) — component template whitespace between ValidationSummary <a>/<span> and Field label/* in Svelte vs JSX; untriaged.",
  },
];

/**
 * Committed parity-debt inventory for the sweep-tier visual report.
 *
 * `effigy visual:report` diffs every specimen slug against its React pair and
 * classifies each failing pair against this file: a pair recorded here is
 * known debt and reports as such; any other failing pair is a new regression
 * and fails the report. The strict tiers (`test:visual*`) do not read this
 * file — a gate failure is always a failure.
 *
 * Every entry names its failure class and a written reason. An entry without
 * a reason is suppression, not bookkeeping — and the end state is an empty
 * file: entries leave when the pair is fixed, not by widening tolerances.
 * Axes are listed explicitly, so an axis added to the sweep fails fresh
 * instead of inheriting debt.
 *
 * Measured 2026-09-27 on the web-gates branch (current main plus type-only
 * React fixes; sweep tier, 344 comparisons): 34 failing pairs across 19
 * slugs — 16 size, 18 pixels, 0 capture. The August figure (53 pairs over
 * 308 comparisons) predates the current specimen set.
 */

export type VisualDebtFailureKind = "capture" | "size" | "pixels";

export type VisualDebtEntry = {
  /** Specimen slug (the `#components/<slug>` route both previews serve). */
  slug: string;
  /** Axis ids this entry covers, e.g. `eclipse-compact-md`. */
  axes: readonly string[];
  kind: VisualDebtFailureKind;
  reason: string;
};

export const DEBT: VisualDebtEntry[] = [
  {
    slug: "audio-meter",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.37-0.42% of pixels differ on both themes — segment/LED meter rasterization drift; untriaged.",
  },
  {
    slug: "audio-switch",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 40px taller (759 vs 799) — switch-row layout divergence; untriaged.",
  },
  {
    slug: "avatar",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "1.26% — initials glyphs sit at different offsets inside the circle between shells (diff reviewed; image avatars match); untriaged.",
  },
  {
    slug: "callout",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 11px taller at the same width (1103 vs 1114) — vertical text/spacing divergence; untriaged.",
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
    axes: ["iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.16% on iceberg only — swatch/track rendering under the low-contrast theme; eclipse matches; untriaged.",
  },
  {
    slug: "dock-region",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.94-1.23% — dock stack/splitter geometry drift; untriaged.",
  },
  {
    slug: "drag-number-field",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 56px taller (924 vs 980) — control layout divergence; untriaged.",
  },
  {
    slug: "embed-preview",
    axes: ["eclipse-compact-md"],
    kind: "pixels",
    reason:
      "14.6% — the remote Vimeo iframe paints a different decoded video frame per capture (diff reviewed; player chrome matches). Remote-media nondeterminism, the sweep-comparable cousin of the skipped media specimens; untriaged.",
  },
  {
    slug: "envelope-editor",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 20px shorter (1872 vs 1852) — editor surface height divergence; untriaged.",
  },
  {
    slug: "fader",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.12-0.15% — fader track/handle antialiasing; untriaged.",
  },
  {
    slug: "gain-reduction-meter",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.05% of pixels differ — just above the 0.02% default floor, glyph/canvas-edge antialiasing scale; untriaged.",
  },
  {
    slug: "markdown-renderer",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 26px taller (1140 vs 1166) — markdown typography/spacing divergence; untriaged.",
  },
  {
    slug: "model-connection-setup",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.03% of pixels differ — just above the 0.02% default floor, antialiasing scale; untriaged.",
  },
  {
    slug: "pill",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.52-0.67% — pill geometry/typography drift; untriaged.",
  },
  {
    slug: "remediation-banner",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 32px taller (780 vs 812) — banner layout divergence; untriaged.",
  },
  {
    slug: "text-input",
    axes: ["iceberg-compact-md"],
    kind: "pixels",
    reason:
      "0.07% on iceberg only — glyph rasterization under the iceberg theme; the eclipse axes match; untriaged.",
  },
  {
    slug: "validation-summary",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 16px taller (553 vs 569) — summary list spacing divergence; untriaged.",
  },
  {
    slug: "value-readout",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React renders the specimen 64px taller (1031 vs 1095) — the sweep's largest size delta; untriaged.",
  },
];

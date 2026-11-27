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
 * Measured 2026-11-26 on main (after the poodle#189 default-style switch;
 * sweep tier, 344 comparisons across 172 active slugs): 39 recorded pairs
 * across 20 slugs — 18 size across 9 slugs, 21 pixels across 11 slugs
 * (including the 4 intermittent flipper pairs for `color-picker` and
 * `text-input`), 0 capture. `embed-preview` moved to `SKIPPED` (remote Vimeo
 * iframe nondeterminism).
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
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen uses inline gap 1.5rem vs Svelte .page gap 1rem across 5 gaps (+40px, 759 vs 799).",
  },
  {
    slug: "avatar",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "pixels",
    reason:
      "1.25% — initials glyphs sit at different offsets inside the circle between shells (diff reviewed; image avatars match); untriaged.",
  },
  {
    slug: "callout",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React SceneSpecimen passes String(undefined) to Callout when content is absent (+27.5px), offset by 0.5rem vs 0.75rem row gap (-16px; net +11px, 1103 vs 1114).",
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
    slug: "drag-number-field",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen uses inline gap 1.5rem vs Svelte .page gap 1rem across 7 gaps (+56px, 924 vs 980).",
  },
  {
    slug: "envelope-editor",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen puts the drag note in a horizontal Row (+7px) and omits the keyboard-nudges <p> note (-27px; net -20px, 1872 vs 1852).",
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
    slug: "listbox",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen lacks Svelte's scoped .poodle-listbox-card-detail rule (margin-top 0.25rem, font-size 0.8125rem; -2px across 3 cards = -6px, 692 vs 686).",
  },
  {
    slug: "markdown-renderer",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen lacks Svelte's scoped .markdown-renderer-policy-warning rule (margin 0 0 0.5rem, 0.8125rem; falls back to 1em p margins and 16px font = +26px, 1140 vs 1166).",
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
      "0.50-0.59% — pill geometry/typography drift; untriaged.",
  },
  {
    slug: "remediation-banner",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "Svelte specimen omits .poodle-specimen flex gap 1rem that React's gallery.css applies across 2 group gaps (+32px, 780 vs 812).",
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
    kind: "size",
    reason:
      "Svelte specimen omits .poodle-specimen flex gap 1rem that React's gallery.css applies across 1 group gap (+16px, 553 vs 569).",
  },
  {
    slug: "value-readout",
    axes: ["eclipse-compact-md", "iceberg-compact-md"],
    kind: "size",
    reason:
      "React specimen uses inline gap 1.5rem vs Svelte .page gap 1rem across 8 gaps (+64px, 1031 vs 1095).",
  },
];

# g10.009 GPUI Token Resolution And Visual Fidelity

Status: complete
Owner: Poodle core
Depends on: g10.008
Updated: 2026-04-13

## Context

GPUI uses `resolve_px` / `resolve_color` in many components but still carries
**literal `px(...)`** for gaps, type sizes, calendar cells, dialog widths, and
miscellaneous layout. That diverges from contract token tables and from Svelte
(CSS custom properties). **Region** documents a GPUI-native border limitation
and previously hardcoded label `text_size`.

Audit hotspots (non-exhaustive): `button.rs` (gap, icon inset), `region.rs`
(label size), `pagination.rs` (stub input box), `calendar.rs`, `field.rs`,
`alert_dialog.rs`, `dock_region.rs`, `split_view.rs`, several composites
(filter toolbar, media preview, selection summary, audio player, etc.).

## Governing Refs

- `CLAUDE.md` (token resolution mandatory for implementations)
- `docs/contracts/components/region.md`
- `docs/contracts/components/button.md`
- `poodle-tokens` semantic definitions
- `docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md` (token literal backlog; dashed border narrative corrected)

## Goals

- eliminate **avoidable** hardcoded pixel typography and spacing where the
  contract names a semantic token
- resolve **Region** label size via the same token path as Svelte
  (`typography.label.size` / spec token method), not `px(11.0)`
- reconcile **Region** label size **comment** vs Svelte resolved value if token
  ≠ fixed 11px
- keep **documented** GPUI deltas honest (e.g. dashed border → solid) until
  GPUI can render contract-correct strokes

## Non-Goals

- rewriting Jetstream components (separate lane)
- forcing every `px(0)` flex hack through tokens without a rule
- eliminating every composite shadow preset or skeleton demo width in one pass

## Execution Plan

### Batch 9.1 — High-traffic primitives

- [x] Button: gap and icon inset from `space.button.gap` and
      `space.button.iconInset` via `ButtonSpec` token helpers
- [x] Region: label typography from spec (`typography.label.size`); default
      label color uses `color.text.tertiary`
- [x] Field: label row gap, supporting text size from spec token methods;
      supporting text uses `typography.counter.size` for the md message row

### Batch 9.2 — Sweep

- [x] Classify remaining `px(` under `packages/gpui/components/src` (calendar,
      composites, shadows, skeleton specimens, etc.) and register the umbrella
      accepted delta as residual literals — see **`g10.012`** token sweep
- [x] Register **region** dashed-vs-solid border as **D-106**

## Validation

- `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
- spot-check GPUI preview specimens for regressions
- `git diff --check`

## Outcome

- **Tokens**: `color.text.tertiary`, `space.button.gap`, `space.button.iconInset`
  added in schema; `effigy tokens:build` regenerates Rust constants.
- **Specs**: `RegionSpec` default label color and `label_text_size_token()`;
  `ButtonSpec::content_gap_token` / `icon_side_inset_token`; `FieldSpec`
  `label_row_gap_token`, `supporting_text_typography_token` → counter size for
  helper/error/optional rows.
- **GPUI**: `button`, `region`, `field` resolve the above via `resolve_px` /
  `resolve_color`; Jetstream and GPUI theme adapters resolve the new paths.
- **Preview**: GPUI token table includes tertiary text and button spacing paths.
- **Follow-up**: D-107 lists surfaces still on literal `px` for a later
  contract-or-schema-driven pass (or `g10.010` scope where behavior overlaps).

# Jetstream notice — component sweep findings for the Poodle thread

Date: 2026-07-26
From: Jetstream thread (g06.026 follow-on)
Full record: `../jetstream/docs/logs/2026-07/26-poodle-component-sweep-findings.md`

Jetstream's editor was rehosted onto Poodle and egui removed, which made the
component library the engine's first real UI consumer. Sweeping all 132
specimens (`packages/jetstream/preview`, `cargo run --bin snap -- specimens`)
turned up six **engine** defects, now fixed in Jetstream, and three things that
belong to this repo.

Nothing here was changed in Poodle: this tree had 12 modified files when the
sweep ran, and all three are component-level anyway.

## Engine fixes that change how your components render

Re-render the specimens after picking up Jetstream HEAD; several will look
different, all in the right direction.

- Bounded text measurement was one ascent too tall (`run.line_y` is the
  baseline, not the line top). This inflated `min-height: auto` on any
  container wrapping text.
- Text wrapped at exactly its own measured width (fractional advance vs the
  renderer's 4px pad).
- **`line_height` was being dropped entirely** — 11 sites hardcoded
  `font_size * 1.2`, so `TextSpec`'s 1.5 never applied. It applies now, so text
  blocks get taller and looser. This is the change most likely to shift your
  specimen layouts.
- Missing icons drew nothing; they now draw a visible placeholder.
- Border-only shapes (border, no background) painted solid — ring spinners were
  filled discs.
- Per-side border colours on rounded shapes were straight strips; they now
  follow the arc, split at the corner diagonals as CSS does.

## 1. `data-table` columns do not align between header and rows

Looks like the body is centred. It is not — each row sizes its own columns.

Measured first-ink x per row in `data-table.png`:

| column | per-row x |
|---|---|
| Name  | 98, 98, 99, 98, 99 — aligned (first column) |
| Email | 290, 272, 285, 276, 273, 290 — drifts up to 18px |

**Cause.** Header cells wrap a label in a growing `div` (label keeps intrinsic
width, sits left). Body cells call `.grow()` on the *label itself*, and
`.grow()` leaves `flex-basis: auto` — so each column's base size is that row's
content and no two rows agree. Name aligns only because it is first.

**Fix.** `flex_basis(0)` on the column cells (`flex: 1 1 0`), or move the table
to a grid. `jetstream_ui::ui_element` already exposes `flex_basis`,
`flex_basis_0`, and Taffy grid — no engine change needed.

Sites: `packages/jetstream/components/src/data_table.rs`, the `.grow()` calls
near lines 136 (header) and 278/296 (body).

## 2. `select` open state

Open-state trigger renders narrower than the closed variants (158px vs 264px)
and the dropdown draws squashed against the left window edge. `ff31bedf`
("Implement anchored overlays") may already cover this — worth re-rendering
before spending on it.

## 3. Icon set is missing 23 referenced names

`packages/gpui/preview/assets/icons` has 51 SVGs. Scanning names referenced by
`packages/jetstream/components` against it found 23 with no file, including
`triangle-alert` — which is why `callout`'s Warning tone rendered an empty
badge. They now rasterise to a visible placeholder rather than nothing, so a
specimen render will show them.

The placeholder is deliberately ugly (boxed diagonal): a missing icon should
fail visibly rather than leave a correctly-sized hole.

# Lab Button Run — Unclassified Findings And Density Observability

Status: open — two unclassified findings await a bounded Poodle diagnosis;
one evidence-integrity observation awaits a lab fixture check
Captured: 2026-09-04
Owner: Chatterbox (planning)
Source: poodle-lab `g01.001` closed Button batch at exact head `fb839407b`
(54 captures, 36 pairs, two independent runs agree). Svelte↔React 18/18
exact; Svelte↔GPUI 2/18 with 16 findings already contracted as
`gpui-omits-box-shadow`. No acceptance claim is made here.

## Finding 1 — 1.0 vs 0.5 logical px edge delta (two fixtures)

- Fixtures: `content-leading-icon` and `state-loading`. Both are the
  leading-slot path: `poodle-render/src/button.rs:123-128` computes
  `icon_inset = rem_to_px(size_icon_inset_rem(size))` and treats loading as
  a leading slot (`has_leading = leading_icon || is_loading`).
- Known: the web expects a half-pixel edge (the inset resolves to 0.5
  logical px at this size); GPUI renders 1.0; deterministic across runs.
- Unknown: whether `rem_to_px` rounds before layout, whether the node
  vocabulary carries fractional insets, or whether GPUI snaps the paint edge
  to whole logical pixels. The first two are Poodle-owned; the third is a
  renderer delta to contract like the shadow omission.
- Route: one bounded diagnosis card in Poodle — reproduce the two fixtures
  through the node inventory, compare the emitted inset to the CSS value,
  and either fix the rounding in `poodle-render` or record a contracted
  known delta `gpui-snaps-subpixel-edge` with its rationale. Do not tune the
  comparator tolerance.

## Finding 2 — density values are not observable at this Button size

- `rest-secondary`, `density-compact`, and `density-comfortable` produce
  byte-identical images: 16 distinct images for 18 identities.
- Known: `packages/core/src/styles/button.css:338-344` defines
  `data-density="compact"` and `"comfortable"` rules, so density is meant
  to change Button geometry. The working rules say every advertised axis
  value must render real evidence in every active runtime.
- Unknown: whether the lab fixtures set `data-density` (a fixture defect),
  whether the density rules resolve to the same padding at the captured
  size (a token defect), or whether the runtimes differ (the identical set
  spans which runtimes is not yet stated in the receipt).
- Route: lab first — the `g01.001` worker states, per runtime, whether the
  three fixtures differ in DOM/node attributes; if the fixtures are correct
  and images still match, Poodle owns a token/contract question for Button
  density. Until then the density rows are not evidence.

## Next check

Both routes return to Chatterbox with evidence before any card. Remove this
note when Finding 1 is a card or a contracted delta and Finding 2 is
resolved on the lab or Poodle side.

# g16.046 — Block Slider And RangeSlider Appearance

Status: complete — merged in PR #154 after exact-head repair
Type: implementation
Opened: 2026-09-01
Closed: 2026-09-02
Merge: `40d251c9b172cea5f043aaa19d220586b1919177`
Proof head: `95a5dbd36d6e5e80b6647aaf7593b44a6a7abed8`
Repair head: `c00a3c73dbd70dc485afc9250ce1e7f616352550`
Height repair: `354a94c081d74712ba5d33917396d4f14e40f3ae`
Ownership repair: `d48a2be5baab81e50ca5c9bc630d266cee6a6dfe`
Repair: parent-owned native fit, real 44×44 web hits, React teardown refs,
production GPUI fallback height, construction ownership, accessible-name
channels (see log)
Log: `../../logs/2026-09/20260902-g16-046-block-sliders.md`
Depends on: merged `g16.034` and operator acceptance recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../contracts/components/size-and-density.md`,
`../../architecture/007-appearance-recipe-contract.md`,
`../../architecture/012-semantic-motion-policy.md`

## Goal

Add one opt-in `appearance="block"` treatment to Slider and RangeSlider across
the active cohort. Preserve current track/embedded defaults and value machines.
Prove visible content, fit fallback, direction, stable thumb ownership,
idempotent terminal delivery, forced colors, and effective targets without
changing paging or admitting vertical RangeSlider early.

## Fixed Public Shape

- Shared `SliderAppearance = "track" | "block"`; `appearance` defaults to
  `track` and stays orthogonal to `variant`.
- Slider adds `visibleLabel` and `formatVisibleValue`. RangeSlider adds the
  same label, per-thumb `formatVisibleValue`, and `formatVisibleRange`.
  Native specs carry resolved strings, not closures.
- `direction: "ltr" | "rtl"` defaults to `ltr`. Horizontal geometry mirrors
  in RTL; arrow keys retain numeric meaning.
- Inline text is all-or-nothing. Every item must fit its assigned region under
  the accepted floor/ceil measurement law or one stable, noninteractive,
  accessibility-hidden line renders after the capsule.
- Exact RangeSlider pointer ties choose lower. The selected thumb remains the
  gesture owner, clamps at its sibling, and never swaps identity.
- Release, cancellation, lost capture, disablement, and teardown share one
  idempotent terminal. The first terminal emits one commit with the latest
  accepted value; cancellation does not roll back.
- Every Slider control and RangeSlider thumb has a measurable 44×44 logical-
  pixel effective target. Forced-color roles follow the accepted role table.
- PageUp/PageDown behavior stays unchanged in this card. Vertical active-
  cohort support stays gated on real native RangeSlider axis geometry.
- Invalid, read-only, indeterminate, tooltip-only, and public fit metrics stay
  outside the component surface. Jetstream remains deferred.

Horizontal-only is a public validity law for both components:

- `appearance="track"` keeps today's horizontal and vertical support.
- `appearance="block"` accepts omitted `orientation` or
  `orientation="horizontal"` only.
- `appearance="block"` with `orientation="vertical"` is invalid in Svelte,
  React, shared Rust composition, and GPUI. Adapters must reject it before
  component paint/construction. They must not coerce orientation, silently
  render track appearance, or split behavior by runtime.
- Vertical block admission requires a later all-runtime contract migration
  after real native RangeSlider axis geometry and mounted evidence exist.

## Ordered Work

1. Amend Slider/RangeSlider and appearance-recipe authority. Add shared types,
   resolved native fields, fit/formatter laws, direction, terminal semantics,
   and the horizontal-only admission boundary.
2. Extend the current normalized machines and terminal effects. Do not fork
   value math for block paint.
3. Implement Svelte/React shared styling and fit measurement with stable
   fallback. Implement shared Rust composition and GPUI horizontal geometry,
   per-thumb metadata, hit bounds, and paint roles.
4. Add paired behavior, boundary-fit, forced-color, package-surface, browser,
   Rust-node, and mounted GPUI evidence. Preserve the track appearance byte-
   for-byte in observable API and callback traces.
5. Close with one execution log. Do not move visual ledger cells from this
   implementation proof.

## Acceptance

- Omitting `appearance` leaves current anatomy, values, callbacks, and track /
  embedded visuals unchanged.
- Formatter inputs are normalized, bounds-guarded, step-snapped values.
  Visible and accessible text channels never feed each other.
- Required-minus-one falls back; equality fits; required-plus-one fits. The
  external readout remains stable across focus, overlap, and thumb movement.
- Horizontal LTR/RTL geometry and numeric keyboard behavior agree in every
  active runtime.
- Both components reject vertical block input before paint/construction while
  vertical track input remains unchanged.
- Equal-value RangeSlider keeps two focusable semantic thumbs. Pointer tie
  selects lower; upper remains directly keyboard operable.
- Every terminal sequence emits at most one commit and preserves live-change
  ordering through callback-triggered teardown.
- Web hit probes, Rust bounds, and mounted GPUI dispatch prove 44×44 targets
  from `xs` through `xl` and every density.
- Native dependent bounds and names/value text are per-thumb. GPUI metadata is
  not described as mounted assistive-technology proof.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Default is inert | existing Slider with no new props | before/after anatomy and callback trace match |
| Fit is atomic | label fits, value misses by one pixel | no inline text; one stable fallback line |
| Channels stay separate | only `ariaLabel="Gain"` is supplied | no visible `Gain` text |
| Tie and gesture identity are stable | `[50,50]`, upper previously focused | pointer chooses lower; drag never swaps |
| Terminal is idempotent | cancel then lost-capture then teardown | one commit, no rollback, later terminals inert |
| Targets are real | `xs` equal-value range | both 44×44 bounds exist; lower pointer tie and upper keyboard work |
| Forced colors preserve roles | selected and remainder both resolve to canvas | role/contrast gate fails |
| Vertical stays gated | either component receives `appearance="block"` with `orientation="vertical"` | every runtime rejects before paint/construction; no coercion or track fallback |

Plant and restore the pre-fix behavior for every row after the real proof is
committed.

## Writable Scope

Slider/RangeSlider contracts; appearance recipe only for the shared role map;
component-local core, Svelte, React, Rust spec/render, GPUI, fixtures, tests,
styles, and public types required by the two controls; this card, one log, and
new papercuts. Do not edit unrelated controls, paging semantics, releases,
workflows, downstream consumers, visual ledger cells, native AT claims, or
Jetstream behavior.

## Validation

Run focused machine/terminal, web shell, package surface, fit/geometry,
forced-color, Rust node, and mounted GPUI checks; relevant drift selectors;
`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`. Never run
`*-windowed` or native-visual selectors.

## Stop Conditions

Stop if the implementation needs public fit thresholds, thumb swapping,
rollback state, new paging semantics, a generic tooltip/readout system,
vertical admission without native axis proof, a new motion role, or a native
AT claim beyond contract 003. A runtime-specific fallback or coercion for
vertical block input is also a stop.

## Continuation

After accepted merge, any PageUp/PageDown convergence is a separate all-
appearance decision. It is not implied by this card.

## Closeout

Implementation landed on `feature/g16-046-block-sliders` as PR
https://github.com/inflatable-cookie/poodle/pull/154. Public names, fit
arithmetic, tie/clamp, terminal, 44×44 hits, forced-color roles, and
horizontal-only rejection match this card. Visual ledger cells were not moved.
Jetstream remains deferred. This card does not change Jetstream preview
behavior.

Orchestrator review on that PR required three repairs, landed on the same
branch without a new worktree:

1. Native block fit uses parent-owned allocated width plus GPUI `shape_line`
   advance. Construction stays `Spec + RenderContext -> Node` in the GPUI
   composition layer. The node backend measures width, shapes named strings,
   and interprets the rebuilt Node. Paint panics without
   `RenderContext::with_block_layout`. `with_block_layout_width` is test-compat
   only and still uses the character heuristic.
2. Web 44×44 hits take pointer events, overflow the 28px `xs` capsule, and
   keep visual thumb/capsule size. Chromium and WebKit prove geometry plus
   pointer dispatch. A CSS token match is not that proof.
3. React Slider and RangeSlider teardown reads the live `onValueCommit` and
   controlledness refs. Adversarial A→B rerender then unmount covers both.

A later exact-head finding on `ed816f3ea` (replayed here as `c00a3c73d`):
production GPUI hosts used `block_*_min_height` as a fixed height, so a
fallback line after the 44px surface could paint outside the reserved box
and overlap the next sibling. Hosts now `request_measured_layout`: inline
reserves the surface; fallback reserves surface plus the GPUI line. Range
fallback is nowrap, matching Slider. Mounted production-host proofs use
production min-height plus a following sibling at 80px and 400px for both
controls. Not a fixed 80px production height.

Exact-head review of `68bdb09f0` requested three more in-bounds repairs:
node-backend no longer depends on `poodle-render` / `poodle-specs` /
`poodle-adapter` or constructs Slider/RangeSlider; Jetstream preview diffs
are gone; Slider §3/§6 treat block `visibleLabel` as not the accessible name,
with a focused proof that `visibleLabel` alone never becomes it. RangeSlider
keeps explicit/default per-thumb names.

Oracle plant-and-restore ran against pre-rebase proof
`eedb4a38ae4ac010aeb86e998b113b7a1d8d0a2c` (`8fd0a885a03147451affc937447f61f0cfeba4af`
on this branch after rebase onto `a52d0d32b`). Every row failed under the
planted pre-fix and passed after restore. The review repair added two biting
plants the original table missed: CSS `--block-hit: 44px` is not a hit-testable
target, and core `POINTER_END` is not React adapter teardown. Rebased onto
merged g16.045 at `1b0d40329`; first repair replayed as `c00a3c73d`; production
host height landed as `354a94c08`. Construction ownership, Jetstream revert,
and accessible-name prose landed as `d48a2be5b`. See the execution log.

# g16.005 — Slider Axis, Keyboard, And Mounted Parity

Date: 2026-08-26
Status: complete — awaiting operator merge
Branch: `t3code/slider-axis-keyboard-parity`
Card: `docs/roadmaps/g16/005-slider-axis-keyboard-and-mounted-parity.md`

## Outcome

Slider has one observable value-control contract across Svelte, React, shared
Rust, and GPUI. Native pointer input follows the rendered axis. Change arrives
during a captured scrub; commit arrives exactly once on release. Keyboard
arrows, Home, and End run through `slider_transition` INPUT then COMMIT. The
focusable node carries slider role, label, value, safe bounds, value text,
orientation, disabled state, and contracted focus treatment.

The generated ledger moves only Slider's GPUI mounted-behaviour cell from
`missing` to `mounted`. Summary: 34 → 35 mounted; 140 → 139 missing. GPUI
accessibility stays `manual`. Visual comparison stays `missing`. Jetstream
stays deferred.

## Shared machines and web

- `packages/core/test/wave1.test.ts` and
  `packages/contracts/headless/src/slider.rs` — press/move/end sequencing,
  `SET_VALUE` rebuild without effects, disabled pointer inertia
- `packages/svelte/components/src/Slider.svelte` and
  `packages/react/components/src/Slider.tsx` — embedded keyboard is inert
  while disabled; public props unchanged
- focused Svelte and React tests for embedded horizontal and vertical pointer
  normalization, all arrows plus Home/End, change-then-commit, ARIA fields,
  and disabled inertia

## Node, backend, and RangeSlider

- `packages/contracts/node/src/lib.rs` — `ScrubAxis` beside `on_scrub`;
  `NodeA11y.value_text`; `NodeStyle.height_pct`
- `packages/gpui/node-backend/` — axis-aware scrub fraction (horizontal
  left→right, vertical bottom→top); `height_pct` maps to `relative` height
- `packages/render/src/range_slider.rs` — every `on_scrub` node sets its axis
  from `RangeSliderSpec::orientation`; mounted scrub regression retained

## Shared Rust Slider

- `packages/render/src/slider.rs` — `SliderHandlers.on_change` /
  `on_value_commit` with no aliases; scrub installs when either callback is
  present; every `SliderEffect` is forwarded; delta/fixed-width fallback
  removed; keyboard on the one focusable node; horizontal and vertical
  geometry; disabled installs no scrub or key handler
- `packages/gpui/preview/src/node_compat.rs` — mechanical field rename
- Jetstream `js_slider` stays on `SliderHandlers::default()`; compile-only

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `slider_axis_keyboard_and_disabled_rebuild_the_host_spec` — horizontal
  press/drag/release with host rebuild after commit; Arrow/Home/End
  change-then-commit; vertical bottom-to-top scrub; disabled inertia and
  node-level intent
- `a_scrub_reports_change_while_dragging_and_commits_once_at_release`
  retained for RangeSlider; its ledger cell does not move

## Remaining gaps

- GPUI accessibility remains `manual`. Node-level slider intent is not broad
  native assistive-technology proof.
- Slider visual comparison remains Button-only / missing on GPUI.
- Page-key amount remains browser-owned and outside strict parity.
- Jetstream preview was not compiled in this worktree. Call-site compatibility
  is mechanical and compile-only.

## Validation

Passed on this revision, entirely headless:

- focused core Slider tests (`wave1`)
- focused Svelte and React Slider tests (20)
- focused `poodle-node`, `poodle-headless`, `poodle-render` Slider/RangeSlider
  tests
- focused `poodle-gpui-node-backend` scrub-axis tests
- `slider_axis_keyboard_and_disabled_rebuild_the_host_spec`
- `a_scrub_reports_change_while_dragging_and_commits_once_at_release`
- `effigy test:parity-evidence-ledger` / `effigy check:parity-evidence-ledger`
  after `--write` (35 mounted / 139 missing)
- `effigy regressions:native` — 76
- `effigy probe:gpui-specimens` — 8
- `effigy ci:native`
- `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check`

No `*-windowed`, native visual, Jetstream preview/QA, release, tag, or
publication selectors. `effigy doctor` stayed red on the known planning-base
generated-in-src, oversized-file, and stale/broad suppression scans; that debt
was not absorbed.

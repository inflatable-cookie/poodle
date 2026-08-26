# g16.004 — ToggleGroup Semantic API And Mounted Parity

Date: 2026-08-26
Status: complete — merged in PR #78 (`06bdb984`)
Branch: `t3code/toggle-group-semantic-api`
Card: `docs/roadmaps/g16/004-toggle-group-semantic-api-and-mounted-parity.md`

## Outcome

ToggleGroup reports the resulting typed selection in every active runtime.
Single mode is a horizontal radiogroup with one selected-or-first-enabled tab
stop and Left/Right wrap that skips disabled options. Multiple mode is ordinary
pressed buttons with no arrow intercept. Same-value activation in
non-deactivating single still emits. Native construction requires
`ToggleGroupHandlers::new(instance_id)`; semantic ids stay `toggle:{value}`
and backend runtime ids are `toggle:{instance_id}:option:{value}`.

The generated ledger moves only ToggleGroup's GPUI mounted-behaviour cell from
`missing` to `mounted`. Summary: 33 → 34 mounted; 141 → 140 missing. GPUI
accessibility stays `manual`. Jetstream stays deferred.

## Web alignment

- `packages/core/src/toggle-group.ts` — `toggleGroupEnabledValues`,
  `toggleGroupTabStopValue`, `toggleGroupArrowTarget`; selection still goes
  through `toggleGroupTransition`
- `packages/svelte/components/src/ToggleGroup.svelte` and
  `packages/react/components/src/ToggleGroup.tsx` — instance-local
  `data-toggle-value` lookup; one single-mode tab stop; multiple omits
  tabindex; public props unchanged

## Handler and call sites

- `packages/render/src/toggle_group.rs` — `ToggleGroupHandlers`; spec →
  `ToggleGroupValue` once per render; every pointer and single-mode arrow
  through `toggle_group_transition`; contracted `FocusRing` on enabled items
- `packages/render/src/lib.rs` — export
- `packages/gpui/preview/src/specimens/toggle_group.rs` — typed result stored
  via existing SetText; specimen membership helper deleted; static scopes
  (`toggle-group-disabled`, `toggle-group-disabled-item`, size/density ids);
  interactive keys (`toggle-group-single`, `toggle-group-four`,
  `toggle-group-multiple`)
- `packages/jetstream/preview/src/compat.rs`, specimen, and `shell.rs` —
  compile-only scopes; no backend-behavior change

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `toggle_group_result_focus_identity_and_disabled_paths` — pointer result
  with host rebuild; same-value emission; disabled-option skip; Left/Right
  wrap; `allow_deactivation` → `Single(None)`; multiple add/remove; multiple
  arrows inert; disabled-group inertia; two mounted groups with identical
  option values and independent focus handles (`toggle:left:option:grid` vs
  `toggle:right:option:grid`)

## Remaining gaps

- GPUI accessibility remains `manual`.
- ToggleGroup visual comparison remains Button-only / missing on GPUI.
- Jetstream preview was not compiled in this worktree: the crate's
  `jetstream-input` path is absent here. The call-site change is mechanical
  and compile-only.

## Validation

Passed on this revision, entirely headless:

- focused core ToggleGroup tests
- shared TS/Rust machine conformance (wave1 + conformance; 66)
- focused Svelte and React ToggleGroup tests (18)
- focused `poodle-render` ToggleGroup tests (filter hits CardToggleGroup too;
  19 including those)
- `toggle_group_result_focus_identity_and_disabled_paths`
- `effigy regressions:native` — 75
- `effigy probe:gpui-specimens`
- `effigy test:parity-evidence-ledger`
- `effigy check:parity-evidence-ledger`
- `effigy ci:native`
- `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

No `*-windowed`, native visual, Jetstream preview/QA, release, tag, or
publication selectors. `effigy doctor` stayed red on the known main-base
oversized-file and broad-suppression scans; that debt was not absorbed.

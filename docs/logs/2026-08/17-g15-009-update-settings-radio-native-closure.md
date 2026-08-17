# g15.009 — Update, Settings, Radio & Context-Provider Native Closure (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/009-update-settings-radio-native-closure.md`
Worktree: `t3code/update-settings-radio-native-closure`

## Summary

Closed the remaining measured native gaps outside the Licence and
model-connection families: UpdateStatus, UpdateCenter, SettingsShell, Radio,
and the IconProvider / UiPresentationProvider render-tier posture. Display
copy for updates lives once in `poodle_headless::update`, a headless mirror of
`packages/core/src/update.ts`. Providers are child-passthrough nodes, not a
declared capability absence. Jetstream is program-deferred.

## Batches

- **Batch A — declarations and render.** `poodle_headless::update` ports the
  five wrong-message vectors, progress superseding availability, and null vs
  zero download fraction. `RadioSpec`, `UpdateStatusSpec`, `UpdateCenterSpec`,
  and `SettingsShellSpec` are cloneable controlled data. `poodle-render`
  implements all four plus the two provider passthroughs. Radio's activate
  fires `true` only when unchecked; an already-checked radio is a no-op.
  Presence `hidden` collapses UpdateCenter. SettingsShell never filters
  `groups`; `page` is a host Node slot; refused close keeps the dialog.
- **Batch B — provider posture.** IconProvider returns the child unchanged
  (GPUI uses a shared registry). UiPresentationProvider returns the child and
  stamps `density` / `size_scale` on `roles` so the non-cascade is visible.
  Native Binding notes land on all six contracts. `contract-spec-drift` treats
  `observe` and SettingsShell `page` as slug-scoped web-only, aliases
  IconProvider `icons` → `icon_set_name`, and drops both providers from
  `NO_SPEC`.
- **Batch C — GPUI evidence.** Four specimens (Radio as a three-option native
  group of standalone radios; UpdateStatus / UpdateCenter / SettingsShell
  through `poodle-render`) plus mounted regressions on the in-memory test
  platform. Existing provider specimens stay.

## Evidence per runtime

One runtime does not borrow another's pass.

- **Svelte / React / Core (TS):** unchanged.
- **Rust headless** (`poodle-headless` `update` module): the five wrong
  messages, progress superseding availability, null vs zero fraction.
- **Rust specs:** Radio, UpdateStatus, UpdateCenter, SettingsShell.
- **Rust render:** the four workstation/primitive modules plus provider
  passthroughs. Radio evidence is single-option, not RadioGroup.
- **GPUI specimens:** `radio`, `update-status`, `update-center`,
  `settings-shell` in the preview catalogue. Provider specimens already
  existed.
- **GPUI mounted regressions:** Radio select-without-uncheck; UpdateStatus
  confirm-then-install through a host that applies `confirm_open` before the
  next paint; UpdateCenter hidden collapse and attention+open status host;
  SettingsShell navigate via `sidebar-nav-appearance` and refused close.
- **Jetstream:** program-deferred. Not run, not counted, not claimed as an
  accepted absence.

## Intentional binding differences

- **`observe` is web-only.** A native host rerenders with fresh props.
- **`confirm_open` is host-owned overlay state.** `confirmInstall` still
  decides whether Install opens that overlay or emits `on_install`.
- **Radio group exclusivity is host-owned.** No browser `name` group; a click
  never unchecks this control. Same-`name` arrow roving is not Radio's job.
- **SettingsShell `page` is a host Node**, not a spec field. The shell does
  not filter `groups`.
- **Providers do not cascade.** Specs own size/density; GPUI uses a shared
  icon registry. Passthrough is the chosen posture, not a silent omission.
- **Null download fraction is indeterminate**, not zero.

## Validation run

- `cargo test -p poodle-render` — 307 passed (via `effigy check:gpui`)
- `effigy check:gpui` — clean
- `effigy regressions:native` — 29 passed
- `effigy docs:spec-drift` — checked 124, OK
- `effigy docs:lint` — validated (runs inside `docs:check`)
- `effigy docs:check` — failed at `report:parity` React preview:
  cannot resolve `@inflatable-cookie/poodle-core/tokens` (worktree has no
  `node_modules`; recorded in `PAPERCUTS.md`)
- `git diff --check` — clean

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector was run.

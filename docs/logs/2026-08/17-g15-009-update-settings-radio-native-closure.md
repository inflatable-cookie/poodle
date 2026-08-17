# g15.009 — Update, Settings, Radio & Context-Provider Native Closure (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/009-update-settings-radio-native-closure.md`
Worktree: `t3code/update-settings-radio-native-closure`

## Summary

Closed the remaining measured native gaps outside the Licence and
model-connection families: UpdateStatus, UpdateCenter, SettingsShell, Radio,
and the IconProvider / UiPresentationProvider render-tier posture. Display
copy for updates lives once in `poodle_headless::update`, a headless mirror of
`packages/core/src/update.ts`. IconProvider uses the allowed global-registry
passthrough. UiPresentationProvider records an honest native capability
absence; a metadata-only node does not count as inheritance. Jetstream is
program-deferred.

## Batches

- **Batch A — declarations and render.** `poodle_headless::update` ports the
  five wrong-message vectors, progress superseding availability, and null vs
  zero download fraction. `RadioSpec`, `UpdateStatusSpec`, `UpdateCenterSpec`,
  and `SettingsShellSpec` are cloneable controlled data. `poodle-render`
  implements all four plus the IconProvider passthrough. Radio's activate
  fires `true` only when unchecked; an already-checked radio is a no-op.
  Presence `hidden` collapses UpdateCenter. SettingsShell never filters
  `groups`; `page` is a host Node slot; refused close keeps the dialog.
- **Batch B — provider posture.** IconProvider returns the child unchanged
  (GPUI uses a shared registry). UiPresentationProvider is a declared
  capability absence: the current Node tree is built before a wrapper can
  propagate ambient size or density. Its GPUI specimen now supplies explicit
  child-spec values and labels them as host equivalents, not cascade evidence.
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
- **Rust render:** the four workstation/primitive modules plus the
  IconProvider passthrough. UiPresentationProvider remains missing by declared
  capability absence. Radio evidence is single-option, not RadioGroup.
- **GPUI specimens:** `radio`, `update-status`, `update-center`,
  `settings-shell` in the preview catalogue. Provider specimens already
  existed.
- **GPUI mounted regressions:** Radio select-without-uncheck; UpdateStatus
  confirm-then-install through a host that applies `confirm_open` before the
  next paint; UpdateCenter hidden collapse, keyboard-open trigger, and
  attention+open status host; SettingsShell navigate via
  `sidebar-nav-appearance` and refused close through the unmodified tree.
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
- **IconProvider uses GPUI's shared registry.** UiPresentationProvider's
  missing native cascade is declared debt and remains a release gap.
- **Null download fraction is indeterminate**, not zero.

## Validation run

- `effigy check:gpui` — clean
- `effigy regressions:native` — 29 passed
- `effigy docs:check` — clean after dependency bootstrap
- `git diff --check origin/main...HEAD` — clean

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector was run.

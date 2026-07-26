# g12.012 — Workstation Tier Removal

**Status: complete.**

## Problem

`poodle-workstation` was a parallel spec crate for shell surfaces. Of its
thirteen specs:

- **six duplicated `poodle-specs`** — `ActionDiscoveryPanelSpec`,
  `AppHeaderSpec`, `CommandPaletteSpec`, `DockRegionSpec`, `ShellStatusBarSpec`,
  `SplitViewSpec`
- **seven existed only there** — `CommandPaletteShellSpec`, `PanelHeaderSpec`,
  `PanelSurfaceSpec`, `PanelTabsSpec`, `ProjectHeaderSpec`, `SurfaceTabsSpec`,
  `WorkspaceShellSpec` — with no component on any target, no contract, and no
  Svelte counterpart

The category was declared retired in `c5a73a84`, which removed the Jetstream
adapter's `render_workstation.rs`. That commit's scope note left GPUI alone
because its `demo_app` and `reference_app` actively used `WorkspaceShellSpec`,
which has no `poodle-specs` equivalent — a decision rather than a sweep.

This card takes the decision: the seven orphans go, unreplaced.

## Two Things Named "Workstation"

Worth stating plainly, because only one of them was removed:

- **the spec tier** (`poodle-workstation` crate) — retired, deleted here
- **the preview category** (`ComponentTag::Workstation`, `"workstation"` in the
  Svelte registry) — alive and correct. It groups real components: AppHeader,
  PageHeader, StatusBar, DockRegion, Toolbar, ActionDiscoveryPanel, DetailSection,
  DetailSectionGroup, DetailShell. Nothing about that changed.

The Svelte and React sides only ever had the category, so they needed no
component changes.

## What Shipped

- `packages/contracts/workstation/` deleted
- GPUI adapter: `render_workstation.rs` → `render_shell.rs`, carrying the six
  real specs from `poodle-specs` instead of the duplicate tier;
  `SUPPORTED_WORKSTATION` → `SUPPORTED_SHELL`
- `demo_app.rs` and `examples/reference_app.rs` ported off the crate. The
  command-and-workspace screen assembles its shell from real components rather
  than rendering a `WorkspaceShellSpec` that drew nothing
- dependency dropped from `packages/gpui/adapter` and `packages/gpui/preview`
- `packages/gpui/workstation-shell-command-layout-baseline.json` deleted (a
  frozen g04.009 baseline for the retired tier), along with its validator in
  `lint-docs.ts` and the dead `docs/contracts/workstation/` machinery beside it
- `native-accessibility-proof.json`'s `workstation` layer re-pointed at the six
  surviving specs and their real contract ids
- crate dropped from `release-manifest.json`, `release-operations.json`,
  `ecosystem-acceptance.json`, both cross-runtime parity reports, and
  `test:contracts`
- Jetstream and GPUI adapter/component READMEs corrected
- `g09/006-delete-workstation-crates.md` — its open "Remaining" section is now
  closed, four generations later

## Release Governance

`release-operations.json` sets `removalGate: "successor-and-generation-delay"`
for preview packages. Satisfied:

- **successor** — `poodle-specs` for the six duplicates, constructors unchanged
- **migration path** — swap `poodle_workstation::X` for `poodle_specs::X`
- **non-replacement decision** — recorded for the seven orphans: shell surfaces
  are composites, assembled from existing components
- **release notes** — recorded in `docs/release-notes/0.1.0.md` under Breaking

Not satisfied: the generation delay. Deprecation was announced within this same
generation. **Waived by explicit owner instruction**, and noted as waived in the
release notes rather than quietly skipped.

## Verification

- `effigy ci` green; `effigy ci:native` green (gpui components, gpui preview,
  jetstream components all check clean)
- `cargo test` on the GPUI adapter: 133 passed
- `docs:lint` green after the validator and artifact removals
- repo-wide grep for `poodle_workstation` / `poodle-workstation` /
  `contracts/workstation` returns only historical roadmap entries and the
  explanatory comments in `render_shell.rs`, `lib.rs` and the READMEs

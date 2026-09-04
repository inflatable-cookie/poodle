# g16.100 — DockRegion `showTabs` Portable Spec And Baseline Closeout

Status: ready
Type: portable capability port — contract, Rust spec, render, React shell,
drift baseline
Opened: 2026-09-04
Depends on: merged `g16.095` (`f297774f4`) and `g16.099` (`660b9510d`)
Governing refs: `../../contracts/components/dock-region.md`,
`../../contracts/001-working-rules.md` (a capability in Svelte and absent
elsewhere is a gap to port), `packages/contracts/components/src/dock_region.rs`,
`packages/render/src/dock_region.rs`,
`packages/svelte/preview/scripts/react-prop-drift.ts` (`BASELINE`)
Operator decisions 2026-09-04: promote `showTabs` to the portable spec;
re-kind `Tree.onEditingChange` and `OrderBy.onActiveSortChange` as
`framework-idiom`
Consumer evidence: Longhorn `longhorn-poodle-svelte/src/poodle/LayoutDockRegion.svelte:121`
passes `showTabs`; Loophole requires `showTabs={false}` when the host owns
the tab strip (`loophole/PAPERCUTS.md:631`)
Dispatch manifest: `../dispatch.md`

## Goal

Make `showTabs` a portable DockRegion capability so the same host layout
works on GPUI, and leave the React drift baseline with no `needs-decision`
entries.

## Fixed Boundary

- Contract: move `showTabs` (`boolean`, default `true`) from the web-only
  note at `dock-region.md:137` into the public props table with its
  semantics: when `false`, the strip omits panel tabs but keeps the collapse
  toggle when `showCollapseToggle` is true; the body still renders the
  active panel; keyboard tab switching is unavailable because there are no
  tabs. `tabVariant` stays where it is.
- Rust: `DockRegionSpec` gains `show_tabs: bool` (default `true`) with a
  builder method; `poodle-render` honours it on every mode that draws a
  strip (`render/src/dock_region.rs` expanded, compact, and icon-strip
  paths), emitting no tab nodes and no tab interaction registrations when
  false. Existing specimens keep `true`; add one GPUI specimen row with
  `false` plus a collapse toggle.
- React: port `showTabs` to `DockRegion.tsx` with the Svelte default and
  behaviour; focused test.
- Baseline: delete the `dock-region` entry; re-kind `tree` and `order-by`
  entries to `framework-idiom` with the reason "React change callback paired
  with the Svelte `$bindable` state (working rules, Runtime Parity
  Authority); zero consumer use 2026-09-04". The gate must pass with no
  `needs-decision` entries.
- Do not change tab drag, DockRegion sizing, or the Tabs component.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Portable field is honoured | `show_tabs=false` still emits tab nodes | node inventory test: zero tab nodes, toggle present |
| Web parity | React `showTabs={false}` still renders tabs | React test |
| Contract moved, not duplicated | prop appears in both the table and the web-only note | `docs:contract-drift` / `docs:spec-drift` green only when it is in one place |
| Baseline is clean | any `needs-decision` entry remains | gate reports zero `needs-decision` kinds |
| Ratchet still bites | re-add `showTabs` as `svelteOnly` | gate refuses: prop no longer drifts |

## Validation

`effigy docs:react-prop-drift`, `effigy docs:check`, `effigy ci:web`,
`effigy test:contracts`, `cargo test -p poodle-render`, `effigy
regressions:native` if a mounted DockRegion regression exists, `git diff
--check origin/main...HEAD`. Never run windowed or release selectors.

## Owned Paths

`docs/contracts/components/dock-region.md`,
`packages/contracts/components/src/dock_region.rs`,
`packages/render/src/dock_region.rs` and its tests,
`packages/react/components/src/DockRegion.tsx` and test,
GPUI specimen row for DockRegion, the `BASELINE` register in
`packages/svelte/preview/scripts/react-prop-drift.ts`, execution log under
`docs/logs/2026-09/`, root `PAPERCUTS.md` (append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop when honouring `show_tabs` on a render mode needs a layout decision the
contract does not settle, or when a Jetstream compile break needs more than
a mechanical field addition. Escalation owner: Chatterbox.

# g16.074 — Nucleus Dialog M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/074-nucleus-dialog-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-074500-g16-074-nucleus-dialog-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/dialog.md`,
`docs/contracts/components/surface.md`,
`docs/contracts/components/button.md`
Branch: `feature/g16-074-nucleus-dialog-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-074-nucleus-dialog-receipt`
Planning base: `cec2670ec` (`origin/main`)
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/180`

## Outcome

`Dialog` now has a validated `M1` execution receipt emitted from the production
GPUI render, GPUI Dialog compat adapter (`node_compat::Dialog`), node backend, and test-platform path in `effigy regressions:native`.
The regression `dialog_dismissal_axes_and_controlled_rebuild_reach_the_mounted_backend`
mounts the Dialog directly via its `IntoElement` path in `HeadlessDriver::new_element_in_box`,
verifying production structure (root backdrop role, overlay posture, backdrop fill, panel elevated background,
border, surface radius, shadow, md width preset, max height, padding, and section spacing),
header title and description typography, close affordance chrome dimensions and pointer cursor,
body composed through production `Surface` with panel tone (96% alpha), subtle border (74% alpha), and Md inset padding,
actions composed through production `Button`s with semantic variant roles, disabled opacity/cursor, secondary background mix, primary accent, and focus rings,
mounted layout bounds (positive dimensions, panel containment within backdrop, child containment within panel, vertical order),
pointer activation with inside-click containment (panel clicks never bubble to backdrop dismissal),
disabled action inertness, Cancel and Confirm action button dispatches, independent dismissal axes
(close button always emits on_request_close, backdrop click respects `dismiss_on_backdrop` policy with false and true postures,
Escape key respects `dismiss_on_escape` policy with false and true postures), controlled host rebuild
with refusal stability (refusal keeps Dialog mounted without duplicate emission) and accepted close (rebuilds host tree
unmounting backdrop, surface, and close controls), emitting the M1 receipt at the terminal boundary.
A dedicated test `nearest_clickable_ancestor_stops_propagation_symmetrically` proves event propagation symmetry across modified-child -> regular-parent and regular-child -> modified-parent.
The manifest, all 10 existing receipts (`AppHeader`, `Button`, `Icon`, `IconButton`, `Menu`, `SegmentedControl`,
`SplitView`, `Surface`, `Tabs`, `Text`), and the new Dialog receipt pin the exact runtime source commit
`b97cec2638de4ab9268da2d3789f4f0ae4725eb4`. The ledger records 11 mounted Nucleus rows out of 29.

## What landed

- Contracts:
  - `packages/contracts/components/src/dialog.rs`: added unit tests validating default spec and builder methods (`open`, `default_open`, `title`, `description`, `role`, `dismiss_on_escape`, `dismiss_on_backdrop`, `dismiss_on_outside_interact`, `aria_label`, `width`, `bare`, `show_close_button`, `close_label`, `size`, `size_role`, `density`), width preset rem dimensions, and semantic token resolvers.
  - `docs/contracts/components/dialog.md` & `docs/parity/dialog.md`: updated GPUI notes and parity deltas to document backend-owned Escape dismissal via overlay layers while explicitly recording outside-interact, A1 (focus trapping, accessibility tree, modal background suppression, initial focus, focus restoration), and nested modal stacks as unproved.
- Renderer:
  - `packages/render/src/dialog.rs`: assigned `poodle-dialog-surface` id to the dialog panel container, registered `poodle-dialog-layer` dismiss layer on the panel surface to define containment boundaries, wired `on_dismiss` reason handler for Escape dismissal, assigned inert `on_activate` on panel when `on_request_close` is present to consume inside clicks, and added unit tests for backdrop/surface rendering, dismissal wiring, bare mode, and width presets.
- GPUI Compatibility Adapter:
  - `packages/gpui/preview/src/node_compat.rs`: updated `Dialog::into_element` to route directly through `poodle_gpui_node_backend::to_gpui(&self.into_node())`, while retaining legacy `native_dialog_element` and `native_dialog_backdrop` for `AlertDialog` and `ConfirmAction`, added `on_request_close` builder, and exposed `into_node`.
- GPUI Node Backend:
  - `packages/gpui/node-backend/src/interaction.rs`: added `cx.stop_propagation()` in `on_activate` and `on_activate_modified` click listeners so activated child nodes consume pointer clicks and do not bubble activation events to parent overlay backdrops.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`:
    - Added `nearest_clickable_ancestor_stops_propagation_symmetrically` proving modified-child to regular-parent and regular-child to modified-parent click propagation containment.
    - Added `dialog_dismissal_axes_and_controlled_rebuild_reach_the_mounted_backend` mounted via `HeadlessDriver::new_element_in_box` across 6 phases:
      1. Production Spec & Token Structure Proof: root backdrop tokens, overlay posture, panel elevated styling, header title/description, close affordance, biting production Surface (tone, border alpha, inset padding), and biting production Buttons (semantic variant role metadata, disabled cursor/opacity/tab index, secondary elevation mix, primary fill, focus rings).
      2. Mounted Host Setup & Layout Containment: positive dimensions, panel containment within backdrop, child containment within panel, and vertical order.
      3. Pointer Activation & Inside Click Containment: inside click on body surface absorbed without triggering backdrop close, disabled action button inertness, Cancel and Confirm button dispatches.
      4. Dismissal Axes Independence Matrix: close button click requests close, backdrop click with `dismiss_on_backdrop: false` remains inert, backdrop click with `dismiss_on_backdrop: true` requests close, Escape key with `dismiss_on_escape: false` remains inert, and Escape key with `dismiss_on_escape: true` requests close.
      5. Controlled Host Rebuild & Refusal Stability: host refusal preserves mounted Dialog without duplicate emission; accepted close rebuilds host tree unmounting backdrop, surface, and close controls.
      6. Terminal Receipt Emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/dialog--nucleus-navigation-dialog.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `Menu`, `SegmentedControl`, `SplitView`, `Surface`, `Tabs`, `Text` with source commit `b97cec2638de4ab9268da2d3789f4f0ae4725eb4`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated Dialog `expected_test` to `dialog_dismissal_axes_and_controlled_rebuild_reach_the_mounted_backend` and `source_commit` to `b97cec2638de4ab9268da2d3789f4f0ae4725eb4`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts`; reports 11 mounted rows.
- Card:
  - `docs/roadmaps/g16/074-nucleus-dialog-m1.md`: marked complete with link to execution log.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute raw backdrop or omit surface id | `poodle-dialog-surface bounds must exist` panics |
| Dependencies are real | replace Surface or Button with raw container | `Production Button must declare ghost variant in semantic roles` or background color/padding assertions fail |
| Input is mounted | invoke handlers directly | mounted observation or request/action trace is absent |
| Dismissal axes stay separate | couple Escape policy to backdrop policy | `assertion left == right failed: Escape key when dismiss_on_escape=false must be inert` |
| Surface clicks stay inside | let panel activation bubble to backdrop | `Click inside dialog surface must not trigger backdrop dismissal` fails |
| Controlled ownership is real | record close without rebuilding supplied tree | dialog remains mounted after accepted close |
| Refusal is stable | remove dialog on refused request | `Refused close must keep backdrop mounted` fails |
| Disabled action is inert | arm disabled action with click | `Disabled button must be inert to pointer activation` fails |
| Receipt is terminal | fail any final mounted assertion | no Dialog receipt is emitted |
| Evidence identity is exact | retain stale source commit SHA | `currentSourceMatchesReceipt` fails due to git diff against `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib dialog::` — 3 passed
- `cargo test --manifest-path packages/render/Cargo.toml dialog::` — 5 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions dialog_dismissal_axes_and_controlled_rebuild_reach_the_mounted_backend` — passed
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 186 passed (all 11 receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, panel/content layout containment, and test-platform pointer / Escape / action dispatches; it does not claim `A1` (accessibility tree verification, modal background suppression, initial focus resolution, or focus trap restoration), `V1` (pixel-level visual comparison), browser body-scroll locking, nested modal stacks, or Nucleus adoption.
- Merge remains orchestrator-owned.

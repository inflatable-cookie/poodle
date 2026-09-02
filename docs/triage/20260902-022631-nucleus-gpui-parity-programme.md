# Nucleus GPUI parity programme

Status: open delegate proposal; pending orchestrator/operator review and promotion.

Captured: 2026-09-02

Source handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260902-021251-nucleus-gpui-parity-programme.md`

Scope: planning only. This packet proposes Poodle work, evidence gates, and a Nucleus adoption sequence. It does not change Poodle production code, the parity ledger, tests, workflows, contracts, Nucleus, Jetstream, or release state.

## Decision and denominator

Nucleus is the first GPUI switch target. The parity bar is the Poodle surface that Nucleus actually renders, not the full catalogue and not an alphabetical subset. The exact denominator is 29 app-rendered Poodle components, ordered below by product leverage:

1. `Icon`
2. `Text`
3. `Surface`
4. `Button`
5. `IconButton`
6. `AppHeader`
7. `SplitView`
8. `SegmentedControl`
9. `Tabs`
10. `Menu`
11. `Dialog`
12. `Popover`
13. `Select`
14. `EditableLabel`
15. `AgentTranscript`
16. `AgentChatInput`
17. `AgentPlan`
18. `AgentQuestion`
19. `ModelPicker`
20. `StatusIndicator`
21. `RadioGroup`
22. `Switch`
23. `TextInput`
24. `Callout`
25. `ConfirmAction`
26. `DetailItem`
27. `CommandPalette`
28. `MessageCenter`
29. `ToastHost`

The raw tag/import scan finds 30 Poodle exports because Nucleus mounts `<IconProvider>` in `apps/desktop/src/App.svelte:330`. `IconProvider` is a construction-time root provider, not an app-rendered component target, so it is excluded from the 29-row parity denominator but is a mandatory composition prerequisite. The packet must retain both facts: `30` raw Poodle exports observed, `29` rendered targets accepted.

Nucleus has a second ambiguity that must be resolved before adoption evidence is attributed to a release. Its committed `apps/desktop/package.json` and `bun.lock` pin `@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte` to `0.2.2`. The local `apps/desktop/node_modules` entries are symlinks to the Poodle checkout and report the current unpublished `0.2.3`. All 29 names were present in the `v0.2.2` Poodle export, but a local symlink is not a reproducible consumer receipt. The adoption card must record the exact Poodle package version, commit, lockfile resolution, and whether the run uses published packages or a deliberate workspace link.

## Nucleus interaction map

The component list is a product journey, not a catalogue assertion. These are the observed interaction groups that the first GPUI proof must preserve.

| Journey | Nucleus evidence | Required interaction shape |
| --- | --- | --- |
| Shell and workspace | `App.svelte:330,349,377,386,400,456-538`; `ProjectWorkspaceStage.svelte:868-899,1122-1125`; `DiffPanel.svelte:229`; `EditorPanel.svelte:770` | Install the icon registry, apply `data-theme="cobalt"`, compact density and `sm` controls, mount a horizontal `SplitView`, support header drag-region semantics, project-details `Popover`, settings and action buttons, root command palette, and root toast host. |
| Project and workspace navigation | `ProjectRail.svelte:506-657`; `WorkspaceSidebar.svelte:105`; `ProjectResourceManager.svelte:185`; `FilesSidebarView.svelte:831,900,968`; `ThreadsSidebarView.svelte:320,360` | Filter/select projects, expand and rename, open project actions, switch tabs, open menus and dialogs, search/select resources, dismiss overlays, commit or cancel an editable label, and restore focus to the invoking control. |
| Agent workflow | `AgentChatPanel.svelte:1210-1420` | Stream transcript output, expand tool runs/calls and changed files, open a child, edit and submit text, stop a busy run, attach files, choose actor/provider/model, accept/revise/dismiss a plan, answer or dismiss a question, and preserve disabled/busy/reviewing-plan gates. |
| Settings and remediation | `AppearanceSettingsPage.svelte:34`; `AgentProviderSettingsPage.svelte:260-360`; `GeneralSettingsPage.svelte:26`; `ForgeCommitComposer.svelte:35`; `NucleusRestoreSettingsPage.svelte:84-152` | Change radio and switch values, edit and submit a validated text input, render restore callouts, inspect key/value detail, and require an explicit confirm action before destructive restore. |
| Command and attention | `src/lib/commands/CommandPalette.svelte:29`; `NotificationPopover.svelte:107`; `App.svelte:537-538` | Open and navigate the command session, read/remove/mark-all-read/select notification actions, and exercise store-driven toast creation, action, dismissal, auto-dismiss, and sticky danger behavior. |

`MessageCenter` is one denominator row even though Nucleus's `NotificationPopover` composes it with an internal `Popover`. Nucleus currently observes the portalled `.poodle-message-center` surface so native overlay plumbing can survive. That observer is an adoption/composition requirement and must not be mistaken for a second Poodle component or for proof that the generic `Popover` contract is complete.

Some surrounding controls are native HTML elements with Poodle `Icon` children. They remain Nucleus-owned and are not silently added to the Poodle denominator. The acceptance journey must test the boundary where those controls host Poodle nodes.

## Proof contract

The current ledger mixes useful expected-test-manifest data with execution claims. A test name, specimen route, source import, or row in a generated map is not executable parity evidence. Every promotion below requires a receipt from a command that actually ran the relevant runtime.

### Levels

| Level | Meaning | Minimum executable receipt |
| --- | --- | --- |
| `M0` construction | The spec can be constructed or appears in the specimen catalogue. It proves neither mounting nor behavior. | `effigy probe:gpui-specimens` output naming the route and commit. This is a discovery baseline only. |
| `M1` mounted execution | A real GPUI in-memory `TestAppContext`/`VisualTestContext` mounts the production render/node/backend path and drives pointer, keyboard, text, focus, overlay, or timer input as applicable. | Component ID, scenario, runtime, action sequence, observed callbacks/state/tree/focus result, assertion result, command, commit, and artifact path. The driver must dispatch through the mounted tree; direct handler calls do not count. |
| `M2` Nucleus cohort journey | A composed Nucleus-shaped GPUI root reaches every denominator component in product context and preserves cross-component state, focus, overlay, and callback flow. | One replayable journey receipt plus per-component links to `M1`, accessibility, and visual receipts. |
| `A0` manual accessibility posture | Existing manual/native posture only. It is a gap marker, not a passing proof. | Current `packages/gpui/native-accessibility-proof.json#currentPosture` reference. No promotion. |
| `A1` executable native semantics | The mounted node tree exposes the contract-required role, name/description, value, checked/selected/expanded state, controls/label relationship, disabled state, and tab index; transitions are asserted after real input. Non-interactive primitives must assert their hidden/decorative or text semantics. | Machine-readable node/accessibility snapshot before and after actions, with component and scenario IDs. |
| `A2` native accessibility authority | Independent validation through the agreed GPUI/native accessibility authority, including keyboard traversal and assistive-technology or platform-inspector evidence where required. | External lab or authority receipt tied to the exact build, OS/runtime, scenario set, and findings. A1 cannot self-waive A2. |
| `V0` visual gap | No GPUI comparison evidence. | None; remains missing. |
| `V1` component visual parity | Deterministic Svelte/React/GPUI captures for the component states Nucleus uses, with fixed theme/density/control-size/viewport and an explicit diff rule. | Capture manifest, hashes, runtime/build receipt, diff output, and reviewer disposition. Local `*-windowed` selectors are not a substitute for the dedicated lab. |
| `V2` composed Nucleus visual parity | The Nucleus-shaped shell and journey states are compared in the dedicated conformance lab. | External lab receipt covering the accepted cohort and exact adapter/manifest versions. |

For a component to pass the first switch gate, the minimum is `M1 + A1 + A2 + V1`; the composed target additionally needs `M2 + V2`. A component can be source-ready while `A2` or `V2` remains externally blocked. That keeps unrelated Poodle repairs moving without making an unverified switch claim.

## Current evidence and shortest honest gap

The current ledger is a baseline, not a pass. For all 29 rows, current construction evidence is `M0`: focused specimen routes through `packages/gpui/preview/src/specimens/mod.rs` and `specimen_probe.rs` via `effigy probe:gpui-specimens` (`175/175` routes). For all 29, current accessibility is `A0` manual. Current web visual evidence is a focused route sweep, not native visual proof. `Button` has the only current GPUI visual comparison (`18` fixtures); the other 28 are `V0`.

The current GPUI mounted evidence is real headless execution through `packages/gpui/preview/src/headless_driver.rs` and `tests/headless_regressions.rs`, but it covers only 12 of the 29 targets. The table records the exact existing mounted receipt or the honest absence of one. For rows marked `M1`, the shortest gap is still a Nucleus-specific state/journey receipt plus `A1/A2/V1`; the existing regression may cover only one slice of the product contract.

| # | Component | Nucleus use | Current `M / A / V` | Shortest honest gap |
| ---: | --- | --- | --- | --- |
| 1 | `Icon` | `App.svelte:400`; `AgentChatPanel.svelte:1227,1291,1300`; `ProjectRail.svelte` throughout | `M0 / A0 / V0` | Mount both decorative and named icons through `IconProvider`; assert hidden/name semantics and Nucleus icon states; then supply geometry/token captures. Icon geometry remains coupled to the g16.049/g16.050 and lab path. |
| 2 | `Text` | `AgentChatPanel.svelte:1224-1229`; `ProjectWorkspaceStage.svelte:874-899,1124-1125` | `M0 / A0 / V0` | Mount the typography variants used by shell, transcript, and empty states; assert the contract's roleless/text semantics and compare typography under Cobalt/compact/sm. |
| 3 | `Surface` | `ProjectWorkspaceStage.svelte:868,885,1122`; `DiffPanel.svelte:229`; settings pages | `M0 / A0 / V0` | Mount nested shell/panel surfaces with their token boundaries; assert composition and region semantics where supplied; compare borders, fills, radius, and density. |
| 4 | `Button` | `App.svelte:456,464`; project rail, agent panel, settings and dialogs | `M1 / A0 / V1` — `a_mounted_button_carries_its_controls_target`; GPUI has 18 visual fixtures | Add Nucleus action states (submit, stop, retry, accept, revise, dismiss, destructive confirm), keyboard/focus/disabled/pressed assertions, A1/A2, and reuse/extend the visual fixture set. |
| 5 | `IconButton` | `App.svelte:472,488`; `RunFleet.svelte:141`; `TaskList.svelte:99`; `Memory.svelte:84` | `M1 / A0 / V0` — `icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard` | Prove Nucleus settings/notification/menu triggers, tooltip timing/label, toggle and disabled paths; add A1 and a visual fixture. Native tooltip behavior is part of the receipt. |
| 6 | `AppHeader` | `App.svelte:377` | `M0 / A0 / V0` | Mount the titlebar with `dragRegion`, `ariaLabel`, project detail popover, and action slots; assert header name, overlay ownership, focus restoration, and shell visual state. |
| 7 | `SplitView` | `App.svelte:349`; workspace and diff/editor compositions | `M1 / A0 / V0` — `two_composed_split_views_do_not_share_a_divider_focus_handle` | Add Nucleus ratio/min-max/collapse callbacks, divider keyboard/focus semantics, and both populated/empty panel states; compare the shell composition. |
| 8 | `SegmentedControl` | `ProjectRail.svelte:506` | `M1 / A0 / V0` — `segmented_control_exclusive_focus_identity_and_disabled_paths` | Drive All/Parked/Archived filtering with selection and disabled states; assert selected semantics and focus identity; add Nucleus visual states. |
| 9 | `Tabs` | `WorkspaceSidebar.svelte:105` | `M1 / A0 / V0` — `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` | Add Nucleus tab set, selected/disabled/roving focus and drag behavior; repair native `showTooltips` propagation before claiming tooltip parity; add A1/V1. |
| 10 | `Menu` | `App.svelte:479`; `ProjectRail.svelte:537,636`; resource sidebars | `M0 / A0 / V0` | Mount project/action menus, keyboard navigation, nested or destructive items, outside/Escape dismissal, and trigger focus restoration; compare menu placement and states. |
| 11 | `Dialog` | `ProjectRail.svelte:462`; `RunDispatchDialog.svelte:100`; `OrchestratorDesignationDialog.svelte:219` | `M0 / A0 / V0` | Mount open/close and restore-confirm flows with focus containment/restoration, Escape, disabled submit, and scrim semantics; add A1/V1. |
| 12 | `Popover` | `App.svelte:386`; `DiffPanel.svelte:240,261`; `EditorPanel.svelte:778`; `OperationPopover.svelte:25` | `M1 / A0 / V0` — `a_nested_popover_paints_without_nesting_deferred_draws` | Drive project details and operation popovers through real trigger/input/dismiss paths, including portalled surface geometry; assert anchor/placement/focus and add V1. |
| 13 | `Select` | `AgentChatPanel.svelte:1279,1362`; provider/resource settings | `M1 / A0 / V0` — `select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds` | Cover actor/provider Nucleus variants (`native={false}`, ghost triggers, icons), search, selection, disabled/loading, outside/Escape dismissal, and focus restoration; add A1/V1. |
| 14 | `EditableLabel` | `ThreadsSidebarView.svelte:320,360` | `M0 / A0 / V0` | Mount rename commit, cancel, blur, keyboard, and Unicode/long-label cases with the owning list; coordinate with g16.045's source-ready work and add A1/V1. |
| 15 | `AgentTranscript` | `AgentChatPanel.svelte:1236` | `M0 / A0 / V0` | Mount append/stream output with `role=log`/polite live semantics, tool run/call and changed-file toggles, child opening, auto-scroll/pinned state, and focus behavior; add A1/V1. |
| 16 | `AgentChatInput` | `AgentChatPanel.svelte:1307` | `M0 / A0 / V0` | Drive controlled editing, submit/stop, attachments, plan/question slots, placeholder/labels, disabled and busy states; assert editor/attachment/action semantics and compare the composer. |
| 17 | `AgentPlan` | `AgentChatPanel.svelte:1326` | `M1 / A0 / V0` — `agent_plan_decisions_rebuild_the_host_spec_through_mounted_input` | Add the composed Nucleus pending-plan path and revise-to-composer focus handoff; assert accept/revise/dismiss state and add A1/V1. |
| 18 | `AgentQuestion` | `AgentChatPanel.svelte:1338` | `M0 / A0 / V0` | Drive single and multi selection, submit, dismiss, option focus and native-host keyboard paths; assert radiogroup/group semantics and add A1/V1. |
| 19 | `ModelPicker` | `AgentChatPanel.svelte:1373` | `M0 / A0 / V0` | Mount model/axis changes, busy/disabled behavior, keyboard selection, and callback routing into the agent panel; add A1/V1. |
| 20 | `StatusIndicator` | `AgentChatPanel.svelte:1294` | `M0 / A0 / V0` | Mount idle/busy/questioning/reviewing-plan states with a semantic label and actor context; assert whether status is exposed or decorative per contract, then compare states. |
| 21 | `RadioGroup` | `AppearanceSettingsPage.svelte:34`; `AgentProviderSettingsPage.svelte:360` | `M1 / A0 / V0` — `radio_group_exclusive_focus_identity_and_disabled_paths` | Drive appearance/provider choices, selected/disabled paths and settings persistence callback; add A1/V1 for the actual Nucleus labels and density. |
| 22 | `Switch` | `GeneralSettingsPage.svelte:26` | `M1 / A0 / V0` — `switch_toggle_readonly_and_disabled_rebuild_the_host_spec` | Mount the general setting with checked/read-only/disabled transitions and persistence callback; add A1/V1. |
| 23 | `TextInput` | `ForgeCommitComposer.svelte:35` | `M1 / A0 / V0` — `text_input_controlled_editing_and_identity_rebuild_the_host_spec` | Drive the commit composer through editing, validation, submit, focus and caret ownership; assert labels/errors and add V1. |
| 24 | `Callout` | `NucleusRestoreSettingsPage.svelte:84,93,99,136,152` | `M1 / A0 / V0` — `callout_dismiss_rebuilds_the_host_spec_through_mounted_input` | Cover restore warning/info/error states, dismiss callback, links/actions and focus behavior; add A1/V1. |
| 25 | `ConfirmAction` | `NucleusRestoreSettingsPage.svelte:141` | `M0 / A0 / V0` | Mount the destructive restore confirmation with explicit invocation, cancel/confirm, disabled/busy, focus and no-accidental-submit assertions; add A1/V1. |
| 26 | `DetailItem` | `NucleusRestoreSettingsPage.svelte:126-128` | `M0 / A0 / V0` | Mount read-only key/value details with long/truncated and empty values; assert text semantics and compare the remediation panel. |
| 27 | `CommandPalette` | `App.svelte:537`; `src/lib/commands/CommandPalette.svelte:29` | `M0 / A0 / V0` | Drive open/search/highlight/execute/Escape and command-state rebuilds from the root; assert focus/session semantics and add V1. |
| 28 | `MessageCenter` | `NotificationPopover.svelte:107` | `M0 / A0 / V0` | Mount unread/read/remove/mark-all/select-action states through the portalled internal popover and Nucleus surface observer; assert action routing, focus, geometry, and A1/V1. |
| 29 | `ToastHost` | `App.svelte:538` (`autoDismissMs=6000`, sticky `danger`, `top-end`) | `M0 / A0 / V0` | Drive store add/update/action/dismiss/timer/sticky behavior from the root; assert live-region and action semantics, stacking/placement, and add V1. |

The current mounted twelve are useful starting points, not a completed cohort. The other seventeen must gain a real mounted scenario before their rows can move beyond the construction baseline. No row should be promoted because a selector merely exists in `headless_regressions.rs` or because a component appears in the generated ledger.

## Bounded paired source repairs

These are source/test repair proposals. They must be dispatched as bounded implementation cards and recorded through their own code/test evidence. Updating a ledger sentence cannot close any of them.

### `PAIR-H1` — recursive HistoryCenter continuation deletion

Evidence: `packages/core/src/history-center.ts:989-1027` creates a new map and calls `next.set(level.anchorEntryId, invalidated)`, which is correct for the root level but leaves a nested child in its parent map. The headless contract already uses recursive `replace_level` in `packages/contracts/headless/src/history_center.rs:1099-1138`, and its nested test at `1810-1849` expresses the intended result.

Bounded scope: align the TypeScript machine's deletion/replacement operation with the recursive headless behavior; add a nested continuation test covering the nested vector, retained sibling/anchor behavior, and existing delete/load effects. Re-run the paired core/headless contract checks. Do not alter Nucleus or claim a GPUI visual result.

Exit: deleting a nested continuation removes that level from the actual nested context, does not leave stale descendants, preserves unrelated branches, and emits the same intended effects in both machines.

### `PAIR-S1` — Slider negative-half rounding law

Evidence: `packages/core/src/slider.ts:26-31` uses `Math.round((value - min) / step)` while `packages/contracts/headless/src/slider.rs:9-14` uses Rust `f64::round()`. A negative half-step is therefore a paired-machine divergence: JavaScript rounds `-0.5` toward positive infinity while Rust rounds away from zero.

Bounded scope: state the rounding law in the shared slider contract, implement that law in both machines, and add an explicit negative-half vector plus min/step boundary cases. Keep the already-aligned safe max behavior covered. Do not fix this in Nucleus or hide it as an evidence-ledger exception.

Exit: identical quantized value and change callback for the same negative-half input in core and headless, with an executable paired test receipt.

### `PAIR-T1` — native Tabs tooltip propagation

Evidence: the public Tabs contract requires `showTooltips` at `docs/contracts/components/tabs.md:75`, and the Rust spec carries `shows_tooltips` at `packages/contracts/components/src/tabs.rs:107-108`. `packages/render/src/tabs.rs:81-97` maps items/activation but does not propagate the field. The native/headless comments currently leave tooltip behavior adapter-side, while the core machine owns the tooltip behavior around `packages/core/src/tabs.ts:247-278`.

Bounded scope: decide and document the native tooltip boundary, then wire the contract field through the GPUI path with the required label/delay/hide behavior and mounted assertions. Keep DOM drag behavior adapter-specific. Include a Nucleus Tabs state in the proof fixture, but keep Nucleus adoption out of the repair.

Exit: `showTooltips` has one documented cross-runtime meaning; native mounted execution proves tooltip exposure/timing for the accepted state, and Svelte/React/native contract tests agree. No local `*-windowed` selector is needed or permitted for this card.

`PAIR-T1` is a direct dependency of the Tabs cohort card. `PAIR-H1` and `PAIR-S1` are parallel paired-runtime confidence repairs: they should not be made prerequisites for unrelated source-ready components unless an orchestrator review finds a direct dependency.

## Dependency graph

The proposed order follows the Nucleus journey from shell substrate to cross-component behavior. It intentionally groups high-leverage composition before lower-frequency leaf states and is not alphabetical.

```text
NP-0 execution receipt contract and fixed 29-row cohort manifest
  ├──> NP-1 shell substrate (IconProvider, Icon, Text, Surface, Button, IconButton,
  │    AppHeader, SplitView)
  │      └──> NP-2 navigation and overlay spine (SegmentedControl, Tabs, Menu,
  │           Dialog, Popover, Select, EditableLabel)
  │                ├──> NP-3 agent workflow (AgentTranscript, AgentChatInput,
  │                │    AgentPlan, AgentQuestion, ModelPicker, StatusIndicator)
  │                └──> NP-4 settings and remediation (RadioGroup, Switch,
  │                     TextInput, Callout, ConfirmAction, DetailItem)
  │                          └──> NP-5 command and attention (CommandPalette,
  │                               MessageCenter, ToastHost)
  ├──> PAIR-H1 HistoryCenter repair (parallel)
  ├──> PAIR-S1 Slider repair (parallel)
  └──> PAIR-T1 Tabs tooltip repair ───────────────> NP-2

NP-1..NP-5 component receipts
  └──> NP-6 composed Nucleus-shaped M2 journey receipt
             ├── waits for A2 GPUI accessibility authority
             └── waits for V2 dedicated conformance lab

NP-6 Poodle proof and version receipt
  └──> NP-7 Nucleus-owned GPUI adoption and switch decision

Jetstream direct adapter ── quarantine / separate admission decision; no edge into proof
```

### Proposed cards

| Order | Card | Small bounded outcome | Dependencies and non-goals |
| ---: | --- | --- | --- |
| 0 | `NP-0` Execution receipt and cohort manifest | Define the machine-readable receipt shape and freeze the 29-row Nucleus manifest, including the `IconProvider` prerequisite and version receipt fields. | Uses existing headless/visual runners. Does not rewrite the current ledger or invent broad cross-runtime claims. |
| 1 | `NP-1` Shell substrate | Produce mounted and semantic receipts for the root theme/density/control-size path, icon registry, typography/surfaces, primary actions, titlebar, and split layout. | `IconProvider` is setup, not a denominator row. Coordinate icon geometry gates. No Nucleus source change. |
| 2 | `NP-2` Navigation and overlay spine | Cover project filtering, tabs, menus, dialogs, popovers, selects, and editable rename as one navigable product spine, with focus/overlay restoration. | Depends on shell and `PAIR-T1` for Tabs tooltip behavior. Coordinate with g16.045 rather than duplicating EditableLabel work. |
| 3 | `NP-3` Agent workflow | Cover transcript, composer, pending plan, question, model/actor selection, and status transitions in a composed chat fixture. | Depends on shell/overlay focus substrate. Does not move Nucleus's agent data/orchestration into Poodle. |
| 4 | `NP-4` Settings and remediation | Cover settings controls and restore remediation, including radio/switch/input validation, callouts, explicit confirmation, and details. | Depends on shell and overlay semantics. Destructive actions remain fixture-only until Nucleus adoption proves its data path. |
| 5 | `NP-5` Command and attention | Cover command palette, notification center, portalled surface observation, and store-driven toast lifecycle. | Depends on overlay/focus receipts. The Nucleus observer remains an adoption composition concern. |
| 6 | `NP-6` Nucleus-shaped acceptance journey | Replay the first shippable journey below and link all 29 component receipts into one M2 receipt. | Blocks the switch decision; does not itself waive A2 or V2. |
| 7 | `NP-7` External proof gates | Run the agreed GPUI accessibility authority and dedicated visual conformance lab against the fixed build and fixture manifest. | External authority/lab work. No local `*-windowed` conformance. Source-ready Poodle cards remain unblocked while these gates are unavailable. |
| 8 | `NP-8` Nucleus adoption and switch decision | Pin the exact package/commit/lock resolution, mount the Nucleus-owned data/actions on GPUI, and record a go/no-go decision. | Nucleus-owned. Starts only after Poodle proof and external gates; no Jetstream evidence substitution. |

`PAIR-H1` and `PAIR-S1` may run alongside `NP-1` unless review identifies a direct contract dependency. `PAIR-T1` must complete before Tabs is admitted to `NP-2`'s full proof. These cards are proposals, not promoted runway items.

## Making the existing ledger execution-backed

The existing `docs/roadmaps/g16/parity-evidence-ledger.md` remains valuable because its expected-test manifest records intended coverage and historical context. It should become an execution-backed view through a follow-up card, not by deleting that data or treating names as receipts.

1. Preserve the current `MOUNTED_BEHAVIOUR_TESTS`/expected-test manifest in `scripts/parity-evidence-ledger.ts` as expected coverage. Keep its component-to-selector mapping and mark it as expected/planned input where necessary.
2. Add an execution receipt emitted by the actual selector runner. At minimum it must carry component, scenario/selector, runtime, command, commit, action/assertion summary, outcome, and artifact paths. A receipt is valid only when the runner exits successfully and the component is observed in the mounted execution path.
3. Make the checker compare expected entries with receipts. An expected selector without a receipt stays missing; an unmanifested receipt is an explicit review item. Do not infer mounted status from a test name, a Rust source reference, a specimen route, or a static import.
4. Keep accessibility and visual receipts separate from mounted receipts. `A0` manual posture must not be upgraded by an `M1` run. Button's current 18-fixture GPUI comparison must not be generalized to the remaining 28 rows.
5. Derive summary counts from validated receipts plus explicit N/A/deferred decisions, while retaining the old expected-manifest counts for planning traceability. This makes the transition auditable and prevents historical useful data from disappearing.
6. Link each receipt to the fixed Nucleus cohort manifest and exact package/build version. A local symlink to Poodle `0.2.3` cannot satisfy a consumer run declared against the committed Nucleus `0.2.2` lock without an explicit resolution record.

This ledger work is a future implementation card under `NP-0`; this packet does not edit the ledger or checker.

## First shippable Nucleus-on-GPUI journey

The first acceptance journey is a seeded, Nucleus-shaped workspace fixture. It is not a fake component gallery and it does not require the Nucleus repository to be mutated during Poodle proof work.

1. Start the real GPUI root with `IconProvider`, Cobalt theme, compact density, `sm` control size, overlay host wiring, drag/drop host, and a two-panel `SplitView`. Seed one project, one workspace tab, one thread, one agent turn, one pending plan/question, one resource, one notification, and one danger toast.
2. In the project rail, choose a project through `SegmentedControl`, expand it, open its `Menu`, rename a thread through `EditableLabel`, and switch the workspace `Tabs`. Open project details through the header `Popover`, exercise an `IconButton`, and change the split divider/collapse state.
3. Open a resource `Dialog`/`Select` path and a command palette session. Search and choose an actor/provider/model. Type in `AgentChatInput`, submit, stop a busy run, and exercise disabled and reviewing-plan states.
4. Accept, revise, and dismiss `AgentPlan` in separate seeded checkpoints. For a question checkpoint, choose single and multi options in `AgentQuestion`, submit, and dismiss. The transcript must append output, expand a tool run/call and changed files, and open a child while preserving log/live and focus semantics. `StatusIndicator` must follow the seeded state transitions.
5. Navigate to settings. Change `RadioGroup` and `Switch` values, edit and validate the `TextInput`, render `Callout` states, inspect `DetailItem`, and invoke `ConfirmAction` only after the restore warning. Verify cancel/confirm and focus restoration.
6. Open `MessageCenter`, mark an item read, remove one, mark all read, select an admitted action, and verify the portalled surface observer. Add, act on, dismiss, auto-dismiss, and retain a sticky-danger `ToastHost` item. Return focus to the originating controls and complete the journey through keyboard navigation.
7. Emit one M2 receipt that names all 29 denominator components and links each to its M1, A1, and V1 artifacts. Attach the external A2 and V2 receipts for the accepted build and fixture set. Record the Poodle package/commit and the Nucleus consumer resolution separately.

### Stop conditions

Stop the relevant card or switch decision when any of these occurs:

- A component has only a specimen route, source import, expected selector, or test-name reference.
- A mounted test bypasses the real render/node/backend path, calls handlers directly, or lacks action-and-observation output.
- Any denominator row lacks its required `M1`, `A1`, `A2`, or `V1` receipt, or the composed journey lacks an `M2` link.
- The dedicated visual lab or GPUI accessibility authority is absent, mismatched to the build, or has unresolved findings. Do not waive the gate; continue unrelated source-ready work where it is safe.
- `PAIR-H1`, `PAIR-S1`, or `PAIR-T1` exposes a runtime divergence in the affected journey. Repair the source/test pair before promoting that component; do not rewrite the ledger to describe the divergence away.
- Nucleus's committed `0.2.2` resolution and the actual Poodle runtime differ without an explicit version/commit receipt.
- The journey needs an unlisted Poodle component, an uncontracted behavior, or an app-owned native control to be treated as Poodle proof. Return that boundary to review.
- A local `*-windowed` conformance selector or native visual shortcut is proposed in place of the approved headless/external paths.
- Jetstream's 108-component direct adapter is used as evidence for Poodle render/node/GPUI parity. Jetstream remains quarantined pending its separate admission decision.

## Ownership and routing

| Concern | Owner and allowed work | Explicit boundary |
| --- | --- | --- |
| Poodle parity implementation | Poodle contracts, specs, renderers, GPUI node/backend, headless execution, receipt/checker, and the three paired repairs. | Shared semantics stay in Poodle's contract/spec path; no app-specific Nucleus workflow logic in Poodle. |
| Nucleus adoption | Nucleus owns GPUI root adoption, data/orchestration, callbacks, app-owned native controls, package/version resolution, and consumer migration. | Nucleus does not define Poodle parity evidence by source presence, and this packet does not mutate Nucleus. |
| Dedicated conformance lab | External lab owns approved visual capture, adapter/manifest versions, diff review, and any native visual authority required by the lab. | The accepted Button tranche is not 29-component Nucleus coverage. No local windowed substitute. |
| GPUI accessibility authority | Agreed native platform/accessibility reviewer owns A2 validation and findings. | Poodle's A1 node snapshot is necessary but cannot self-certify broad assistive-technology parity. |
| Jetstream | Remains quarantined under its existing hold; admission is a separate review of the direct adapter and 108-component surface. | No Jetstream component count, adapter, or visual result enters the Nucleus/Poodle proof denominator. |
| Orchestrator/operator | Promote these proposals, sequence dispatch, review receipts, decide external authority, and merge approved PRs. | This packet grants no implementation, release, or merge authority. |

## Delegate recommendations for review

1. Accept 29 app-rendered components as the Nucleus parity denominator and retain `IconProvider` as a separately tracked composition prerequisite.
2. Treat `M1 + A1 + A2 + V1`, plus the composed `M2/V2` journey, as the minimum first-switch bar. Keep source-ready work moving while external gates are pending.
3. Promote `NP-0` first so the existing expected-test manifest becomes auditable execution evidence without losing historical coverage data.
4. Dispatch the three paired repairs as bounded source/test cards, with Tabs tooltip propagation ahead of the navigation cohort.
5. Build the proof in Nucleus-leverage order: shell, navigation/overlays, agent workflow, settings/remediation, command/attention, then the composed journey and adoption.
6. Keep Nucleus adoption, the dedicated conformance lab, GPUI accessibility authority, and Jetstream admission as separate ownership and decision surfaces.

These are delegate proposals only. The orchestrator/operator must promote, alter, or reject them before implementation dispatch.

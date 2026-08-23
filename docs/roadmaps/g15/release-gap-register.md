# g15 — Release Gap Register

Status: active release register — compiled by `g15.001`, updated by review
Date: 2026-08-23
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Source: `docs/roadmaps/g15/release-baseline-roster.md` (frozen 175-component denominator)

Every incomplete surface below was measured from the tree; nothing is inferred
from another runtime's pass. Owners are the roadmap cards compiled from these
gaps (`g15.002`–`g15.053`). Absence of downstream use is recorded as context
but is **not** a gap — see the note at the end.

## Svelte Release Blocker Class

Contract, implementation, export, specimen, and **focused component evidence**
posture are complete for all 175 components. The last open Svelte-denominator
surface closed with `g15.005`: the 29 foundation display & shell components
(`g15.002`), the 26 foundation forms, inputs & overlays (`g15.003`), the 35
composites & media components (`g15.004`), and the final 24 workstation systems
and agent surfaces (`g15.005`) all carry focused evidence on both Svelte and
React sides — see `release-baseline-roster.md` for the named case files.

### Svelte focused evidence (24) — closed by `g15.005`

| Family | Components | Count | Posture |
| --- | --- | ---: | --- |
| Workstation systems | StatusBar, StatusIndicator, Surface, Text, TextLink, Table, TokenInput, TimeInput, TimeZoneSelect, ToggleGroup, Toolbar, Tooltip, TriStateSwitch, UiPresentationProvider, VideoPlayer, DateTimeZonePicker | 16 | complete — `packages/svelte/components/test/<Name>.test.ts` |
| Agent surfaces | AgentMessage, AgentPlanRecord, AgentQuestion, AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall, ToolCallGroup | 8 | complete — `packages/svelte/components/test/<Name>.test.ts` |

No Svelte-denominator gap class remains open. The roster reads 175/0 focused
Svelte evidence.

## React Mirror Gaps

`g15.006` closed the two missing implementations/export (AgentPlan,
AgentPlanRecord), the six missing gallery specimens (AgentMessage, AgentPlan,
AgentPlanRecord, ChangedFiles, ToolCall, ToolCallGroup), and the five residual
focused React gaps (AgentPlan, Icon, IconProvider, Tree, SplitView). `g15.005`
then closed the final 23 focused React gaps alongside its Svelte tranche. The
roster reads 175/0 React implementation, 175/0 React gallery, and 175/0 focused
React evidence.

| Gap | Components | Owner | Posture |
| --- | --- | --- | --- |
| Focused React test gaps for the Svelte evidence tranches | paired with the same batches: each Svelte evidence tranche mirrors its contract cases on the React side | `g15.002`–`g15.005` | complete — `packages/react/components/test/<Name>.test.tsx` |
| Focused React test gaps paired with the final Svelte evidence tranche | the 24 `g15.005` components less `AgentSubagent`, which already carried React evidence | `g15.005` | complete — 23 mirrored case files |

No React mirror gap class remains open.

## Shared Rust Composition and GPUI Gaps

Rust declarations (`<Name>Spec`) and render modules (`poodle-render`) are
recorded independently; a component missing either is a native gap even when
the other exists. Counts below are `missing` only; `MeterSurface` is
`not-applicable` on the Rust declaration, Rust render, and GPUI axes by fixed
decision (spec 068) and is excluded from every missing count. Reproducible
count method: `docs/roadmaps/g15/release-baseline-roster.md#count-method`.

Summary of native gaps: Rust declaration 0 missing (+ 1 not-applicable),
Rust render 1 missing (+ 1 not-applicable), GPUI specimen 0 missing
(+ 1 not-applicable).

| Family | Missing Rust declaration | Missing Rust render | Missing GPUI specimen | Owner |
| --- | --- | --- | --- | --- |
| Licence (approved `g14.017` requirements) | — closed by `g15.007` (`LicenceActivationSpec`, `LicenceSeatsSpec`, `LicenceStatusSpec`) | — closed by `g15.007` (`packages/render/src/licence_{activation,seats,status}.rs`) | — closed by `g15.007` (`packages/gpui/preview/src/specimens/licence_{activation,seats,status}.rs`) | `g15.007` |
| Model connection (approved `g14.020` requirements) | — closed by `g15.008` (`ModelConnectionPickerSpec`, `ModelConnectionSetupSpec`, `ModelConnectionCardSpec`, `ModelCatalogueEditorSpec`) | — closed by `g15.008` (`packages/render/src/model_{connection_picker,connection_setup,connection_card,catalogue_editor}.rs`) | — closed by `g15.008` (`packages/gpui/preview/src/specimens/model_{connection_picker,connection_setup,connection_card,catalogue_editor}_specimen.rs`) | `g15.008` |
| Update & settings | — closed by `g15.009` (`UpdateStatusSpec`, `UpdateCenterSpec`, `SettingsShellSpec`) | — closed by `g15.009` (`packages/render/src/{update_status,update_center,settings_shell}.rs`) | — closed by `g15.009` (`packages/gpui/preview/src/specimens/{update_status,update_center,settings_shell}.rs`) | `g15.009` |
| Radio | — closed by `g15.009` (`RadioSpec`) | — closed by `g15.009` (`packages/render/src/radio.rs`) | — closed by `g15.009` (`packages/gpui/preview/src/specimens/radio.rs`) | `g15.009` |
| Context providers | — | IconProvider closed by `g15.009` (`packages/render/src/icon_provider.rs`); UiPresentationProvider implemented by `g15.043` as architecture 010's construction-time cascade (`packages/render/src/context.rs`: `RenderContext`, `ui_presentation_provider`, scoped `SlotBuilder` host slots) | closed by `g15.043`: the GPUI specimen demonstrates the real cascade (root, inherited scopes, nested override, explicit reset) with mounted headless geometry evidence; PR #71 corrected explicit-size parity so roles map inherited scale only | `g15.043` complete in PR #70; review repair in PR #71 |
| SegmentedControl option presentation | closed by `g15.038`: dedicated public `SegmentedControlOption` (breaking, pre-1.0, operator-approved 2026-08-20) | closed: labelled-icon and icon-only rendering through `poodle-render` | closed: GPUI specimen teaches the contract's Effects/Instruments icon-only example | `g15.038` |
| AgentTranscript scroll/follow | — | closed by `g15.037`: shared render owns content and the jump-control recipe | closed by `g15.037`: retained GPUI state owns a real tracked viewport, detach latch, and jump-to-bottom | `g15.037` complete in PR #48; no remaining release blocker |
| Stepper selection and re-run | — | — the shared composition already carried `on_change`, `on_rerun`, and `on_collapsed_change` | closed by `g15.042`: `node_compat.rs` binds all three, and the specimen retains the current step plus one re-run receipt; proved through the mounted headless backend in `packages/gpui/preview/tests/headless_regressions.rs` (`stepper_selection_and_rerun_reach_separate_mounted_controls`, `stepper_collapse_stays_independent_in_a_mounted_window`) and through the adapter and specimen seam in `specimen_probe.rs` (`stepper_route_selection_and_rerun_run_through_the_preview_adapter`). Pointer activation and keyboard activation after pointer focus only — keyboard **entry** stays open below | `g15.042` on PR #60; found and accepted out of scope in PR #49 |
| Specimen axis domains | — closed by `g15.034` | — closed by `g15.034` | fake panes removed by `g15.019`; exact domains closed by `g15.034` | complete in PR #41 |
| Display, workstation & agent specimens | — | — | — closed by `g15.010` (18 named GPUI specimen files under `packages/gpui/preview/src/specimens/`) | `g15.010` |
| MeterSurface | not-applicable — web-only by fixed decision (spec 068) | not-applicable | not-applicable | none |

The headless regressions (`effigy regressions:native`) certify Button,
RangeSlider, Popover, — since `g15.007` — grouped CodeInput, the generic
FileUpload browse seam, LicenceActivation's segmented key path, LicenceSeats
release, and LicenceStatus display, — since `g15.008` — the
ModelConnectionPicker's roving focus and disabled-route guard,
ModelConnectionSetup's direct-add path, ModelConnectionCard's independent
disclosure and focus restoration, and ModelCatalogueEditor's keyboard
grab/move/cancel and focus-after-hide, and — since `g15.009` — Radio's
single-option select-without-uncheck, UpdateStatus's confirm-then-install,
UpdateCenter's hidden collapse and open status host, and SettingsShell's
navigate plus refused close, and — since `g15.010` — Callout dismiss,
RemediationBanner action and dismiss, ActionDiscoveryPanel selection,
DockRegion tab and collapse, AgentPlan accept/revise/dismiss,
AgentPlanRecord and AgentSubagent disclosure, ChangedFiles disclosure and
file selection, and ToolCall / ToolCallGroup disclosure, and — since `g15.042` —
Stepper's separate selection, re-run, and collapse controls under pointer
activation and under keyboard activation of an already-focused control. Native tranches land their evidence as
focused owner-local tests, not by extending a shared comparator.

## Package-Install Surface

PR #64 (`g15.048`) proves exact clean-tarball root-import reachability for all
175 Svelte and all 175 React component names. Missing and extra names fail
explicitly, package contents and public subpaths are checked, and an
independent temporary `0.2.0` manifest mutation passed. Runtime behavior stays
bounded to the representative mount set: 9 Svelte and 13 React components in
`test/package-install/web-preview.ts`. Import reachability is not presented as
behavior evidence.

## Native Interaction And Focus Gaps

Interaction gaps that survive a component's wiring being correct. These are
measured from the mounted tree, not inferred from a contract read.

| Gap | Current evidence | Owner |
| --- | --- | --- |
| GPUI Button and Stepper focus rings; Stepper keyboard entry | closed by PR #69: one reusable `poodle-node` ring channel projects out-of-flow in GPUI; the Button comparator reports zero focus-ring findings under unchanged policy; Stepper keyboard entry, activation, blur, and inset/outset rings are proved mounted without pointer input. The 16 annotated shadow findings and the separately recorded web-only Stepper arrow / `Home` / `End` delta remain explicit | `g15.052` complete; evidence `docs/logs/2026-08/20260822-g15-052-native-focus-ring-parity.md`; merge `b2cc1dff` |

## Visual And Release-Path Gaps

| Gap | Current evidence | Owner |
| --- | --- | --- |
| GPUI headless pixels | adopted by `g15.045` in PR #62, then repinned by `g15.051` to the minimal licence-safe fork `inflatable-cookie/zed@87d9afbe`; `smoke:gpui-offscreen-capture` still renders a real Button offscreen with typed receipts and one hash across repeated captures (`be94eace…`, matching the g15.044 proof); inset shadows project, headless regressions remain green, and Rust 1.95 holds. Captures remain 2×-only and macOS-only; cross-renderer tolerance is `g15.047` | landed |
| Primitive named fixtures and comparison | closed by PR #68: 18 Button identities produced 54 deterministic captures; Svelte↔React is exact; Svelte↔GPUI geometry and pixels pass the fixed renderer-aware policy; the operator accepted the mechanism and existing shadow annotation. The measured native focus-ring defect was not waived and moved to `g15.052` | `g15.047` and parent `g15.012` complete |
| Packed full-roster reachability | closed by PR #64: exact 175/175 Svelte and 175/175 React clean-root imports plus bounded 9/13 runtime mounts | `g15.048` complete |
| Native pre-tag workflow | closed by PR #66: the pinned manual workflow installs Bun 1.3.14 and Rust 1.95, then executes the supported `effigy ci:native` board | `g15.049` complete |
| Read-only release-gate claim | closed by PR #66: `effigy release gates` executes exactly one configured `headless` gate backed by the complete `effigy qa` board | `g15.049` complete |
| GPUI/Zed dependency licence policy | closed by PR #67: exact fork `inflatable-cookie/zed@87d9afbe` removes GPL tracing from both normal GPUI graphs; `libbz2-rs` terms and attribution ship on both notice surfaces; five immutable Git sources are admitted fail-closed | `g15.051` complete; merge `30e2aae3` |
| Breadcrumb item icons | closed by PR #71: every authored crumb supports text-only, icon-plus-label, or accessible icon-only presentation; Svelte, React, shared Rust, and the GPUI specimen share the contract and explicit size semantics | `g15.053` complete |
| v0.2.0 candidate | manifests remain 0.1.0 and no 0.2.0 release note exists | `g15.050`, then operator gate `g15.013` |

## Carried Requirements (recorded, not implemented)

| Requirement | Status | Owner |
| --- | --- | --- |
| Licence native completion (`g14.017`) | closed by `g15.007` — declarations, render, and GPUI specimens landed | `g15.007` |
| Model-connection native completion (`g14.020`) | closed by `g15.008` — declarations, headless mirror, render, and GPUI specimens landed | `g15.008` |
| Human-centred specimen catalogue audit (`g14.026`) | complete: screening, pilots, defect-led curation, native probe, all six screen-clear children, routed repairs, and operator review landed through PR #63 | `g15.011` complete |
| Primitive-first visual conformance lane | complete through PR #68: exact capture feasibility, adoption, fixture inventory, comparison, and operator review landed without becoming component authority | `g15.012`, `g15.044`–`g15.047` complete |
| Release-gate remediation | closed by PR #31 — `nanoid@3.3.18`; `bun audit` clean | `g15.014` |
| v0.2.0 release certification | packed proof, truthful automation, dependency-licence remediation, and candidate preparation are explicit children; tag/publish stays operator-owned | `g15.048`–`g15.051`, then `g15.013` |

## Not Gaps

- Jetstream is program-deferred for every component; no component registers a
  Jetstream gap.
- 65 components have no downstream consumer use found across the 16 canonical
  consumers. Absence of a consumer is not a release failure (handoff
  boundary); it is recorded as context in the roster.
- The former `bun audit` nanoid advisory (GHSA-2v37-7h3g-55p8) was security
  debt rather than a component gap. `g15.014` closed it in PR #31 before
  `g15.013` certification.
- `effigy doctor` baseline findings (generated-in-src, god-file,
  stale-suppression, comment-ratio) are known board health, not component
  gaps.

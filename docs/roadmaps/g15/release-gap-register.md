# g15 — Release Gap Register

Status: active release register — compiled by `g15.001`, updated by review
Date: 2026-08-22
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Source: `docs/roadmaps/g15/release-baseline-roster.md` (frozen 175-component denominator)

Every incomplete surface below was measured from the tree; nothing is inferred
from another runtime's pass. Owners are the roadmap cards compiled from these
gaps (`g15.002`–`g15.050`). Absence of downstream use is recorded as context
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
| Context providers | — | IconProvider closed by `g15.009` (`packages/render/src/icon_provider.rs`); UiPresentationProvider remains a declared capability absence because ambient presentation cannot cross an already-built Node tree | — | `g15.043`; architecture decision required before dispatch |
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

`test:web-pack-install` proves packed-tarball reachability and mounting for 9
Svelte components (DockRegion, LicenceActivation, LicenceSeats, LicenceStatus,
ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard,
ModelCatalogueEditor, MeterSurface) and 13 React components (AgentPlan and
AgentPlanRecord added by `g15.006` as new root exports; the rest as recorded
in `test/package-install/web-preview.ts`). `g15.048` replaces the vague
"mount everything" continuation with exact clean-tarball root-import proof for
all 175 names in both web packages plus a small representative mount set.

## Native Interaction And Focus Gaps

Interaction gaps that survive a component's wiring being correct. These are
measured from the mounted tree, not inferred from a contract read.

| Gap | Current evidence | Owner |
| --- | --- | --- |
| GPUI Stepper keyboard entry and focus ring | The trigger, rerun, and summary set `interaction.focusable` but declare no focus treatment, so the node backend registers no focus handle for them (`tracks_focus` needs `style.focus`) and no focus ring paints. `g15.042`'s mounted regressions reach these controls by key **only after a pointer press has focused one**. `stepper.md` §6 requires `Tab` entry and order between trigger and rerun and activation of a focused control; §8 requires the ring on Trigger and Summary. Arrow / `Home` / `End` movement is a separate, already-recorded web-only delta (`stepper.md` §10). Closing this means a shared-render focus treatment for a control with no resting border to take the ring, and it lands on Jetstream too | open — found by `g15.042`; needs its own card, and a decision on the native focus treatment, before `g15.013` certification |

## Visual And Release-Path Gaps

| Gap | Current evidence | Owner |
| --- | --- | --- |
| GPUI headless pixels | adopted by `g15.045` in PR #62: `gpui`/`gpui_platform` pinned to `zed-industries/zed@1ea16c1a` in node-backend and preview; `smoke:gpui-offscreen-capture` renders a real Button offscreen with typed receipts, one hash across repeated captures (`be94eace…`, matching the g15.044 proof), inset shadows now project instead of being dropped; headless regressions 56/56, Rust 1.95 floor preserved. Captures remain 2×-only and macOS-only; cross-machine tolerance is `g15.047` — see `docs/logs/2026-08/20260821-g15-045-gpui-offscreen-capture-adoption.md` | landed |
| Primitive named fixtures and comparison | rejected g14 corpus is removed; web/native capture foundations are ready for a bounded inventory | `g15.046` ready, then `g15.047`, under parent `g15.012` |
| Packed full-roster reachability | 9 Svelte / 13 React mounted cases only; closure card is in flight | `g15.048` |
| Native pre-tag workflow | `.github/workflows/ci-native.yml` references deleted `packages/gpui/components/Cargo.toml` | `g15.049`, blocked on explicit workflow-edit approval |
| Read-only release-gate claim | `effigy release gates` reports success with zero configured gates; not certification evidence | `g15.049` |
| v0.2.0 candidate | manifests remain 0.1.0 and no 0.2.0 release note exists | `g15.050`, then operator gate `g15.013` |

## Carried Requirements (recorded, not implemented)

| Requirement | Status | Owner |
| --- | --- | --- |
| Licence native completion (`g14.017`) | closed by `g15.007` — declarations, render, and GPUI specimens landed | `g15.007` |
| Model-connection native completion (`g14.020`) | closed by `g15.008` — declarations, headless mirror, render, and GPUI specimens landed | `g15.008` |
| Human-centred specimen catalogue audit (`g14.026`) | complete: screening, pilots, defect-led curation, native probe, all six screen-clear children, routed repairs, and operator review landed through PR #63 | `g15.011` complete |
| Primitive-first visual conformance lane | recompiled into exact capture feasibility, adoption, fixture, and comparison children | `g15.012`, `g15.044`–`g15.047` |
| Release-gate remediation | closed by PR #31 — `nanoid@3.3.18`; `bun audit` clean | `g15.014` |
| v0.2.0 release certification | packed proof, truthful automation, and candidate preparation are explicit children; tag/publish stays operator-owned | `g15.048`–`g15.050`, then `g15.013` |

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

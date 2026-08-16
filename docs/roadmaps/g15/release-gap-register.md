# g15 — Release Gap Register

Status: complete — compiled by `g15.001`
Date: 2026-08-16
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Source: `docs/roadmaps/g15/release-baseline-roster.md` (frozen 175-component denominator)

Every incomplete surface below was measured from the tree; nothing is inferred
from another runtime's pass. Owners are the roadmap cards compiled from these
gaps (`g15.002`–`g15.014`). Absence of downstream use is recorded as context
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

Summary of native gaps: Rust declaration 11 missing (+ 1 not-applicable),
Rust render 13 missing (+ 1 not-applicable), GPUI specimen 29 missing
(+ 1 not-applicable).

| Family | Missing Rust declaration | Missing Rust render | Missing GPUI specimen | Owner |
| --- | --- | --- | --- | --- |
| Licence (approved `g14.017` requirements) | LicenceActivation, LicenceSeats, LicenceStatus | same three | same three | `g15.007` |
| Model connection (approved `g14.020` requirements) | ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard, ModelCatalogueEditor | same four | same four | `g15.008` |
| Update & settings | UpdateStatus, UpdateCenter, SettingsShell | same three | same three | `g15.009` |
| Radio | Radio | Radio | Radio | `g15.009` |
| Context providers (render passthrough only) | — | IconProvider, UiPresentationProvider | — | `g15.009` |
| Display, workstation & agent specimens | — | — | Avatar, Callout, RemediationBanner, MetaItem, Pill, Spinner, EmptyState, StateTile, ActionDiscoveryPanel, DockRegion, AgentMessage, AgentPlan, AgentPlanRecord, AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall, ToolCallGroup | `g15.010` |
| MeterSurface | not-applicable — web-only by fixed decision (spec 068) | not-applicable | not-applicable | none |

The six retained headless regressions (`effigy regressions:native`) certify
Button, RangeSlider, and Popover only. All native tranches must land their
evidence as focused owner-local tests, not by extending a shared comparator.

## Package-Install Surface

`test:web-pack-install` proves packed-tarball reachability and mounting for 9
Svelte components (DockRegion, LicenceActivation, LicenceSeats, LicenceStatus,
ModelConnectionPicker, ModelConnectionSetup, ModelConnectionCard,
ModelCatalogueEditor, MeterSurface) and 13 React components (AgentPlan and
AgentPlanRecord added by `g15.006` as new root exports; the rest as recorded
in `test/package-install/web-preview.ts`). Extending the mounted proof across
the roster is folded into the release-certification card.

## Carried Requirements (recorded, not implemented)

| Requirement | Status | Owner |
| --- | --- | --- |
| Licence native completion (`g14.017`) | web references approved and landed (`g14.015`/`g14.016`); native missing | `g15.007` |
| Model-connection native completion (`g14.020`) | web references approved and landed (`g14.018`/`g14.019`); native missing | `g15.008` |
| Human-centred specimen catalogue audit (`g14.026`) | rubric and boundary intact; unexecuted | `g15.011` |
| Primitive-first visual conformance lane | seam recorded in `conformance-estate.md`; harness not designed | `g15.012` |
| Release-gate remediation | `bun audit` nanoid advisory remains open | `g15.014` |
| v0.2.0 release certification | after all Svelte-denominator blockers close | `g15.013` |

## Not Gaps

- Jetstream is program-deferred for every component; no component registers a
  Jetstream gap.
- 65 components have no downstream consumer use found across the 16 canonical
  consumers. Absence of a consumer is not a release failure (handoff
  boundary); it is recorded as context in the roster.
- The `bun audit` nanoid advisory (GHSA-2v37-7h3g-55p8) is a security debt,
  not a component gap; it is owned as a pre-certification remediation
  prerequisite by `g15.014` before `g15.013` certification.
- `effigy doctor` baseline findings (generated-in-src, god-file,
  stale-suppression, comment-ratio) are known board health, not component
  gaps.

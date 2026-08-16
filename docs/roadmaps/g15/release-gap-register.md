# g15 — Release Gap Register

Status: complete — compiled by `g15.001`
Date: 2026-08-16
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Source: `docs/roadmaps/g15/release-baseline-roster.md` (frozen 175-component denominator)

Every incomplete surface below was measured from the tree; nothing is inferred
from another runtime's pass. Owners are the roadmap cards compiled from these
gaps (`g15.002`–`g15.013`). Absence of downstream use is recorded as context
but is **not** a gap — see the note at the end.

## Svelte Release Blocker Class

Contract, implementation, export, and specimen posture are complete for all
175 components. The only open Svelte-denominator surface is **focused
component evidence**: 114 components mount in the anatomy smoke but have no
named focused test file/case asserting contract behaviour.

### Svelte focused evidence (114) — owned by `g15.002`–`g15.005`

| Family | Components without focused evidence | Count |
| --- | --- | ---: |
| Foundation display & shell | Accordion, AlertDialog, Avatar, Box, Breadcrumbs, BulkActionBar, Callout, Card, Code, CollapseToggle, Collapsible, DetailItem, Eyebrow, Grid, HoverCard, ListCardCounter, ListGrid, MetaBar, MetaItem, NavCard, Pill, Progress, Rating, Region, Skeleton, Spacer, Stack, Stepper, Spinner | 29 |
| Foundation forms, inputs & overlays | AudioPlayer, ColorPicker, Calendar, DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker, DurationInput, EditableLabel, Field, FieldSet, FormActions, IconButton, Meter, NumberInput, Pagination, PaginationSummary, PasswordRequirements, Radio, RadioGroup, ResizeHandle, RangeSlider, SegmentedControl, ScrollShell, Separator, Slider | 26 |
| Composites & media | ActionDiscoveryPanel, EditableList, ErrorBoundary, BlockEditor, CardRadioGroup, CardToggleGroup, CommandPalette, ConfirmAction, DataTable, DetailSectionGroup, DetailSection, DetailShell, EmbedInput, EmbedPreview, EmptyState, FilterToolbar, FormLayout, InlineListSection, DebugDialog, LogList, ListContainer, MarkdownEditor, PageLoading, MediaPicker, MediaBrowsePanel, MediaPreview, MediaThumbnail, PageHeader, PickerShell, RelationPicker, SelectionSummary, SidebarNav, MetricTile, ToastStack, ToastHost | 35 |
| Workstation systems | StatusBar, StatusIndicator, Surface, Text, TextLink, Table, TokenInput, TimeInput, TimeZoneSelect, ToggleGroup, Toolbar, Tooltip, TriStateSwitch, UiPresentationProvider, VideoPlayer, DateTimeZonePicker | 16 |
| Agent surfaces | AgentMessage, AgentPlanRecord, AgentQuestion, AgentQuestionRecord, AgentSubagent, ChangedFiles, ToolCall, ToolCallGroup | 8 |

Priority within each tranche: downstream-used components first (see the
roster's Downstream use column; Longhorn is the primary consumer).

## React Mirror Gaps

| Gap | Components | Owner |
| --- | --- | --- |
| No React implementation/export | AgentPlan, AgentPlanRecord | `g15.006` |
| No React gallery specimen | AgentMessage, AgentPlan, AgentPlanRecord, ChangedFiles, ToolCall, ToolCallGroup | `g15.006` |
| No focused React test (smoke only) | the React counterparts of the components in the Svelte focused-evidence tranches | `g15.006` (mirrors `g15.002`–`g15.005`) |

## Shared Rust Composition and GPUI Gaps

Rust declarations (`<Name>Spec`) and render modules (`poodle-render`) are
recorded independently; a component missing either is a native gap even when
the other exists.

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
ModelCatalogueEditor, MeterSurface) and 11 React components. Extending the
mounted proof across the roster is folded into the release-certification card.

## Carried Requirements (recorded, not implemented)

| Requirement | Status | Owner |
| --- | --- | --- |
| Licence native completion (`g14.017`) | web references approved and landed (`g14.015`/`g14.016`); native missing | `g15.007` |
| Model-connection native completion (`g14.020`) | web references approved and landed (`g14.018`/`g14.019`); native missing | `g15.008` |
| Human-centred specimen catalogue audit (`g14.026`) | rubric and boundary intact; unexecuted | `g15.011` |
| Primitive-first visual conformance lane | seam recorded in `conformance-estate.md`; harness not designed | `g15.012` |
| v0.2.0 release certification | after all Svelte-denominator blockers close | `g15.013` |

## Not Gaps

- Jetstream is program-deferred for every component; no component registers a
  Jetstream gap.
- 82 components have no downstream consumer use found. Absence of a consumer
  is not a release failure (handoff boundary); it is recorded as context in the
  roster.
- `effigy doctor` baseline findings (generated-in-src, god-file,
  stale-suppression, comment-ratio) and the pre-existing `bun audit` nanoid
  advisory are known board health, not component gaps.

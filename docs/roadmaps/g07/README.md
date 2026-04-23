# g07 GPUI Rendering Build-Out

Status: completed
Updated: 2026-03-14

> Historical note: references here to `poodle-primitives` and
> `poodle-composites` describe the crate layout at the time. The live component
> spec surface now ships from `poodle-specs` after `g10.004`.

## Context

`g06` restructured the shared Rust contract layer to be explicitly
multi-renderer: renamed crates, typed token resolution, layout intent
abstraction, event model, style descriptors, renderer adapter traits, and
expanded spec coverage to the full 124-component Svelte surface.

`g07` builds the GPUI rendering adapter — the code that takes Poodle's
renderer-agnostic spec structs, resolved styles, and semantic events and maps
them to GPUI's native element, styling, and event APIs. This generation also
brings GPUI up to parity with the g04-expanded Svelte surface, implements the
GPUI demo app, proves downstream adoption, and publishes documentation.

`g07` and `g08` (Jetstream build-out) can proceed in parallel since both
consume the shared contract layer from `g06` without modifying it.

## Starting State

- shared contract crates (`poodle-primitives`, `poodle-composites`, `poodle-workstation`,
  `poodle-tokens`) provide renderer-agnostic specs for all 124 components
- typed token resolution emits `[f32; 4]` colors, `f32` pixel values
- renderer adapter trait is defined and documented
- style descriptor IR captures resolved visual properties
- layout intent types map to both GPUI and Jetstream layout models
- semantic event model is defined

## Exit State

- GPUI rendering adapter crate (`poodle-gpui`) implements the adapter trait for
  all component categories
- GPUI implementations exist for all g04-added components where parity is
  appropriate
- GPUI demo app achieves parity with Svelte demo app
- cross-runtime parity evidence covers the full g04-expanded surface
- at least one meaningful GPUI downstream or reference-app proof exists
- published docs surface supports external evaluator review
- native-only deltas for new components are explicitly documented

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | GPUI adapter crate setup and theme integration | g06.015 | Foundation | Completed |
| 002 | GPUI structural and layout primitives | 001 | Core build | Completed |
| 003 | GPUI action, text-entry, and field primitives | 002 | Core build | Completed |
| 004 | GPUI selection, value, feedback, and temporal primitives | 003 | Core build | Completed |
| 005 | GPUI overlay, disclosure, navigation, and menu primitives | 004 | Core build | Completed |
| 006 | GPUI informational, code, color, and file primitives | 005 | Core build | Completed |
| 007 | GPUI form, validation, and remediation composites | 003-006 | Depth | Completed |
| 008 | GPUI data, browse, detail, and media composites | 007 | Depth | Completed |
| 009 | GPUI editing, navigation, and operational composites | 008 | Depth | Completed |
| 010 | GPUI workstation shell and layout updates | 009 | Workstation | Completed |
| 011 | Cross-runtime parity report and delta register | 010 | Hardening | Completed |
| 012 | GPUI demo-app parity implementation | 011 | Alignment | Completed |
| 013 | GPUI downstream reference-app adoption proof | 012 | Adoption | Completed |
| 014 | Published docs platform and evaluator onboarding | 012, 013 | Adoption | Completed |
| 015 | Generation closeout | 013, 014 | Closure | Completed |

## Dependency Shape

```text
g06.015 Shared Contracts Complete
  -> 001 GPUI Adapter Setup
      -> 002 Structural Primitives
          -> 003 Action / Input Primitives
              -> 004 Selection / Feedback Primitives
                  -> 005 Overlay / Navigation Primitives
                      -> 006 Informational / Code / Color Primitives
                          -> 007 Form Composites
                              -> 008 Data / Media Composites
                                  -> 009 Editing / Navigation Composites
                                      -> 010 Workstation Updates
                                          -> 011 Parity Report
                                              -> 012 Demo App
                                                  -> 013 Reference App
                                                  -> 014 Published Docs
                                                      -> 015 Closeout
```

## Execution Lanes

### Lane A: Core Rendering Adapter

`001 -> 002 -> 003 -> 004 -> 005 -> 006`

### Lane B: Composite and Workstation Depth

`007 -> 008 -> 009 -> 010`

### Lane C: Parity Evidence and Adoption

`011 -> 012 -> 013 -> 014 -> 015`

## Milestone Details

### 001 — GPUI Adapter Crate Setup and Theme Integration

Create `poodle-gpui` crate that implements the renderer adapter trait from g06.007.
Set up GPUI theme integration using g06.003's typed token values. Verify that
GPUI's styling API can consume resolved style descriptors from g06.006. This
crate replaces the old `poodle-gpui-*` re-export pattern with an actual rendering
implementation.

### 002–006 — Primitive Rendering Batches

Implement GPUI rendering for each primitive category. Each batch implements
the adapter trait for a set of spec structs, producing GPUI elements with
correct styling, layout, interaction, and accessibility.

**002 — Structural and layout (est. 8 components):**
Box, Stack, Grid, Surface, Separator, ScrollShell, Banner, CallOut

**003 — Action, text-entry, and field (est. 12 components):**
Button, IconButton, Field, TextInput, TextArea, SearchField, FormActions,
TimeField, EditableLabel, NumberEntry, PinInput, Toolbar

**004 — Selection, value, feedback, and temporal (est. 14 components):**
Checkbox, RadioGroup, Switch, Select, SegmentedControl, Slider, RangeSlider,
Progress, Badge, StatusIndicator, Meter, Rating, Skeleton, TriStateSwitch

**005 — Overlay, disclosure, navigation, and menu (est. 12 components):**
Accordion, Collapsible, Dialog, Drawer, Popover, Tooltip, HoverCard, Menu,
ContextMenu, Tabs, TabStrip, NavigationMenu, Menubar

**006 — Informational, code, color, file, and temporal (est. 10 components):**
Code, ColorPicker, FileUpload, Eyebrow, Pill, TimeAgo, DurationInput,
TimeZoneSelect, DateTimeZonePicker, DatePicker (and other date/time variants
if not covered in 004)

### 007–009 — Composite Rendering Batches

**007 — Form, validation, and remediation (est. 6 components):**
FormShell, ValidationSummary, RemediationBanner, InlineRemediation,
ConfirmAction, FormDialog

**008 — Data, browse, detail, and media (est. 12 components):**
DataTable, DetailShell, DetailSection, FilterToolbar,
PickerShell, RelationPicker, SelectionSummary, PaginationSummary,
MediaThumbnail, MediaPreview

**009 — Editing, navigation, list interaction, and operational (est. 16 components):**
AudioPlayer, VideoPlayer, MediaPicker, MarkdownEditor, EmbedInput,
EmbedPreview, EmbedShell, AutonomousList, ReorderableList, Breadcrumbs,
CardRadioGroup, InlineEditableField, ListCard, NavCard, NavCardGrid, OrderBy,
PageHeader, SlugField, LogList, PageLoading, StateTile, ToastStack

### 010 — Workstation Shell and Layout Updates

Update workstation GPUI implementations to use the new contract layer from g06.
All 12 workstation specs should already have implementations from g05; this
milestone migrates them to the new adapter pattern and verifies correctness.

### 011 — Cross-Runtime Parity Report and Delta Register

Update the parity matrix to cover the full g04-expanded surface. Document all
intentional deltas between Svelte and GPUI implementations. Verify visual and
behavioral parity for tier-1 (strict) and tier-2 (visual) requirements.

### 012 — GPUI Demo-App Parity Implementation

Implement the GPUI demo app against the explicit demo-app contract from g05.013.
Cover all 6 screen families. Side-by-side review with Svelte demo app. Document
remaining gaps.

### 013 — GPUI Downstream Reference-App Adoption Proof

Deploy GPUI components in a real downstream or reference application. Validate
multi-app consumption of the `poodle-gpui` crate. Document integration patterns
and any friction points.

### 014 — Published Docs Platform and Evaluator Onboarding

Publish component documentation for external review. Include Svelte and GPUI
examples, token reference, and integration guides. Support external evaluator
onboarding workflow.

### 015 — Generation Closeout

Verify all milestones complete. Document deferred items. Confirm g08 (Jetstream
build-out) can proceed independently.

## Next Task

All milestones complete. g08 (Jetstream Rendering Build-Out) can proceed.

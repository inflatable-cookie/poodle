# g04.001 Underlay Parity Audit, Gap Register, And Implementation Priority

Status: completed
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g03.014
Primary repos: `poodle`

## Goals

- [ ] formalize the Underlay component audit into a machine-readable gap register
- [ ] assign implementation priority (high, medium, lower) to each gap
- [ ] define layer assignment (primitive vs composite) for each new component
- [ ] identify which gaps require new contracts vs extensions to existing contracts
- [ ] record non-goals and Underlay-specific items that will not be ported

## Execution Checklist

- [ ] create `docs/specs/underlay-parity-gap-register.json` with structured
  entries for each identified gap
- [ ] classify each gap as new-component, feature-extension, or pattern-only
- [ ] assign target layer: `@poodle/svelte`
- [ ] define contract requirements: new contract, contract amendment, or
  implementation-only
- [ ] identify shared patterns that multiple new components will need (e.g.,
  drag-and-drop, rich text rendering)
- [ ] record app-specific Underlay surfaces that are out of scope (auth flows,
  Nightfire deep internals, Underlay-specific data patterns)

## Acceptance Criteria

- [ ] gap register covers all ~39 identified items from the Underlay audit
- [ ] each item has priority, layer, and contract classification
- [ ] non-goals are explicit
- [ ] implementation order across g04 milestones is consistent with the register

## Gap Register Summary

### High Priority New Components

| Component | Layer | Source Pattern |
|-----------|-------|---------------|
| AlertDialog | Primitive | Underlay `AlertDialog` (bits-ui based) |
| FileUpload | Primitive | Underlay `FileUpload` with drag-drop and preview |
| SplitButton | Primitive | Underlay `SplitButton` with dropdown actions |
| FormDialog | Composite | Underlay `FormDialog` combining Dialog + form |
| TimeAgo | Primitive | Underlay `TimeAgo` relative timestamp display |
| ReorderableList | Composite | Underlay `ReorderableList` with drag handles |
| CardRadioGroup | Composite | Underlay `CardRadioGroup` rich option cards |
| ConfirmAction | Composite | Underlay `ConfirmAction` with confirmation flow |

### High Priority Feature Extensions

| Component | Enhancement | Source |
|-----------|-------------|--------|
| TextInput | Async validation, prefix/suffix slots | Underlay `TextInput` |
| Skeleton | Data-shape presets (table, card, list) | Underlay `DataSkeleton` |

### Medium Priority New Components

| Component | Layer | Source Pattern |
|-----------|-------|---------------|
| Code | Primitive | Underlay `Code` syntax-highlighted display |
| ColorPicker | Primitive | Underlay `ColorPicker` with swatches and input |
| DurationInput | Primitive | Underlay `DurationInput` hours/minutes/seconds |
| ListCard | Composite | Underlay `ListCard` structured list item |
| NavCard | Composite | Underlay `NavCard` navigation-oriented card |
| NavCardGrid | Composite | Underlay `NavCardGrid` grid of navigation cards |
| OrderBy | Composite | Underlay `OrderBy` sort control toolbar |
| SlugField | Composite | Underlay `SlugField` auto-generated URL slug |
| LogList | Composite | Underlay `LogList` timestamped event display |
| PageLoading | Composite | Underlay `PageLoading` full-page loading state |
| InlineEditableField | Composite | Underlay `InlineEditableField` click-to-edit |

### Medium Priority Feature Extensions

| Component | Enhancement | Source |
|-----------|-------------|--------|
| StateTile | Trend indicators, sparklines | Underlay `StateTile` |
| DataTable | CSV export, column visibility toggles | Underlay `DataTable` |
| Card | Radio-selection mode, specialized variants | Underlay card patterns |
| Select | Grouped options with headings | Underlay `Select` |
| Field | Grid-spanning layout modes | Underlay `Field` |
| Button | Split variant (primary + dropdown) | Underlay `SplitButton` |

### Lower Priority New Components

| Component | Layer | Source Pattern |
|-----------|-------|---------------|
| MarkdownEditor | Composite | Underlay `MarkdownEditor` (EasyMDE based) |
| MediaPicker | Composite | Underlay `MediaPicker` asset selection |
| EmbedInput | Composite | Underlay `EmbedInput` URL-to-embed |
| EmbedPreview | Composite | Underlay `EmbedPreview` rich embed display |
| AudioPlayer | Composite | Underlay `AudioPlayer` audio playback |
| VideoPlayer | Composite | Underlay `VideoPlayer` video playback |
| RelationSelector | Composite | Underlay `RelationSelector` enhanced picker |
| AutonomousList | Composite | Underlay `AutonomousList` self-managing list |

### Exploration Only

| Component | Notes |
|-----------|-------|
| Block editor | Nightfire-informed; scope and feasibility TBD |

## Next Task

Open `g04.002` and begin implementing dialog and confirmation pattern
components.

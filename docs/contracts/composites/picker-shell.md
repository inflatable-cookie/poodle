# PickerShell

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `PickerShell`
- Layer: `composites`
- Summary: a reusable workflow shell for selecting one or more items from a
  searchable candidate set; provides layout framing, not item semantics
- In scope: title/description header, search toolbar slot, selected-summary
  region, result list body, state messaging, status announcements,
  confirm/cancel footer, inline/popover/modal posture
- Out of scope: domain-specific relation logic, fetch policy, item renderer
  semantics, destructive confirmation policy

## 2. Core Rule

`PickerShell` owns workflow framing, not item semantics.

Hosts still own:

- which candidates exist
- query execution
- selection state
- confirm/cancel consequences

## 3. Anatomy

```text
[Root]
  ├── [Header]
  │     ├── [Title]
  │     ├── [Description]  (optional)
  │     └── [Meta]
  │           ├── [ResultCount]    (optional)
  │           └── [SelectionCount]
  ├── [Status]             (optional, live region)
  ├── [Toolbar]            (slot, optional)
  ├── [Selection]          (slot, optional)
  ├── [Body]               (default slot, shown when state="ready")
  ├── [State]              (slot or fallback, shown when state!="ready")
  │     ├── [StateTitle]
  │     └── [StateMessage]  (optional)
  └── [Footer]             (slot, optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | picker container `<section>` | border-subtle, radius-surface, background-panel |
| Header | yes | title/description and meta row | layout only |
| Title | yes | `<h3>` heading | font-size 1.25rem |
| Description | no | `<p>` subheading | text-secondary, font-size 0.8125rem |
| Meta | yes | result count and selection count display | text-secondary, font-size 0.8125rem |
| Status | no | `role="status"` live region for screen readers | text-secondary |
| Toolbar | no | slot for search field and filters | layout only |
| Selection | no | slot for selection summary | layout only |
| Body | yes | default slot for candidate list (visible when state="ready") | layout only |
| State | yes | fallback state display (visible when state!="ready") | surface background, border-subtle |
| Footer | no | slot for confirm/cancel actions | layout only |

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | -- | yes | picker heading text |
| `description` | `string \| null` | `null` | no | subheading below title |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | workflow posture |
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | candidate-set posture; controls body vs state display |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the `<section>` |
| `resultCount` | `number \| null` | `null` | no | displayed in meta area as "{n} results" |
| `selectionCount` | `number` | `0` | no | displayed in meta area as "{n} selected" |
| `stateTitle` | `string \| null` | `null` | no | heading shown in state area when state!="ready" |
| `stateMessage` | `string \| null` | `null` | no | description shown in state area |
| `statusText` | `string \| null` | `null` | no | live-region status text for screen readers |
| `statusId` | `string \| null` | `null` | no | DOM id for the status element (for aria-describedby) |

### Slots

| Slot | Purpose | When Rendered |
|------|---------|---------------|
| `toolbar` | search field, filters, breadcrumbs | always (if provided) |
| `selection` | selection summary chips | always (if provided) |
| `state` | custom state content override | when `state!="ready"` and slot provided |
| `footer` | confirm/cancel actions | always (if provided) |
| default | candidate list / result content | when `state="ready"` |

### Controlled And Uncontrolled

- display/layout composite; all data is externally driven
- state prop controls which region is visible (body vs state area)

## 5. Variants

| Variant | Visual Behavior |
|---------|-----------------|
| `inline` | picker stays embedded in surrounding content |
| `popover` | compact transient chooser, `max-width: 30rem`, overlay elevation shadow |
| `modal` | focused selection task, dialog elevation shadow, elevated background |

Variant changes posture and layout emphasis. It does not change selection meaning.

## 6. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | body slot visible with candidate content |
| empty | `state="empty"` | state area shown with empty message |
| loading | `state="loading"` | state area shown with loading message |
| error | `state="error"` | state area shown with error message |
| no-results | `state="no-results"` | state area shown with no-results message |

### Component States

No internal state. PickerShell is a layout container.

## 7. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | PickerShell is a layout shell; events come from slotted children |

## 8. Accessibility

### Semantics

- Role: `<section>` landmark
- Optional attributes: `aria-label` for composite accessible name
- Status element uses `role="status"`, `aria-live="polite"`, `aria-atomic="true"`
- Status element has configurable `id` for `aria-describedby` references

### Keyboard

| Key | Behavior |
|-----|----------|
| none | PickerShell itself is non-interactive; keyboard handled by slotted children |

### Focus And Announcement

- focus entry: not directly focusable; focus goes to slotted toolbar/content
- live-region behavior: `statusText` announced via polite live region
- GPUI-native accessibility mapping notes: GPUI must preserve picker title,
  search grouping, candidate list semantics, selection summary, and
  confirm/cancel actions without relying on HTML dialog/popover defaults

## 9. Layout

### Sizing

- gap between sections: `--poodle-space-stack-md`
- padding: `--poodle-space-panel-y` / `--poodle-space-panel-x`
- header uses flex wrap with `justify-content: space-between`
- state area has additional padding (1.5x panel-y) and inner border

### Composition

- parent expectations: inline containers, popovers, modal dialogs
- child expectations: search fields, selection summaries, candidate lists,
  form action bars
- resizing rules: shell stretches to fill container; popover variant caps width

## 10. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-variant` | root `<section>` | `"inline"`, `"popover"`, `"modal"` |
| `data-state` | root `<section>` | `"ready"`, `"empty"`, `"loading"`, `"error"`, `"no-results"` |

### Root

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-md)` |
| padding | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| border | `0.0625rem solid var(--poodle-color-border-subtle)` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |

#### Variant: Popover (`[data-variant="popover"]`)

| Property | Value |
|----------|-------|
| max-width | `30rem` |
| box-shadow | `var(--poodle-elevation-overlay)` |

#### Variant: Modal (`[data-variant="modal"]`)

| Property | Value |
|----------|-------|
| box-shadow | `var(--poodle-elevation-dialog)` |
| background | `color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent)` |

### Header

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| justify-content | `space-between` |
| gap | `var(--poodle-space-inline-md)` |

### Title (h3)

| Property | Value |
|----------|-------|
| margin | `0` |
| font-size | `1.25rem` |
| line-height | `1.2` |

### Description, Meta, State Text, Status

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.5` |

### Status

| Property | Value |
|----------|-------|
| margin | `0` |

### Meta

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| gap | `var(--poodle-space-inline-sm)` |
| align-items | `baseline` |

### State Area

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-sm)` |
| padding | `calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x)` |
| border | `0.0625rem solid var(--poodle-color-border-subtle)` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent)` |

### Light Theme Overrides

None.

## 11. Svelte Notes

- root is `<section class="picker-shell">` with `data-variant` and `data-state` attributes
- uses Svelte `$$slots` checks for conditional slot rendering
- state fallback shows `stateTitle` and optional `stateMessage` when no `state` slot provided

## 12. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::picker_shell`
- implementation should preserve slot-equivalent regions as child containers

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] variant visual differences match (inline, popover, modal)
- [ ] state prop controls body vs state area visibility
- [ ] status live region semantics match

### Tier 2: Visual Parity

- [ ] surface treatment matches across variants
- [ ] typography hierarchy matches
- [ ] spacing and padding match token usage

### Tier 3: Implementation Freedom

- [ ] rendering internals and slot mechanism stay internal

## 14. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 15. Specimen Definitions

### Inline Variant (Ready)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Inline variant (ready) | `title="Select a component"`, `description="Browse and select from available components."`, `resultCount={12}`, `variant="inline"`, three Surface children as candidate items | Picker shell with title, description, result count, and three candidate rows visible in a constrained container |

### No Results

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| No results | `title="Select an item"`, `state="no-results"`, `stateTitle="No matches"`, `stateMessage="Try a different search term."`, `variant="inline"` | Picker shell showing empty state with "No matches" title and guidance message |

## 16. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: RelationPicker, media pickers, entity selectors
- future follow-up: build concrete workflows such as `RelationPicker` on top of
  `PickerShell` instead of redefining picker framing per feature

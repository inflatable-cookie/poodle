# PickerShell

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `PickerShell`
- Layer: `composites`
- Summary: a reusable workflow shell for selecting one or more items from a
  searchable candidate set; provides layout framing, not item semantics
- In scope: title/description header, meta area with result and selection counts,
  snippet-based toolbar and selection areas, result list body, state messaging
  with loading spinner, status live-region announcements, snippet-based footer,
  inline/popover/modal posture
- Out of scope: domain-specific relation logic, fetch policy, item renderer
  semantics, destructive confirmation policy

PickerShell owns workflow framing, not item semantics. Hosts still own which
candidates exist, query execution, selection state, and confirm/cancel
consequences.

## 2. Anatomy

```text
[Root <section>]
  ├── [Header]
  │     ├── [TitleBlock]
  │     │     ├── [Title <h3>]
  │     │     └── [Description <p>]  (optional)
  │     └── [Meta]
  │           ├── [ResultCount]    (optional)
  │           └── [SelectionCount]
  ├── [Toolbar]                (snippet, optional)
  ├── [Selection]              (snippet, optional)
  ├── [Status <p>]             (optional, visually hidden sr-only live region)
  ├── [Body]                   (children snippet, shown when state="ready"; scrollable)
  ├── [State]                  (shown when state!="ready")
  │     ├── [Spinner]          (when state="loading", fallback only)
  │     ├── [StateTitle <strong>]
  │     └── [StateMessage <p>] (optional)
  └── [Footer]                 (snippet, optional)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<section>` | Class `picker-shell`, `data-variant` and `data-state` attributes |
| Header | `<div>` | Flex row wrapping title block and meta |
| TitleBlock | `<div>` | Contains title and optional description |
| Title | `<h3>` | Picker heading text |
| Description | `<p>` | Optional subheading below title |
| Meta | `<div>` | Result count and selection count display |
| Toolbar | `<div>` | Snippet wrapper for search field and filters |
| Selection | `<div>` | Snippet wrapper for selection summary |
| Status | `<p>` | Visually hidden (sr-only); `role="status"`, `aria-live="polite"`, `aria-atomic="true"`; placed after toolbar and selection in DOM order |
| Body | `<div>` | Children snippet, visible only when `state="ready"`; scrollable (`overflow-y: auto`, `min-height: 0`) |
| State | `<div>` | Fallback state display, visible when `state!="ready"` |
| Spinner | `<span>` | Wraps `Spinner` primitive (`variant="grid"`, `tone="accent"`), shown in loading state fallback |
| Footer | `<div>` | Snippet wrapper for confirm/cancel actions |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | -- | yes | Picker heading text |
| `description` | `string \| null` | `null` | no | Subheading below title |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | Workflow posture |
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | Candidate-set posture; controls body vs state display |
| `ariaLabel` | `string \| null` | `null` | no | Accessible name for the `<section>` |
| `resultCount` | `number \| null` | `null` | no | Displayed in meta area as "{n} results" |
| `selectionCount` | `number` | `0` | no | Displayed in meta area as "{n} selected" |
| `stateTitle` | `string \| null` | `null` | no | Heading shown in state area when `state!="ready"`; defaults to "Picker state" when not provided |
| `stateMessage` | `string \| null` | `null` | no | Description shown in state area |
| `statusText` | `string \| null` | `null` | no | Live-region status text for screen readers |
| `statusId` | `string \| null` | `null` | no | DOM id for the status element (for `aria-describedby`) |

### Snippets

| Snippet | Purpose | When Rendered |
|---------|---------|---------------|
| `toolbar` | Search field, filters, breadcrumbs | Always (if provided) |
| `selection` | Selection summary chips | Always (if provided) |
| `stateContent` | Custom state content override | When `state!="ready"` and snippet provided |
| `footer` | Confirm/cancel actions | Always (if provided) |
| `children` | Candidate list / result content | When `state="ready"` |

### Controlled And Uncontrolled

Display/layout composite. All data is externally driven. The `state` prop
controls which region is visible (body vs state area).

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | Body snippet visible with candidate content |
| empty | `state="empty"` | State area shown with fallback title/message |
| loading | `state="loading"` | State area shown with Spinner (`variant="grid"`, `tone="accent"`) and loading message |
| error | `state="error"` | State area shown with error message |
| no-results | `state="no-results"` | State area shown with no-results message |

No internal component state. PickerShell is a layout container.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | -- | -- | PickerShell is a layout shell; events come from nested snippet content |

## 6. Accessibility

- Root is a `<section>` landmark with optional `aria-label`
- Status element uses `role="status"`, `aria-live="polite"`, `aria-atomic="true"`
- Status element has configurable `id` via `statusId` prop for `aria-describedby` references
- Keyboard interaction is delegated entirely to nested snippet content
- Focus entry goes to toolbar/content inside the supplied snippets, not the shell itself
- Spinner in loading state is `aria-hidden="true"`

## 7. Layout

### Sizing

- Root uses `display: grid` with `grid-template-rows: auto` and `gap: var(--poodle-space-stack-md)`
- Padding: `var(--poodle-space-panel-y) var(--poodle-space-panel-x)`
- Header uses `flex-wrap` with `justify-content: space-between`
- State area has 1.5x panel-y padding and inner border

### Composition

- Parent expectations: inline containers, popovers, modal dialogs
- Child expectations: search fields, selection summaries, candidate lists, form action bars
- Resizing rules: shell stretches to fill container; popover variant caps width at `30rem`

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-variant` | Root `<section>` | `"inline"`, `"popover"`, `"modal"` |
| `data-state` | Root `<section>` | `"ready"`, `"empty"`, `"loading"`, `"error"`, `"no-results"` |

### `.picker-shell` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-rows` | `auto` |
| `gap` | `var(--poodle-space-stack-md)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |

### Variant: Popover (`[data-variant="popover"]`)

| Property | Value |
|----------|-------|
| `max-width` | `30rem` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Variant: Modal (`[data-variant="modal"]`)

| Property | Value |
|----------|-------|
| `box-shadow` | `var(--poodle-elevation-dialog)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 96%, transparent)` |

### `.picker-shell__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |

### `.picker-shell__title`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `1.25rem` |
| `line-height` | `1.2` |

### `.picker-shell__description`

| Property | Value |
|----------|-------|
| `margin` | `0` |

### Description, Meta, State text, Status (shared)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

### `.picker-shell__status`

Status text is visually hidden using the `sr-only` pattern, but remains in the DOM for screen reader announcements.

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `position` | `absolute` |
| `width` | `1px` |
| `height` | `1px` |
| `padding` | `0` |
| `margin` | `-1px` |
| `overflow` | `hidden` |
| `clip` | `rect(0, 0, 0, 0)` |
| `white-space` | `nowrap` |
| `border` | `0` |

### `.picker-shell__body`

| Property | Value |
|----------|-------|
| `min-height` | `0` |
| `overflow-y` | `auto` |

### `.picker-shell__meta`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `baseline` |

### `.picker-shell__state` (State Area)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `justify-items` | `start` |
| `padding` | `calc(var(--poodle-space-panel-y) * 1.5) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent)` |

### `.picker-shell__spinner`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |

### State area title and message

| Property | Value |
|----------|-------|
| `margin` | `0` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Root is `<section class="picker-shell">` with `data-variant` and `data-state` attributes
- Uses snippet presence checks for conditional toolbar, selection, state, and footer rendering
- Status text is visually hidden (sr-only) but kept in the DOM for screen reader live region announcements
- Status element is placed after toolbar and selection snippets in the DOM order
- Body area is scrollable with `overflow-y: auto` and `min-height: 0`
- Grid uses `grid-template-rows: auto` instead of explicit row template
- State fallback shows `stateTitle` (or "Picker state") and optional `stateMessage` when no `stateContent` snippet is provided
- Loading state prepends shared `Spinner` primitive (`variant="grid"`, `tone="accent"`) before state title
- Imports `Spinner` from `@poodle/svelte`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::picker_shell`
- Implementation should preserve snippet-equivalent regions as child containers
- Status live region needs platform-appropriate accessibility announcement

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] variant visual differences match (inline, popover, modal)
- [ ] state prop controls body vs state area visibility
- [ ] status live region semantics match
- [ ] loading state shows spinner before state title

### Tier 2: Visual Parity

- [ ] surface treatment matches across variants
- [ ] typography hierarchy matches
- [ ] spacing and padding match token usage
- [ ] state area inner border and padding match

### Tier 3: Implementation Freedom

- [ ] rendering internals and snippet mechanism stay internal

## 12. Specimen Definitions

### Inline Variant (Ready)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Inline variant (ready) | `title="Select a component"`, `description="Browse and select from available components."`, `resultCount={12}`, `variant="inline"`, three Surface children as candidate items | Picker shell with title, description, result count, and three candidate rows visible in a constrained container |

### No Results

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| No results | `title="Select an item"`, `state="no-results"`, `stateTitle="No matches"`, `stateMessage="Try a different search term."`, `variant="inline"` | Picker shell showing state area with "No matches" title and guidance message |

# DetailShell

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `DetailShell`
- Layer: `composites`
- Summary: a reusable information-display shell for a single record, entity, or
  settings scope
- In scope: header region, scrollable detail body, empty/loading/error posture,
  state title and message, section stack
- Out of scope: domain-specific data fetching, editable form workflows,
  workstation panel chrome

## 2. Types

### BrowseState (subset)

DetailShell uses `Exclude<BrowseState, "no-results">`:

```ts
type DetailShellState = "ready" | "empty" | "loading" | "error";
```

## 3. Anatomy

```text
[Root Shell]  <section> aria-label
  ├── [Header Region]  (optional)
  │     └── slot:header  OR  <h2> title
  ├── [Body]  (when state="ready")
  │     └── slot:default
  └── [State Region]  (when state != "ready")
        └── slot:state  OR  default state content
              ├── <strong> stateTitle
              └── <p> stateMessage  (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Shell | yes | overall detail container | spacing |
| Header Region | no | page identity and top actions | spacing |
| Body | yes (when ready) | main content area | spacing |
| State Region | yes (when not ready) | empty, loading, or error content | spacing, background |

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | convenience shorthand rendered as `<h2>` when no header slot |
| `scrollMode` | `"shell" \| "body"` | `"body"` | no | who owns vertical scrolling; exposed as `data-scroll-mode` |
| `state` | `"ready" \| "empty" \| "loading" \| "error"` | `"ready"` | no | high-level content posture |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the section |
| `stateTitle` | `string \| null` | `null` | no | heading text for state region; falls back to "Detail state" |
| `stateMessage` | `string \| null` | `null` | no | body text for state region |

## 5. Slots

| Slot | Purpose | Fallback |
|------|---------|----------|
| `header` | custom header content (PageHeader, actions) | `<h2>{title}</h2>` when title is set |
| `state` | custom state region content | default `<strong>` + `<p>` state display |
| default | body content when `state="ready"` | none |

## 6. Events

No component-owned events.

## 7. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | body slot content visible |
| empty | `state="empty"` | state region replaces body |
| loading | `state="loading"` | state region replaces body |
| error | `state="error"` | state region replaces body |

The state region renders with `data-state` attribute reflecting the current
state value.

## 8. Accessibility

### Semantics

- Element: `<section>` with `aria-label` when provided
- Heading hierarchy: `<h2>` when using title prop fallback
- State region uses host-provided content or slot for custom accessibility

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters header actions, state content, and body content in logical order |
| scroll keys | operate on the documented scroll owner when focus enters it |

### Focus And Announcement

- The shell itself is not focusable by default
- State transitions should preserve or restore a sensible focus target when
  content changes
- GPUI-native accessibility mapping notes: GPUI must preserve named-region
  semantics, scroll ownership, and focus continuity when state changes

## 9. Layout

- Root: grid with `gap: --pug-space-stack-lg`
- Body and state regions: grid with `gap: --pug-space-stack-lg`
- State region: padded with doubled panel spacing, subtle background,
  `border-radius: --pug-radius-surface`

## 10. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Shell | `space-stack-lg` | section stacking |
| State Region | `background-panel`, `background-elevated`, `radius-surface` | state container |
| State message | `text-secondary`, `typography-body-*` | state body text |

## 11. Specimen Definitions

### Layout Structure

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Layout structure | header slot with colored Region, three body Regions (Section 1, 2, 3) | shell with distinct header region and stacked body sections, each shown as colored placeholder blocks |

### Multi-Section Layout With Header

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Multi-section layout with header | PageHeader with title, eyebrow, subtitle, Badge and Edit button; three DetailSections (General, Configuration with Reset action, Integrations) separated by Separators | complete detail page with identity header, action controls, and grouped detail rows across sections |

### Loading State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Loading state | `title="Loading"`, `state="loading"` | shell with loading/progress indicator replacing body content |

### Error State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `title="Error"`, `state="error"`, `stateTitle="Failed to load"`, `stateMessage="Something went wrong. Please try again."` | shell with error callout replacing body content, showing title and message |

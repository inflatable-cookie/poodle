# AppHeader

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `AppHeader`
- Layer: `composites`
- Summary: a global shell header for app identity, global actions, and
  window-level utility status
- In scope: app identity with title and subtitle, global action slots, optional
  utility indicators, drag-region posture, responsive collapse
- Out of scope: project-specific title/details, transport controls, timeline or
  mixer widgets

## 2. Anatomy

```text
[Root Header]  <header>
  ├── [Identity Region]
  │     └── [Title Group]  (when no identity slot)
  │           ├── <strong> title
  │           └── <span> subtitle  (optional)
  ├── [Actions Region]  (optional slot)
  └── [Utility Region]  (optional slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | top shell chrome | background, border, height |
| Identity Region | yes | app name/icon or custom identity slot | typography, icon, spacing |
| Title Group | no | default identity when no slot provided | title + subtitle layout |
| Actions Region | no | global shell actions | gap, action roles |
| Utility Region | no | connection/status indicators | text, status, spacing |

## 3. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | visible app title; ignored when `identity` slot is provided |
| `subtitle` | `string \| null` | `null` | no | secondary text shown alongside title in baseline alignment |
| `isDragRegion` | `boolean` | `false` | no | enables native window drag posture via `data-drag-region` attribute |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the header; falls back to `title` |

## 4. Slots

| Slot | Purpose | Fallback |
|------|---------|----------|
| `identity` | custom identity content (logo, branded element) | title/subtitle text |
| `actions` | primary global actions (buttons, menubar) | none |
| `utility` | trailing utility controls (icon buttons, status) | none |

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| standard | default | steady shell header with three-column grid |
| drag-region | `isDragRegion=true` | header supports window dragging where supported |
| collapsed | viewport <= 45rem | single-column layout; utility region left-aligned |

## 6. Events

No component-owned events. Child action behavior is host-owned.

## 7. Accessibility

### Semantics

- Element: `<header>` with `aria-label` (falls back to `title`)
- Drag-region behavior must not suppress or hide interactive controls from
  assistive technology

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches shell actions and utility controls in logical order |

### Focus And Announcement

- The header itself is not focusable by default
- Utility/status updates should not reorder global actions
- GPUI-native accessibility mapping notes: GPUI must preserve labeled header or
  toolbar structure even when integrated with native title-bar mechanics

## 8. Layout

### Default (>45rem)

- Three-column grid: `minmax(0, 1fr) auto auto`
- Gap: `--pug-space-inline-md`
- Min-height: `2.75rem`
- Padding: `0.375rem --pug-space-panel-x`
- Border-bottom: `0.0625rem solid --pug-color-border-subtle`

### Responsive (<=45rem)

- Single-column grid: `1fr`
- Utility region switches to `justify-content: flex-start`

### Composition

- Parent expectations: top-level workspace shell
- Child expectations: action clusters, status indicators, identity text/icon
- Resizing rules: identity remains stable while utility actions compress first

## 9. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | `background-panel`, `border-subtle` | shell chrome |
| Identity title | font-size `0.9375rem`, line-height `1.2` | app identity |
| Subtitle | `text-secondary`, font-size `0.75rem` | secondary text |
| Actions/Utility | `space-inline-sm` gap | control grouping |

## 10. Specimen Definitions

### Full App Window Header (Title + Menubar + Utility)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Full app window header | `title="Pug Studio"`, actions slot with inline Menubar (File, Edit, View, Help menus with shortcuts), utility slot with 3 ghost IconButtons (search, bell, settings) | Full-width header with app title, integrated menubar, and trailing utility icons; simulated app body area below |

### With Title, Actions, And Utility

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With title, actions, and utility | `title="My Application"`, actions slot with 2 ghost Buttons ("New", "Open"), utility slot with settings ghost IconButton | Header with title, action buttons in primary region, settings icon trailing |

### Title Only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Title only | `title="Pug Workstation"` | Minimal header displaying only the app title, no actions or utility controls |

### Custom Identity Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom identity slot | identity slot with custom logo badge ("P") and bold "Pug Studio" text, utility slot with bell and user ghost IconButtons | Header with custom branded identity region replacing default title, trailing utility icons |

# ActionDiscoveryPanel

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `ActionDiscoveryPanel`
- Layer: `composites`
- Summary: an inline discovery surface for suggested, recent, or scoped command
  groups; used standalone or composed inside `CommandPalette`
- In scope: grouped action sections, active-result tracking, keyboard navigation,
  loading/error/empty/no-results posture, badge and shortcut display
- Out of scope: global modal launch behavior, full ranking engines, persistence
  of recents, or app-specific command generation

## 2. Types

### CommandActionItem

```ts
type CommandActionItem = {
  id: string;
  title: string;
  description?: string | null;
  group?: string | null;
  shortcut?: string | null;
  keywords?: string[];
  badge?: string | null;
  isDisabled?: boolean;
};
```

### DiscoveryState

```ts
type DiscoveryState = "ready" | "loading" | "error" | "empty" | "no-results";
```

## 3. Anatomy

```text
[Root]  role="listbox"
  ├── [State Region]  (loading / error / empty / no-results)
  │     ├── [Skeleton Rows]  (loading)
  │     └── [EmptyState]     (error / empty / no-results)
  └── [Group...]  (ready)
        ├── [Eyebrow]  group heading
        └── [List]  <ul>
              └── [Item...]  <li> role="option" aria-selected
                    └── [ListCard]
                          ├── title
                          ├── subtitle  (description)
                          └── trailing slot
                                ├── [Badge]  (optional)
                                └── [Kbd]    (optional shortcut)
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | scrollable listbox container |
| Group | yes (when ready) | section wrapper with Eyebrow heading |
| List | yes (when ready) | unordered list of action items |
| Item | yes (when ready) | individual action option with ListCard |
| Skeleton Rows | yes (when loading) | 5 placeholder rows |
| EmptyState | yes (when error/empty/no-results) | contextual empty message |

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `CommandActionItem[]` | `[]` | no | full list of action items; grouped by `item.group` (defaults to `"Commands"`) |
| `state` | `DiscoveryState` | `"ready"` | no | controls which region renders |
| `activeId` | `string \| null` | `null` | no | currently highlighted item id; two-way bindable |
| `ariaLabel` | `string` | `"Actions"` | no | accessible label for the listbox root |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `itemSelect` | user clicks an item or calls `selectActive()` | `{ id: string }` |
| `activeChange` | active item changes via keyboard or mouse | `{ id: string \| null }` |

## 6. Exported Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `moveActive` | `(step: 1 \| -1) => void` | move active highlight up or down through enabled items, wrapping at boundaries |
| `moveToBoundary` | `(direction: "start" \| "end") => void` | jump active highlight to first or last enabled item |
| `selectActive` | `() => void` | dispatch `itemSelect` for the currently active item |
| `getEnabledItems` | `() => CommandActionItem[]` | return the current list of non-disabled items |

These methods are designed for parent components (such as `CommandPalette`) to
drive keyboard navigation from the outside.

## 7. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` and items present | grouped action list rendered |
| loading | `state="loading"` | 5 skeleton rows shown |
| error | `state="error"` | EmptyState with title "Could not load actions" |
| empty | `state="empty"` | EmptyState with title "No actions available" |
| no-results | `state="no-results"` | EmptyState with title "No matching actions", search variant |

## 8. Accessibility

### Semantics

- Root: `role="listbox"` with `aria-label`
- Each item: `role="option"` with `aria-selected` reflecting active state
- Disabled items are rendered but not navigable via `moveActive`

### Keyboard

Keyboard navigation is not handled internally. The parent component must call
the exported methods (`moveActive`, `moveToBoundary`, `selectActive`) in
response to keyboard events.

### Focus And Announcement

- The active item scrolls into view automatically via `scrollIntoView({ block: "nearest" })`
- Mouse enter and focus on a ListCard sets that item as active

## 9. Visual Rules

- Items grouped by `item.group` field; default group name is `"Commands"`
- Active item receives accent-tinted background with inset box-shadow
- Badge and shortcut (kbd) rendered in trailing slot with pill styling
- Shortcut uses monospace (`code-family`) typography
- Skeleton rows during loading: two Skeleton elements per row (48% and 20% width)

### Token Usage — Exact CSS Values

#### `.action-discovery-panel` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.75rem` |
| `min-height` | `0` |
| `overflow` | `auto` |
| `overscroll-behavior` | `contain` |

#### `.action-discovery-panel__group`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |

#### `.action-discovery-panel__list`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.25rem` |
| `margin` | `0` |
| `padding` | `0` |
| `list-style` | `none` |

#### `.action-discovery-panel__list li[aria-selected="true"] :global(.list-card)` (Active Item)

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 18%, var(--flint-color-background-elevated))` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-accent-base) 22%, transparent)` |

#### `.action-discovery-panel__trailing`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `0.375rem` |
| `align-items` | `center` |

#### `.action-discovery-panel__badge`, `.action-discovery-panel__kbd` (Shared)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-height` | `1.5rem` |
| `padding` | `0 0.5rem` |
| `border-radius` | `var(--flint-radius-control)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-surface) 76%, transparent)` |
| `color` | `var(--flint-color-text-secondary)` |
| `font-size` | `0.75rem` |

#### `.action-discovery-panel__kbd` (Additional)

| Property | Value |
|----------|-------|
| `font-family` | `var(--flint-typography-code-family)` |

#### `.action-discovery-panel__state`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--flint-space-stack-sm)` |

#### `.action-discovery-panel__skeletons`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--flint-space-stack-sm)` |

#### `.action-discovery-panel__skeleton-row`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `gap` | `var(--flint-space-inline-md)` |
| `padding` | `0.875rem` |
| `border-radius` | `calc(var(--flint-radius-surface) - 0.125rem)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-surface) 72%, transparent)` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `aria-selected` | `<li>` items | targets active item styling via `li[aria-selected="true"]` |

## 10. Composition

- Composes: `Eyebrow`, `ListCard`, `Skeleton`, `EmptyState`
- Used by: `CommandPalette`
- Items are filtered internally to derive `enabledItems` (items where `isDisabled` is falsy)

## 11. Specimen Definitions

### Grouped Actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Grouped actions | `items` with 7 actions across 3 groups (File: Save/Open File/Close Tab, Edit: Find in Files/Find and Replace, View: Toggle Terminal/Toggle Sidebar), each with shortcut hints | Grouped action list with section headings (File, Edit, View), action rows showing title and shortcut label |

### With Descriptions And Badges

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With descriptions and badges | `items` with 3 actions across 2 groups (CI/CD: Deploy to Production with description and "Dangerous" badge, Open Preview with description and shortcut; Tools: Run Linter with shortcut) | Grouped action list with description text below titles, badge pill on dangerous action, shortcut hints on applicable rows |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `items=[]`, `state="empty"` | Empty discovery posture with "No actions available" message |

# Breadcrumbs

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Breadcrumbs`
- Layer: `foundation`
- Summary: a compact hierarchical path navigation trail for product pages or
  detail surfaces
- In scope: path items, current-page indication, truncation/overflow posture,
  separators
- Out of scope: global navigation bars, history stacks, tab navigation

## 2. Anatomy

```text
[Root Nav]
  └── [Path List]
        └── [Path Item...]
              ├── [Link or Label]
              └── [Separator] (except last)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Nav | yes | breadcrumb navigation region | spacing |
| Path List | yes | ordered path container | gap, alignment |
| Path Item | yes | one hierarchy step | typography, text color |
| Separator | no | visual delimiter | icon, text-muted |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ value: string; label: string; href?: string; current?: boolean }>` | none | yes | hierarchy items |
| `ariaLabel` | `string` | `"Breadcrumb"` | no | navigation label |
| `maxVisibleItems` | `number \| null` | `null` | no | optional truncation threshold |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `on:navigate` | — | — | no | Svelte event: `dispatch("navigate", { value: item.value })` |

### Controlled And Uncontrolled

- declarative path model
- navigation may be link-driven or callback-driven

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | intermediate item | link-style item |
| current | `current=true` or last item | non-link current page indicator |
| truncated | path exceeds threshold | overflow treatment visible |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `navigate` | non-current item activates | `{ value: string }` | dispatched via Svelte `dispatch("navigate", { value: item.value })` |

## 6. Accessibility

### Semantics

- Role: navigation landmark with breadcrumb list semantics
- Required attributes: navigation label and current-page indication
- Optional attributes: overflow disclosure labeling when truncation exists
- Labeling rules: only one item may represent the current location

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through interactive path items in order |
| `Enter` or `Space` | activates focused path item when callback-driven |

### Focus And Announcement

- focus entry: only interactive path items participate in tab order
- focus exit: current location remains perceivable after link focus leaves
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose breadcrumb
  navigation as hierarchical path navigation with a current-location marker, not
  as decorative inline text

## 7. Layout

### Sizing

- path may wrap or truncate according to parent policy
- separators should remain compact and secondary

### Composition

- parent expectations: page headers, detail shells, nested settings views
- child expectations: path items only
- resizing rules: current page remains visible when truncation occurs
- composition rule: breadcrumbs provide hierarchy context before local page
  identity; they do not replace the page heading itself

## 8. Token Usage — Exact Values

### Root Nav `.breadcrumbs`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `font-size` | `var(--poodle-typography-body-size)` |

### Path Item (link)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Current Item

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Separator

| Property | Value |
|----------|-------|
| `opacity` | `0.4` |

### Size adjustments

| Size | font-size |
|------|-----------|
| `xs` | `0.75rem` |
| `sm` | `0.8125rem` |
| `md` | `var(--poodle-typography-body-size)` |
| `lg` | `0.9375rem` |
| `xl` | `1rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- expected substrate: semantic nav/list structure with links or buttons
- wrapper strategy: truncation mechanics stay internal as long as current-item
  semantics remain intact

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::breadcrumbs`
- implementation-only details: GPUI may render separators and overflow using
  native layout and menu surfaces, but path semantics and current location still
  need explicit mapping

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] navigation role and current-location semantics match
- [ ] item activation order and meaning match
- [ ] truncation preserves current-item accessibility in both runtimes

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] hierarchy emphasis and separator treatment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] overflow presentation stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overflow interaction may differ | runtime layout constraints differ | allowed | keep navigation meaning and current-item exposure strict |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Basic

A three-item breadcrumb trail with navigation callback:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Basic | items: Home > Projects > Poodle (current) | trail with link-style intermediate items and non-link current item; navigation callback reports clicked value |

### Deep path

A six-item breadcrumb trail with no truncation:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Deep path | items: Home > Workspace > Projects > Poodle Design System > Primitives > Button (current) | full trail visible with all intermediate links and current-page terminus |

### Collapsed (max 3 visible)

A six-item breadcrumb trail with truncation enabled:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Collapsed (max 3 visible) | same deep items, `maxVisibleItems=3` | overflow treatment visible; only three items shown with truncation indicator |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: nested settings, detail pages, catalog hierarchies
- future follow-up: add richer overflow breadcrumb menu if real adopters need it

## Next Task

Use `Breadcrumbs` above `PageHeader` title identity for hierarchical location
context, not as a replacement for tabs or shell navigation.

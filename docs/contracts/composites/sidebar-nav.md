# SidebarNav

Status: seed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `SidebarNav`
- Layer: `composites`
- Summary: grouped sidebar navigation list for catalogue, settings, inspector, and verification surfaces
- In scope: active-item state, optional section headings, grouped and ungrouped list posture, anchor or button items, compact sidebar presentation
- Out of scope: router ownership, page layout, breadcrumb trails, global shell toolbars, nested tree disclosure

## 2. Anatomy

```text
[Root Nav]
  ├── [Group]* 
  │     ├── [Group Title] (optional)
  │     └── [Item List]
  │            └── [Item Link or Button]*
```

| Part | Required | Description |
|------|----------|-------------|
| Root Nav | yes | semantic navigation region |
| Group | yes | optional separated section of related items |
| Group Title | no | small uppercase label distinguishing a section |
| Item List | yes | list container for navigation items |
| Item Link or Button | yes | selectable item representing a destination or local view |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `groups` | `SidebarNavGroup[]` | `[]` | yes | each group contains zero or more nav items |
| `value` | `string \| null` | `null` | no | currently active item value |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the navigation region |
| `size` | `ControlSize \| null` | `null` | no | explicit absolute sizing override |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | semantic size intent |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |

### Group Type

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `id` | `string` | yes | stable key |
| `label` | `string \| null` | no | optional visual group title |
| `items` | `SidebarNavItem[]` | yes | items rendered in order |

### Item Type

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | stable active key |
| `label` | `string` | yes | visible item label |
| `href` | `string \| null` | no | when present, renders an anchor |
| `disabled` | `boolean` | no | disabled items render inertly |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| plain list | one untitled group | items render as one continuous list without extra group chrome |
| grouped | multiple groups or titled group | each group reads as a distinct section through spacing and separators |
| active | item value matches `value` | active item shows emphasized fill and accent rail |
| disabled | item `disabled` | reduced emphasis, no activation |
| compact density | `density="compact"` | tighter row padding, tighter title spacing, and stronger tracking |
| comfortable density | `density="comfortable"` | looser row padding, looser title spacing, and gentler tracking |
| size scaling | `size` or presentation context changes | item height, item font size, and title size scale together |

## 5. Events

| Event | Detail | When |
|-------|--------|------|
| `valueChange` | `{ value: string }` | user activates an item |

## 6. Accessibility

- Root is a semantic `nav` region.
- `ariaLabel` should be provided whenever surrounding context does not already label the navigation clearly.
- Active items should expose `aria-current="page"` regardless of anchor or button rendering.
- Keyboard interaction follows native link/button behavior; the component does not implement roving focus or composite-menu semantics.

## 7. Layout

- Intended for narrow sidebar columns and stacked verification/catalogue rails.
- Single untitled groups should read as one continuous list.
- Titled or multiple groups should visually separate related item clusters through spacing and separators rather than card framing.
- Item content should wrap cleanly for long titles rather than forcing horizontal scroll.
- Group titles should read as structural metadata, not selectable rows: uppercase, smaller than items, accent-colored, and heavier.

## 8. Token Usage

- size should resolve through the shared presentation context when `size` is omitted
- density should resolve through the shared presentation context when `density` is omitted
- item height, item font size, item padding, title font size, title spacing, and title tracking should all vary with the resolved size/density posture
- active fill should use subtle accent mixing rather than solid accent blocks
- active state should combine a subtle fill with a clear accent rail rather than a heavy card-like selected container
- grouped sections should use spacing and separator treatment strong enough to distinguish adjacent sections without introducing card-like containers

## 9. Composition Notes

- `SidebarNav` is the preferred grouped-sidebar list for Poodle preview catalogue sections and Loophole verification surfaces.
- Do not use it for hierarchical tree navigation; use a dedicated disclosure/tree surface instead.
- Do not duplicate local sidebar list CSS when this component can own the interaction and grouping treatment.

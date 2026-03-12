# BulkActionBar

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `BulkActionBar`
- Layer: `composites`
- Summary: a selection-aware action surface that appears when multiple rows or
  items are selected
- In scope: selection summary, action cluster, clear-selection affordance
- Out of scope: selection model ownership, destructive confirmation dialogs,
  domain-specific bulk workflows

## 2. Anatomy

```text
[Root]
  ├── [Selection Summary]
  └── [Action Cluster]
        ├── [Bulk Action...]
        └── [Clear Selection]
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `selectionCount` | `number` | `0` | yes | selected visible or scoped item count |
| `totalCount` | `number \| null` | `null` | no | optional visible-scope count |
| `actions` | `Array<{ id: string; label: string; tone?: "default" \| "danger" }>` | `[]` | no | visible bulk actions |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| hidden | no selection | bar usually absent |
| active | one or more items selected | summary and actions visible |
| destructive-ready | danger-toned action present | destructive affordance visible but not yet confirmed |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onAction` | bulk action requested | `{ id }` | host owns action execution |
| `onClear` | clear selection invoked | none | host clears current selection |

## 6. Accessibility

- Role: labeled region or group
- Required behavior: selection summary must be textual and explicit
- keyboard rule: actions participate in normal tab order; the bar itself is not
  focusable
- GPUI-native accessibility mapping notes: GPUI must preserve summary text,
  action order, and clear-selection naming without relying on HTML toolbar
  defaults

## 7. Composition

- parent expectations: `DataTable`, list selection surfaces, picker composites
- child expectations: bulk actions only; execution and confirmation stay
  host-owned

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | accent/background/surface roles | selection shell emphasis |
| Summary | body typography and text roles | visible count |
| Actions | control and focus roles | action affordances |

## 9. Svelte Notes

- simple flex or stack layout is sufficient
- do not hide the selection summary behind icon-only affordances

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::bulk_action_bar`
- GPUI implementation must preserve the summary/action grouping and action order

## 11. Parity Checklist

- [ ] selection summary meaning matches
- [ ] action ordering and clear-selection affordance match
- [ ] danger-toned actions are visually and semantically distinct in both runtimes

## Next Task

Use `BulkActionBar` only when there is real selected scope to act on, rather
than as a permanent second toolbar.

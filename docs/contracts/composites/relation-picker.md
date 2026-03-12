# RelationPicker

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `RelationPicker`
- Layer: `composites`
- Summary: a selection workflow for attaching or choosing related entities, assets, presets, or references
- In scope: candidate browse, search, single or multiple selection, selected summary, confirm/cancel flow, inline/popover/modal posture
- Out of scope: graph semantics, persistence, authorization, domain-specific validation rules

## 2. Anatomy

```text
[PickerShell]
  ├── [Search Toolbar]
  ├── [Selection Summary]
  ├── [Candidate List]
  │     └── [Candidate Row...]
  └── [Confirm / Cancel Footer]
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ id: string; label: string; description?: string; meta?: string }>` | none | yes | available candidates |
| `selectedIds` | `string[]` | `[]` | no | controlled selection |
| `query` | `string` | `""` | no | controlled search query |
| `selectionMode` | `"single" \| "multiple"` | `"multiple"` | no | selection semantics |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | workflow posture |
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | candidate-set posture |

## 4. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onQueryChange` | query changes | `{ value }` | host owns filtering or remote search |
| `onSelectionChange` | selection changes | `{ selectedIds }` | host owns final state |
| `onConfirm` | confirm requested | `{ selectedIds }` | host commits relation change |
| `onCancel` | cancel requested | none | host decides whether selection resets |

## 5. Accessibility

- candidate list must remain searchable and selectable from the keyboard
- candidate movement should support adjacent navigation and boundary jumps where
  results are ordered
- `single` and `multiple` selection meaning must remain explicit
- multi-select checkbox workflows should not pretend to be listbox semantics if
  the interaction model is checkbox-driven instead
- selected-summary region must remain readable and removable
- confirm/cancel actions must remain reachable after candidate browsing
- GPUI-native accessibility mapping notes: GPUI must expose candidate list, selection state, selected-summary items, and confirm/cancel actions as a coherent workflow rather than unrelated controls

## 6. Composition Guidance

- use `single` when choosing one relation target
- use `multiple` when attaching several related entities or assets
- use `inline` when the picker is part of a larger form or detail flow
- use `popover` for compact transient selection
- use `modal` when the selection task needs focused workspace and explicit commit/cancel rhythm

## 7. Next Task

Use `RelationPicker` for attach-or-choose workflows and keep domain-specific relation validation above the composite layer.

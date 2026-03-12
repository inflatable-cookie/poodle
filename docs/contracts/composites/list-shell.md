# ListShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ListShell`
- Layer: `composites`
- Summary: a reusable shell for vertically listed content with optional filter
  controls, state handling, and scroll ownership
- In scope: list header area, scroll boundary, empty/loading/error posture,
  summary slots
- Out of scope: row virtualization, row selection logic, domain-specific item
  rendering

## 2. Anatomy

```text
[Root Shell]
  ├── [Header Region] (optional)
  ├── [State Region] (optional)
  └── [List Viewport]
        └── [List Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Shell | yes | browse shell | spacing, surface |
| Header Region | no | filter toolbar, summary, actions | spacing, separator |
| State Region | no | empty/loading/error region | spacing |
| List Viewport | yes | list-owned scroll region | scroll, surface, border |
| List Content | yes | row container | spacing, separators |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | collection posture |
| `ariaLabel` | `string \| null` | `null` | no | label for the list region when needed |
| `itemCount` | `number \| null` | `null` | no | optional summary metadata |
| `scrollMode` | `"shell" \| "list"` | `"list"` | no | scroll ownership |

### Controlled And Uncontrolled

- declarative browse shell
- row data, selection, and actions remain host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | list content visible |
| empty | `state="empty"` | empty-state region visible |
| no-results | `state="no-results"` | query/filter-specific empty posture visible |
| loading | `state="loading"` | progress or skeleton state visible |
| error | `state="error"` | callout/banner state visible |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onScroll` | viewport scrolls | framework-native event | optional passthrough |

## 6. Accessibility

### Semantics

- Role: labeled list region or neutral browse section depending on child list
  semantics
- Required attributes: accessible label when the list is an addressable browse
  destination
- Optional attributes: summary description and state-region associations
- Labeling rules: `ListShell` must not override child list/item semantics; it
  wraps them

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters header controls and list content in logical order |
| scroll keys | operate on the documented scroll owner when focus enters it |

### Focus And Announcement

- focus entry: the shell itself is not focusable by default unless it owns a
  keyboard-reachable scroll destination
- focus exit: state transitions should preserve focus on active controls or a
  reasonable fallback
- live-region behavior: item-count and state changes may be announced only when
  the host decides they materially affect browse results
- GPUI-native accessibility mapping notes: GPUI must preserve labeled browse
  region semantics and avoid flattening list state, empty state, and scroll
  ownership into one inaccessible custom canvas
- distinction rule: `empty` means no collection exists yet; `no-results` means
  the current query/filter scope produced zero matches

## 7. Layout

### Sizing

- shell fills available width and assigned height
- viewport may own scrolling or delegate to the shell based on `scrollMode`

### Composition

- parent expectations: settings lists, libraries, inspectors, entity browsers
- child expectations: `FilterToolbar`, `ScrollShell`, `EmptyState`, row
  composites, progress/skeleton/callout primitives, and optional
  `PaginationSummary`
- resizing rules: header remains visually separate from scrolling content
- loading guidance: progressive loading may append rows below the current
  viewport without changing the shell contract; exact load-more policy remains
  host-owned

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Shell | spacing and surface roles | browse frame |
| Header Region | separator and spacing roles | controls |
| State Region | spacing roles | transient state display |
| List Viewport | `ScrollShell`, border, and surface roles | content viewport |
| List Content | separator and spacing roles | row cadence |

## 9. Svelte Notes

- expected substrate: `Stack`, `ScrollShell`, `FilterToolbar`, and row children
- wrapper strategy: keep row semantics external so lists can be plain lists,
  selectable lists, or richer composites without changing the shell contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::list_shell`
- implementation-only details: GPUI may use native list or scroll views, but
  labeled-region semantics, state posture, and focus continuity remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] state posture, including `empty` versus `no-results`, matches
- [ ] shell labeling and child-list neutrality match
- [ ] focus continuity across state changes matches

### Tier 2: Visual Parity

- [ ] header/body separation and row cadence use comparable token roles

### Tier 3: Implementation Freedom

- [ ] viewport internals and virtualization strategy stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native list behavior may differ internally | runtime list primitives differ | allowed | keep shell semantics and child neutrality strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings browsers, entity lists, compact inspectors
- future follow-up: pair with richer table/data-grid composites later if needed

## Next Task

Use `ListShell` when progressive loading and contextual continuity matter more
than fixed page ranges, while keeping row semantics and selection models
host-owned or specialized elsewhere.

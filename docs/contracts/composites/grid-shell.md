# GridShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `GridShell`
- Layer: `composites`
- Summary: a reusable shell for card- or tile-based collections with optional
  filter controls and collection-state handling
- In scope: grid header area, responsive card-grid region, empty/loading/error
  posture, scroll ownership
- Out of scope: masonry algorithms, drag-and-drop layout, domain-specific tile
  rendering

## 2. Anatomy

```text
[Root Shell]
  ├── [Header Region] (optional)
  ├── [State Region] (optional)
  └── [Grid Viewport]
        └── [Grid Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Shell | yes | browse shell | spacing, surface |
| Header Region | no | filter toolbar, summary, actions | spacing, separator |
| State Region | no | empty/loading/error region | spacing |
| Grid Viewport | yes | grid-owned scroll region | scroll, surface |
| Grid Content | yes | card/tile container | gap, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | collection posture |
| `ariaLabel` | `string \| null` | `null` | no | label for the grid region when needed |
| `itemCount` | `number \| null` | `null` | no | optional summary metadata |
| `minColumnWidth` | `"sm" \| "md" \| "lg"` | `"md"` | no | semantic column density hint |
| `scrollMode` | `"shell" \| "grid"` | `"grid"` | no | scroll ownership |

### Controlled And Uncontrolled

- declarative browse shell
- card/tile data and interactions remain host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | grid content visible |
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

- Role: labeled grid/list region or neutral browse section depending on child
  tile semantics
- Required attributes: accessible label when the grid is an addressable browse
  destination
- Optional attributes: summary description and state-region associations
- Labeling rules: `GridShell` should not force ARIA grid semantics when the
  children are simply cards; child semantics own item roles

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters header controls and grid content in logical order |
| scroll keys | operate on the documented scroll owner when focus enters it |

### Focus And Announcement

- focus entry: the shell itself is not focusable by default unless it owns a
  keyboard-reachable scroll destination
- focus exit: card or tile focus order should remain stable under responsive
  layout changes
- live-region behavior: collection-state changes may be announced by the host
  when materially relevant
- GPUI-native accessibility mapping notes: GPUI must preserve labeled browse
  region semantics and should not misrepresent a card grid as a spreadsheet-like
  control unless the child semantics actually require it
- distinction rule: `empty` means no underlying collection exists; `no-results`
  means the active query/filter scope matched nothing

## 7. Layout

### Sizing

- shell fills available width and assigned height
- grid columns respond to container width according to semantic density hints

### Composition

- parent expectations: content browsers, template galleries, asset pickers,
  overview pages
- child expectations: `FilterToolbar`, `Card`, `EmptyState`, progress/skeleton
  primitives, `PaginationSummary`, and host-owned tile/card children
- resizing rules: item focusability and reading order remain stable as columns
  reflow
- pagination guidance: grid shells often pair more naturally with pagination
  when users need explicit range awareness; exact page policy stays host-owned

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Shell | spacing and surface roles | browse frame |
| Header Region | separator and spacing roles | controls |
| State Region | spacing roles | transient state display |
| Grid Viewport | `ScrollShell` and surface roles | content viewport |
| Grid Content | gap and alignment roles | tile cadence |

## 9. Svelte Notes

- expected substrate: `Stack`, `Grid`, `ScrollShell`, `FilterToolbar`, and
  card/tile children
- wrapper strategy: responsive track calculation is implementation detail as
  long as semantics and order remain stable

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::grid_shell`
- implementation-only details: GPUI may use native wrapping layout or custom
  tile views, but label, state, and focus-continuity semantics remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] state posture, including `empty` versus `no-results`, matches
- [ ] shell labeling and child-grid neutrality match
- [ ] focus continuity under responsive reflow matches

### Tier 2: Visual Parity

- [ ] grid cadence, spacing, and state-region hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] responsive track algorithm stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact reflow breakpoints may differ | runtime measurement differs | allowed | keep order and semantic labeling strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: library browsers, overview pages, asset galleries
- future follow-up: pair with picker and relation workflows later if needed

## Next Task

Use `GridShell` for card/tile browsing where paged result ranges remain useful,
and keep domain-specific tile behavior outside the generic composite contract.

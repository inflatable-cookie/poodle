# Card

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Card`
- Layer: `composites`
- Summary: a generic information container that groups related content, summary
  metadata, and optional actions into a readable surface
- In scope: header/body/footer structure, optional media slot, clickable vs
  non-clickable posture, emphasis variants
- Out of scope: collection layout, navigation shell ownership, app-specific
  record fields

## 2. Anatomy

```text
[Root Card]
  ├── [Media] (optional)
  ├── [Header] (optional)
  ├── [Body]
  └── [Footer Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Card | yes | grouping surface | background, border, radius, elevation |
| Media | no | preview or visual summary | radius, aspect spacing |
| Header | no | title, metadata, badges, utility actions | typography, spacing |
| Body | yes | main informational content | spacing, text color |
| Footer Actions | no | related actions | spacing, separator, action roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"default" \| "outlined" \| "elevated"` | `"default"` | no | semantic visual emphasis |
| `isInteractive` | `boolean` | `false` | no | card behaves like a clickable summary surface |
| `hasMedia` | `boolean` | `false` | no | reserves media region |
| `ariaLabel` | `string \| null` | `null` | no | label required when no visible title exists and the card is addressable |
| `onPress` | `() => void` | none | no | only when interactive |

### Controlled And Uncontrolled

- largely declarative shell component
- `isInteractive` changes input semantics and focusability expectations

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | base variant | neutral grouped surface |
| elevated | `variant="elevated"` | stronger elevation treatment |
| outlined | `variant="outlined"` | stronger boundary emphasis |
| summary metric | compact summary content | stronger value emphasis within body content |
| interactive | `isInteractive=true` | hover and focus affordances present |
| disabled | host-owned disabled posture | muted non-interactive card when used |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onPress` | interactive card activates | none | only when `isInteractive=true` |

## 6. Accessibility

### Semantics

- Role: usually `article`, `group`, `button`, or neutral container depending on
  whether the card is addressable or interactive
- Required attributes: accessible name when the card is interactive or
  explicitly navigable
- Optional attributes: description relationship and region labeling when used
  as a named summary section
- Labeling rules: if a visible title exists, it should usually supply the
  accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches interactive card or internal actions in logical order |
| `Enter` or `Space` | activates the card when the card itself is interactive |

### Focus And Announcement

- focus entry: non-interactive cards do not become focusable by default
- focus exit: focus treatment clears without changing grouped content semantics
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must keep grouped-content
  semantics distinct from button semantics and expose a meaningful accessible
  name when the card is interactive or addressable

## 7. Layout

### Sizing

- card may size intrinsically or fill parent track width
- media and body spacing should remain stable across content lengths

### Composition

- parent expectations: list/grid results, dashboards, settings overviews,
  summary panes
- child expectations: body content may include foundation text, badges, and
  action primitives
- resizing rules: footer actions should not collapse body readability
- summary/detail rule: cards can summarize a detail destination, but they do
  not replace structured label/value rows when metadata scanning is primary

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Card | `Surface` and semantic border/elevation roles | shell |
| Header and Body | spacing and typography roles | content hierarchy |
| Footer Actions | separator and action roles | trailing controls |
| Interactive emphasis | focus and selected/accent roles | hover/focus treatment |

## 9. Svelte Notes

- expected substrate: composition of `Surface`, `Stack`, `Inline`, and button
  primitives when interactive
- wrapper strategy: card semantics stay Pug-owned even if internal slots are
  framework-driven

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::card`
- implementation-only details: GPUI may use native view grouping and theme
  helpers rather than HTML landmarks, but interactive posture and naming still
  need explicit mapping

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] grouped vs interactive card semantics match
- [ ] naming and focusability rules match
- [ ] activation behavior matches when interactive

### Tier 2: Visual Parity

- [ ] elevation, boundary, and spacing hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] media rendering and internal slot mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact hover animation may differ | runtime animation mechanics differ | allowed | keep grouping and activation semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings overview cards, library/result cards, detail
  summaries
- future follow-up: pair with richer result-card and record-card composites if
  real adopters need stronger structure

## Next Task

Use `Card` as a grouped summary surface alongside `DetailSection` and
`DetailRow`, not as a catch-all replacement for structured readonly detail.

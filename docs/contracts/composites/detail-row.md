# DetailRow

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `DetailRow`
- Layer: `composites`
- Summary: a labeled information row for readonly detail or settings-display
  content
- In scope: label/value pairing, optional supporting description, optional
  inline action, value truncation posture
- Out of scope: editable form fields, table virtualization, domain-specific
  record rendering

## 2. Anatomy

```text
[Root Row]
  ├── [Label Block]
  │     ├── [Label]
  │     └── [Description] (optional)
  ├── [Value]
  └── [Action] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Row | yes | row container | spacing, separator |
| Label Block | yes | label and optional description | typography, text color |
| Value | yes | readonly value display | typography, text emphasis |
| Action | no | trailing related action | gap, icon/action roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | none | yes | row label |
| `description` | `string \| null` | `null` | no | supporting context |
| `value` | `string \| null` | `null` | no | simple value shorthand |
| `truncateValue` | `boolean` | `false` | no | allow truncation posture |
| `ariaLabel` | `string \| null` | `null` | no | optional row label override when necessary |

### Controlled And Uncontrolled

- declarative information row
- richer value and action content remain host-owned children

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| simple | label and value only | single-row display |
| descriptive | description present | expanded label block |
| actionable | trailing action present | value and action alignment retained |
| empty | no visible value content | placeholder or muted empty treatment |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond optional child action behavior.

## 6. Accessibility

### Semantics

- Role: usually description-list row, group, or neutral row depending on parent
  composite
- Required attributes: label/value relationship must remain perceivable
- Optional attributes: description relation and action labeling when present
- Labeling rules: row labels should not rely only on visual column position

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches any interactive trailing action in logical order |

### Focus And Announcement

- focus entry: non-interactive values do not become focus targets
- focus exit: trailing action focus must not obscure the label/value relation
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve label/value
  relationships explicitly, especially when rendering detail rows as custom
  layout rather than a native form or table control

## 7. Layout

### Sizing

- label and value columns may stack at narrow widths
- long values may wrap or truncate according to host policy

### Composition

- parent expectations: detail sections, settings summaries, inspector metadata
- child expectations: concise readonly values and optional actions
- resizing rules: labels remain readable even when values are long
- composition rule: detail rows are for readonly label/value structure, not
  editable field semantics or summary-card highlights

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Row | spacing and separator roles | row structure |
| Label Block | subdued typography and text roles | metadata labeling |
| Value | stronger text roles | value emphasis |
| Action | action spacing and icon roles | trailing affordance |

## 9. Svelte Notes

- expected substrate: `Inline`, `Stack`, `Separator`, and button/text
  primitives
- wrapper strategy: description-list semantics should be used when parent
  composition supports them cleanly

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::detail_row`
- implementation-only details: GPUI may represent the row as grouped labeled
  text plus optional action, but accessible label/value mapping remains required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label/value semantics match
- [ ] empty-value treatment meaning matches
- [ ] action ordering and labeling match when present

### Tier 2: Visual Parity

- [ ] label/value hierarchy and spacing use comparable token roles

### Tier 3: Implementation Freedom

- [ ] stack-vs-columns breakpoint behavior stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| line wrapping may differ | text metrics differ | allowed | keep label/value relationship strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings summaries, inspector details, metadata panes
- future follow-up: pair with editable detail/form rows in later form-system
  milestones

## Next Task

Use `DetailRow` for readonly information display inside `DetailSection`, and
keep editable input rows in the form-system roadmap.

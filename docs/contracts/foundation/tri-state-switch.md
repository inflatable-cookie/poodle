# Tri-State Switch

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `TriStateSwitch`
- Layer: `foundation`
- Summary: a ternary selection control for explicit exclude/default/include
  style flows
- In scope: three distinct states, directional movement, labeled state options
- Out of scope: binary on/off toggles, arbitrary multi-option segmentation

## 2. Anatomy

```text
[Root]
  └── [Option Segment...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | segmented ternary host | border, spacing, focus context |
| Option Segment | yes | one of three mutually exclusive choices | background, text, selected state |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `"excluded" \| "default" \| "included"` | `"default"` | no | controlled ternary state |
| `options` | optional fixed triple metadata | built-in | no | labels may be overridden but semantics stay ternary |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string \| null` | none | yes | names the ternary control |
| `onValueChange` | `(value) => void` | none | no | selection callback |

### Controlled And Uncontrolled

- controlled by default in this baseline contract
- uncontrolled mode can be added later if needed, but semantics remain ternary

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| excluded | value selected | negative segment selected |
| default | value selected | neutral segment selected |
| included | value selected | positive segment selected |
| focus | control or segment focused | visible focus treatment |
| disabled | `isDisabled=true` | muted non-interactive state |

### Component States

Roving-focus and single-selected-option state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | selected segment changes | ternary value | one selected segment at a time |

## 6. Accessibility

### Semantics

- Role: radio-group-like semantics with three exclusive options
- Required attributes: accessible name for the group and per-option selected
  state
- Optional attributes: per-option accessible labels when visible glyphs are
  symbolic
- Labeling rules: symbol-only options must still expose meaningful accessible
  names such as excluded/default/included

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves selection/focus across segments |
| `Space` | selects the focused segment |
| `Tab` | enters or exits the control through one tabbable segment |

### Focus And Announcement

- focus entry: one segment participates in the tab sequence
- focus exit: current segment remains selected and roving focus is preserved
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose exclusive-choice
  semantics and meaningful names for symbolic segments, not just their glyphs

## 7. Layout

### Sizing

- three segments share a stable common height
- segment widths may auto-fit or equalize according to the implementation, but
  selection meaning must remain stable

### Composition

- parent expectations: filters, policy overrides, inclusion/exclusion controls
- child expectations: fixed ternary option set
- resizing rules: segments remain visually grouped as one exclusive-choice
  control

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | border/background roles and control radius | shell |
| Segments | text/background roles | default/unselected state |
| Selected segment | accent/status roles | current ternary state emphasis |
| Focus ring | accent focus roles | focus |
| Disabled | state opacity roles | disabled treatment |

## 9. Svelte Notes

- can wrap a segmented/radio-group substrate, but the public contract owns the
  ternary semantics and accessible labels

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::tri_state_switch`
- GPUI implementation must preserve symbolic-option accessible labeling and
  exclusive-choice keyboard semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] ternary value meaning matches
- [ ] symbolic segment accessible labels match
- [ ] directional navigation and roving focus match

### Tier 2: Visual Parity

- [ ] selected-state emphasis uses comparable semantic roles

### Tier 3: Implementation Freedom

- [ ] segment sizing and animation details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| equal-width vs content-width segments may differ | layout internals are runtime-specific | allowed | keep exclusivity and labels strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: rule filters, inclusion/exclusion workflows
- future follow-up: coordinate with filter bars and relation pickers later

## Next Task

Use `TriStateSwitch` when three-state policy semantics are real, not as a
general-purpose segmented control substitute.

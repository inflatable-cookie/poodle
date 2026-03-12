# Select

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Select`
- Layer: `foundation`
- Summary: a single-select control with a trigger, list of options, and
  selected-value presentation
- In scope: selected value, trigger semantics, option list semantics, disabled
  options, placeholder behavior
- Out of scope: arbitrary menu content, multi-select tagging, relation-picker
  workflows

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Value or Placeholder]
  │     └── [Indicator]
  └── [Listbox Overlay]
        └── [Option...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | select host | state and focus context |
| Trigger | yes | opens option list | background, border, radius, focus ring |
| Selected Value | yes | current value or placeholder | typography, text color |
| Indicator | yes | disclosure icon | icon color |
| Listbox Overlay | conditional | option container | surface, elevation, border |
| Option | yes | selectable value row | text, background, selected/highlight state |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | shown when no value selected |
| `options` | `Array<{ value: string; label: string; isDisabled?: boolean }>` | none | yes | option list |
| `isDisabled` | `boolean` | `false` | no | disables trigger and listbox |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `descriptionId` | `string \| null` | `null` | no | optional descriptive relation |
| `onValueChange` | `(value: string) => void` | none | no | selection callback |
| `onOpenChange` | `(open: boolean) => void` | none | no | overlay-state callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- open state may later gain a controlled API; this baseline keeps it
  implementation-owned with `onOpenChange` notification

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no selected value | placeholder visible |
| selected | selected value present | selected label visible |
| open | listbox open | trigger reflects expanded state |
| focus | trigger or listbox focused | visible focus/highlight treatment |
| disabled | `isDisabled=true` | non-interactive state |

### Component States

Open/closed state, highlighted option state, and selected value state are all
required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | selection commits | selected value | one value at a time |
| `onOpenChange` | listbox opens or closes | boolean | optional |

## 6. Accessibility

### Semantics

- Role: select trigger with combobox/button plus listbox-style option semantics
- Required attributes: accessible name, expanded state, selected-option state,
  trigger/listbox relationship
- Optional attributes: description relation, placeholder announcement where
  supported
- Labeling rules: placeholder is not the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` | opens trigger or selects highlighted option |
| `Arrow Down/Up` | opens listbox and/or moves highlight |
| `Escape` | closes listbox and restores focus to trigger |
| `Home/End` | moves to first/last option when listbox is open |
| `Tab` | exits the control when closed; closes and exits according to pattern when open |

### Focus And Announcement

- focus entry: trigger participates in the tab order
- focus transition: opening the listbox transfers active descendant or focus
  context into the option list
- focus restoration: closing the listbox returns focus to the trigger
- live-region behavior: none; value and expanded state must be exposed through
  control semantics
- GPUI-native accessibility mapping notes: GPUI must explicitly preserve
  trigger/listbox relationships, selected value exposure, highlight movement,
  and focus restoration

## 7. Layout

### Sizing

- trigger height follows shared control-size tokens
- overlay width should at minimum match trigger width unless constrained

### Composition

- parent expectations: forms, filter bars, settings rows
- child expectations: option list only in this baseline contract
- resizing rules: trigger remains stable when value changes between options of
  different label lengths

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Trigger | control background, border, radius, size roles | field chrome |
| Selected Value | typography and text roles | value display |
| Indicator | icon roles | disclosure icon |
| Overlay | `Surface` and `ScrollShell` roles | listbox shell |
| Option | text/background/highlight roles | option rows |
| Focus treatment | accent focus roles | trigger and listbox focus |

## 9. Svelte Notes

- may compose headless select/listbox primitives, but the public contract owns
  placeholder, trigger, and selection semantics
- listbox keyboard behavior should follow established accessible select patterns

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::select`
- GPUI implementation must intentionally model expanded state, option
  highlighting, trigger/listbox relationships, and focus restoration in the
  native accessibility tree

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger, expanded, and selected-value semantics match
- [ ] listbox navigation and commit behavior match
- [ ] focus restoration and trigger/listbox accessibility relationships match

### Tier 2: Visual Parity

- [ ] trigger and option-shell roles use comparable token mappings

### Tier 3: Implementation Freedom

- [ ] overlay positioning and rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| overlay placement details may differ | positioning internals are runtime-specific | allowed | keep open/focus/value semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings, filters, inspectors
- future follow-up: align against finalized `Popover` and overlay-layering
  rules during first implementation review

## Next Task

Treat `Select` as a value control built on the overlay baseline, not as a
general menu replacement.

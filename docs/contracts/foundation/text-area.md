# Text Area

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `TextArea`
- Layer: `foundation`
- Summary: a multiline text entry control for longer text content
- In scope: multiline editing, line breaks, scroll behavior, validation state,
  submit/cancel semantics when intentionally enabled
- Out of scope: rich text formatting, markdown semantics, code editor features

## 2. Anatomy

```text
[Root]
  └── [Text Area Control]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | field chrome container | background, border, radius, focus ring |
| Text Area Control | yes | multiline editing surface | typography, text color, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled value |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | hint text |
| `rows` | `number` | `4` | no | initial visible rows |
| `isDisabled` | `boolean` | `false` | no | disables editing |
| `isReadOnly` | `boolean` | `false` | no | allows selection without editing |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive state |
| `ariaLabel` | `string \| null` | `null` | no | required when no external label exists |
| `descriptionId` | `string \| null` | `null` | no | descriptive relationship |
| `errorMessageId` | `string \| null` | `null` | no | validation relationship |
| `onValueChange` | `(value: string) => void` | none | no | change callback |
| `onSubmit` | `(value: string) => void` | none | no | optional explicit submit behavior |
| `onCancel` | `() => void` | none | no | optional cancel behavior |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral multiline field |
| focus | focus enters | visible active treatment |
| disabled | `isDisabled=true` | muted non-interactive field |
| readOnly | `isReadOnly=true` | selectable but not editable |
| invalid | `validationState="invalid"` | error emphasis |
| pending | `validationState="pending"` | progress treatment |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | content edits | current string | multiline-safe |
| `onSubmit` | explicit submit action | current string | typically chord-based rather than bare enter |
| `onCancel` | escape or explicit cancel action | none | optional |

## 6. Accessibility

### Semantics

- Role: native multiline text input
- Required attributes: accessible name from label or `ariaLabel`
- Optional attributes: description and error relationships, invalid and readonly
  state
- Labeling rules: placeholder is not the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | inserts text |
| `Enter` | inserts line break by default |
| `Shift+Arrow` | extends selection |
| platform clipboard shortcuts | operate on multiline selection |
| `Tab` | leaves control unless explicit indentation mode is a higher-order contract |
| explicit submit chord | optional, app- or contract-owned |
| `Escape` | optional cancel behavior |

### Focus And Announcement

- focus entry: visible focus treatment with insertion point
- focus exit: field remains scrollable/selected but no longer active
- live-region behavior: validation announcement is parent-owned, but invalid
  state relationships must be exposed
- GPUI-native accessibility mapping notes: multiline role, value exposure,
  caret/selection movement, and embedded scrollability must be surfaced through
  native accessibility APIs

## 7. Layout

### Sizing

- min height based on `rows`
- internal vertical scrolling occurs when content exceeds visible height

### Composition

- parent expectations: forms, notes panels, comments, descriptions
- child expectations: none
- resizing rules: may be fixed-height or resizable by higher-level wrappers

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.color.background.panel` and border roles | field chrome |
| Root | `semantic.radius.control` | shape |
| Root | `semantic.space.control.*` | interior spacing |
| Text Area Control | `semantic.typography.body.*` | text styling |
| Text Area Control | `semantic.color.text.primary/secondary` | content and placeholder text |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus |
| Validation | `semantic.color.status.*` | state emphasis |

## 9. Svelte Notes

- should prefer native `<textarea>`
- browser line wrapping, selection, and IME behavior should remain native

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::text_area`
- GPUI implementation must intentionally support multiline caret movement,
  selection, vertical scrolling, IME composition, and text-focused shortcut
  suppression

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] multiline editing semantics match
- [ ] accessible naming and invalid/readonly exposure match
- [ ] enter-as-line-break behavior matches by default
- [ ] text-focused shortcut suppression matches

### Tier 2: Visual Parity

- [ ] spacing, typography, and validation emphasis use the same token roles

### Tier 3: Implementation Freedom

- [ ] browser `<textarea>` internals vs GPUI multiline editor internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native scrollbar visuals may differ | platform-native scrolling visuals are acceptable | allowed | keep editing and accessibility semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: notes fields, descriptions, long-form content editors
- future follow-up: split code/editor primitives into a separate family later

## Next Task

Keep multiline text entry distinct from code or rich text editors when later
composite editing surfaces arrive.

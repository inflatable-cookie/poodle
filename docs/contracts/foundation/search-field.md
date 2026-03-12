# Search Field

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `SearchField`
- Layer: `foundation`
- Summary: a search-oriented text entry control with explicit query semantics
  and optional clear action
- In scope: search affordance, query change, clear action, submission,
  placeholder guidance
- Out of scope: result list semantics, command palette behavior, async result
  loading surfaces

## 2. Anatomy

```text
[Root]
  ├── [Search Icon]
  ├── [Input Control]
  └── [Clear Action] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | search field chrome | background, border, radius, focus ring |
| Search Icon | yes | persistent search affordance | icon color, icon size |
| Input Control | yes | query entry control | typography, text color |
| Clear Action | no | clears non-empty query | icon color, action state |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled query |
| `defaultValue` | `string` | `""` | no | uncontrolled query |
| `placeholder` | `string` | `"Search"` | no | query hint |
| `ariaLabel` | `string` | `"Search"` | no | accessible name fallback |
| `isDisabled` | `boolean` | `false` | no | disables query entry |
| `isReadOnly` | `boolean` | `false` | no | keeps query selectable but fixed |
| `showClearButton` | `boolean` | `true` | no | clear affordance visibility |
| `onValueChange` | `(value: string) => void` | none | no | query change callback |
| `onSubmit` | `(value: string) => void` | none | no | enter-to-search callback |
| `onClear` | `() => void` | none | no | explicit clear callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no query | placeholder and search icon visible |
| populated | query present | clear action may appear |
| focus | input focused | visible active treatment |
| disabled | `isDisabled=true` | non-interactive field |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | query changes | current string | immediate query update |
| `onSubmit` | enter confirms query | current string | search dispatch |
| `onClear` | clear action invoked | none | should also result in empty query |

## 6. Accessibility

### Semantics

- Role: native search or text input semantics
- Required attributes: accessible name via external label or `ariaLabel`
- Optional attributes: description relationship for search scope/help text
- Labeling rules: search icon does not provide the accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | updates query |
| `Enter` | fires `onSubmit` |
| `Escape` | may clear or cancel when that behavior is explicitly enabled |
| `Tab` | moves between input and clear action if both are separately focusable |

### Focus And Announcement

- focus entry: input receives visible focus treatment
- focus exit: clear button visibility must not strand keyboard focus
- live-region behavior: search result announcement belongs to result surfaces,
  not the field itself
- GPUI-native accessibility mapping notes: search purpose, query value, clear
  action naming, and input-focused shortcut suppression must all be explicit in
  native accessibility behavior

## 7. Layout

### Sizing

- control height follows shared control-size tokens
- search icon and clear action must not collapse query-edit width below usable
  minimum

### Composition

- parent expectations: toolbars, browsers, lists, command/search shells
- child expectations: icon and optional clear action only
- resizing rules: input grows to consume remaining width

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.size.control.height`, background, border, radius roles | field chrome |
| Search Icon | `semantic.color.icon.muted` and `semantic.icon.size.default` | persistent affordance |
| Input Control | `semantic.typography.body.*` and `semantic.color.text.*` | query text |
| Clear Action | action/icon token roles | clear affordance |
| Focus treatment | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus |

## 9. Svelte Notes

- should prefer native `<input type="search">` or equivalent when browser
  behavior is useful
- clear button can be Pug-owned rather than relying on browser-specific search
  decorations

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::search_field`
- GPUI implementation must intentionally preserve query-edit keyboard semantics
  while text input is focused and ensure the clear action has its own accessible
  name when focusable

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] query-change semantics match
- [ ] submit and clear behavior match
- [ ] accessible naming for input and clear action match
- [ ] text-focused shortcut suppression matches

### Tier 2: Visual Parity

- [ ] persistent search affordance and clear affordance use the same token roles

### Tier 3: Implementation Freedom

- [ ] native search-input internals vs GPUI composition internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| browser-native search decorations may differ or be suppressed | platform-specific visuals are acceptable | allowed | keep semantics and focus behavior strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura browser/search fields, Spark browser/plugin search
- future follow-up: command-palette-specific ranking and discovery heuristics
  belong in workstation-layer milestones, not in the foundation search field

## Next Task

Use `SearchField` as the search-specific wrapper over `TextInput`, not as a
catch-all command palette or result-list contract.

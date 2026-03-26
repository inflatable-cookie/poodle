# Search Field

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `SearchField`
- Layer: `foundation`
- Summary: a search-oriented text entry control that composes TextInput with a
  persistent search icon, optional clear button, and explicit query semantics
- In scope: search affordance (leading icon), query change, clear action,
  submission, placeholder guidance, composition over TextInput
- Out of scope: result list semantics, command palette behavior, async result
  loading surfaces, autocomplete/suggestion dropdowns

## 2. Anatomy

```text
[Root]  (delegates to TextInput)
  ├── [Search Icon] (leading affordance, via TextInput slot)
  ├── [Input Control] (from TextInput, type="search")
  └── [Clear Button .search-field__clear] (conditional, when value non-empty and showClearButton)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | search field chrome (from TextInput) | background, border, radius, focus ring |
| Search Icon | yes | persistent search affordance in leading position | icon color, icon size |
| Input Control | yes | query entry control (TextInput with type="search") | typography, text color |
| Clear Button | no | clears non-empty query; visible only when value is non-empty | icon color, hover background, border-radius |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | none | yes | element id for label association |
| `value` | `string \| null` | `null` | no | controlled query value |
| `defaultValue` | `string` | `""` | no | uncontrolled initial query |
| `placeholder` | `string` | `"Search"` | no | query hint text |
| `ariaLabel` | `string` | `"Search"` | no | accessible name fallback |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `disabled` | `boolean` | `false` | no | disables query entry |
| `readOnly` | `boolean` | `false` | no | keeps query selectable but not editable |
| `debounce` | `number \| null` | `null` | no | delays `valueChange` while typing |
| `showClearButton` | `boolean` | `true` | no | whether clear button appears when value is non-empty |
| `validationState` | `"none" \| "invalid" \| "valid" \| "pending"` | `"none"` | no | visual and assistive validation state |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |

### Controlled And Uncontrolled

- controlled: `value` (non-null) plus `valueChange` event
- uncontrolled: `defaultValue` sets the initial value
- inherits controlled/uncontrolled semantics from TextInput

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no query value | placeholder visible, search icon visible, clear button hidden |
| populated | query value non-empty | clear button appears (when showClearButton is true) |
| focus | input focused | visible focus treatment (inherited from TextInput) |
| disabled | `disabled=true` | non-interactive field (inherited from TextInput) |
| readOnly | `readOnly=true` | selectable but not editable |
| invalid | `validationState="invalid"` | error border emphasis |
| valid | `validationState="valid"` | success border emphasis |
| pending | `validationState="pending"` | accent border emphasis |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | query text changes | `{ value: string }` | immediate query update, passed through from TextInput |
| `submit` | Enter key pressed | `{ value: string }` | search dispatch, passed through from TextInput |
| `clear` | clear button activated | `void` | should also result in empty query value |
| `cancel` | Escape key pressed | `void` | passed through from TextInput |

## 6. Accessibility

### Semantics

- Role: native `<input type="search">` via TextInput composition
- `id`: from prop, used for external `<label for>` association
- `aria-label`: defaults to `"Search"` when no external label exists
- `aria-describedby`: from describedBy prop
- Search icon: `aria-hidden="true"` (decorative, not the accessible name)
- Clear button: must have accessible name (e.g. "Clear search")
- Labeling rules: search icon does not provide the accessible name; the
  `ariaLabel` prop or external label does

### Keyboard

| Key | Behavior |
|-----|----------|
| character input | updates query |
| `Enter` | fires `submit` event |
| `Escape` | fires `cancel` event (inherited from TextInput) |
| `Tab` | moves focus out of the search field |

### Focus And Announcement

- focus entry: input receives visible focus treatment (inherited from TextInput)
- focus exit: clear button visibility must not strand keyboard focus; if clear
  button was focused and query becomes empty, focus returns to input
- live-region behavior: search result announcement belongs to result surfaces,
  not the field itself
- GPUI-native accessibility mapping notes: search purpose, query value, clear
  action accessible name, and input-focused shortcut suppression must all be
  explicit in native accessibility behavior

## 7. Layout

### Sizing

- control height follows TextInput sizing (shared control-size tokens)
- search icon and clear button must not collapse query-edit width below usable
  minimum
- inherits all sizing behavior from TextInput

### Composition

- parent expectations: toolbars, browser bars, list headers, command/search
  shells, filter toolbars
- child expectations: search icon (leading) and optional clear button (trailing)
- resizing rules: input grows to consume remaining width after icon and clear
  button allocation
- composition model: SearchField composes TextInput rather than duplicating
  its internals

## 8. Token Usage — Exact Values

### Inherited from TextInput

SearchField inherits all root, input control, and affordance styling from
TextInput. The treatment CSS custom properties use the `--poodle-text-input-*`
namespace since the root element is a TextInput. See the TextInput contract
Section 8 for the full root, focus, validation, and disabled token tables.

### Clear Button `.search-field__clear`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-icon-size-default)` |
| `height` | `var(--poodle-icon-size-default)` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-icon-muted)` |
| `cursor` | `pointer` |
| `border-radius` | `calc(var(--poodle-treatment-interactive-subtle-radius) - 0.0625rem)` |

### Clear Button hover

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-treatment-interactive-subtle-fill-hover, color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent))` |
| `color` | `var(--poodle-color-text-primary)` |

### Clear Button focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Size adjustments

SearchField delegates `size` and `sizeRole` to the wrapped TextInput. No
additional size-specific CSS rules exist on SearchField itself; all min-height,
padding, and font-size scaling is inherited from the TextInput size adjustments.

## 9. Svelte Notes

- Composes TextInput with `type="search"` rather than building a parallel
  input implementation
- Search icon is placed in the leading affordance slot of TextInput
- Clear button is a custom `<button>` element placed in the trailing affordance
  area, not the browser-native search clear decoration
- Browser-native search input decorations (WebKit clear button, etc.) should be
  suppressed via `::-webkit-search-cancel-button { display: none }`
- Clear button visibility is conditional on value being non-empty AND
  `showClearButton` being true
- Clear action both fires the `clear` event and resets the value to empty string
- Emits `data-size` on root element reflecting the resolved size (via TextInput)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::search_field`
- GPUI implementation should compose or extend the TextInput primitive rather
  than duplicating text-editing internals
- clear button must have its own accessible name ("Clear search" or equivalent)
- clear button hover background uses the same treatment-hover token as the
  TextInput treatment system
- the clear button border-radius is slightly smaller than the root radius
  (`treatment-radius - 0.0625rem`) to nest cleanly inside the input chrome

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] query-change semantics match (via TextInput composition)
- [ ] Enter fires submit event
- [ ] clear button fires clear event and empties value
- [ ] accessible naming for input ("Search" default) and clear button match
- [ ] text-focused shortcut suppression matches
- [ ] clear button hidden when value is empty

### Tier 2: Visual Parity

- [ ] persistent search icon uses icon-muted color
- [ ] clear button sizing matches (icon-default width/height)
- [ ] clear button hover background matches (treatment-hover or color-mix fallback)
- [ ] clear button hover color matches (text-primary)
- [ ] clear button border-radius matches (treatment-radius - 0.0625rem)
- [ ] focus ring on clear button matches
- [ ] all TextInput visual parity items apply
- [ ] all five sizes visually match (delegates to TextInput size adjustments)

### Tier 3: Implementation Freedom

- [ ] native search-input internals vs GPUI composition internals stay internal
- [ ] browser-native search decorations may be suppressed differently
- [ ] composition strategy (slot injection vs wrapper) is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| browser-native search decorations may differ or be suppressed | platform-specific visuals are acceptable | allowed | keep semantics and focus behavior strict |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |
| clear button border-radius nesting formula | GPUI must achieve same visual result by any means | allowed | verify visual parity |

## 13. Specimen Definitions

### Default

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default | `placeholder="Search components..."`, `ariaLabel="Search components"` | Search field with leading search icon and placeholder text; typing shows clear button and displays current query and submitted value below |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="locked query"`, `disabled` | Search field with pre-filled value, reduced opacity, non-interactive |

### Read-only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Read-only | `value="active filter"`, `readOnly` | Search field with pre-filled value, selectable but not editable |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Aura browser/search fields, Spark browser/plugin search,
  filter toolbars, browse-search-shell
- future follow-up: command-palette-specific ranking and discovery heuristics
  belong in workstation-layer milestones, not in the foundation search field

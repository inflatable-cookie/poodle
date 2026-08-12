# Select

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Select`
- Layer: `foundation`
- Summary: a unified single-select control that operates in native mode
  (platform `<select>` element) or custom mode (styled dropdown overlay with
  optional search input), supporting flat options, option groups, lazy option
  loading, searchable filtering, freeform text entry, custom option rendering
  via snippet props, and filter-friendly clear/reset behavior
- In scope: single value selection, placeholder, flat options, grouped options
  (optgroup), disabled options, lazy option/group loading, clearable reset
  state, native select accessibility, custom dropdown with keyboard navigation,
  searchable/filterable lists, freeform text-as-value, custom option/trigger/empty
  snippet rendering
- Out of scope: multi-select, command-palette ranking, arbitrary menu content
- Supersedes: `Combobox` (deprecated; use `Select` with `searchable` prop instead)

## 2. Anatomy

### Native Mode

```text
[Root .select]  <div>
  ├── [Control .select__control]  <select>
  │     ├── [Placeholder <option>] (conditional, when placeholder set and no value)
  │     ├── [Option <option>]... (flat options)
  │     └── [Option Group <optgroup>]... (grouped options)
  │           └── [Option <option>]...
  └── [Indicator .select__indicator]  <span> (decorative chevron)
```

### Custom Mode

```text
[Root .select.select--custom]  <div>
  ├── [Trigger Area .select__trigger-area]  <div role="combobox"> (searchable)
  │     ├── [Input .select__input]  <input type="text"> (searchable)
  │     ├── [Clear Button .select__clear]  <button> (clearable + has selection)
  │     └── [Indicator .select__indicator]  <span> (decorative chevron)
  ├── [Trigger Button .select__trigger]  <button> (non-searchable)
  │     ├── [Value/Snippet .select__value | trigger snippet]
  │     ├── [Clear Button .select__clear]  <button> (clearable + has selection)
  │     └── [Indicator .select__indicator]  <span> (decorative chevron)
  ├── [Hidden Input]  <input type="hidden"> (when name prop set, for form submission)
  └── [Listbox .select__listbox]  <div role="listbox"> (conditional, when open)
        ├── [Group .select__group]  <div role="group"> (grouped options)
        │     ├── [Group Label .select__group-label]
        │     └── [Option Button .select__option]  <button role="option">
        │           ├── [option snippet] (custom rendering) OR
        │           ├── [Option Icon .select__option-icon] (optional)
        │           ├── [Option Label .select__option-label]
        │           └── [Option Description .select__option-description] (optional)
        ├── [Option Button .select__option]  <button role="option"> (flat options)
        └── [Empty .select__empty | empty snippet]  (when no filtered matches)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | styled wrapper providing field chrome | background, border, radius, shadow, focus ring |
| Control | native mode | native `<select>` element | typography, text color, appearance reset |
| Indicator | conditional | decorative disclosure chevron (Icon component); always present in native, searchable custom, and non-searchable custom mode — ghost keeps it too | icon color |
| Trigger Area | custom + searchable | combobox container with ARIA role | position |
| Trigger Button | custom + non-searchable | button that opens the dropdown | typography, text color |
| Input | custom + searchable | text query input for filtering | border, background, typography |
| Hidden Input | custom + name prop | hidden input for form submission | none |
| Listbox | custom mode | dropdown overlay containing options | position, border, radius, background, shadow |
| Option Button | custom mode | selectable option in dropdown | padding, radius, background, color, cursor |
| Option Icon | no | icon before option label | icon size |
| Option Description | no | secondary text under option label | color, font-size |
| Group | no | labeled group of options in dropdown | none (container) |
| Group Label | no | group header text | font-weight, color, font-size |
| Empty | custom mode | "no results" message or snippet | color, font-size, padding |
| Clear Button | no | clears selection back to default | icon color |
| Option | native mode | selectable value in native select | text color, font-weight |
| Option Group | native mode | labeled group of options in native select | font-weight, text color |

### Snippet Props (Svelte / Custom Mode)

| Snippet | Props | Description |
|---------|-------|-------------|
| `option` | `{ option, highlighted, selected, index }` | Custom rendering for each option in the dropdown |
| `trigger` | `{ selectedOption, open, placeholder }` | Custom rendering for the trigger button content (non-searchable only) |
| `empty` | `{ query }` | Custom rendering for the empty state when no options match the search query |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string \| undefined` | `undefined` | no | HTML id for the select element |
| `value` | `string \| null \| undefined` | `undefined` | no | bindable selected value; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `placeholder` | `string \| null` | `null` | no | shown when no value selected |
| `options` | `SelectItems` | — | yes | array of `SelectOption` or `SelectOptionGroup` |
| `disabled` | `boolean` | `false` | no | disables the select |
| `required` | `boolean` | `false` | no | forwards native required semantics |
| `validationState` | `ValidationState` | `"none"` | no | drives trigger border color (`invalid`→danger, `valid`→success, `pending`→accent) and sets `aria-invalid` when `"invalid"` |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `name` | `string \| undefined` | `undefined` | no | form field name; in custom mode a hidden input is rendered for form submission |
| `clearable` | `boolean` | `false` | no | keeps the placeholder option selectable so callers can reset to `defaultValue` |
| `valueLabel` | `string \| null` | `null` | no | temporary label for the current selection before lazy options load |
| `searchable` | `boolean` | `false` | no | renders custom dropdown with search/filter text input instead of native select |
| `freeform` | `boolean` | `false` | no | with `searchable`, query text becomes the value if no option is selected |
| `native` | `boolean \| undefined` | `undefined` | no | explicit mode override: `true` forces native select, `false` forces custom dropdown, `undefined` uses auto-detection |
| `emptyMessage` | `string` | `"No matches"` | no | text shown in custom dropdown when no options match the search query |
| `loadOptions` | `SelectLoadOptions \| null` | `null` | no | unified async option loader; returns flat or grouped options |
| `loadKey` | `string \| null` | `null` | no | invalidates cached lazy options when it changes |
| `variant` | `"default" \| "ghost"` | `"default"` | no | visual variant; `"ghost"` strips border, background, box-shadow, padding, and min-height, but keeps the chevron indicator |
| `menuMinWidth` | `string \| null` | `null` | no | minimum width for the dropdown listbox (e.g. `"12rem"`); when set, listbox uses `width: max-content` instead of matching trigger width, and viewport-aware horizontal anchor flipping occurs (right-anchors if menu would overflow right edge) |
| `dismissOnOutsideInteract` | `boolean` | `true` | no | outside dismissal: clicking outside the trigger and listbox closes the dropdown |
| `onValueChange` | `((value: string) => void) \| undefined` | `undefined` | no | callback fired when the selected value changes |
| `onQueryChange` | `((query: string) => void) \| undefined` | `undefined` | no | callback fired when the searchable query changes |
| `onOpenChange` | `((open: boolean) => void) \| undefined` | `undefined` | no | callback fired when the custom dropdown opens or closes |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Type Definitions

```
SelectOption: { value: string; label: string; isDisabled?: boolean; disabled?: boolean; icon?: IconProp; description?: string; group?: string }
SelectOptionGroup: { label: string; options: SelectOption[] }
SelectItems: SelectOption[] | SelectOptionGroup[]
SelectLoadOptions: () => Promise<SelectItems>
SelectTriggerRenderState: { selectedOption: SelectOption | null; open: boolean; placeholder: string | null }
SelectOptionRenderState: { option: SelectOption; highlighted: boolean; selected: boolean; index: number }
SelectEmptyRenderState: { query: string }
```

### Controlled And Uncontrolled

- controlled: `bind:value` or `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- lazy: `loadOptions` populates internal options once per `loadKey`

### Mode Resolution

The component automatically determines whether to render a native `<select>` or a custom dropdown overlay. The resolution cascade is:

1. `native=true` -- always native `<select>` (searchable/snippets ignored)
2. `native=false` -- always custom dropdown
3. `searchable=true` -- custom dropdown (search input as trigger)
4. `option` snippet present -- custom dropdown (custom option rendering requires overlay)
5. `trigger` snippet present -- custom dropdown
6. Otherwise -- native `<select>`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| placeholder | no value selected, placeholder prop set | placeholder text in secondary color, `data-placeholder="true"` |
| clearable reset | `clearable=true` | placeholder option remains selectable and maps to `defaultValue` |
| selected | value matches an option | option label displayed in primary color |
| focus | select receives focus | focus ring via border-color change, background shift, box-shadow |
| disabled | `disabled=true` | reduced opacity on root, non-interactive |
| loading | lazy loader pending | native fallback option shows `Loading...` |
| load error | lazy loader fails | native fallback option shows the error message |
| open (custom) | dropdown is visible | `data-open="true"` on root, listbox rendered |
| highlighted (custom) | keyboard nav or hover over option | `data-highlighted="true"` on option, accent background mix |
| empty results (custom) | searchable query matches no options | empty message or empty snippet rendered in listbox |
| ghost variant | `variant="ghost"` | no border, background, box-shadow, padding, or min-height on root; the chevron indicator stays on the non-searchable trigger (the control still signals it opens a list); focus-within treatment fully transparent |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| value selected | user picks an option | `value` updates and `onValueChange` fires with selected option value |
| placeholder shown | no value and placeholder set | placeholder option displayed, disabled in dropdown |
| query active (custom) | user types in searchable input | options filtered by query, `onQueryChange` fires |
| highlight tracked (custom) | ArrowDown/ArrowUp or hover | one option visually highlighted via `data-highlighted` |
| value committed (custom) | Enter on highlighted option or click | `value` updates, dropdown closes |
| dismissed (custom) | Escape or click outside | dropdown closes without changing value |
| freeform commit (custom) | blur or Enter with no highlight, `freeform=true` | query text becomes the value |

### Behavior Machine

Behavior classification: machine-backed via core machinery

Select composes `@inflatable-cookie/poodle-core` machinery; lazy loading, query state,
freeform mode, and native-mode delegation stay adapter-side.

- Option lists: `flattenSelectOptions` (grouped or flat),
  `filterSelectOptions` (enabled + case-insensitive label match),
  `filterSelectGroups` (per-group filter, empty groups dropped),
  `isSelectOptionDisabled` (honors `disabled` and `isDisabled`)
- Open placement: `selectMenuPlacement` — flips above when under 280px
  remain below the trigger; right-aligns a fixed min-width menu that would
  overflow the right edge but fits against the trigger's right edge
- Open highlight: `selectOpenHighlightIndex` — selected option when
  present, else first
- Dismissal: escape + outside interaction via the dismissable-layer stack
  (innermost-first); the outside-interaction path is guarded by
  `dismissOnOutsideInteract` (default `true`)
- Keyboard highlight movement is clamp-based (no wrap) and operates on the
  already-enabled filtered list — adapter-side by current behavior

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | user selects a different option | `value: string` | fires on native change or custom option commit |
| `onQueryChange` | user types in searchable input | `query: string` | custom mode with `searchable` only; fires on every input change |
| `onOpenChange` | dropdown opens or closes | `open: boolean` | custom mode only; fires on open and close transitions |

## 6. Accessibility

### Semantics (Native Mode)

- Role: native `<select>` element provides built-in accessibility
- Required attributes: accessible name from external label or `ariaLabel`
- Optional attributes: `aria-describedby` from `describedBy`
- `disabled` attribute set on root select when `disabled`
- Disabled individual options use native `disabled` attribute on `<option>`
- Labeling rules: placeholder text is not the accessible name

### Semantics (Custom Mode)

- Searchable: trigger area has `role="combobox"`, `aria-expanded`, `aria-haspopup="listbox"`, `aria-controls` pointing to listbox id
- Searchable input: `aria-autocomplete="list"`, `aria-activedescendant` pointing to highlighted option id
- Non-searchable: trigger button has `aria-expanded`, `aria-haspopup="listbox"`, `aria-controls`
- Listbox: `role="listbox"`, unique id referenced by `aria-controls`
- Option: `role="option"`, `aria-selected` on the currently selected option
- Group: `role="group"` with `aria-label` from group label
- Disabled options: native `disabled` attribute on button
- Clear button: `aria-label="Clear selection"`
- Per-instance generated ids back the combobox/listbox ARIA relationships when `id` is not supplied

### Keyboard (Native Mode)

| Key | Behavior |
|-----|----------|
| `Space` / `Enter` | opens native select dropdown (platform-dependent) |
| `Arrow Down` / `Arrow Up` | navigates options within native dropdown |
| `Escape` | closes native dropdown |
| `Tab` | exits the control |

### Keyboard (Custom Mode)

| Key | Behavior |
|-----|----------|
| `Arrow Down` | highlights next option; opens dropdown if closed |
| `Arrow Up` | highlights previous option |
| `Enter` | selects the highlighted option, closes dropdown |
| `Escape` | closes dropdown without selecting |
| `Home` | highlights first option |
| `End` | highlights last option |
| `Tab` | closes dropdown if open, exits control |
| typing (searchable) | filters options, opens dropdown if closed |

### Focus And Announcement

- focus entry: root receives visible focus treatment (border-color, background, box-shadow changes)
- focus exit: focus treatment clears
- custom mode: highlight moves via `aria-activedescendant` (no DOM focus change); closing the dropdown keeps focus on the input/trigger
- live-region behavior: none; native select handles value announcement; custom mode uses `aria-activedescendant`
- GPUI-native accessibility mapping notes: GPUI must expose select/combobox semantics with option list, selected value, and expanded state through the native accessibility tree

## 7. Layout

### Sizing

- Root min-height follows `size-control-height` token
- Root uses grid layout with two columns: select fills available space, indicator is auto-sized
- Indicator is pointer-events: none to allow click-through to native select

### Composition

- parent expectations: forms, filter bars, settings rows, toolbars
- child expectations: options and optgroups only
- resizing rules: select stretches to parent width; value display truncates if needed

## 8. Token Usage — Exact Values

### Root `.select`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-recipe-select-fill, var(--poodle-color-background-surface))` |
| `box-shadow` | `var(--poodle-recipe-select-shadow, none)` |
| `transition` | `border-color, box-shadow, background` |

### Root — focus-within

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-recipe-select-focus-border, var(--poodle-color-accent-focusRing))` |
| `background` | `var(--poodle-recipe-select-focus-fill, var(--poodle-color-background-surface))` |
| `box-shadow` | `var(--poodle-recipe-select-focus-shadow, 0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent))` |

### Root — has disabled select

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Control `.select__control`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `width` | `100%` |
| `height` | `calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2))` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |
| `outline` | `0` |
| `appearance` | `none` |

### Control — placeholder state `.select[data-placeholder="true"] .select__control`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Indicator `.select__indicator`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-icon-muted)` |
| `pointer-events` | `none` |

### Option Group `.select__control optgroup`

| Property | Value |
|----------|-------|
| `font-weight` | `600` |
| `color` | `var(--poodle-color-text-secondary)` |

### Option `.select__control option`

| Property | Value |
|----------|-------|
| `font-weight` | `normal` |
| `color` | `var(--poodle-color-text-primary)` |

### Ghost variant `.select[data-variant="ghost"]`

| Property | Value |
|----------|-------|
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `0` |
| `background` | `transparent` |
| `box-shadow` | `none` |

### Ghost variant -- focus-within

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |
| `background` | `transparent` |
| `box-shadow` | `none` |

### Ghost variant -- trigger area

| Property | Value |
|----------|-------|
| `padding` | `0` |
| `min-height` | `0` |

### Ghost variant -- trigger button

| Property | Value |
|----------|-------|
| `min-height` | `0` |
| `padding` | `0` |
| `line-height` | `1` |

### Ghost variant -- indicator

The chevron indicator **is rendered** in non-searchable custom mode when
`variant="ghost"`. Ghost drops the border and the fill, not the signal that the
control is a select; the trigger still reserves the indicator's decoration lane
so the value ellipsizes before it.

### Listbox -- auto-width `.select__listbox--auto-width`

When `menuMinWidth` is set, the listbox detaches from the trigger's width:

| Property | Value |
|----------|-------|
| `right` | `auto` (instead of `-0.0625rem`) |
| `width` | `max-content` |
| `min-width` | value of `menuMinWidth` prop (set via inline style) |

### Listbox -- align-end `.select__listbox--align-end`

Applied when viewport-aware horizontal flipping determines the menu would overflow the right edge:

| Property | Value |
|----------|-------|
| `left` | `auto` |
| `right` | `-0.0625rem` |

### Size adjustments

| Size | min-height | padding | control height | font-size |
|------|------------|---------|----------------|-----------|
| `xs` | `calc(control-height - 0.5rem)` | `0 calc(space-control-x - 0.125rem)` | `calc(control-height - 0.5rem - border * 2)` | `0.75rem` |
| `sm` | `calc(control-height - 0.375rem)` | `0 calc(space-control-x - 0.0625rem)` | `calc(control-height - 0.375rem - border * 2)` | `typography-body-size` |
| `md` | `control-height` | `0 space-control-x` | `calc(control-height - border * 2)` | `typography-body-size` |
| `lg` | `calc(control-height + 0.375rem)` | `0 calc(space-control-x + 0.125rem)` | `calc(control-height + 0.375rem - border * 2)` | `0.9375rem` |
| `xl` | `calc(control-height + 0.5rem)` | `0 calc(space-control-x + 0.1875rem)` | `calc(control-height + 0.5rem - border * 2)` | `1rem` |

## 9. Svelte Notes

### Native Mode

- Uses a native `<select>` element for full platform accessibility without a custom overlay
- `appearance: none` on the select removes native browser chrome; the custom indicator provides the disclosure chevron
- `data-placeholder="true"` attribute on root signals placeholder state for CSS targeting
- Component Recipe hooks (`--poodle-recipe-select-*`) provide scoped
  appearance overrides with semantic-token fallbacks.
- Placeholder rendered as a disabled `<option>` with `selected` when no value is set
- Option groups rendered as native `<optgroup>` elements
- Emits `data-size` on root element reflecting the resolved size
- `data-density` -- resolved density value (`compact`, `default`, or `comfortable`)

### Custom Mode

- Root has `select--custom` class and `data-open` attribute tracking dropdown visibility
- Dropdown placement: calculated from viewport space -- positions above (`select__listbox--above`) when less than 280px below the trigger, otherwise below
- Hidden `<input type="hidden">` rendered when `name` prop is set, for form submission in custom mode
- Keyboard navigation: `ArrowDown`/`ArrowUp` move highlight, `Enter` commits, `Escape` closes, `Home`/`End` jump to first/last option
- Click-outside detection via document `mousedown` listener closes the dropdown
- Module-level `nextSelectId` counter generates unique ids for ARIA relationships (`aria-controls`, `aria-activedescendant`)
- `aria-activedescendant` on the input/trigger tracks the currently highlighted option by referencing its stable id (`{listboxId}-option-{index}`)
- Searchable mode: client-side filtering matches query against option labels (case-insensitive substring)
- Freeform mode: when `freeform=true` and `searchable=true`, the query text becomes the selected value on commit if no option is highlighted
- `data-variant` attribute on root reflects the `variant` prop; ghost variant strips all field chrome (border, background, shadow, padding, min-height) but keeps the chevron indicator on non-searchable triggers
- `menuMinWidth` prop sets an inline `min-width` style on the listbox and switches it to `width: max-content` (class `select__listbox--auto-width`); on open, viewport-aware horizontal anchor flipping checks whether left-anchoring would overflow the right edge and applies `select__listbox--align-end` if so
- Three named snippet props available in custom mode:
  - `option` -- snippet props: `{ option, highlighted, selected, index }` -- custom rendering for each option row
  - `trigger` -- snippet props: `{ selectedOption, open, placeholder }` -- custom trigger button content (non-searchable only)
  - `empty` -- snippet props: `{ query }` -- custom empty state when no options match
- Options support optional `icon` (rendered via Icon component) and `description` (secondary text) fields
- Clearable mode renders a clear button (X icon) inside the trigger when a value is selected
- When the dropdown is closed, the searchable input text resets to the selected option's label (or empty if no selection, unless freeform)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::select`
- GPUI must model the select as a trigger that opens a platform-appropriate option list
- Must expose selected value, expanded state, and option list through native accessibility tree
- Recipe fallback chains use the component override when set and otherwise use
  the semantic-token default.
- The indicator chevron is decorative and does not need separate accessibility exposure

## 10a. Jetstream Notes

- `Select::from_spec(spec, theme).on_toggle(...).on_clear(...)`.
- The clear pill takes a handler of its own, inert when unwired, so clearing
  never also opens the panel it was clearing.
- `on_change` carries the chosen option's value.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value and onValueChange semantics match
- [ ] placeholder behavior matches (displayed when no value, secondary color)
- [ ] disabled state matches (whole control and individual options)
- [ ] option groups supported on both platforms
- [ ] accessible name from label or ariaLabel matches

### Tier 2: Visual Parity

- [ ] control height uses control-height token
- [ ] Recipe hooks and semantic-token fallbacks match for fill, border, and shadow
- [ ] focus-within treatment matches (border-color, background, box-shadow)
- [ ] placeholder color (text-secondary) matches
- [ ] indicator color (icon-muted) matches
- [ ] disabled opacity matches
- [ ] optgroup font-weight (600) and color (text-secondary) match
- [ ] all five sizes visually match (height, padding, font-size per size table)

### Tier 3: Implementation Freedom

- [ ] native `<select>` dropdown vs GPUI custom overlay stays internal
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native select dropdown appearance differs per platform | native `<select>` renders platform-native dropdowns | allowed | keep value/selection semantics strict |
| GPUI may use custom overlay instead of native select | GPUI has no native `<select>` equivalent | allowed | must preserve option group support and keyboard navigation |
| Recipe fallback chain | CSS custom property vs Rust spec/token override | allowed | same visual result required |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default (flat options)

One select with flat option list:

| Placeholder | Options | Initial |
|-------------|---------|---------|
| Choose a fruit | Apple, Banana, Cherry, Dragonfruit, Elderberry | none selected |

### Grouped options

One select with grouped options:

| Placeholder | Groups | Notes |
|-------------|--------|-------|
| Choose an item | Fruits (Apple, Banana, Cherry), Vegetables (Carrot, Broccoli, Spinach [disabled]), Grains (Rice, Wheat) | Spinach is disabled within its group |

### Disabled

One disabled select with pre-selected value:

| Placeholder | Options | Initial | Props |
|-------------|---------|---------|-------|
| Choose a fruit | Apple, Banana, Cherry | Banana | `disabled: true` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings, filters, inspectors, form fields
- future follow-up: align with Popover overlay rules if GPUI implementation uses custom overlay

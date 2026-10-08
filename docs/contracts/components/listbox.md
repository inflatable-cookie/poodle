# Listbox

Status: detailed contract
Updated: 2026-10-08

## 1. Purpose

- Component name: `Listbox`
- Layer: `composites`
- Summary: a selectable list whose options are rendered by the host while the
  Listbox owns option semantics, focus, keyboard navigation, and selection
- In scope: stable option values and labels, rich host-rendered rows, single
  and multiple selection, linear roving focus, disabled options, typeahead,
  range selection, activation callbacks, vertical and horizontal orientation
- Out of scope: 2D keyboard navigation for card grids, filtering, virtualization,
  drag-and-drop, nested lists, option actions, and interactive descendants

## 2. Anatomy

```text
[Root <div role="listbox">]
  └── [Option <div role="option">]*
        └── (host-rendered row content; no interactive descendants)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | Listbox semantics, orientation, and keyboard event boundary | focus ring when empty |
| Option | yes, per item | Listbox-owned semantic option and roving focus target | focus ring |
| Row content | yes | Host-rendered content for the option | host-owned |

The Listbox renders each option element and supplies `role="option"`,
`aria-label`, `aria-selected`, `aria-disabled` when needed, and roving
`tabindex`. Hosts render only the content inside each option. Each `value` must
be unique and stable for the lifetime of that row. Each `label` is the option's
accessible name and the text used by typeahead; it must be non-empty and should
describe the host-rendered row.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ListboxItem[]` | `[]` | no | Options in their keyboard and reading order. Each item has `value`, `label`, and optional `disabled`. |
| `selectionMode` | `"single" \| "multiple"` | `"single"` | no | Select one option or several. |
| `value` | `string \| null \| undefined` | `undefined` | no | Controlled selected value in single mode. |
| `values` | `string[] \| undefined` | `undefined` | no | Controlled selected values in multiple mode. |
| `defaultValue` | `string \| null` | `null` | no | Initial selected value in uncontrolled single mode. |
| `defaultValues` | `string[]` | `[]` | no | Initial selected values in uncontrolled multiple mode. |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | no | Chooses the one-dimensional arrow-key axis. |
| `disabled` | `boolean` | `false` | no | Disables focus, selection, and activation for the entire Listbox. |
| `ariaLabel` | `string \| null` | `null` | one of | Accessible name for the listbox. |
| `ariaLabelledby` | `string \| null` | `null` | one of | ID reference for a visible label. |
| `onValueChange` | `((value: string) => void) \| undefined` | `undefined` | no | Requests a new selection in single mode. |
| `onValuesChange` | `((values: string[]) => void) \| undefined` | `undefined` | no | Requests a new selection in multiple mode. |
| `onActivate` | `((value: string) => void) \| undefined` | `undefined` | no | Called by Enter or double-click on an enabled option. |

### Host-Rendered Rows

Svelte accepts a `children(item, selected, focused)` snippet. React accepts a
`renderItem(item, selected, focused)` function. The component passes the same
item data and selection/focus state in both frameworks. Without a renderer, the
item's `label` is rendered as plain text.

Do not put buttons, links, inputs, or other interactive descendants inside an
option. An option is one selectable unit and its keyboard interaction belongs
to Listbox. Development builds warn when an interactive descendant is found.
For row-level actions, render those actions outside the listbox.

### Controlled And Uncontrolled

In single mode, supplying `value` makes selection controlled; otherwise
`defaultValue` seeds internal selection. In multiple mode, supplying `values`
makes selection controlled; otherwise `defaultValues` seeds internal
selection. Selection callbacks request changes and do not mutate controlled
values. Focus is always owned by the component.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | enabled, not selected | option is in the roving sequence and can be selected |
| focused | option owns DOM focus | option has the focus-visible ring and the roving tab stop |
| selected | value appears in the current selection | option exposes `aria-selected="true"` |
| disabled option | item `disabled=true` | option exposes `aria-disabled="true"` and is skipped by interaction |
| disabled listbox | root `disabled=true` | root exposes `aria-disabled="true"`; no option can receive focus or selection |

### Component States

The machine has two interaction states. In `enabled`, events may update the
focus, range anchor, typeahead buffer, or selection context. In `disabled`, all
interaction events are inert. Focus and selection are separate: multiple mode
does not select when focus moves; single mode selects when keyboard movement
changes focus.

### Behavior Machine

Behavior classification: `machine-backed`

#### Context

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| `items` | `ListboxItem[]` | `[]` | yes | Ordered options; values are unique and stable. |
| `selectionMode` | `"single" \| "multiple"` | `"single"` | yes | Select one option or several. |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | yes | Adapter maps the matching arrow pair to linear movement. |
| `disabled` | `boolean` | `false` | yes | Makes all interaction events inert. |
| `selectedValues` | `string[]` | `[]` | yes | Current selection represented as a list in either mode. |
| `focusedValue` | `string \| null` | first selected enabled value, else first enabled value | no | Current roving focus target. |
| `anchorValue` | `string \| null` | `null` | no | Anchor for pointer and keyboard range selection. |
| `typeahead` | `string` | `""` | no | Lowercased printable-key buffer. |
| `typeaheadAt` | `number` | `0` | no | Time of the last printable key, supplied by the adapter. |

#### States

| State | Description |
|-------|-------------|
| `enabled` | Focus and selection transitions are available. |
| `disabled` | All interaction events are ignored. |

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| `FOCUS` | `value` | DOM focus |
| `MOVE` | `direction`, optional `extendSelection` | Arrow key |
| `BOUNDARY` | `first` or `last` | Home or End |
| `TYPEAHEAD` | `character`, `now` | Printable key |
| `SPACE` | none | Space key |
| `SELECT` | `value`, optional `additive` and `range` | Pointer click |
| `SELECT_ALL` | none | Ctrl/Cmd+A |
| `ACTIVATE` | optional `value` | Enter or double-click |

#### Transitions

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| enabled | `FOCUS` | value identifies an enabled option | enabled | Update focus. |
| enabled | `MOVE` | next enabled option exists | enabled | Update focus; select it in single mode; extend the enabled range when requested in multiple mode; emit focus and selection effects. |
| enabled | `BOUNDARY` | an enabled option exists | enabled | Focus first or last; select it in single mode; emit focus and selection effects. |
| enabled | `TYPEAHEAD` | buffer matches an enabled option label | enabled | Update buffer and focus; select the match in single mode; emit focus and selection effects. |
| enabled | `SPACE` | focused option is enabled | enabled | Select in single mode or toggle in multiple mode; emit selection effect when the value set changes. |
| enabled | `SELECT` | target option is enabled | enabled | Update focus and selection; additive and range behavior apply in multiple mode; emit focus and selection effects. |
| enabled | `SELECT_ALL` | selection mode is multiple | enabled | Select every enabled option; emit selection effect when changed. |
| enabled | `ACTIVATE` | target option is enabled | enabled | Emit activation effect. |
| disabled | any interaction event | none | disabled | Ignore event without effects. |

#### Effects

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| `focus` | Adapter moves DOM/native focus to the named option. | None; focus remains until another focus transition. |
| `selectionChanged` | Adapter calls `onValueChange` or `onValuesChange`, and updates internal state when uncontrolled. | None; values are replaced by the next selection transition. |
| `activate` | Adapter calls `onActivate` with the enabled option value. | None. |

#### Part Attribute Output

| Part | Attribute | Value |
|------|-----------|-------|
| Root | `data-scope` / `data-part` | `listbox` / `root` |
| Root | `data-orientation` / `data-disabled` | resolved `orientation` / root disabled state |
| Root | `role` | `listbox` |
| Root | `aria-orientation` | Resolved `orientation`. |
| Root | `aria-label` / `aria-labelledby` | Supplied accessible name. One is required. |
| Root | `aria-disabled` | `true` when globally disabled; otherwise omitted. |
| Root | `aria-multiselectable` | `true` in multiple mode; otherwise omitted. |
| Root | `tabindex` | `0` only when enabled and no enabled option exists; omitted when options own the tab stop; `-1` when disabled. |
| Option | `data-scope` / `data-part` | `listbox` / `option` |
| Option | `data-selected` / `data-focused` | whether the option is selected / owns roving focus |
| Option | `role` | `option` |
| Option | `aria-label` | The item's `label`. |
| Option | `aria-selected` | Whether its value is selected. |
| Option | `aria-disabled` | `true` when the item or Listbox is disabled; otherwise omitted. |
| Option | `tabindex` | `0` for the focused enabled option; `-1` for every other option. |

#### Machinery Dependencies

No shared focus service is required. Core owns the pure navigation and
selection transitions. The adapter translates keys and pointer events, then
executes focus and callback effects.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | A single selection changes | selected `value` | Selection request for controlled usage. |
| `onValuesChange` | Multiple selection changes | selected `values` | Selection request for controlled usage. |
| `onActivate` | Enter or double-click targets an enabled option | option `value` | Does not change selection by itself. |

## 6. Accessibility

### Semantics

- Role: `listbox` on the root and `option` on each row.
- Required attributes: option `aria-label` and `aria-selected`; listbox accessible name.
- Optional attributes: root `aria-orientation` and `aria-multiselectable`; `aria-disabled` on disabled root/options.
- Labeling rules: `ariaLabel` or `ariaLabelledby` names the listbox; item `label` names its option and supplies typeahead text.
- Interactive descendants are prohibited because the option is one focus and selection unit. A development warning supplements this authoring requirement.

### Keyboard

| Key | Behavior |
|-----|----------|
| Down / Up (`vertical`) | Move focus to the enabled next / previous option. In single mode, also select it. |
| Right / Left (`horizontal`) | Move focus to the enabled next / previous option. In single mode, also select it. |
| Home / End | Focus first / last enabled option. In single mode, also select it. |
| Printable character | Move focus to the next enabled label matching the typeahead buffer; buffer resets after 500ms. |
| Space | Select the focused option in single mode; toggle it in multiple mode. |
| Shift + arrow | In multiple mode, move focus and replace selection with the enabled contiguous range from the range anchor. |
| Ctrl/Cmd + A | Select all enabled options in multiple mode. |
| Enter | Call `onActivate` for the focused option. |

Arrow navigation stops at the enabled ends and does not wrap. Disabled options
are excluded from focus, typeahead matches, and range selection. Shift ranges
include enabled endpoints and skip disabled options. The range anchor is the
most recently clicked, toggled, typeahead-matched, or explicitly focused
boundary option; if there is no anchor, range selection starts at the focused
option when Shift navigation begins.

Click selects the clicked option in single mode and only that option in
multiple mode. Ctrl/Cmd+click toggles in multiple mode. Shift+click replaces
multiple selection with the enabled contiguous range from the range anchor.
Double-click calls `onActivate` for an enabled option.

### Focus And Announcement

- Focus entry: the enabled selected option, or the first enabled option when
  there is no enabled selection.
- Focus exit: the browser's next or previous tab stop; Listbox does not trap or
  restore focus.
- Exactly one enabled option has `tabindex="0"`; all other options have
  `tabindex="-1"`. The root is not an additional tab stop while it has enabled
  options. An empty or all-disabled Listbox has one root tab stop; a globally
  disabled Listbox is not tabbable.
- `aria-activedescendant` is not used. Focus moves onto the option element.
- Live-region or announcement behavior: none; role, selected state, and DOM
  focus are exposed through the accessibility tree.
- GPUI-native accessibility mapping: expose the list and option roles, direct
  labels, selected/disabled state, multiple-selection state, and focused
  option. The `ariaLabelledby` ID relationship is not projected under shared
  native accessibility rule 003.

The `items` order is the linear navigation order. A host may arrange card rows
in a CSS grid, but Listbox still navigates that sequence along one axis; 2D
row/column navigation is explicitly out of scope. Use a different interaction
pattern if spatial grid navigation is required.

## 7. Layout

### Sizing

- Minimum size: content-driven width with a zero minimum to allow narrow hosts.
- Maximum size: host constrained.
- Overflow behavior: host owns clipping and scrolling; Listbox adds no scroll container.

### Composition

- Parent expectations: supply a meaningful listbox name and stable option data.
- Child expectations: render one non-interactive row per item through the host renderer.
- Resizing rules: vertical orientation stacks rows; horizontal orientation lays rows in one line. Hosts may override layout while preserving item order.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Option | `--poodle-border-width-focus` | Focus ring width. |
| Option | `--poodle-color-accent-focusRing` | Focus ring color. |

Option selection visuals are host-owned; the renderer receives `selected` and
`focused` state to style rich content without Listbox taking over row design.

### Focus Ring Geometry

| Part | Outline offset | Paint order |
|------|----------------|-------------|
| Option (Svelte/React) | `-0.0625rem` | The focused option paints above its siblings. |
| Option (GPUI) | `-border.width.focus` | The full stroke stays inside the option bounds. |
| Empty root | `0.125rem` | The root retains its outset ring when it owns the tab stop. |

Inset option rings remain visible over host-rendered row backgrounds. GPUI
insets its full focus-ring stroke because it paints options in tree order, so a
later sibling cannot cover the ring. The empty root keeps an outset ring
because it has no option row to overlap.

## 9. Svelte Notes

- Expected substrate: host `items` plus `children(item, selected, focused)` snippet.
- Wrapper strategy: one root listbox and one Listbox-owned option element per item.
- Implementation-only details: event handlers translate into the shared core machine; focus uses the option DOM element.
- Known browser-specific deltas: none.

## 10. GPUI Notes

- Expected crate/module surface: `poodle_specs::ListboxSpec` with `with_*`
  builders, `poodle_headless::listbox` for the machine mirror, and
  `poodle_render::listbox` / `listbox_with_rows` for semantic composition.
- Theme access strategy: use shared focus-ring tokens and host-provided row content.
- Implementation-only details: native focus and selection effects map to the same contract transitions.
- Known GPUI-native deltas: ID-based `ariaLabelledby` is carried by the spec
  but is not projected to AccessKit; direct `ariaLabel` is projected.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] semantic inputs have the same meaning
- [ ] state transitions match
- [ ] event timing and payload meaning match
- [ ] accessibility rules and keyboard behavior match
- [ ] accessible name, role, state, and value exposure match
- [ ] focus order and restoration behavior match when relevant

### Tier 2: Visual Parity

- [ ] token roles match
- [ ] spacing and sizing match within platform limits
- [ ] overall proportions and hierarchy match

### Tier 3: Implementation Freedom

- [ ] implementation-only differences are documented
- [ ] no implementation detail leaks into the public contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI carries `ariaLabelledby` but does not project its ID relationship to AccessKit. | Shared native accessibility rule 003; direct labels and list/option semantics are projected. | Recorded in this contract. | Project ID relationships when the shared native node vocabulary maps stable IDs to AccessKit node IDs. |

## Example

```svelte
<script lang="ts">
  import { Listbox, type ListboxItem } from "@inflatable-cookie/poodle-svelte";

  const items: ListboxItem[] = [
    { value: "mix-a", label: "Mix A" },
    { value: "mix-b", label: "Mix B" },
  ];
</script>

<Listbox {items} selectionMode="multiple" ariaLabel="Saved mixes" onValuesChange={saveSelection}>
  {#snippet children(item, selected, focused)}
    <div data-selected={selected} data-focused={focused}>{item.label}</div>
  {/snippet}
</Listbox>
```

```tsx
<Listbox
  items={items}
  selectionMode="multiple"
  ariaLabel="Saved mixes"
  onValuesChange={saveSelection}
  renderItem={(item, selected, focused) => (
    <div data-selected={selected} data-focused={focused}>{item.label}</div>
  )}
/>
```

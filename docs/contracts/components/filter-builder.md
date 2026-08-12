# FilterBuilder

Status: detailed contract
Updated: 2026-07-15

## 1. Purpose

- Component name: `FilterBuilder`
- Layer: `composites`
- Summary: an anchored popover that lets applications supply arbitrary filter-field
  definitions, build a stack of active filter clauses, and show those clauses as
  editable/removable pills under one bounded `Match all` / `Match any` combinator
- In scope: a compact Filter trigger with active-clause count, a popover for
  adding and editing clauses (field → operator → operand editor → Add), a draft
  that only commits when valid, active clauses rendered as pills via
  `SelectionSummary`, a single root AND/OR combinator, controlled expression value
- Out of scope: evaluating or executing the filter, persistence, URL
  serialization, database queries, network behavior, recursively nested Boolean
  groups, mixed per-pair connectors (`A OR B AND C`), application vocabulary
  (formats, tags, vendors, DAW concepts), clause reordering

A companion to `OrderBy`: where `OrderBy` builds an ordered stack of sort
directives, `FilterBuilder` builds an unordered stack of filter clauses. Poodle
understands fields, operators, operands and Boolean combination; the host owns
evaluation and serialization.

## 2. Anatomy

```text
[Popover .filter-builder-popover] <div>  (position: relative wrapper, carries data-size/data-density)
  ├── [Field .filter-builder] <div role="group">  (single bordered block, flex-wrap: opener + inline pills + reset)
  │   ├── [Trigger .filter-builder__trigger] <button aria-expanded aria-controls aria-haspopup="dialog">
  │   │   ├── [Label .filter-builder__label] <span>  (hidden when compact; "Filter", or the live match mode "All"/"Any" — see Label part)
  │   │   ├── [Summary .filter-builder__summary] <span>  ("Filter" / "N filters"; shown only when pills are NOT shown)
  │   │   └── [Chevron .filter-builder__chevron] <span aria-hidden="true">
  │   ├── [Pill .filter-builder__pill .selection-summary__chip--split] <span> (repeated inline; conditional: showPills && clauses non-empty)
  │   │   ├── [Activate .selection-summary__chip-activate] <button>  (edit clause; aria-label "Edit {clauseLabel}")
  │   │   └── [Remove] IconButton (icon="x", size="xs", ghost; aria-label "Remove {clauseLabel}")
  │   └── [Trailing .filter-builder__trailing] <span> (conditional: clauses non-empty; margin-left:auto, right-aligned)
  │       ├── [Count .filter-builder__count] <span aria-hidden> (badge; when showPills — the single count indicator)
  │       └── [Reset .filter-builder__reset] <span> (when showClearButton)
  │           └── IconButton (icon="x", variant="ghost", ariaLabel="Clear filters")
  └── [Surface .filter-builder__surface > .filter-builder__panel] <div role="dialog"> (rendered inline when open)
      ├── [Combinator .filter-builder__combinator] SegmentedControl (conditional: showCombinator && clauses.length >= 2 && not editing a chip)
      │       └── "Match all" / "Match any"
      ├── [Draft .filter-builder__draft] <div>  (add or edit a clause)
      │   ├── [Field Select] Select (placeholder "+ Add filter"; disabled while editing)
      │   ├── [Operator Select] Select (conditional: a field is chosen and it has > 1 operator)
      │   ├── [Operand Editor] (conditional on operator's operandKind — see §4.4)
      │   └── [Draft Actions .filter-builder__draft-actions] <div>
      │       ├── Button "Add" (adding) or "Update" (editing) — disabled until draft valid
      │       └── Button "Cancel" (conditional: editing, or a field is chosen)
      └── [Empty .filter-builder__empty] <p> (conditional: no clauses and no active draft)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Popover | yes | `position: relative` wrapper anchoring the surface; carries `data-size`, `data-density` | — |
| Field | yes | single bordered `flex-wrap` block holding the opener, inline clause pills, and reset; `role="group"`, carries `data-disabled`, `data-compact`, `data-open` | `--poodle-size-control-height`, `--poodle-radius-control`, `--poodle-color-background-surface`, `--poodle-color-border-default` |
| Trigger | yes | borderless opener button inside the field; shows label + (conditional) summary + chevron; `aria-expanded` / `aria-controls` / `aria-haspopup="dialog"` | — |
| Label | yes | uppercase text, hidden when `compact`. Reads "Filter" normally; when the combinator is live (`showCombinator` + 2+ clauses) it instead reflects the match mode — "All" (`combinator="and"`) or "Any" (`combinator="or"`) — so the mode is visible without opening the popover. Carries `data-combinator`; the mode form is rendered in `text-primary` (vs `text-secondary`) to read as a live value | `--poodle-color-text-secondary` / `--poodle-color-text-primary` (mode) |
| Summary | no | "Filter" placeholder / "N filter(s)" — shown only when pills are NOT displayed (empty, or `showPills=false`); avoids duplicating the count the pills already convey | `--poodle-color-text-primary` / `--poodle-color-text-muted` (empty) |
| Chevron | yes | popover indicator arrow (`▾`) | `--poodle-color-text-secondary` |
| Pill | no | inline clause chip reusing SelectionSummary's split-chip treatment: a separate activation button (the clause label) + a remove IconButton (no nested buttons); flows inline within the field. The field supplies the SelectionSummary chip CSS variables (font/padding/min-height per size/density) since the chip classes are used outside their usual root | (SelectionSummary chip classes) |
| Trailing | no | right-aligned (`margin-left:auto`) group holding the count badge + reset; shown when clauses non-empty | — |
| Count | no | small badge with the active-clause count; the single count indicator, shown when `showPills` (the opener summary text is suppressed to avoid duplicating it) | `--poodle-color-accent-base`, `--poodle-color-text-inverse` |
| Reset | no | `IconButton icon="x" variant="ghost"` (aria-label `"Clear filters"`), the single clear-all, shown when `showClearButton` and at least one clause is active | (IconButton primitive) |
| Surface | yes | anchored `role="dialog"` popover surface (`tabindex="-1"`) containing the combinator + draft editor | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` |
| Combinator | no | `SegmentedControl` choosing `and` / `or`; the mode *switch* shows only when `showCombinator` is true, 2+ clauses exist, **and** the popover was opened from the trigger (the All/Any label) — it is hidden while editing an individual chip, since the combinator combines the whole stack, not one clause. The opener label still reflects the mode in every state | (SegmentedControl contract) |
| Draft | no | the add/edit row: field select, operator select, operand editor, actions | — |
| Operand Editor | no | value editor whose shape depends on the operator's operand kind (§4.4) | (composed primitives) |
| Empty | no | placeholder text ("No filters") when no clauses and no active draft | `--poodle-color-text-secondary` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `fields` | `FilterFieldDefinition[]` | `[]` | yes | host-supplied filter fields the user can choose from |
| `value` | `FilterExpression \| undefined` | `undefined` | no | controlled expression: combinator + committed clauses; when supplied, the source of truth |
| `ariaLabel` | `string` | `"Filter"` | no | accessible name for root group and trigger |
| `disabled` | `boolean` | `false` | no | disables all interactive controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role for inherited sizing |
| `size` | `ControlSize \| null` | `null` | no | explicit size override (`"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"`) |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override (`"compact"`, `"default"`, `"comfortable"`) |
| `dismissOnOutsideInteract` | `boolean` | `true` | no | outside dismissal: a mousedown outside the trigger and surface closes the popover (and drops the draft) |
| `maxClauses` | `number \| null` | `null` | no | maximum simultaneously active clauses; `null` means no limit; when reached the draft add row is hidden |
| `compact` | `boolean` | `false` | no | when true, hides the static "Filter" label in the trigger |
| `showClearButton` | `boolean` | `true` | no | when false the reset `×` IconButton is never rendered |
| `showPills` | `boolean` | `true` | no | when false, active clauses are not rendered as external pills (trigger count only) |
| `showCombinator` | `boolean` | `false` | no | when true, the `Match all` / `Match any` root-combinator toggle appears (only ever with 2+ clauses). Off by default — most filter sets are AND-only, so the toggle is irrelevant noise. The expression still carries a `combinator` (defaults `"and"`); this only gates the UI switch, not the data model |
| `onChange` | `((value: FilterExpression) => void) \| null` | `null` | no | fired on every committed mutation (add, update, remove, clear, combinator change) |

### Naming Rules

Follows Poodle conventions: `camelCase` multi-word props, `on*` handler props,
`size` opts into shared control sizes. `disabled`/`compact` are documented without
the `is` prefix (brevity convention); the Rust spec keeps `is_disabled`/`is_compact`.

### Shared Types

Defined in `@inflatable-cookie/poodle-svelte` `types.ts` and re-exported from the package root;
redefined identically in `@inflatable-cookie/poodle-react`; mirrored in `poodle-specs` (snake_case).

```typescript
type FilterCombinator = "and" | "or";

type FilterFieldKind = "boolean" | "enum" | "multi-enum" | "text" | "number" | "range";

type FilterOperandKind = "none" | "text" | "number" | "boolean" | "options" | "range";

type FilterOperand =
  | { kind: "none" }
  | { kind: "text"; value: string }
  | { kind: "number"; value: number }
  | { kind: "boolean"; value: boolean }
  | { kind: "options"; values: string[] }
  | { kind: "range"; min: number | null; max: number | null };

type FilterOption = { value: string; label: string; disabled?: boolean; group?: string };

type FilterOperatorDefinition = { key: string; label: string; operandKind: FilterOperandKind };

type FilterFieldDefinition = {
  key: string;
  label: string;
  kind: FilterFieldKind;
  operators?: FilterOperatorDefinition[];
  options?: FilterOption[];
  defaultOperator?: string;
  allowMultiple?: boolean;   // default false
  disabled?: boolean;
};

type FilterClause = { id: string; key: string; operator: string; operand: FilterOperand };

type FilterExpression = { combinator: FilterCombinator; clauses: FilterClause[] };
```

Clause `id`s are stable, opaque UI identities. They are required because a field
with `allowMultiple` may hold more than one active clause.

### Standard Operators By Field Kind

Poodle defines operator identifiers and presentation; a field may restrict or
relabel the set via `operators`. Defaults:

| Kind | Operators (key: label) | operandKind |
|------|------------------------|-------------|
| `boolean` | `is`: is | `boolean` (true/false) |
| `enum` | `is`: is · `is_not`: is not | `options` (exactly one) |
| `multi-enum` | `any_of`: is any of · `all_of`: is all of · `none_of`: is none of | `options` (one or more) |
| `text` | `contains` · `not_contains`: does not contain · `equals` · `starts_with`: starts with · `ends_with`: ends with | `text` |
| `number` | `eq`: equals · `neq`: not equal · `gt`: greater than · `gte`: at least · `lt`: less than · `lte`: at most | `number` |
| `range` | `between` · `outside` | `range` (min/max) |

### Controlled And Uncontrolled

- Controlled: provide `value`; the component mirrors edits through `onChange` and
  re-renders from any externally replaced `value`
- Uncontrolled fallback: when `value` is omitted the component owns local
  expression state, still emitting `onChange`
- Default expression when unset: `{ combinator: "and", clauses: [] }`

## 4. States

### 4.1 Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no active clauses | opener shows "FILTER" + "Filter" placeholder (muted); no pills; reset hidden; panel shows "No filters" and the add row |
| populated | one or more clauses | pills render inline in the field; opener summary text suppressed; a count badge + reset sit right-aligned at the field's trailing edge (single count, single clear) |
| compact | `compact=true` | the static "Filter" label is hidden |
| disabled | `disabled=true` | root reduced to disabled opacity; all controls disabled; pills non-interactive |
| popover open | user clicks trigger or a pill | anchored dialog surface appears below the trigger |
| adding | a field is chosen in the draft, no clause being edited | operator + operand editor + "Add"/"Cancel" shown; "Add" disabled until draft valid |
| editing | a pill is activated | draft pre-filled from that clause; field select disabled; "Update"/"Cancel" shown |
| combinator shown | `showCombinator` && `clauses.length >= 2` && `editingId === null` | `Match all` / `Match any` SegmentedControl appears (default off; hidden while editing a chip — the opener label still shows the mode) |
| maxClauses reached | active count equals `maxClauses` and not editing | the add row (field select) is hidden |

### 4.2 Summary Text Logic

- Empty: `"Filter"` (placeholder styling)
- One clause: `"1 filter"`
- N clauses: `"N filters"`

### 4.3 Draft Lifecycle (adapter-owned, plain state)

The draft is local and never emitted until valid:

1. **idle** — no field chosen; the field Select shows "+ Add filter"
2. **choosing operand** — a field is chosen → operator defaults to the field's
   `defaultOperator` (or first operator); operand seeded blank for the operator's
   `operandKind`; "Add" disabled
3. **valid draft** — operator + operand are complete (`isClauseComplete`) → "Add"
   enabled
4. **commit** — "Add" appends a clause with a freshly generated `id`; draft resets
   to idle; `onChange` fires
5. **editing** — activating a pill loads its clause into the draft (retaining
   `id`); field Select is disabled; "Update" commits the edited clause in place;
   "Cancel" discards without emitting

Changing the operator re-seeds the operand when the new operator's `operandKind`
differs from the current operand's kind. Incomplete drafts are never emitted.

### 4.4 Operand Editors By operandKind

| operandKind | Editor | Valid when |
|-------------|--------|------------|
| `none` | (none) | always |
| `boolean` | SegmentedControl / toggle: "True" / "False" | always (defaults true) |
| `text` | Input (text) | trimmed length > 0 |
| `number` | NumberInput | value is finite |
| `options` | Select (enum, single) or multi-select (multi-enum) driven by `field.options` | at least one value selected |
| `range` | two NumberInputs (min, max) | at least one bound set |

### 4.5 Behavior Machine

Behavior classification: `styled-only (no machine)` — adapter-owned interaction.

Like `OrderBy`, the popover open/dismiss uses the shared dismissable-layer stack
(outside dismissal guarded by `dismissOnOutsideInteract`, default `true`)
and the draft/clause logic is plain component state; there is no `@inflatable-cookie/poodle-core`
state machine and no conformance vectors. The pure model helpers
(`defaultOperatorsForKind`, `isClauseComplete`, `clauseLabel`, …) are the
cross-renderer semantic reference and live in `filter-builder-model.ts` (TS) and
as methods on `FilterBuilderSpec` (Rust).

#### Machinery Dependencies

Dismissable layer (`registerDismissLayer` from `@inflatable-cookie/poodle-core`), presentation
context (size/density), id wiring for the dialog surface. No focus trap, no
roving tabindex.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onChange` | any committed mutation: add clause, update clause, remove clause, clear all, combinator change | `FilterExpression` | never fired for an in-progress draft |

## 6. Accessibility

### Semantics

| Element | Attribute | Value |
|---------|-----------|-------|
| Root | `role` | `"group"` |
| Root | `aria-label` | from `ariaLabel` (default `"Filter"`) |
| Root | `data-disabled` | `"true"` when disabled |
| Trigger | `aria-label` | from `ariaLabel`, suffixed with active count (e.g. `"Filter, 3 active"`) |
| Trigger | `aria-haspopup` | `"dialog"` |
| Trigger | `aria-expanded` | `"true"` when open |
| Trigger | `aria-controls` | dialog surface id when open |
| Trigger | `disabled` | native attribute when `disabled` |
| Count badge | `aria-hidden` | `"true"` (count is conveyed in the trigger accessible name) |
| Dialog surface | `role` | `"dialog"` |
| Dialog surface | `aria-label` | `"Edit filters"` (or `ariaLabel`) |
| Dialog surface | `tabindex` | `"-1"` |
| Combinator | `ariaLabel` | `"Combine filters"`; options named "Match all" / "Match any" with selected state |
| Field Select | `ariaLabel` | `"Add filter field"`; placeholder `"+ Add filter"` |
| Operator Select | `ariaLabel` | `"Operator for {fieldLabel}"` |
| Operand editor | `ariaLabel` | `"Value for {fieldLabel}"` (range: "Minimum/Maximum for {fieldLabel}") |
| Add/Update button | text | `"Add"` / `"Update"`; disabled until draft valid |
| Reset IconButton | `ariaLabel` | `"Clear filters"` |
| Pills (inline split chips) | per chip | activation control `aria-label="Edit {clauseLabel}"`; remove control `aria-label="Remove {clauseLabel}"` |

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter`/`Space` on trigger | toggles the popover |
| `Enter`/`Space` on a pill's activation control | opens the popover editing that clause |
| `Tab` | moves between controls (child components own their focus rings) |
| `Escape` | dismisses the popover and returns focus to the invoking control |
| (operand editors) | each field, operator and operand editor is keyboard-reachable via its primitive |

### Focus And Announcement

- focus entry: on open, focus moves to the first focusable control in the surface
  (the combinator when shown, else the field/operator select)
- focus exit: Escape and outside-dismiss return focus to the trigger (or the pill
  that opened an edit)
- no nested interactive elements — pills use separate activation and remove
  controls (see SelectionSummary extension)
- invalid draft: the Add/Update button is disabled rather than announcing an error
- GPUI-native accessibility mapping: no ARIA API — accessible names map to native
  exposure where available; documented as an accepted delta

## 7. Layout

### Sizing

- Popover: `position: relative`, `display: flex`, `flex-direction: column`,
  `width: 100%`, `min-width: 0`
- Trigger: `inline-flex`, `flex: 1`, `min-height` from
  `var(--poodle-size-control-height)` (size-stepped), ellipsis summary overflow
- Surface: portalled and viewport-positioned (`002-anchored-overlays.md`),
  requesting `bottom-start` with an `8px` offset; `min-width: 16rem`,
  `max-width: min(24rem, 90vw)`, `top: calc(100% + 0.5rem)`
- Pills: flow inline inside the single field block, filling the row beside the
  opener and wrapping to further rows only when needed; long labels ellipsis at a
  max width. The field grows vertically as pills wrap

### Composition

- parent expectations: toolbar areas, list headers, filter panels, data-table
  toolbars — composes cleanly beside `OrderBy` and does not extend `FilterToolbar`
- child expectations: SelectionSummary chip treatment for pills (reused inline,
  not the section component), `SegmentedControl` (combinator +
  boolean operand), `Select`, `Input`, `NumberInput`, `Button`, `IconButton`
- the popover surface is owned locally by `FilterBuilder`, not by `Popover`

## 8. Token Usage

References semantic roles; reuses `OrderBy` trigger/surface treatment and
`SelectionSummary` pill treatment rather than duplicating them.

| Part | Token | Purpose |
|------|-------|---------|
| Trigger | `--poodle-size-control-height` | size-stepped height |
| Trigger | `--poodle-radius-control` | corner radius |
| Trigger | `--poodle-color-background-surface` | fill |
| Trigger | `--poodle-color-border-default` | border |
| Trigger hover | `color-mix(surface 84%, elevated)` | hover fill |
| Trigger focus | `--poodle-color-accent-focusRing`, `--poodle-border-width-focus` | focus ring |
| Label | `--poodle-color-text-secondary` | uppercase label |
| Summary placeholder | `--poodle-color-text-muted` | empty state |
| Pill | (SelectionSummary chip classes) | inline clause chip treatment |
| Chevron | `--poodle-color-text-secondary` | indicator |
| Surface | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` | anchored panel |
| Draft actions | (Button primitive) | Add/Update/Cancel |
| Empty | `--poodle-color-text-secondary` | placeholder |

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Popover, Root | `"xs"`–`"xl"` |
| `data-density` | Popover, Root | `"compact"`, `"default"`, `"comfortable"` |
| `data-disabled` | Root | `"true"` / `"false"` |
| `data-compact` | Root | `"true"` / `"false"` |
| `data-placeholder` | Summary | `"true"` when empty |
| `data-combinator` | Label | `"true"` when the label shows the live match mode (All/Any) |
| `data-open` | Root | `"true"` when the popover is open |

## 9. Svelte Notes

- owns its open state and anchored panel surface directly (mirrors `OrderBy`)
- imports `registerDismissLayer` from `@inflatable-cookie/poodle-core`; dismiss on outside
  interact / Escape via a `$effect` guarded by `open`
- size/density resolve via `getUiPresentation` + `resolveSemanticControlSize`
- controlled/uncontrolled via `$bindable(value)` + a `sync()` that writes `value`
  (controlled) or local state (uncontrolled) then calls `onChange`
- draft is local `$state`; committed only through `isClauseComplete`
- pure logic imported from `./filter-builder-model`
- clause pills rendered inline in the field, reusing SelectionSummary's
  split-chip classes (`poodle-selection-summary__chip--split` + `__chip-activate`)
  for a single CSS source — not the SelectionSummary section component, so the
  pills flow inline in the single-block field
- clause ids generated from a module counter: `${fieldKey}-${++n}`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::filter_builder`
- theme access via `GpuiThemeProvider`; all dimensions/colors resolved from tokens
- the render is a faithful function of the **full** spec state: `FilterBuilderSpec`
  carries `draft: Option<FilterDraft>` (adding vs editing), so GPUI + Jetstream
  render the complete draft editor (operator select + operand editor per kind +
  Add/Update/Cancel) and gate the combinator on `!is_editing()`, matching
  Svelte/React for equivalent states. The one native limitation (shared by all
  components) is that the preview does not itself drive clicks — live open / select
  / add / edit / remove is host-event-loop work; anchored positioning is
  platform-owned (surface renders inline)
- no ARIA API — accessible-name intent documented as an accepted delta

## 10a. Jetstream Notes

- `FilterBuilder::from_spec(spec, theme)` then the intent surface:
  `.on_remove(id)`, `.on_reset()`, `.on_toggle()` (opener pressed),
  `.on_picker_toggle("add-field" | "operator" | "operand")`,
  `.on_field_pick(key)`, `.on_operator_change(key)`,
  `.on_operand_change(option)`, `.on_combinator_change("and" | "or")`,
  `.on_commit()`, `.on_cancel()`.
- The contract's `onChange` carries a whole clause list, and a pointer produces
  one intent on one control — so, as with `OrderBy`, the events name the intent
  and the host rebuilds the expression and draft it already holds.
  `on_operand_change` reports the option pressed (`"true"`/`"false"` for
  boolean operands, the option value for enum and multi-enum); the host flips
  membership itself.
- `FilterBuilderSpec.open_picker: Option<FilterBuilderPicker>` says which
  nested Select inside the panel shows its option list — native-only state
  that the web's Selects keep internal. The host flips it from
  `on_picker_toggle`.
- Known Deltas: the typed operands (text, number, range) stay host-side — the
  runtime raises no key events — and drag-to-collapse has no drag surface
  here.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `fields` accepted; all six field kinds representable
- [ ] `value` controlled; external replacement re-renders
- [ ] draft never emits an incomplete clause
- [ ] add / update / remove / clear mutate the expression and fire `onChange`
- [ ] `Match all` / `Match any` combinator; opt-in via `showCombinator` (default off), only with 2+ clauses, and switch hidden while editing a chip (opener label still reflects the mode). Native renders the overview state only (no interactive edit mode)
- [ ] multi-enum any/all/none semantics preserved via operator keys
- [ ] `allowMultiple` governs duplicate-field clauses (default single)
- [ ] `maxClauses` hides the add row when reached
- [ ] pills activate (edit) and remove independently, no nested buttons
- [ ] single-block field: pills inline (chip vars supplied by the field); one count badge + one clear, right-aligned; summary text only when pills hidden (no duplicate count)
- [ ] disabled suppresses all interaction
- [ ] no domain vocabulary; no expression evaluation

### Tier 2: Visual Parity

- [ ] trigger dimensions/border/radius/background match `OrderBy` treatment
- [ ] trigger hover / focus ring match
- [ ] label hidden in compact mode
- [ ] surface anchoring, radius, elevation match
- [ ] pills match `SelectionSummary` chip treatment
- [ ] all five sizes match the size ladder
- [ ] all three densities match

### Tier 3: Implementation Freedom

- [ ] popover/animation/portal behavior is platform-owned
- [ ] operand-editor primitive internals are platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI/Jetstream preview doesn't drive live clicks | native render is a faithful function of the full spec state (draft editor + edit-scoping rendered), but the preview event loop doesn't dispatch open/select/add/edit/remove — shared render-only posture across all components | accepted | host wires interaction; nothing FilterBuilder-specific left |
| No nested Boolean groups | v1 scope; single commutative combinator only | accepted (by design) | forward-compatible expression shape retained for a future grouped-node model |
| Clause `id` generated per-instance counter | opaque UI identity only; host may replace ids on round-trip | accepted | documented risk; host owns durable identity |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending review
- downstream adopters: Soundcheck (Scan-tab plugin catalogue filtering), Loophole
- future follow-up: grouped/nested clauses; operator extensibility conventions

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): empty builder; single
enum filter; multiple filters with Match all; multiple filters with Match any;
multi-enum any/all/none; boolean; text; numeric comparison; range; editing a
clause; removing a clause; clearing all; a field with `allowMultiple`; disabled;
max-clause state; overflowing pill summary; full size ladder; density variants;
controlled value with live serialized output.

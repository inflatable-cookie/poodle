# NumberInput

Status: active — g16.030 value/draft/mounted parity on worker PR; awaiting review/merge
Updated: 2026-08-29

## 1. Purpose

- Component name: `NumberInput`
- Layer: `foundation`

`NumberInput` edits one finite numeric value. Its public model separates the
committed application value from the transient text a person is editing.

This is one numeric component, not a number-or-string union:

- the committed value is `number | null` on web and `Option<f64>` in Rust;
- the optional draft channel carries raw text such as `"-"`, `"01.20"`, or an
  empty field while editing; and
- prefix/suffix text, validation, precision, and steppers decorate that numeric
  model without changing its value type.

The pre-1.0 migration in `g16.030` removes the old string-value mode and the
old source-specific step callbacks. No alias or compatibility path remains.

## 2. Public Contract

- Web imports: `@inflatable-cookie/poodle-svelte` and
  `@inflatable-cookie/poodle-react`
- Rust spec: `poodle_specs::NumberInputSpec`
- Shared renderer: `poodle_render::number_input`

### Core Props

| Prop | Type | Default | Meaning |
| --- | --- | --- | --- |
| `value` | `number | null | undefined` | `undefined` | controlled committed value; `null` is committed empty |
| `defaultValue` | `number | null` | `null` | initial committed value for uncontrolled web use |
| `draftValue` | `string | null | undefined` | `undefined` | optional controlled raw draft; `null` means no draft override |
| `min` | `number | null` | `null` | optional inclusive lower bound |
| `max` | `number | null` | `null` | optional inclusive upper bound |
| `step` | `number | null` | `null` | optional positive finite step; omitted means `1` |
| `precision` | `number | null` | `null` | optional non-negative maximum fractional digits and fixed committed display scale; max `324` |
| `prefix` | `string | null` | `null` | non-value text before the editor |
| `suffix` | `string | null` | `null` | non-value text after the editor |
| `validate` | `InputValidator | undefined` | `undefined` | optional async validation of a committed canonical decimal string |
| `validationContext` | `unknown` | `undefined` | host context forwarded to `validate` |
| `validationState` | `ValidationState` | `"none"` | externally supplied validation presentation |
| `showSteppers` | `boolean` | `false` | show increment/decrement buttons |

Standard control props remain `id`, `name`, `placeholder`, `disabled`,
`readOnly`, `required`, `ariaLabel`, `describedBy`, `size`, `sizeRole`, and
`density`.

`value`, `defaultValue`, `min`, `max`, and `step` do not accept strings.
Non-finite committed values or bounds, non-positive/non-finite steps,
non-integer/negative precision, precision above `324` (the maximum fractional
scale needed to express a finite IEEE-754 double without exponent syntax), and
`min > max` are invalid authored configuration. The semantic machine reports
invalid configuration and produces no mutation effect; adapters must not invent
a replacement value.

### Draft Ownership

- With `draftValue` undefined, the web adapter owns the transient draft.
- With `draftValue` authored, the host owns it and applies
  `onDraftValueChange` results.
- Rust's declarative spec carries `draft_value: Option<String>` and its host
  wrapper stores draft/caret/focus state between rebuilds.
- `draftValue=null` / `draft_value=None` renders the formatted committed value
  or the placeholder when committed empty.
- An external controlled-value replacement discards an uncontrolled draft. A
  host echo of the value just emitted by the active edit does not erase that
  edit's draft.

The ownership mechanism may differ, but the observable draft/value transitions
must match.

## 3. Value And Draft Semantics

### Portable Decimal Syntax

Direct entry accepts a base-10 decimal with an optional leading minus sign,
digits, and at most one `.` separator. Leading zeroes and a trailing decimal
separator are allowed in the draft. Exponents, radix prefixes, grouping,
locale separators, whitespace, `NaN`, and infinity are not portable syntax.

`""`, `"-"`, `"."`, and `"-."` are incomplete drafts. Empty is also the
explicit clear gesture; it emits committed `null` but remains raw draft text
until commit or replacement.

### Validity

A complete draft can change the committed value only when it:

1. parses to a finite base-10 number;
2. contains no more fractional digits than `precision`, when present;
3. falls within inclusive `min`/`max`; and
4. aligns to `step`, anchored at `min` when present or zero otherwise.

Step and precision checks use decimal-safe normalization in the paired shared
semantics, not renderer-specific binary-float epsilon guesses.

Incomplete, malformed, non-finite, over-precision, out-of-range, and off-step
drafts remain visible while editing, expose invalid draft state, and emit no
committed value.

### Editing Transitions

| Event | Result |
| --- | --- |
| raw text edit | report the exact draft; emit a value only when the complete draft is constraint-valid |
| clear whole field | report draft `""` and committed `null` |
| Enter on valid/current draft | resolve the draft and fire `onCommit` with the committed value |
| Enter on unresolved draft | no value or commit callback; keep editing invalid draft |
| blur on valid/current draft | resolve the draft and fire `onCommit` |
| blur on unresolved draft | discard it and restore the last committed display; no value or commit callback |
| Escape | discard the active draft and restore the last committed display; no value or commit callback |
| external committed replacement | discard an uncontrolled draft and display the authored value |
| Arrow Up/Down or stepper | step from a valid draft when present, otherwise committed value; report value, normalized draft, and commit |
| Home/End | move to a finite valid minimum/maximum respectively; otherwise inert |
| disabled/read-only interaction | inert through text, key, pointer, clear, and step routes |

Direct editing never silently clamps or snaps. Step controls stop at bounds and
never fire when their next result would be invalid. An empty step baseline is
`min` when present or zero otherwise.

When `precision` is present, a draft with too many fractional digits is invalid
rather than silently rounded. Resolved committed values display with exactly
that many digits; without precision they use the shortest canonical decimal
form produced by the shared semantics.

### Behavior Machine

Behavior classification: machine-backed.

Paired pure TypeScript/Rust semantics own:

- authored configuration validation;
- decimal draft classification and parsing;
- committed formatting and precision checks;
- bound and step alignment;
- increment/decrement/Home/End results;
- clear, Enter, blur, Escape, and external replacement transitions; and
- the resulting draft/value/commit effects.

Adapters own DOM/native events, focus and caret storage, drawing, async
validation orchestration, and accessibility projection. The pure machine runs
no callbacks, timers, focus operations, I/O, or renderer code.

## 4. Callbacks

| Callback | Payload | When it fires |
| --- | --- | --- |
| `onDraftValueChange` | `string | null` | every raw draft edit; `null` asks a controlled host to discard its draft override |
| `onValueChange` | `number | null` | a distinct valid committed value forms, or the field is cleared |
| `onCommit` | `number | null` | valid Enter/blur or a successful step action resolves the current value |
| `onValidationChange` | validation result | async validation state changes for a committed value |
| `onFocus` / `onBlur` | platform focus event | web-only focus observation; not portable value semantics |

Callbacks are silent when the semantic result is unchanged, except
`onCommit`, which reports an explicit valid commit boundary. Async `validate`
runs only for a non-null committed value and receives its canonical decimal
string. Empty returns validation to idle.

The old `onSubmit`, `onIncrement`, and `onDecrement` callbacks are removed by
the clean migration. `onCommit` replaces the useful persistence boundary;
`onValueChange` already reports the result of a step.

## 5. Usage

### Ordinary Numeric Binding

```svelte
<script lang="ts">
  import { NumberInput } from "@inflatable-cookie/poodle-svelte";

  let quantity: number | null = 1;
</script>

<NumberInput bind:value={quantity} min={0} max={100} showSteppers />
```

### Host-Owned Form Draft

```svelte
<script lang="ts">
  let year: number | null = 2026;
  let yearDraft: string | null = null;
</script>

<NumberInput
  name="year"
  bind:value={year}
  bind:draftValue={yearDraft}
  min={1900}
  max={2100}
/>
```

The rendered input's `name` submits its current text through ordinary browser
form behavior. Consumers that need raw form state bind `draftValue`; they do
not turn the committed value back into a string union.

## 6. Accessibility

- The editable root exposes spin-button semantics and requires an accessible
  name from an associated label or `ariaLabel`.
- `aria-valuenow` reflects the current constraint-valid draft when available,
  otherwise the committed value; it is absent when committed empty and the
  draft is unresolved.
- `aria-valuemin` / `aria-valuemax` reflect finite authored bounds.
- Unresolved drafts and external invalid state expose `aria-invalid="true"`.
- Pending validation exposes `aria-busy="true"`.
- `describedBy` maps to `aria-describedby`.
- Stepper buttons are labelled Increment and Decrement and are disabled at
  bounds, while read-only, or while the whole component is disabled.
- The field owns one focus ring. Stepper focus must not draw a second competing
  control ring.
- GPUI exposes the editable node as `SpinButton`, projects current value and
  finite bounds, and routes real text/key/focus dispatch through the node
  backend.

## 7. Layout And Presentation

- The control stretches to its parent width and follows the shared control
  size and density axes.
- Prefix and suffix are presentation only and never become part of draft or
  committed values.
- Optional steppers occupy the trailing edge without changing the editor's
  semantic identity.
- Disabled, read-only, invalid, pending, and focused states use existing
  semantic tokens and the component Recipe surface. No component-specific
  theme colors are introduced by the value-model migration.

## 8. Active-Cohort Notes

### Svelte And React

- Both adapters use the paired pure semantic machine.
- Both keep an uncontrolled raw draft locally and support the same optional
  controlled draft channel.
- The input remains text-based with decimal input mode so partial drafts can be
  represented consistently; adapters provide explicit spin-button semantics.
- Controlled prop echoes do not erase an active draft, while genuinely
  external replacement does.

### Shared Rust And GPUI

- `NumberInputSpec` uses optional committed/default values and an optional raw
  draft; infinite sentinels are removed from public state.
- Shared rendering produces one editable value node with text, selection,
  focus, submit, cancel, and replacement-text channels rather than a static
  label plus pointer-only steppers.
- The GPUI host wrapper retains draft, selection, and focus between rebuilds
  and applies the same transition results as web.
- ColorPicker and FilterBuilder are regression consumers of the clean spec;
  they do not get component-specific fallback behavior.

### Jetstream

Jetstream remains program-deferred. It receives only mechanical compile
maintenance required by the renderer-neutral spec and handler migration.
Typing or parity is not claimed until backend admission.

## 9. Parity Checklist

### Tier 1: Strict Parity

- [x] committed `number | null` and raw draft channels match
- [x] syntax, precision, bounds, and step validity match
- [x] live valid change, clear, Enter, blur, Escape, and external replacement match
- [x] Arrow/stepper/Home/End results and disabled/read-only inertia match
- [x] callback timing and payloads match
- [x] controlled and uncontrolled draft ownership produces the same observable result
- [x] accessible name, value, bounds, invalid, busy, and focus semantics match

### Tier 2: Visual Parity

- [ ] control geometry, affixes, steppers, validation, and focus treatment use the existing recipe/tokens
- [ ] all size and density axes remain coherent

### Tier 3: Implementation Freedom

- [ ] DOM versus GPUI text/caret mechanisms remain adapter-owned
- [ ] async validation scheduling remains adapter-owned behind the same callback boundary

## 10. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
| --- | --- | --- | --- |
| focus event object callbacks are web-only | platform event objects are not portable value semantics | allowed | native exposes observable focus through its host/state channels |
| Jetstream editing evidence is deferred | backend admission is program-deferred | allowed | do not report it as passing or complete |

## 11. Migration Boundary

The `g16.030` worker must inventory every in-repository string-value and old
callback use, migrate Poodle's own composites/specimens/tests, and record a
downstream migration table for inspected sibling consumers. It must not edit
sibling repositories.

Required clean removals:

- string branches from `value`, `defaultValue`, `min`, `max`, and `step`;
- value-mode inference and string-coercion helpers;
- silent `parseStep(...)->1`, clamping, snapping, and numeric fallbacks for
  invalid drafts/configuration;
- `onSubmit`, `onIncrement`, and `onDecrement`; and
- the static, concrete-`f64`, pointer-only native editor path.

# Token Input

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `TokenInput`
- Layer: `foundation`
- Summary: tokenizing text entry control for small multi-value lists such as
  tags, labels, and keywords; recognizes separator boundaries and renders each
  committed value as an inline pill
- In scope: controlled token arrays, repeated hidden form inputs, separator
  parsing, dedupe policy, per-token removal, keyboard commit behavior, read-only
  and disabled states, size and density inheritance
- Out of scope: async suggestions, arbitrary object chips, drag reorder,
  autocomplete menus, remote lookup, and relation search workflows

## 2. Anatomy

```text
[Root .token-input] <div>
  ├── [Hidden Inputs] <input type="hidden"> (optional, one per committed token when name is set)
  └── [Token Row .token-input__tokens]
      ├── [Token .token-input__token]*
      │   └── [Pill]
      │       ├── [Token Label .token-input__token-label]
      │       └── [Remove Button .token-input__remove] (optional)
      └── [Input Control .token-input__control] <input type="text">
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | field chrome container with border, radius, focus ring, and inherited size/density treatment |
| Hidden Inputs | no | repeated form payload slots when `name` is provided |
| Token Row | yes | wrapping flex row that holds committed tokens and the live text control |
| Token | no | wrapper around each committed token pill |
| Token Label | no | visible token text inside the pill; long values wrap instead of overflowing |
| Remove Button | no | clear affordance for a committed token; omitted in disabled or read-only mode |
| Input Control | yes | plain text entry used for the next token draft |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | `""` | no | input id for label association |
| `values` | `string[]` | `[]` | no | controlled committed tokens; bindable |
| `name` | `string \| undefined` | `undefined` | no | when set, emits one hidden input per token for native form submission |
| `placeholder` | `string \| null` | `null` | no | placeholder shown only in the live text control |
| `disabled` | `boolean` | `false` | no | disables entry and token removal |
| `readOnly` | `boolean` | `false` | no | keeps values visible but blocks editing and removal |
| `required` | `boolean` | `false` | no | forwarded to the live text input only; callers should treat overall required validation as form-owned |
| `spellcheck` | `boolean \| undefined` | `false` | no | forwarded to the live text input |
| `autocapitalize` | `string \| undefined` | `"none"` | no | forwarded to the live text input |
| `autocomplete` | `string \| undefined` | `"off"` | no | forwarded to the live text input |
| `ariaLabel` | `string \| null` | `null` | no | direct label for the live input when no field wrapper exists |
| `describedBy` | `string \| null` | `null` | no | forwarded to the live input |
| `size` | `ControlSize \| null` | `null` | no | explicit size override; otherwise resolves from presentation context |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role used when `size` is null |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `separators` | `string[]` | `[","]` | no | token commit delimiters; each non-empty character becomes part of the split set |
| `dedupe` | `boolean` | `true` | no | prevents duplicate committed values when true |
| `commitOnBlur` | `boolean` | `true` | no | commits the current draft token on blur |
| `maxLength` | `number \| null` | `null` | no | forwarded to the live text input |
| `resolveToken` | `(value: string, values: string[]) => string \| null \| undefined` | `undefined` | no | per-token validation/transform hook run on commit; receives the trimmed draft and current committed values; return a non-empty string to accept (optionally transformed), or any non-string (`null`/`undefined`) to reject the draft |
| `onValuesChange` | `(values: string[]) => void` | `undefined` | no | fires whenever committed token values change |
| `onTokenReject` | `(value: string) => void` | `undefined` | no | fires with the trimmed draft when `resolveToken` rejects it (returns a non-string or an empty string after trim) |

## 4. Behavior Rules

### Commit Semantics

- separator characters commit the token immediately
- `Enter` commits the draft token and prevents form submission when a draft is
  present
- `Tab` commits the draft token before focus leaves the field
- blur commits the draft token when `commitOnBlur=true`
- empty tokens are discarded after trimming
- whitespace around tokens is trimmed before commit
- when `resolveToken` is supplied, the trimmed draft is passed through it before
  commit; a returned non-empty string (trimmed again) becomes the committed
  value, while a non-string return or an empty result rejects the draft, drops
  it, and fires `onTokenReject` with the trimmed draft

### Removal Semantics

- clicking the token remove button removes that token
- `Backspace` with an empty draft removes the last committed token
- removal is disabled when `disabled=true` or `readOnly=true`

### Duplicate Policy

- when `dedupe=true`, committed tokens preserve first occurrence order and
  ignore later duplicates
- when `dedupe=false`, repeated tokens are allowed

### Form Submission

- when `name` is present, each committed token renders a hidden input with the
  same name and that token value
- the live draft text is never submitted unless it has been committed first
- consumers should use `formData.getAll(name)` rather than `formData.get(name)`
  when reading the component payload

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via core machinery (g11 extraction sweep): token merging
with dedupe (`mergeTokens`), separator-driven splitting with remainder
(`splitTokenInput`), and the backspace-removes-last-chip guard
(`tokenBackspaceRemoves`) live in `@inflatable-cookie/poodle-headless`. Token
resolution/rejection hooks stay adapter-side (app-defined); the pointer
listener is local focus routing.

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral field chrome with wrapped tokens and draft input |
| focus | focus within live input | border, fill, and shadow switch to focus treatment |
| disabled | `disabled=true` | opacity reduced, input disabled, remove buttons hidden |
| read-only | `readOnly=true` | input remains non-editable, remove buttons hidden |
| empty | `values=[]` and empty draft | only placeholder input is visible |
| populated | one or more committed values | committed values render as pills before the draft input |
| wrapped | narrow width or many tokens | tokens wrap onto additional lines without breaking field chrome |
| long value | a committed token is wider than the available field | token text wraps inside the pill without forcing horizontal overflow |

## 6. Callbacks

| Callback | When It Fires | Payload |
|----------|---------------|---------|
| `onValuesChange` | committed token list changes | `string[]` |
| `onTokenReject` | `resolveToken` rejects a draft (non-string or empty result) | `string` (trimmed draft) |

## 7. Accessibility

### Semantics

- root uses a plain `div`; callers should normally wrap the control in `Field`
  or another labeled form surface
- live input is a native `<input type="text">`
- `aria-label` comes from `ariaLabel` when provided
- `aria-describedby` comes from `describedBy` when provided
- hidden inputs are form payload only and not user-facing
- token remove buttons expose `aria-label="Remove <token>"`

### Keyboard

| Key | Behavior |
|-----|----------|
| printable characters | edit the live draft token |
| `Enter` | commit draft token if non-empty |
| `Tab` | commit draft token if non-empty, then allow focus to move |
| `Backspace` on empty draft | remove last token |
| browser text navigation shortcuts | operate on the live draft input |

### Screen Reader Expectations

- callers should provide a visible label through `Field` or an equivalent form
  wrapper
- token remove buttons must remain reachable by keyboard when editable
- committed token values should remain available in DOM text, not only icon or
  custom paint

## 8. Styling And Tokens

- inherits size and density from `UiPresentationProvider` unless overridden
- uses the same interactive subtle field treatment family as `TextInput`
- committed tokens are rendered with `Pill` and must remain visually secondary
  to the active draft input
- root fills available width by default and shell clicks focus the draft input
  when editable
- token text must wrap safely inside narrow widths without forcing layout overflow

## 8a. Jetstream Notes

- `TokenInput::from_spec(spec, theme).on_remove(...)`, carrying the token's
  **text**. A host removing by index would delete the wrong token whenever two
  removals arrive between renders.
- A disabled or read-only field draws no remove control at all.
- Entry is typing, which the runtime does not raise, so removal is the only
  wired route.

## 9. Usage Notes

- use `TokenInput` for light multi-string entry only
- use `RelationPicker` or relation-selector patterns when values come from real
  entities or remote search
- use repeated hidden inputs plus `formData.getAll()` when integrating with
  native forms
- prefer explicit caller-owned validation messages for required token sets or
  invalid business rules

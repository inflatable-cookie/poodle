# SlugField

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `SlugField`
- Layer: `composites`
- Summary: a form field that auto-generates a URL-safe slug from a source string, with manual override and reset capability
- In scope: auto-slugification from source, manual override detection, reset to auto, prefix display, validation integration, max length
- Out of scope: server-side uniqueness checking, custom slugify algorithms, real-time URL preview, copy-to-clipboard

## 2. Anatomy

```text
[Root]
  └── [Field]  (label, description, error, validation)
        └── [Row]
              ├── [TextInput]  (with prefix, displays slug value)
              └── [Reset Button]  (only when manual override is active)
  └── [Hint]  (only when manual override is active, shows auto-slug)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Grid container with `space-stack-sm` gap |
| field | `Field` | Wraps label, description, error, and validation state |
| row | `<div>` | Flex row holding the text input and optional reset button |
| text-input | `TextInput` | Displays slug value with prefix; accepts manual edits |
| reset-button | `<button>` | Appears when user has manually overridden; resets to auto-slug |
| hint | `<p>` | Shows the auto-generated slug when manual override is active |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | -- | yes | Input ID for label association |
| `label` | `string` | `"Slug"` | no | Field label text |
| `source` | `string` | `""` | no | Source string to auto-generate slug from |
| `value` | `string` | `""` | no | Current slug value (two-way bindable) |
| `placeholder` | `string` | `"auto-generated-slug"` | no | Input placeholder text |
| `isManualOverride` | `boolean` | `false` | no | Whether user has manually edited the slug (two-way bindable) |
| `isDisabled` | `boolean` | `false` | no | Disables the input and reset button |
| `validationState` | `ValidationState` | `"none"` | no | Validation visual state |
| `error` | `string \| null` | `null` | no | Error message text |
| `description` | `string \| null` | `"URL-safe identifier auto-generated from the title."` | no | Help text below label |
| `maxLength` | `number \| null` | `null` | no | Maximum character length for the slug |
| `prefix` | `string \| null` | `null` | no | URL prefix displayed before the slug input; defaults to `"/"` |

### Types

```ts
type ValidationState = "none" | "valid" | "invalid";
```

### Slots

None.

### Controlled / Uncontrolled

`value` and `isManualOverride` both support two-way binding. When `isManualOverride` is false, `value` is automatically derived from `source`.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| auto | `isManualOverride=false` | Input shows auto-generated slug; no reset button or hint |
| manual-override | User edits slug manually | Reset button appears; hint shows auto-slug value |
| disabled | `isDisabled=true` | Input and reset button disabled |
| invalid | `validationState="invalid"` | Error styling on input, error message shown |
| valid | `validationState="valid"` | Success styling on input |
| reset-hover | Mouse over reset button | Background and border darken slightly |
| reset-focus | Focus-visible on reset button | Focus ring |

### Component States

| State | Description |
|-------|-------------|
| auto-tracking | Slug automatically follows changes to `source` |
| manual-override | User has edited the slug; auto-tracking paused until reset |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `change` | Slug value changes (manual edit or reset) | `{ value: string; isManual: boolean }` |

## 6. Accessibility

### Semantics

- Uses `Field` primitive for label-input association via `id`
- `TextInput` receives `describedBy` from `Field` for description/error linking
- Reset button has `aria-label="Reset to auto-generated slug"`

### Keyboard

- Standard text input keyboard behavior
- Tab navigates from input to reset button (when visible)
- Enter/Space on reset button returns to auto-generated slug

### Focus

- Input focus managed by `TextInput` primitive
- Reset button: `border-width-focus` solid `accent-focusRing`, offset `0.125rem`

## 7. Layout

### Sizing

- Root: grid layout, gap `space-stack-sm`
- Row: flex row, centered, gap `0.5rem`
- TextInput: flex 1
- Reset button: inline-flex, height `size-control-height`, padding `0 0.625rem`, font-size `0.75rem`
- Hint: font-size `0.75rem`, code snippet inside uses `typography-code-family` at `0.6875rem`

### Composition

Composes `Field` and `TextInput` from primitives. The `TextInput` `prefix` prop displays the URL prefix.

## 8. Token Usage And Precise CSS

### Root

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-sm)` |

### Row

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.5rem` |
| `.text-input` child | `flex: 1` |

### Reset Button

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| height | `var(--poodle-size-control-height)` |
| padding | `0 0.625rem` |
| border | `0.0625rem solid var(--poodle-color-border-default)` |
| border-radius | `var(--poodle-radius-control)` |
| background | `var(--poodle-color-background-surface)` |
| color | `var(--poodle-color-text-secondary)` |
| font-family | `var(--poodle-typography-label-family)` |
| font-size | `0.75rem` |
| font-weight | `var(--poodle-typography-label-weight)` |
| white-space | `nowrap` |
| transition | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### Reset Button States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated))` |
| `:hover` | border-color | `color-mix(in srgb, var(--poodle-color-border-default) 78%, var(--poodle-color-text-primary))` |
| `:focus-visible` | outline | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `:focus-visible` | outline-offset | `0.125rem` |

### Hint

| Property | Value |
|----------|-------|
| margin | `0` |
| font-size | `0.75rem` |
| color | `var(--poodle-color-text-secondary)` |

### Hint Code

| Property | Value |
|----------|-------|
| font-family | `var(--poodle-typography-code-family)` |
| font-size | `0.6875rem` |
| padding | `0.0625rem 0.25rem` |
| border-radius | `0.1875rem` |
| background | `color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated))` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- `slugify()` converts source to lowercase, strips non-word chars, replaces spaces/underscores with hyphens
- Manual edit handler re-slugifies user input (lowercase, strips special chars, spaces to hyphens) but does not remove trailing hyphens (allowing continued typing)
- Reactive statement `$: if (!isManualOverride && autoSlug !== value)` keeps value in sync with source
- `resetToAuto()` sets `isManualOverride = false` and dispatches change with `isManual: false`
- `displayPrefix` defaults to `"/"` when `prefix` is null

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Auto-slug from source | Yes | -- | -- |
| Manual override detection | Yes | -- | -- |
| Reset to auto | Yes | -- | -- |
| URL prefix display | Yes | -- | -- |
| Validation integration | Yes | -- | -- |
| Max length | Yes | -- | -- |
| Hint with auto-slug preview | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Auto-Generated From Title

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Auto-generated from title | `source` bound to a TextInput for title, slug auto-updates | Field with `/` prefix, slug updates as title changes; manual edit shows reset button and hint |

### With Custom Prefix

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom prefix | `source="Products Page"`, `prefix="/products/"`, custom description | Slug field with `/products/` prefix |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `source="Fixed content"`, `isDisabled=true` | Input disabled, no interaction |

## 14. Approval And Adoption Notes

Use `SlugField` alongside a title or name input in content creation forms. The component pairs naturally with `Field` and `TextInput` primitives. For uniqueness validation, the consuming application should check the slug against the backend and set `validationState` and `error` accordingly.

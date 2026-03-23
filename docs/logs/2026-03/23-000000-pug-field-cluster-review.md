---
title: Pug field cluster review — FieldHint, FormError, FieldSet
status: complete
owner: Platform
updated: 2026-03-23
tags: [review, field, underlay-adoption]
---

## Context

This review was requested by the Underlay Pug-adoption roadmap (`g01.042`) to decide whether three Underlay field-adjacent surfaces should move into Pug, fold into the existing `Field` contract, or remain outside Pug.

The three surfaces: `FieldHint`, `FormError`, and `FieldSet` (plus `FieldSetGrid`).

## Methodology

Each surface was compared directly against the existing Pug `Field` primitive (`packages/svelte/primitives/src/Field.svelte`) and the broader Pug primitive/layout surface to determine whether missing behavior should become a `Field` feature, a new standalone contract, or external composition.

The Pug eligibility rubric was applied: domain-neutral, stable contract shape, cross-app recurrence, no product nouns, primarily UI structure/interaction/accessibility/theming.

---

## Per-Surface Decisions

### 1. FieldHint — `fold_into_existing_field`

**What it does in Underlay:** A small circular "?" button positioned next to a field label that opens a popover with explanatory text. Built as a `Popover` wrapper with a pill-shaped trigger.

**Analysis:**

- The behavior is domain-neutral — contextual help hints next to form labels are a standard design-system pattern.
- The existing Pug `Field` already has a `description` prop that renders static help text below the label. A popover-style hint is the progressive-disclosure equivalent for longer or less-essential guidance.
- This should not become a standalone Pug component. It is a `Field` capability, not a reusable icon or button.

**Decision: Fold into `Field` as a new `hint` prop.**

Minimal contract addition to `Field`:

```
hint: string | null = null
```

When `hint` is set, `Field` renders a small help-trigger button (circular "?" or info icon) inline with the label that opens a `Tooltip` or `Popover` with the hint content. The existing `description` prop remains for always-visible inline help text; `hint` is for progressive-disclosure help.

**Implementation shape:**

- Add `hint` prop to `Field.svelte`
- Render an inline trigger next to the label text (after the label, before the optional indicator)
- Use the existing Pug `Tooltip` or `Popover` primitive to display the content — no need to build new overlay behavior
- The trigger should be a small icon button (info icon from the built-in internal set) styled at label-adjacent scale
- The hint popover should use standard Pug overlay tokens for background, border, shadow, and typography

**What this does NOT include:**

- Rich/HTML hint content (string only, matching `description`)
- Custom trigger styling or positioning beyond the standard field-label placement

### 2. FormError — `fold_into_existing_field`

**What it does in Underlay:** A conditionally-rendered `<p>` with `role="alert"` and `aria-live="polite"` that displays a form-level error message in danger-colored text. Takes a single `message: string | null` prop.

**Analysis:**

- The behavior is domain-neutral — form-level error banners are a standard pattern.
- The existing Pug `Field` already has per-field error rendering: when `validationState="invalid"` and `error` is set, it renders a message with `aria-live="polite"`.
- However, `FormError` is a **form-level** message, not a field-level message. It sits above or outside any individual field. The existing `Field` component cannot serve this role because it wraps a single control.
- The Pug `FormLayout` composite already has an `error` prop for form-level error display. The Pug `Callout` primitive with `tone="danger"` also serves this purpose with richer anatomy.

**Decision: Already covered. No Pug changes needed.**

Underlay `FormError` maps directly to:

1. `FormLayout` `error` prop — for forms using the composite layout
2. `Callout` with `tone="danger"` — for standalone form-level error display
3. Simple composition: `{#if error}<p class="..." role="alert" aria-live="polite">{error}</p>{/if}` — if neither composite fits

The Underlay `FormError` is a 17-line component with no behavior beyond conditional rendering and basic styling. It does not warrant a new Pug export. The migration path is composition over existing Pug surfaces.

**Migration guidance:** Replace `<FormError message={error} />` with `<Callout tone="danger">{error}</Callout>` or use `FormLayout`'s `error` prop. If neither fits, a single `<p>` with Pug token-styled color is sufficient — this is not a component-level concern.

### 3. FieldSet / FieldSetGrid — `stay_outside_pug`

**What they do in Underlay:**

- `FieldSet`: A `<fieldset>` wrapper with optional `legend` (styled as a small uppercase eyebrow), a `full` prop for grid-column spanning, and an inner grid container for child fields.
- `FieldSetGrid`: A responsive multi-column grid container with progressive column breakpoints via container queries, `columns` prop (1–6 or "auto"), and `full` prop for spanning.

**Analysis:**

- `FieldSet` is semantically a `<fieldset>` with a legend — this is a valid accessibility pattern for grouping related fields. However:
  - The legend styling is identical to the existing Pug `Eyebrow` component (small, uppercase, muted, letter-spaced).
  - The inner layout is a single-column grid with a gap — this is just `Grid` or `Stack`.
  - The `full` prop (`grid-column: 1 / -1`) is already supported by `Field`'s `span="full"` prop and works with any CSS grid child.

- `FieldSetGrid` is a responsive CSS Grid layout with container-query breakpoints. However:
  - Pug `Grid` already provides grid layout with `columns`, `gap`, and `padding`.
  - Pug `FormLayout` already provides responsive multi-column form layout with breakpoints.
  - The container-query responsive behavior in `FieldSetGrid` is useful but is **layout composition**, not a field-level or form-level design-system contract.
  - The breakpoint logic is tightly coupled to Underlay's specific responsive thresholds (400px, 600px).

- The grouped-field question: Is there a real "field group" contract that Pug is missing?
  - **No.** The semantic `<fieldset>` + `<legend>` pattern is just:
    ```svelte
    <fieldset>
      <Eyebrow>{legend}</Eyebrow>
      <Grid columns="repeat(2, 1fr)" gap="md">
        <Field ...> ... </Field>
        <Field ...> ... </Field>
      </Grid>
    </fieldset>
    ```
  - This is **composition over existing Pug primitives**, not a contract gap.
  - A `FieldGroup` contract would need to carry real grouped-field semantics (coordinated validation, group-level error state, fieldset-level disabled state) to justify its existence as a standalone Pug contract. The Underlay `FieldSet` does not carry any of these — it is purely visual grouping with a legend.

**Decision: Stay outside Pug. Composition over existing primitives.**

Migration path:
- Replace `<FieldSet legend="Personal Info">` with `<fieldset><Eyebrow>Personal Info</Eyebrow><Grid ...>` using Pug primitives
- Replace `<FieldSetGrid columns={2}>` with `<Grid columns="repeat(2, 1fr)" gap="md">` or `<FormLayout columns={2}>`
- The `full` prop maps to `style="grid-column: 1 / -1"` or `Field`'s `span="full"`

**Unresolved risk:** If future adoption reveals that coordinated group-level validation (all fields in a group sharing a validation state, group-level error message, group-level disabled state) is a real cross-app need, a `FieldGroup` contract could be opened. The current Underlay surfaces do not demonstrate this need — they are purely visual.

---

## Summary Table

| Surface | Decision | Rationale |
|---------|----------|-----------|
| `FieldHint` | `fold_into_existing_field` | Add `hint` prop to `Field`. Progressive-disclosure help is a standard field feature. |
| `FormError` | Already covered | Maps to `FormLayout` `error` prop or `Callout` with `tone="danger"`. No new surface needed. |
| `FieldSet` | `stay_outside_pug` | Visual grouping with a legend is composition over `Eyebrow` + `Grid`. No grouped-field contract gap. |
| `FieldSetGrid` | `stay_outside_pug` | Responsive grid layout is composition over `Grid` or `FormLayout`. No contract gap. |

## Pug Implementation Work

**One item:** Add `hint` prop to `Field.svelte`.

This is a small, backward-compatible addition. The implementation should:

1. Add `hint: string | null = null` prop
2. Render an inline info-icon trigger next to the label when `hint` is set
3. Display the hint in a `Tooltip` (preferred for short text) positioned above the trigger
4. Use the built-in `info` icon at label-adjacent scale
5. Update the component-docs entry for `Field`
6. Update the `FieldSpecimen` to demonstrate the hint

## Underlay Inventory Update

The `promote_to_pug_contract_review` group should be resolved:

- `FieldHint` → `resolve_via_existing_pug_surface` (after the `hint` prop is added to `Field`)
- `FormError` → `resolve_via_existing_pug_surface` (maps to `Callout` or `FormLayout.error`)
- `FieldSet` → `resolve_via_existing_pug_surface` (composition over `Eyebrow` + `Grid`)

## Next Task

Implement the `hint` prop on `Field.svelte`, update the specimen and docs, then update the Underlay inventory to close the field cluster review.

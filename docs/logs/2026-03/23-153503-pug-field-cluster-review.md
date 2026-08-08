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

Each surface was compared directly against the existing Pug `Field` primitive (`packages/svelte/components/src/Field.svelte`) and the broader Pug primitive/layout surface to determine whether missing behavior should become a `Field` feature, a new standalone contract, or external composition.

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

### 3. FieldSet — `belongs_in_pug` (revised)

**What it does in Underlay:** A `<fieldset>` wrapper with optional `legend` (styled as a small uppercase eyebrow), a `full` prop for grid-column spanning, and an inner grid container for child fields.

**Analysis (revised):**

Initial review recommended composition over `Eyebrow` + `Grid`. On reconsideration, `FieldSet` earns its place as a named primitive:

- **Semantic HTML** — `<fieldset>` + `<legend>` is an actual accessibility contract. Screen readers announce the group label. Composition over `Eyebrow` + `Grid` doesn't give you that — developers would need to remember `<fieldset>` every time.
- **Recurring pattern** — virtually every form with more than a few fields groups them. Every developer writes the same boilerplate without this.
- **Declarative vocabulary** — `<FieldSet legend="Address" columns={2}>` reads instantly. The manual alternative is noisy and easy to get subtly wrong.
- **Passes the eligibility rubric** — domain-neutral, stable contract, cross-app, no product nouns, pure UI structure + accessibility.

**Decision: Add to Pug as a standalone primitive.**

Contract shape:

```
legend: string | null = null        — group label rendered as <legend>
columns: number = 1                 — grid columns for child fields
gap: SpaceScale = "md"              — gap between children
span: number | "full" | null = null — column span in parent grid
```

### 4. FieldSetGrid — `stay_outside_pug`

**What it does in Underlay:** A responsive multi-column grid container with container-query breakpoints, `columns` prop (1–6 or "auto"), and `full` prop for spanning.

**Decision: Stay outside Pug.** The responsive container-query breakpoint logic is tightly coupled to Underlay's specific thresholds (400px, 600px). The new `FieldSet` with its `columns` prop covers the static multi-column case. Apps that need progressive responsive collapse should compose `FieldSet` with their own container-query CSS or use `FormLayout`.

---

## Summary Table

| Surface | Decision | Rationale |
|---------|----------|-----------|
| `FieldHint` | `fold_into_existing_field` | Add `hint` prop to `Field`. Progressive-disclosure help is a standard field feature. |
| `FormError` | Already covered | Maps to `FormLayout` `error` prop or `Callout` with `tone="danger"`. No new surface needed. |
| `FieldSet` | `belongs_in_pug` (revised) | Semantic `<fieldset>` + `<legend>` is an accessibility contract. Recurring pattern, declarative vocabulary. Added as a standalone primitive. |
| `FieldSetGrid` | `stay_outside_pug` | Responsive container-query breakpoints are Underlay-specific. `FieldSet` + `FormLayout` cover the static case. |

## Pug Implementation Work

**Two items:**

1. **Add `hint` prop to `Field.svelte`** — progressive-disclosure help via info-icon tooltip.
2. **Add `FieldSet.svelte` primitive** — semantic `<fieldset>` + `<legend>` with grid layout.

Both are small, backward-compatible additions. Both have been implemented, with specimens, docs, and guide updates.

## Underlay Inventory Update

The `promote_to_pug_contract_review` group should be resolved:

- `FieldHint` → `resolve_via_existing_pug_surface` (after the `hint` prop is added to `Field`)
- `FormError` → `resolve_via_existing_pug_surface` (maps to `Callout` or `FormLayout.error`)
- `FieldSet` → `resolve_via_existing_pug_surface` (composition over `Eyebrow` + `Grid`)

## Next Task

Implement the `hint` prop on `Field.svelte`, update the specimen and docs, then update the Underlay inventory to close the field cluster review.

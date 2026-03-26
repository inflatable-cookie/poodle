# Poodle Guides

Reusable implementation guidance for teams building real applications with
Poodle.

Contracts define what each component does. Guides explain how to compose those
components into stable application patterns without reintroducing wrapper churn
or product-specific abstractions into the design system.

## Current Guides

1. [Svelte Developer Guide](./svelte-developer-guide.md)
   End-to-end package setup, theming, and general component usage.
2. [Form Layout And Field Recipes](./001-form-layout-and-field-recipes.md)
   Default form composition rules, field messaging posture, and slug-input
   composition.
3. [Action Pattern Recipes](./002-action-pattern-recipes.md)
   Save-intent actions, action menus, and grouped icon-action rules.
4. [List And Filter Recipes](./003-list-and-filter-recipes.md)
   List-page shell composition over `ListContainer` and `FilterToolbar`.
5. [Dialog And Detail Recipes](./004-dialog-and-detail-recipes.md)
   Modal form and readonly detail-page composition over Poodle-owned surfaces.
6. [Async Validation Recipes](./005-async-validation-recipes.md)
   `TextInput` async validation, `Field` messaging posture, and app-owned
   validator composition.
7. [Form Validity Recipes](./006-form-validity-recipes.md)
   App-owned form-wide validity over Poodle field-level state and validation.
8. [Slug Field Recipes](./007-slug-field-recipes.md)
   App-owned slug generation, validation, and submit gating over `Field` and
   `TextInput`.
9. [File Upload Recipes](./008-file-upload-recipes.md)
   Generic file intake in Poodle with app-owned upload orchestration.
10. [Media Picker Workflow Recipes](./009-media-picker-workflow-recipes.md)
   Lightweight selector posture for Poodle `MediaPicker` and the boundary for
   heavier media-library workflow shells.

## Rules

- Prefer adding a new focused guide over growing one catch-all recipes file.
- Each guide should capture a stable, reusable decision proven in real app work.
- If a pattern is generic and awkward, improve Poodle.
- If a pattern is app vocabulary, keep it in app/shared code and document the
  composition rule instead of adding a new canonical Poodle wrapper.

## Next Task

Add the next migration-backed guide family only when a new retained workflow
boundary is proven in real app work instead of growing speculative picker or
table recipes ahead of the migration line.

# Poodle Guides

Reusable implementation guidance for teams building applications and components
with Poodle across all three implementation targets (Svelte, GPUI, Jetstream).

Contracts define what each component does. Guides explain how to integrate
Poodle into an app, how to implement components for a given target, and how to
compose components into stable application patterns without reintroducing
wrapper churn or product-specific abstractions into the design system.

## Developer Guides

Start here if you are integrating Poodle into an application or implementing
components for a runtime target:

1. [Svelte Developer Guide](./svelte-developer-guide.md)
   End-to-end package setup, theming, and general component usage for Svelte
   web applications.
2. [GPUI Developer Guide](./gpui-developer-guide.md)
   Cargo dependency setup, `GpuiThemeProvider`, component structs, token
   resolution, and writing new GPUI components.
3. [Jetstream Developer Guide](./jetstream-developer-guide.md)
   Cargo dependency setup, `JetstreamThemeProvider`, `js_<component>()` render
   functions, the `JsEl` fluent builder, and writing new Jetstream components.

## Application Pattern Recipes

Svelte-backed composition guidance for real application workflows:

1. [Form Layout And Field Recipes](./001-form-layout-and-field-recipes.md)
   Default form composition rules, field messaging posture, and slug-input
   composition.
2. [Action Pattern Recipes](./002-action-pattern-recipes.md)
   Save-intent actions, action menus, and grouped icon-action rules.
3. [List And Filter Recipes](./003-list-and-filter-recipes.md)
   List-page shell composition over `ListContainer` and `FilterToolbar`.
4. [Dialog And Detail Recipes](./004-dialog-and-detail-recipes.md)
   Modal form and readonly detail-page composition over Poodle-owned surfaces.
5. [Async Validation Recipes](./005-async-validation-recipes.md)
   `TextInput` async validation, `Field` messaging posture, and app-owned
   validator composition.
6. [Form Validity Recipes](./006-form-validity-recipes.md)
   App-owned form-wide validity over Poodle field-level state and validation.
7. [Slug Field Recipes](./007-slug-field-recipes.md)
   App-owned slug generation, validation, and submit gating over `Field` and
   `TextInput`.
8. [File Upload Recipes](./008-file-upload-recipes.md)
   Generic file intake in Poodle with app-owned upload orchestration.
9. [Media Picker Workflow Recipes](./009-media-picker-workflow-recipes.md)
   Lightweight selector posture for Poodle `MediaPicker` and the boundary for
   heavier media-library workflow shells.
10. [Auth UI And Workflow Recipes](./010-auth-ui-and-workflow-recipes.md)
    Poodle-first auth page framing, one-time-code entry, and password-policy
    checklist composition.
11. [Page Shell And Admin Recipes](./011-page-shell-and-admin-recipes.md)
    Poodle-first detail/list/header/tab composition for admin and back-office
    apps.
12. [Media Library And Upload Recipes](./012-media-library-and-upload-recipes.md)
    Poodle-first media browse, upload, thumbnail, and picker-shell composition.
13. [Admin Feature Delivery Recipes](./013-admin-feature-delivery-recipes.md)
    Implementation-order guidance for real admin list/detail/edit flows using
    Poodle-first UI composition.
14. [Admin App Shell Recipes](./014-admin-app-shell-recipes.md)
    Poodle-first sidebar, mobile header, toast host, and context-panel shell
    composition.

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

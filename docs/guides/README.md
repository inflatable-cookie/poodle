# Poodle Guides

Guides explain how to integrate Poodle and compose its components into stable
application patterns. For exact component inputs, states, events, and
accessibility behavior, use the [component contracts](../contracts/components/README.md).

## Runtime Guides

- [Svelte](svelte-developer-guide.md) — setup, tokens, themes, icons, and
  component use
- [React](../../packages/react/components/README.md) — current experimental
  React surface
- [GPUI](gpui-developer-guide.md) — Rust specs, shared rendering, and the GPUI
  node backend
- [Jetstream](jetstream-developer-guide.md) — deferred paired integration for
  Rust specs, shared rendering, and the Jetstream node conversion boundary

## Application Recipes

These recipes cover reusable application composition proven in downstream
work. They use Svelte examples, but their ownership rules apply across runtimes.

1. [Form layout and fields](001-form-layout-and-field-recipes.md)
2. [Action patterns](002-action-pattern-recipes.md)
3. [Lists and filters](003-list-and-filter-recipes.md)
4. [Dialogs and detail pages](004-dialog-and-detail-recipes.md)
5. [Async validation](005-async-validation-recipes.md)
6. [Form validity](006-form-validity-recipes.md)
7. [Slug fields](007-slug-field-recipes.md)
8. [File upload](008-file-upload-recipes.md)
9. [Media picker workflows](009-media-picker-workflow-recipes.md)
10. [Authentication UI](010-auth-ui-and-workflow-recipes.md)
11. [Page shells and admin UI](011-page-shell-and-admin-recipes.md)
12. [Media libraries and uploads](012-media-library-and-upload-recipes.md)
13. [Admin feature delivery](013-admin-feature-delivery-recipes.md)
14. [Admin application shells](014-admin-app-shell-recipes.md)

## Ownership Rule

Poodle owns reusable primitives, composites, and general workstation shells.
Applications own product vocabulary, domain workflows, persistence, routing,
and service orchestration. If a generic pattern is awkward in every app, fix
Poodle; if it only makes sense in one product, keep it in that product.

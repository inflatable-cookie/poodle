# Hover Card Rating And Navigation Ownership

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- widened the primitive catalogue with `HoverCard` and `Rating`
- documented the utility overlay and judgment-control ownership line
- explicitly deferred `NavigationMenu` and `Menubar` instead of leaving them in
  an ambiguous “maybe foundation” bucket

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers rather than true Bits-backed
  implementations
- navigation-menu ownership is still intentionally unresolved and needs its own
  explicit contract batch later

## Next Task

Choose whether the next tranche should finally define navigation-menu
ownership, or stay off shell-structure semantics and take another clearly
foundation-safe family first.

# Navigation Menu And Menubar Foundation Baseline

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- promoted `NavigationMenu` and `Menubar` into the foundation primitive surface
- kept the split explicit between navigation disclosure and command menu
  semantics instead of treating them as one generic nav bucket
- documented the ownership boundary in
  `docs/specs/033-navigation-menu-and-menubar-foundation-baseline.md`
- added foundation seed contracts for the new navigation-family primitives

## Validation

- targeted Svelte compilation of the new primitive files
- `bun run docs:build`
- `git diff --check`

## Risks

- these controls are still Svelte-native wrappers rather than true Bits-backed
  implementations
- router-aware navigation, native menu bridges, and deeper submenu hierarchy
  semantics are still intentionally outside this baseline

## Next Task

Choose the next family batch deliberately, with timezone-aware ranges, richer
navigation integration, or a revisit of command or data ownership now the main
remaining widening decisions.

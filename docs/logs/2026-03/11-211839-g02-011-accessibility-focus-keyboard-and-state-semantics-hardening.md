# g02.011 Accessibility Focus Keyboard And State Semantics Hardening

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- completed `g02.011`
- hardened the Svelte implementation of `CommandPalette`, `DataTable`,
  `PickerShell`, `RelationPicker`, `ToastStack`, and `ActionDiscoveryPanel`
  around focus containment, boundary keyboard movement, explicit sort and row
  semantics, picker status announcements, and clearer notification severity
  posture
- added the normative hardening baseline at
  `docs/specs/019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`
- revised the affected contracts so accessibility guidance now matches the
  implementation-bearing surfaces instead of remaining generic

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche intentionally tightened semantics where the existing interaction
  model already supported them, rather than claiming richer roles that the
  current surfaces did not actually implement
- the GPUI burden is now more explicit: modal focus scope, keyboard boundary
  movement, row/sort meaning, picker workflow semantics, and notification
  announcements are all contract bugs if they go missing downstream

## Next Task

Open
`docs/roadmaps/g02/012-docs-site-examples-and-component-discoverability-baseline.md`
and turn the now-hardened surface into a more discoverable docs and examples
baseline.

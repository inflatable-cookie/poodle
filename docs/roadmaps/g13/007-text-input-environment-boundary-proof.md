# g13.007 TextInput Environment-boundary Proof

Status: planned
Owner: Poodle core
Depends on: `g13.006`

## Objective

Prove that Rust authority can coexist with browser and native text systems
without generating lifecycle code or weakening input semantics.

## Deliverables

- TextInput definition covering props, slots, validation, accessibility,
  recipes, size/density axes, and semantic event intent.
- Typed capability boundary for focus, selection, composition/IME, clipboard,
  measurement, and native text editing.
- Svelte/React native input lowering and GPUI/Jetstream text-system adapters.
- Composition, selection, focus, keyboard, screen-reader, and visual evidence.

## Acceptance

- IME composition and selection behavior remain runtime-native and correct.
- Generated artifacts contain declarations, not framework lifecycle logic.
- Capability gaps are visible and typed; none are silently ignored.
- Existing public APIs remain stable through the pilot.

## Next

`g13.008` decides whether the IR is adopted, revised, or rejected.

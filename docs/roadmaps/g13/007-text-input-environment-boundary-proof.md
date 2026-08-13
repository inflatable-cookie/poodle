# g13.007 TextInput Environment-boundary Proof

Status: complete
Owner: Poodle core
Depends on: `g13.006`
Closed by: `g13-b048` (web half) + `g13-b049` (natives, this card closes
the milestone)

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

## Outcome

The milestone's acceptance line — *"capability gaps are visible and typed;
none are silently ignored"* — is answered honestly: the boundary is typed as
an *existence* claim, the ownership split is prose, and the first real test
of the line (a component one runtime delegates, one implements, and one
does not have at all) fails it: **the IR cannot express that a runtime
lacks a declared capability.** Jetstream renders a text field that cannot
be typed into and nothing in the model says so; the `text-input` vector's
`applies_to` even lists Jetstream as conforming. That finding is
`g13.008`'s decision input, recorded in
`docs/logs/2026-08/13-g13-049-text-input-slice-native-runtimes.md` (R2/R3).

## Next

`g13.008` decides whether the IR is adopted, revised, or rejected — with
the per-runtime capability gap as the revision candidate.

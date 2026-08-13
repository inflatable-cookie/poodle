# g13.018 Capability And Anatomy Amendments

Status: ready
Owner: Poodle core
Depends on: `g13.017`

## Objective

Fix the two expressiveness gaps the pilot found. Both are vocabulary, not
behaviour, so both are in scope after the narrowing.

## Deliverables

- **Per-runtime capability expression, including absence.**
  `CapabilityRequirement` carries only `capability` and a prose `purpose`, so
  ownership is untyped and "this runtime does not have it" cannot be said.
  Jetstream renders a text field nobody can type into, declared identically to
  GPUI, which implements the whole editing model.
- **Repeated anatomy with per-item identity.** `PartKind::Repeated` requires a
  `List` prop and yields identical instances; its own doc comment names the two
  RangeSlider thumbs as the example it cannot serve, and both web and native
  renderers hard-code "two".
- A gate that fails when a runtime consumes a capability it does not declare,
  or declares one it does not have.

## Acceptance

- The TextInput definition can state that Jetstream lacks text editing, and
  something fails if that stops being true.
- The RangeSlider definition can express two thumbs with per-position identity
  without the renderer hard-coding the count.
- Public APIs and pixels unchanged; no baseline refreshed.

## Next

`g13.019` extends vocabulary coverage across the corpus.

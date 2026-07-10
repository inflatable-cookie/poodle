# g11.006 Rust Headless Mirror

Status: planned
Owner: Poodle core
Depends on: `g11.004` (machine shape stabilized through at least waves 1–3)
Updated: 2026-07-10

## Purpose

Mirror the headless machine layer to the Rust side so GPUI and Jetstream
implementations consume the same behavioral contract instead of re-deriving
interaction logic per runtime.

## Decision To Make First (record in spec `062`)

Port strategy, decided against the real TS machine corpus:

- **hand-port**: idiomatic Rust machines in a new crate (or inside
  `poodle-specs`), kept in sync by contract + conformance tests
- **codegen**: if `g11.002` chose a declarative machine-spec source, generate
  both TS and Rust skeletons from it; hand-written guards/effects only

Also decide consumption depth: do GPUI/Jetstream adapters drive full machines
(events in, state + attribute intents out), or only consume the declarative
spec for states/ARIA while keeping their existing interaction code? Full
machines are the goal; partial adoption is an acceptable staged outcome if
recorded as explicit debt.

## Scope

1. `poodle-headless` crate (name TBD): machine runtime (pure transitions,
   effect intents as data — no runtime side effects in core, matching the TS
   design)
2. port/generate machines for the wave 1–3 component set
3. **conformance harness**: shared machine test vectors (event sequences →
   expected states/attributes) executed against both TS and Rust machines.
   This is the parity mechanism — build it before porting broadly. Vectors
   live framework-free (JSON) so both sides load them.
4. adapt one GPUI component family (suggest overlay: dialog/menu) onto the
   Rust machines end-to-end as proof
5. Jetstream: consume machines where Jetstream maturity allows; explicitly
   low priority per current program posture — record gaps, do not force

## Compatibility

No web consumer impact (Rust-side only). GPUI preview must keep building
(`effigy gpui:build`); per memory, Rust previews are build-verified only —
put behavioral confidence in the conformance vectors and `cargo test`, not
runtime preview claims.

## Exit Criteria

- port-strategy and consumption-depth decisions recorded in spec `062`
- Rust machine crate with wave 1–3 coverage passing shared conformance
  vectors
- one GPUI family running on machines
- promotion: cross-runtime machine contract promoted into
  `docs/architecture/` and the parity rules in `docs/specs/002` lineage

## Validation

- `cargo test` on the new crate, `effigy gpui:build`, `effigy gpui:test`
- conformance vector suite green on both runtimes

## Next Task

`g11.007` multi-framework adapters.

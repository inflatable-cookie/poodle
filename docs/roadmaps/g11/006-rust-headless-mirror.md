# g11.006 Rust Headless Mirror

Status: complete (2026-07-10)
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

## Progress (2026-07-10)

Complete:

- **Port strategy decided: hand-port** (per the spec-062 lean — the
  interesting content is guards and effects, which codegen cannot
  generate). Crate: `packages/contracts/headless` → `poodle-headless`,
  zero runtime dependencies, `serde_json` dev-only.
- All 11 behavior machines ported: checkbox, switch, single-select,
  toggle-group, disclosure, slider + range-slider, popover, modal, menu
  (+ menu-list navigation), hover, tabs (main chart; tooltip sub-machine
  and drag plumbing stay adapter-side, as in TS). Plus `nav` roving
  helpers.
- **Conformance harness built**: 41 shared JSON vectors in
  `packages/contracts/headless/vectors/machines.json`, executed by both
  runtimes — `packages/core/test/conformance.test.ts` (bun) and
  `tests/conformance.rs` (cargo). Effects compare order-sensitively;
  numbers canonicalized to f64 to neutralize serializer differences.
  Both sides green: TS 162 total tests, Rust 10 suites / 41 cases.
- `poodle-gpui` still builds (crate is additive, unreferenced so far).

Validation commands: `cargo test` in `packages/contracts/headless`;
`bun test` in `packages/core`.

GPUI adoption (complete, 2026-07-10):

- **Consumption depth decided: machine-guarded handlers.** GPUI parents own
  open state (per the existing spec architecture), so components route
  dismissal decisions through the machines — escape/backdrop handlers call
  `modal_transition` and execute the emitted `EmitOpenChange` intents
  instead of gating inline on spec flags. Handlers attach unconditionally;
  the machine guards.
- Adopted: `Dialog` and `Drawer` (modal machine, escape + backdrop paths),
  `Menu` (wrapping keyboard navigation via `menu_list_navigate`).
- Validation: `poodle-gpui-components` and `poodle-gpui-preview` build;
  `poodle-gpui` adapter tests green (143). Per the build-verified-only
  posture, behavioral confidence comes from the conformance vectors, not
  runtime preview.

Recorded follow-on debt (not exit criteria):

- Domain-math port (dates/calendar grid, colors, durations, pagination,
  select options, positioning resolver) — pure and mechanical; port each
  piece alongside the GPUI/Jetstream feature that consumes it.
- Jetstream consumption: explicitly deferred per program posture; adopt the
  same machine-guarded-handler shape when Jetstream work resumes.
- Remaining GPUI families (selection/value, hover, tabs) adopt the same
  pattern opportunistically as those files are next touched.

## Next Task

`g11.007` multi-framework adapters and the Mitosis decision.

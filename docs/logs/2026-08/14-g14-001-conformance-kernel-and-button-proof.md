# g14.001 — Conformance Kernel And Button Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Spec: `docs/specs/066-executable-component-conformance.md`
Architecture: `docs/architecture/009-cross-runtime-component-conformance.md`
PR: #10
Verdict: accepted feasibility proof

## Outcome

```text
one portable Button interface + one typed 20-case corpus
  -> Svelte / React / shared Rust / GPUI execution
  -> normalized component-observation.v1
  -> three corpus-projected specimen views
  -> one strict active-cohort completion gate
```

Jetstream is program-deferred. It is not installed, executed, or reported as
passing by this board.

## Final Corrections

- Portable props and events derive from the interface value. Type-level tests
  pin zero-, single-, and multi-field payload projections; multi-field events
  receive one payload object.
- The corpus closes fixtures, regions, parts, states, events, values, axes,
  token roles, and geometry over the interface. TypeScript authoring and Rust
  loading both reject unknown or malformed values.
- Geometry requires an explicit finite non-negative tolerance. Cross-runtime
  comparison keys the bound by case, observation frame, part, and field; no
  runtime fallback or case-wide tolerance remains.
- Shared observers resolve declared part identity. Required unobservable
  fields fail their owning runtime. GPUI activation uses the backend listener
  path through real AppKit events.
- The canonical JSON fixtures are included directly. Retired GPUI copies and
  the `conformance-cases` target are gone.
- `ci:conformance` runs read-only authority checks, web execution, real-window
  GPUI execution, and normalized comparison. A dedicated path-scoped macOS PR
  workflow calls it. Headless `qa` and `ci:native` remain unchanged.

## Cost

`effigy conformance:cost`, base `5180ac16`:

| Surface | Cost |
| --- | ---: |
| Reusable authority | 597 LOC |
| Codegen | 708 LOC |
| Generic observers and runners | 1,372 LOC |
| Generic runtime deltas | 16 LOC |
| Wiring | 254 LOC |
| **Generic kernel** | **2,947 LOC** |
| Button authored authority | 362 LOC |
| Button generated Rust | 131 LOC |
| Button harness and runtime deltas | 1,082 LOC |
| **Button pilot increment** | **1,575 LOC** |
| **Source mechanism total** | **4,522 LOC** |
| Generated interface + corpus JSON | 33,392 bytes |
| Replaced declaration + three active specimen fixtures | 619 LOC |

The cost stop is triggered: the Button increment alone exceeds what it
replaces. The orchestrator accepts this as a bounded feasibility proof because
it adds an executable cross-runtime guarantee the replaced surfaces did not
provide. This does not approve rollout. Cards `002`–`007` must demonstrate
reuse or extraction of the 1,052-LOC Button harness; `008` owns the final
adopt/revise/reject verdict.

## Planted Failures

| Plant | Failure |
| --- | --- |
| Rename portable prop or event | consuming shells, corpus, or Rust generation |
| Unknown fixture/state/axis/token role | authoring or Rust corpus validation |
| Missing geometry tolerance | TypeScript authoring and Rust corpus validation |
| Geometry inside/outside bound | inside passes; outside names runtime/case/frame/part/field/bound |
| Missing renderer role | owning GPUI assertion reports not observed |
| Wrong web icon identity | owning part assertion |
| Inert GPUI listener | pointer and keyboard press cases |
| Missing GPUI registry entry | completion gate |
| Stale generated artifact or orphan | read-only check |

## Validation

- `effigy test --plan`
- `effigy conformance:check`
- `effigy conformance:test-web`
- `cargo test --manifest-path packages/codegen/Cargo.toml conformance::tests`
- `bun test packages/core/test/component-case-authority.test.ts`
- `effigy conformance:cost`
- `effigy ci:conformance` — 20 cases × 3 active runtimes
- `effigy ci:rust`
- `effigy ci:native`
- `effigy docs:check`
- `effigy test:web-pack-install` — 5/5 packed-consumer tests
- `bunx vitest run` through `ci:web` — 1,249 tests across 92 files
- `git diff --check`

`ci:web` reaches its final Svelte component check, then fails on the three
known `AppHeaderCenterHarness.svelte` Snippet identity errors. They predate
this card, reproduce on main, and remain outside its diff. All earlier web
legs pass.

## Retained / Retired

- Retained: typed interface and corpus, corpus-projected specimens, generated
  Rust declaration, data-driven observers, real-window GPUI execution,
  normalized comparison, strict completion.
- Retired: copied native fixtures, `conformance-cases`, Button branches in
  shared observers, blanket geometry fallbacks, callback casts, Jetstream
  execution, stale four-runtime claims.

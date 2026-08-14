# g14.001 — Conformance Kernel And Button Proof

Post-completion correction: commit `8ac863b4` restored the curated Button
specimens. The corpus still owns executable fixtures and exhaustive evidence;
it no longer replaces the catalogue `Examples` view.

Status: complete — replacement proof accepted in PR #10
Depends on: none
Governing spec: `../../specs/066-executable-component-conformance.md`

## Outcome

Deliver the smallest complete conformance loop through Button:

```text
one portable interface + one typed case corpus
  -> Svelte / React / GPUI execution
  -> normalized observations
  -> optional exhaustive diagnostic projections
  -> one failing completion gate
```

This is a replacement proof, not a platform build. Keep the generic surface
as small as Button needs while preserving extension points named by spec 066.

## Acceptance Ruling

PR #10's replacement pass closes the proof blockers:

- infer web portable prop and event types from the interface value; a second
  hand-written `ButtonPortableProps` or `ButtonPortableEvents` shape is not an
  authority
- bind case fixtures, regions, parts, states, events, and values to that
  interface at compile time and validate the serialized artifact against it
- make shared observers data-driven from stable runtime part identity; shared
  code may not hardcode Button's part list or tree-search rules
- fail a required assertion when any active runtime cannot observe it;
  cross-runtime "someone exercised it" vacuity is not completion
- execute activation through the GPUI backend binding; converting a node and
  then calling `Node.interaction.on_activate` directly is not backend evidence
- keep Jetstream opt-in and program-deferred; it cannot substitute for missing
  GPUI focus, token, or input evidence
- remeasure cost after removing copied artifacts and the Jetstream execution
  lane; stop again if reusable mechanism still grows faster than replaced
  declarations and executable fixtures without a concrete amortization proof

The final measurement still triggers the cost stop: 2,947 LOC of generic
kernel plus a 1,575-LOC Button pilot increment was initially measured against
619 LOC of declarations and specimen fixtures. Curated specimen deletion is no
longer accepted as replacement credit. The orchestrator accepts this as a bounded feasibility
proof because it establishes the missing executable guarantee; it is not an
economics verdict for rollout. Cards `002`–`007` must demonstrate reuse rather
than clone the 1,052-LOC Button harness. Card `008` retains the
adopt/revise/reject decision, and cards `009`–`014` remain blocked until then.

The executed GPUI leg now has a dedicated path-scoped macOS PR workflow.
Headless `qa` and `ci:native` remain unchanged; Jetstream remains deferred.

## Scope

1. Inventory Button's contract, Svelte/React props, `ButtonSpec`, renderer,
   both backends, current tests, and four specimens. Record exact before LOC
   and contradictions.
2. Add the constrained portable interface module for Button. Svelte and React
   consume its portable types; generated Rust replaces the equivalent
   `ButtonSpec` declaration surface. Keep platform extensions explicit.
3. Add a typed Button case corpus with fixture, diagnostic projection metadata, actions,
   and assertions. Emit deterministic neutral JSON for Rust.
4. Implement thin harnesses for Svelte, React, and GPUI. Exercise the real
   Button implementation and GPUI backend event path; no mock renderer may
   satisfy native. Keep the Rust fixture and observation boundary usable by a
   later Jetstream runner without importing Jetstream types.
5. Emit `component-observation.v1` from each runtime and compare exact Button
   part, role/name/state, token role, focusability, event order, and bounded
   geometry.
6. Make all three active runtime adapters consume the same case structure.
   Keep any exhaustive projection separate from the curated catalogue.
7. Add `conformance:build`, `conformance:check`, selected
   `conformance:test`, `conformance:test-windowed`,
   `conformance:complete-windowed`, and `conformance:cost` Effigy selectors.
   Wire read-only enforcement into the applicable standing gates.
8. Classify old machine-interface, scene, capability, and Button-specific
   evidence as keep/adapt/replace/retire in `conformance-estate.md`. Delete
   only surfaces fully replaced by this proof.

## Required Button Cases

- default labelled button
- every contract variant and tone
- disabled and loading
- leading/trailing icon regions
- press by pointer and keyboard
- focus-visible state
- theme, density, and control-size specimen axes

Case coverage may use matrices, but the serialized artifact must enumerate
stable case and capture IDs.

## Acceptance

- Changing one portable prop or event name fails the consuming TypeScript
  shells and Rust generation or compile without editing a second type mirror.
- An unknown fixture prop, region, part, state, event, enum value, or axis fails
  authoring or serialization; it cannot be ignored by every runtime.
- Changing a fixture/group/caption changes all three active runtime executions
  and any optional diagnostic projections.
- A planted event, role, token-role, part, or geometry divergence fails and
  names runtime, case, step, and field.
- A required field that GPUI cannot observe fails GPUI completion rather than
  becoming vacuous because Svelte or React observed it.
- Removing Button from the GPUI registry fails completion.
- Replacing the GPUI backend listener binding with an inert one fails an
  executed case. Calling the source node callback directly is forbidden.
- Shared runners and observers contain no Button identifier, part list, icon
  name, or Button-specific tree branch.
- Double generation is byte-identical; check mode never mutates the tree and
  catches orphans.
- Existing hand-written portable declarations and duplicate executable fixture
  content are removed, not left beside the new authority.
- Full cost report shows every mechanism line and what it replaced.
- `conformance:complete-windowed --component button` passes the active cohort and
  reports Jetstream as program-deferred, never passing.

## Stop Conditions

- The interface or case schema needs transitions, guards, derivations, or
  arbitrary callback bodies.
- Generic runners need Button-specific branches.
- Native success can be obtained without exercising GPUI backend output, or
  the portable Rust surface embeds GPUI-specific types that obstruct a later
  backend.
- Any required active-runtime assertion can pass as vacuous.
- A second hand-authored type or fixture vocabulary mirrors the interface.
- Total mechanism grows without deleting equivalent declarations/specimens.
- The worker must redesign another component to make Button pass.

Stop with evidence and options. Do not widen the model around the finding.

## Writable Scope

- conformance package or module chosen from existing package ownership
- Button sources, tests, contracts, generated artifacts, and specimens
- preview harnesses/registries needed for Button
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g14/conformance-estate.md`
- one August batch log and append-only `PAPERCUTS.md`

Do not edit other component implementations, architecture/specs, generation
status, dispatch ledger, release workflows, or external repositories.

## Validation

Use `effigy test --plan` before choosing the test shape. Run the new selectors,
the narrow Button suites, `ci:web`, `ci:rust`, `ci:native` from the main
checkout where required, `docs:check`, and `git diff --check`. Record known
baseline failures separately from branch regressions.

## Handoff

Open a PR with the before/after cost table, planted-failure evidence, exact
runtime results, retained/retired experiment list, and any schema pressure.

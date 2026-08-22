# g15.043 — UiPresentationProvider Native Cascade

Status: **ready — operator approved the atomic pre-v1 Rust API migration on
2026-08-23**
Found by: `g15.009`, retained by `g15.026`
Depends on: `../../architecture/010-native-presentation-construction-context.md`
Unblocks: `g15.050`, then `g15.013`
Governing refs: `../../architecture/001-poodle-system-shape.md`,
`../../architecture/010-native-presentation-construction-context.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/components/ui-presentation-provider.md`,
`release-gap-register.md`

## Goal

Implement one real native presentation cascade. Outer and nested provider
scopes must change descendant size/density defaults during shared Rust
construction, while explicit child inputs always win. The provider remains
layout-, paint-, interaction-, and accessibility-neutral.

This is one atomic migration because `main` must not carry two Rust renderer
APIs or a mixture of concrete and inheritable presentation inputs.

## Fixed Architecture

Implement architecture 010. Do not reopen it inside the worker lane.

- Add `poodle_render::RenderContext`, holding a borrowed token-only
  `ThemeProvider` plus effective size-scale and density defaults.
- Every public component renderer receives `&RenderContext` instead of a bare
  `&dyn ThemeProvider`.
- Semantic component `size` and `density` inputs use `Option`; `None` inherits
  from the context and `Some` always wins.
- Semantic-role mapping happens after choosing explicit or inherited base
  size.
- A provider builds its child through a closure receiving a nested context and
  returns that child unchanged.
- A composite that establishes an internal provider scope for host content
  uses an immediate context-aware child builder, not a prebuilt child node.
- `ThemeProvider`, `poodle-node`, and GPUI remain free of inheritance logic.

No compatibility wrapper, alias, default-value comparison, global/thread-local
state, post-built Node mutation, backend provider, or universal component tree
may remain.

## Measured Migration Surface

The planning audit found:

- 125 component spec files with semantic `ControlSize` or `ControlDensity`;
- 107 concrete size surfaces and 117 concrete density surfaces;
- seven files already preserving one or both inputs as optional;
- 103 shared-render modules reading presentation inputs;
- 168 shared-render modules accepting `ThemeProvider` directly;
- 113 shared-render modules constructing descendant specs; and
- 14 paired-web components creating internal provider scopes.

These counts are starting evidence, not hard-coded completion numbers. Record
the final measured denominator and explain every delta in the execution log.

## Execution Batches

### A — Render context and resolver laws

- Add the explicit borrowed context at the shared-render boundary.
- Root defaults are exactly `md` / `default`.
- A scoped context replaces both defaults without mutating its parent.
- Add helpers for base size, role-resolved size, and density.
- Keep `ThemeProvider` token-only. Theme-only internal helpers may still accept
  it explicitly through `context.theme()`; public component renderers may not.
- Unit-test root, outer, nested, sibling restoration, explicit `md`, and
  explicit `default` precedence.

### B — Native spec explicitness

- Migrate every semantic `ControlSize` / `ControlDensity` component input to
  `Option`.
- Defaults become `None`; existing `with_size` / `with_density` builders store
  `Some` and retain their public names.
- Keep component-specific domains such as `AvatarSize`, `IconSize`, meter
  dimensions, and unrelated numeric sizes unchanged.
- Refactor contract/spec helper methods that currently read concrete
  presentation fields to receive resolved values or otherwise consume the
  context-derived result. Do not call `unwrap_or_default` before the context
  has had the chance to resolve omission.
- Update current repository consumers and tests directly. Do not preserve the
  old field shape through a twin.

### C — Shared renderer and composition migration

- Move every public `poodle-render` component entrypoint to `&RenderContext`.
- Resolve each component's presentation once, then use those effective values
  for its own recipe and any descendant specs it constructs.
- Pass the same context through shared composite calls unless that component
  deliberately establishes a nested scope.
- Audit all 14 paired-web internal-provider owners named by architecture 010.
  Their native composition must either establish the equivalent scope or
  document, with source evidence, why no host child crosses that boundary.
- Where a host child must inherit the new scope, replace the prebuilt `Node`
  slot with the smallest immediate child-building closure. Do not generalise
  it into a stored scene or component abstraction.
- Adapt current GPUI preview facades, specimens, headless tests, examples, and
  any compile-required deferred Jetstream call sites to the new root context.
  Mechanical Jetstream compilation is not parity evidence and must not add a
  link/setup requirement.

### D — Real provider, specimen, and standing guard

- Add the shared provider construction function and delete the GPUI preview's
  no-op provider facade.
- Rebuild the GPUI specimen with real shared-render output for:
  root defaults, outer scope, nested scope, and explicit child reset.
- Use at least Button, TextInput, one composite that constructs descendants,
  and one host-provided scoped slot across focused tests/evidence.
- Prove the exact child Node is returned: no wrapper layout, paint, role,
  focusability, tab stop, or accessibility grouping.
- Add one standing source audit and Effigy selector that fail when:
  - a component spec reintroduces concrete semantic `ControlSize` or
    `ControlDensity` defaults;
  - a public shared component renderer accepts bare `ThemeProvider`; or
  - the GPUI UiPresentationProvider route returns to explicit host-equivalent
    values or a passthrough claim.
- Include that selector in the existing headless native board. Do not edit a
  workflow.

### E — Contract, migration, and evidence closeout

- Update the provider contract and GPUI guide with the construction-time API,
  nesting example, explicit-reset example, and breaking migration note.
- Update the system shape only where needed to point at architecture 010; do
  not restate the decision in multiple homes.
- Record one August execution log with starting/final counts, changed public
  signatures, internal-provider audit, proof cases, exact validation, and any
  compile-only Jetstream adaptation.
- Reconcile this card and `release-gap-register.md`. Do not advance
  `g15.050`; the orchestrator owns merge closeout and continuation.

## Writable Scope

- `packages/contracts/components/` and the smallest shared spec/type exports
  needed for semantic size/density optionality
- `packages/render/` for `RenderContext`, provider construction, all renderer
  signatures, resolver use, composition, and focused tests
- current in-repository Rust callers required by the break, including GPUI
  adapter/preview/headless tests and compile-required Jetstream callers
- GPUI UiPresentationProvider specimen/facade files and focused mounted proof
- one narrow source-audit script plus `effigy.toml` /
  `tasks/effigy.tasks.toml` wiring into the existing native board
- `docs/contracts/components/ui-presentation-provider.md`,
  `docs/guides/gpui-developer-guide.md`, and the smallest links from current
  architecture/working docs
- one `docs/logs/2026-08/20260823-g15-043-*.md` execution log
- `release-gap-register.md`, this card, and root `PAPERCUTS.md` only when
  evidence or newly encountered execution friction changes them

Do not edit Svelte or React components/CSS, public web props, tokens, themes,
component specimen teaching content outside the GPUI provider page, node
vocabulary, GPUI painting/layout behavior, visual comparator policy/assets,
package versions, workflows, release notes, tags, publication, Longhorn, the
conformance lab, or unrelated triage work.

## Acceptance

- [ ] Every public shared component renderer receives `RenderContext`; one
      token-only `ThemeProvider` remains behind it.
- [ ] Every native semantic component size/density input preserves omission;
      explicit `md` / `default` wins under non-default providers.
- [ ] Root, outer, nested, sibling-restoration, and explicit-reset resolver
      laws have focused tests.
- [ ] Real shared output proves inherited size and density on Button and
      TextInput plus one descendant-building composite and one scoped host
      slot.
- [ ] The 14 internal-provider owners have an evidence-backed native audit;
      every real native boundary passes context during construction.
- [ ] UiPresentationProvider returns its exact child with no wrapper Node,
      layout delta, paint, role, focus target, tab stop, or accessibility
      group.
- [ ] The GPUI specimen demonstrates real cascade rather than manually copying
      provider values into child specs.
- [ ] Mounted headless GPUI evidence observes the expected geometry and
      unchanged accessibility surface without opening or focusing a window.
- [ ] The standing source audit catches planted concrete-field, bare-renderer,
      and preview-passthrough regressions, and runs in the native board.
- [ ] Current Rust callers compile on the new API with no compatibility twin;
      any Jetstream change is mechanical and makes no parity claim.
- [ ] Contract, architecture, GPUI guide, gap register, card, and execution log
      agree.

## Validation

- focused `poodle-specs` and `poodle-render` tests for context, resolver,
  explicitness, provider identity, and representative composition
- focused GPUI specimen probe and mounted headless regressions
- the new presentation source-audit selector, including three planted-failure
  proofs restored before commit
- `effigy check:gpui`
- `effigy ci:rust`
- `effigy ci:native`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Run the broad boards once after the atomic migration compiles and focused tests
pass. Never run `*-windowed`, `test:native-visual`, a GPUI preview window, a
Jetstream QA/preview selector, a workflow, release mutation, tag, or
publication command.

## Stop Conditions

- The context cannot stay explicit and borrow-scoped without global,
  thread-local, backend, or Node metadata.
- A public renderer or scoped host child cannot migrate without retaining an
  old-signature twin.
- A component's Rust size/density surface disagrees with its contract or web
  semantics and the correct behavior is not already decided.
- An internal-provider owner requires a general scene/component abstraction
  rather than a bounded immediate child builder.
- The source audit cannot distinguish semantic `ControlSize` /
  `ControlDensity` from component-specific size domains without false claims.
- Required evidence needs a visible/focused window, Jetstream setup, workflow
  change, package version, or release mutation.
- Scope expands into repairing unrelated size/density visual parity rather
  than implementing truthful inheritance over the existing recipes.

Stop with exact files, types, and options. Do not substitute a partial roster,
specimen-only cascade, default-value heuristic, or undocumented exclusion.

## Continuation

One worker owns the complete atomic migration and opens one PR. The worker does
not merge or advance the runway. After acceptance, the orchestrator records
the final counts, closes the context-provider gap, and compiles `g15.050` from
the accepted release-candidate SHA.

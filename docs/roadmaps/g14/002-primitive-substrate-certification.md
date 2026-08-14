# g14.002 — Primitive Substrate Certification

Status: ready
Depends on: `g14.001` — replacement proof accepted in PR #10
Governing refs: `../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`,
`../../contracts/001-working-rules.md`,
`../../contracts/003-native-accessibility.md`
Input evidence: `001-conformance-kernel-and-button-proof.md`,
`conformance-estate.md`,
`../../logs/2026-08/14-g14-001-conformance-kernel-and-button-proof.md`

## Outcome

Turn the renderer vocabulary beneath component cases into executed evidence:

```text
one finite primitive-capability roster
  -> web observer probes
  -> renderer-neutral poodle-node probes
  -> real GPUI backend probes
  -> primitive-capability-report.v1
  -> component completion rejects missing or unexecuted requirements
```

Certify reusable substrate, not component behaviour. Cards `003`–`007` still
own controlled values, collection navigation, overlay behaviour, text editing,
and composite scenarios. This card proves that the generic case, observer, and
driver machinery can express and verify the primitives those profiles use.

Jetstream remains program-deferred. Do not install, link, build, execute, or
write Jetstream code for this card.

## Current Baseline

Trust the landed `g14.001` record, not earlier handoffs:

- generic kernel: 2,947 LOC
- Button pilot increment: 1,575 LOC
- Button-specific harness: 1,052 LOC
- declarations and active specimen fixtures replaced: 619 LOC
- active cohort: Svelte, React, shared Rust composition, and GPUI execution
- Jetstream: program-deferred

The cost stop remains live. Extract or reuse the Button harness. Copying it,
adding a parallel primitive harness architecture, or preserving two completion
paths is a stop condition.

Known baseline findings:

- `ci:web` reached three pre-existing `AppHeaderCenterHarness.svelte` Snippet
  identity errors during `g14.001`; recheck main before classifying them.
- `packages/contracts/headless/capabilities/capabilities.json` plus
  `packages/svelte/preview/scripts/capability-drift.ts` prove source traces,
  not executed parity. Adapt them as debt/evidence or retire them.
- GPUI 0.2.2 has no mounted accessibility tree for Poodle content. Contract
  003 forbids scheduling a parallel GPUI accessibility implementation.
- the GPUI visual gate passes `--control-size`; the preview parses `--size`
- current GPUI baselines were measured stale: chrome still matches while the
  specimen region reflects an older layout
- comparison mode writes a missing baseline, and update mode overwrites the
  only local reference without preserving before/after evidence

## Required Primitive Roster

Freeze a finite inventory before changing the mechanism. Account for every
public field or variant exposed through:

- `packages/contracts/node/src/lib.rs`: `Node`, `NodeKind`, `NodeStyle`,
  `StylePatch`, `NodePosition`, `Interaction`, `NodeA11y`, caret, drag, scrub,
  key, modifier, drop, animation, text, icon, and semantic-role channels
- the `poodle-layout` and `poodle-style` types re-exported by `poodle-node`
- `component-observation.v1`
- the web observer and GPUI backend channels that claim to interpret them

Group the inventory under these stable capability families:

1. structure and stable part identity
2. layout, sizing, positioning, clipping, and scroll
3. surface, border, radius, shadow, opacity, and resolved visual channels
4. text, icon, image, intrinsic value, typography, wrapping, and truncation
5. semantic token roles and control state
6. focus, keyboard, pointer, activation, drag, scrub, and drop intent
7. accessibility metadata projection
8. overlay, layer order, placement, and dismissal intent
9. input value, caret, selection, clipboard, submit/cancel, and IME boundary

Each public vocabulary item must map to one capability row, an explicit
profile-owned deferral, or a documented retirement candidate. No unclassified
field. A family heading is not a passing row.

Profile-owned deferral means this card certifies the reusable channel but the
first real component remains responsible for its semantics:

- `003`: controlled values, multi-part identity, scrub/capture behaviour
- `004`: repeated identity, selection, roving focus, navigation
- `005`: placement result, dismissal, focus transfer and restore
- `006`: editing semantics, validation, selection, clipboard, IME
- `007`: host commands, hierarchy, composite scenarios

Do not pre-implement those component cases here.

## Capability Authority And Report

Extend the existing conformance package; do not create a second package or
schema family.

Add one canonical typed capability roster. Stable capability IDs used by a
component interface must derive from or validate against it. An unknown,
removed, or misspelled required capability must fail authoring, serialization,
Rust loading or generation, and completion without a second hand-written list.

Emit deterministic `primitive-capability-report.v1` evidence. Each row records:

- stable capability ID and family
- the public vocabulary items it covers
- owning profile and whether this card or a later profile owns semantic proof
- required observation fields
- evidence for Svelte, React, renderer-neutral Rust, and GPUI
- `passing`, `failing`, or `missing` per required layer
- executed probe/case IDs; a source filename or regex is not execution evidence
- a governing contract for any platform-classified exclusion

Jetstream appears once as program-deferred outside the capability rows. It may
not appear as `passing`, `missing`, `not-applicable`, or an accepted per-row
waiver.

The generated JSON is the machine gate. Render its complete current result as
a Markdown matrix in the batch log. Do not hand-edit status in either form.

## Evidence Rules

Use the smallest evidence that can detect an inert implementation:

- Svelte and React: real DOM, computed style, focus, input, and dispatched
  event paths through the generic web runner or its extracted observer layer
- renderer-neutral Rust: real `poodle-render` output or a direct reusable
  `poodle-node` probe when the claim belongs below component composition
- GPUI: `poodle-node` conversion plus the real node backend; use the windowed
  driver for focus, pointer, keyboard, layout, clipping, layer, or raster claims
  that a crate-level observation cannot establish
- image evidence: shared capture ID plus a bounded regional comparison only
  when semantic or computed observations cannot prove drawing, clipping, or
  layer composition

Test-only probe fixtures may differ by runtime mechanism. Their capability ID,
action, expected observation, and verdict may not. Do not introduce a shared
web/native render tree or a public dummy component.

Static source traces may remain as cheap debt detection, but cannot mark a
capability passing. A required row with no executed observation is `missing`
and fails completion.

For every evidence class, plant one representative error in the owning layer:

- web observer or style projection
- `poodle-render` / `poodle-node`
- GPUI style or layout interpretation
- GPUI focus, keyboard, pointer, or event dispatch

The failure must name capability, runtime/layer, probe, and mismatched field.
Revert all plants before handoff.

## Accessibility Boundary

Certify these as separate claims:

- Svelte and React project required DOM accessibility semantics.
- shared Rust composition projects `NodeA11y` metadata.
- the GPUI backend explicitly receives that metadata and its omission remains
  deliberate and observable.

Do not claim mounted assistive-technology proof for GPUI. Report mounted GPUI
accessibility once as the contract-003 forced acceptance, outside passing
capability rows. Do not build a parallel GPUI accessibility tree. Jetstream's
working AccessKit path is outside this card and cannot substitute for GPUI.

## GPUI Capture Repair

Repair `test/native-visual/` as part of certification evidence:

1. Use one canonical control-size flag end to end. A focused run must prove
   the requested non-default size reached the preview; do not infer this only
   from pixels.
2. Comparison mode is read-only. A missing baseline fails with the explicit
   refresh/bootstrap command; it does not write a new reference.
3. Refresh mode preserves the previous reference before replacement and emits
   a machine-readable manifest with slug, axis, dimensions, old/new hashes,
   output paths, and reason field.
4. Keep before, after, and diff evidence available until review. No silent
   overwrite of the only capture.
5. Reclassify stale GPUI baselines through the explicit flow. Do not treat a
   bulk refresh as proof that current rendering is correct.
6. Update the native visual README to match the implemented frame-wait,
   stability, baseline, and refresh behaviour.

Baselines remain local unless this card proves a smaller committed evidence
shape is necessary. Do not add large PNG sets to Git.

## Work Order

1. Record exact before LOC and inventory the public vocabulary, current
   observers, GPUI interpretations, legacy capability declarations, and visual
   workflow.
2. Freeze the capability roster and row ownership. Stop on contradictions
   before building probes.
3. Extract the reusable Button observer/runner/driver seams needed by primitive
   probes. Remove the corresponding Button copy; do not leave adapters beside
   their replacements.
4. Add typed capability authority, deterministic report generation, and
   strict completion checks.
5. Add the smallest executed probes needed to certify the current roster.
   Repair shared primitives or GPUI interpretation when a probe exposes a real
   substrate defect.
6. Repair and exercise the GPUI capture workflow.
7. Route the legacy capability registry and drift script into the new report as
   non-passing debt evidence, or retire them with their standing task wiring.
8. Re-run Button unchanged through the extracted mechanism. It remains the
   component-level regression proof.
9. Measure full mechanism cost and update `conformance-estate.md` plus one
   August batch log.

## Acceptance

- The inventory accounts for every required public vocabulary item and every
  `component-observation.v1` field.
- Svelte, React, renderer-neutral Rust, and GPUI execute every row owned by
  this card; missing required evidence fails the report and completion gate.
- Component capability names are closed over the canonical roster. An unknown
  capability fails before runtime.
- Planting a backend-only style, semantic, focus, or event error fails the
  matching observation with precise identity.
- Generic runners and observers contain no component identifier, part list,
  icon name, or component-specific tree branch.
- Button still passes through the active cohort with no copied harness path.
- A later profile can reference certified capability IDs without adding a
  second capability vocabulary or bespoke backend tolerance.
- GPUI mounted accessibility is described exactly as contract 003 requires;
  no false pass and no scheduled replacement tree.
- Native visual comparison cannot write references. Refresh preserves and
  reports before/after evidence. The control-size axis is demonstrably live.
- Legacy capability tooling has one recorded `adapt` or `retire` disposition
  and one remaining canonical gate per claim.
- The full cost report separates authority, generated output, generic probes,
  runtime adapters, capture repair, wiring, deleted mechanism, and ongoing
  per-component cost.
- Headless `qa` / `ci:native` remain window-free. Jetstream remains outside
  the active cohort and outside this card's dependency graph.

## Stop Conditions

- Certification requires a universal web/native render tree, executable
  behaviour schema, or public probe component.
- The Button pilot harness is copied instead of extracted or reused.
- Generic code needs a component-specific identifier, anatomy branch, fixture,
  or tolerance.
- A source regex, declaration, direct callback call, or pre-backend node is
  offered as proof of backend execution.
- A backend cannot expose enough observation to distinguish implemented from
  inert. Stop with the exact row, attempted probes, and bounded options.
- The work schedules a GPUI accessibility tree or uses Jetstream as substitute
  active-cohort evidence.
- The card starts implementing RangeSlider, Tabs, Popover, TextInput, or
  HistoryCenter semantics.
- Snapshot refresh would overwrite the only reference or require committing a
  large machine-specific baseline set.
- New generic mechanism grows without retiring or extracting equivalent
  Button, capability, observer, or capture machinery.
- Another component implementation must change to make a primitive probe pass.

Stop with evidence and options. Do not widen the model around the finding.

## Writable Scope

- `packages/core/src/conformance/`, its conformance scripts and focused tests
- conformance codegen, neutral fixtures, and generated Rust declarations
- `packages/contracts/node/`, `packages/contracts/layout/`, and
  `packages/contracts/style/`
- generic conformance support in `packages/render/`
- `packages/gpui/node-backend/` and GPUI conformance/preview support
- `test/conformance/` and `test/native-visual/`
- legacy capability declaration and drift surfaces named by this card
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g14/conformance-estate.md`
- one August batch log and append-only `PAPERCUTS.md`

Do not edit component implementations except the existing Button conformance
adapters needed for extraction. Do not edit architecture, specs, contracts,
other roadmap files, generation status, dispatch ledgers, release workflows,
external repositories, or Jetstream sources.

Workers do not change this card's status or the g14 runway status.

## Validation

Use `effigy test --plan` before choosing the test shape. Run:

- the new primitive report and focused probes in every required layer
- `effigy conformance:check`
- `effigy ci:conformance`
- `effigy conformance:cost`
- focused `effigy test:native-visual` runs covering comparison, explicit
  refresh preservation, and at least two control sizes
- `effigy ci:web`
- `effigy ci:rust`
- `effigy ci:native`
- `effigy docs:check`
- `git diff --check`

Run windowed selectors from a live macOS session. Record known main failures
separately from branch regressions. Do not run Jetstream selectors or create a
Jetstream worktree symlink.

## Handoff

Open a PR with:

- the complete executed primitive matrix
- exact inventory coverage and all explicit deferrals
- before/after cost table against the landed `g14.001` state
- planted-failure commands and representative diagnostics
- Button regression result
- GPUI capture migration manifest and stale-baseline ruling
- retained/adapted/retired capability and snapshot tooling
- exact selector results, baseline failures, unresolved rows, and schema
  pressure

Stop after pushing the PR. The orchestrator reviews evidence, records the
verdict, merges, and changes roadmap status.

# g14.001 Second Rework Handoff

## Task

Rework PR #10 in place from replacement commit `0d041108`. Do not merge it,
start `g14.002`, or broaden the conformance model around another component.

Read these main-checkout authorities before editing:

- `docs/architecture/009-cross-runtime-component-conformance.md`
- `docs/specs/066-executable-component-conformance.md`
- `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
- `docs/logs/2026-08/14-g14-001-delivery-review.md`

The delivery review contains the full replacement re-review. Treat it as the
acceptance source; the PR description and branch batch log are stale.

## Required Correction

Complete one coherent replacement batch:

1. Preserve nullable/absent semantics in generated Rust. Add a case proving
   explicit `defaultPressed=false` has the same toggle behavior in all three
   active runtimes. Audit every portable prop for equivalent TypeScript/Rust
   shape rather than fixing that field alone.
2. Make stable parts normalize to the same semantics. The current `label` part
   already disagrees across web and GPUI. Compare the normalized observations,
   not only selected case assertions, and make a planted unasserted observation
   divergence fail.
3. Remove Button knowledge from shared observers. No Button class selector,
   `NodeKind::Button` branch, part list, or icon name may remain in shared
   runner/observer code. If the runtime lacks stable generic part identity,
   stop with options rather than add another Button descriptor workaround.
4. Observe and assert icon identity. Do not redefine parity as presence because
   the current web DOM lacks an identity channel. Apply the same rule to every
   field the Button proof claims to compare.
5. Put executed GPUI evidence on a standing enforcement surface. Compile-only
   does not catch an inert listener. If a real window path cannot be enforced
   safely in the existing standing boards, stop and present the smallest
   viable enforcement options to the orchestrator.
6. Rebuild the cost report from an exhaustive file inventory. Count all four
   committed JSON artifacts and every mechanism line, identify concrete
   deletions, and honor the stop condition if reusable mechanism still grows
   faster without demonstrated second-component reuse.
7. Rebase onto current main, resolve both committed conflict markers, update
   the batch log and PR description, and remove claims the evidence no longer
   supports.

Jetstream stays program-deferred and absent from setup, execution, and
completion. Do not touch the licence work.

## Validation

Use Effigy. Run `effigy test --plan`, every conformance selector, the narrow
Button suites, packed web install, applicable standing gates, and
`git diff --check`. Record exact results and planted failures. A green
`conformance:complete` is necessary but not sufficient: show that nullable
shape drift, a label observation mismatch, wrong icon identity, an inert GPUI
listener, and an orphan artifact each fail the gate that owns the claim.

Push the amended PR and stop for orchestrator review.

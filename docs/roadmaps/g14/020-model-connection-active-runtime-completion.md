# g14.020 — Model Connection Active-Runtime Completion

Status: held for g14.022 carry-forward; web reference approved
Depends on: `g14.019`, `g14.021`
Governing ref: `../../specs/067-model-connection-management.md`

Disposition: do not dispatch this version. `g14.008` rejected the pipeline it
assumes. Preserve its component requirements, then recompile the native work
after cleanup without the portable-interface/shared-corpus claim.

## Outcome

Move the approved model-connection reference suite into the adopted shared
case pipeline and complete Svelte, React, Rust declarations, shared
renderer/node behavior, and GPUI execution. Preserve the approved curated
specimens. Keep Jetstream deferred.

## Goals

- [ ] Author each portable interface and executable case corpus once.
- [ ] Keep the approved web specimens human-centred; share their ordered
      outline later through the specimen-catalogue lane rather than replacing
      them with exhaustive cases.
- [ ] Add normalized semantic observations for selection, setup stages,
      disclosure/enable separation, reorder, visibility, focus, and status.
- [ ] Add Rust declarations and shared renderer/node support without a
      model-provider backend dependency.
- [ ] Execute the required cases in Svelte, React, and GPUI.

## Surviving Native Infrastructure (recorded by `g14.021`)

Recorded, not designed. `g14.021` removed the pipeline this card's plan
assumes; what a rewrite can actually build on is:

- `poodle-render` — the shared native composition tier. One implementation per
  component, interpreted by the GPUI and Jetstream backends. Unchanged, and
  still where native component work belongs.
- `poodle-specs` — hand-written Rust declarations again. No codegen step, no
  interface JSON, no byte-exact authority check.
- `packages/gpui/preview/src/headless_driver.rs` — the in-memory GPUI test
  platform: mount a `poodle-node` tree, drive real pointer/key/drag input
  through the real dispatch tree, read real backend focus. No OS window, no
  focus theft, ~0.05s. Exercised by `tests/headless_regressions.rs` through
  `effigy regressions:native`.
- `effigy test:native-visual` — pixel compare/refresh with `--control-size`.
  Local-only; needs a window.
- The existing drift gates (`docs:spec-drift`, `drift:roles`, `drift:events`,
  `drift:handlers`) — each covers one projection, none proves completion.

What does **not** exist any more: portable interface modules, typed case
corpora, normalized observation, the primitive capability report, the
cross-runtime comparator, and `conformance:complete`. Every acceptance
criterion and plan step above that names one of them needs restating before
this card is dispatchable.

`g14.022` decides the replacement execution method. Do not design it here.

## Acceptance Criteria

- [ ] Every reviewed specimen remains useful documentation; executable case
      coverage is assessed separately.
- [ ] Required interactions execute and compare in all active runtimes.
- [ ] Missing provider marks or host-rendered config are represented as
      composition inputs, not native omissions.
- [ ] Jetstream remains visible deferred evidence, not a completion claim.
- [ ] No Nucleus, Swallowtail, Longhorn, credential, or route authority enters
      Poodle packages.

## Stop Conditions

- g14.008 records revise or reject.
- Completion requires a second component/schema system beside adopted cases.
- GPUI receives static screenshots or declared absence instead of execution.
- The work tries to make Jetstream a prerequisite.

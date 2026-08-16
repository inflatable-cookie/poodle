# g15.009 — Update, Settings, Radio & Context-Provider Native Closure

Status: **blocked** — pending orchestrator review of `g15.001`
Depends on: `g15.001` (measured gaps)
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the measured native gaps that do not belong to the Licence or
model-connection families: the Update and settings workstation group
(UpdateStatus, UpdateCenter, SettingsShell), the standalone Radio primitive,
and the two context providers (IconProvider, UiPresentationProvider) whose
render-tier absence must be either implemented as a passthrough or declared
as a documented capability absence. Primitives and dependencies before
composites; Radio precedes the composites that compose it.

## Scope

| Component | Missing Rust declaration | Missing Rust render | Missing GPUI specimen |
| --- | --- | --- | --- |
| UpdateStatus | yes | yes | yes |
| UpdateCenter | yes | yes | yes |
| SettingsShell | yes | yes | yes |
| Radio | yes | yes | yes |
| IconProvider | — | yes | — |
| UiPresentationProvider | — | yes | — |

## Goals

- [ ] Add hand-written specs and render implementations for UpdateStatus,
      UpdateCenter, SettingsShell, and Radio.
- [ ] Decide and document the render-tier posture for IconProvider and
      UiPresentationProvider: implement a child-passthrough node, or record a
      declared capability absence with the contract's GPUI notes as the
      reason. Either way the absence must be declared — a silent omission is
      drift on every side.
- [ ] Add GPUI specimens and focused headless evidence for the four
      workstation/primitive components.
- [ ] Radio's native evidence must cover single-option semantics without
      borrowing RadioGroup's pass.

## Acceptance

- [ ] Every scoped surface has evidence named in the card log or a declared
      capability absence with reason.
- [ ] `cargo test -p poodle-render`, `effigy check:gpui`, and
      `effigy regressions:native` pass.
- [ ] Jetstream reported as program-deferred, not as an accepted absence.

## Stop Conditions

- A portable interface, shared corpus, or comparator reappears under a new
  name.
- An absence is inferred from silence instead of declared with a reason.
- Work expands beyond the six scoped components without a new card.

## Writable Scope

- Rust declarations, render modules, GPUI specimens, focused tests
- contract notes only where a capability absence must be declared
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `cargo test -p poodle-render`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.

# g15.019 — GPUI Specimen Structure

Status: **planned** — orchestrator review required before dispatch
Depends on: `g15.011` (audit), `g15.017` (axis placement)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Cross-Runtime Agreement), `../../contracts/001-working-rules.md`
(Runtime Parity Authority)

## Outcome

The native catalogue teaches what the web catalogue teaches. Not with the same
layout mechanics — with the same evidence.

`g15.010` closed the last missing GPUI specimen, so every component in the
active cohort has a native page. `g15.011` graded what those pages show, and
found two structural gaps.

- Only 64 of 174 native pages use `specimen_layout`. The rest have no
  `Sizes`/`Densities` panes, so the axis evidence their web counterpart shows
  is simply absent. The audit names the ones where the web page does show it.
- Six native pages render their examples with no captions at all.

Per the working rules, a capability present in Svelte and absent from another
active runtime is a gap to port, not an accepted delta. Layout mechanics are
runtime-owned; the evidence is not.

## Scope

- the GPUI specimen modules named in the audit's GPUI columns
- `specimen_layout` adoption where the web page teaches an axis
- Jetstream stays program-deferred and out of scope

## Goals

- [ ] Every native page whose component takes `size`/`density`, and whose web
      page teaches it, has the matching panes.
- [ ] The six caption-less native pages caption their examples.
- [ ] Native pages that legitimately keep a bounded renderer-owned adapter
      publish their outline instead, and the audit row records that choice.
- [ ] No native page reproduces web layout mechanics for their own sake.

## Acceptance

- [ ] Axis-pane presence matches the web catalogue for every eligible
      component, or the difference is recorded with its reason.
- [ ] No native page renders uncaptioned examples.
- [ ] `effigy check:gpui` and the headless native regression board pass.
- [ ] No Jetstream selector run; no `*-windowed` selector run.

## Stop Conditions

- Native pages start copying web DOM structure.
- The card reaches for a shared render tree. The outline is a document.
- Jetstream parity is smuggled in before its admission gate.

## Writable Scope

- `packages/gpui/preview/src/specimens/*`
- `packages/render/src/audio_specimens.rs`, if `g15.017` has not already split
  its axis groups
- one batch log

## Validation

- `effigy check:gpui`, `effigy regressions:native`, `effigy docs:check`,
  `git diff --check`
- headless only. Never `test:native-visual`, `qa:jetstream`, or a
  `*-windowed` selector.

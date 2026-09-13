# g18.033 — GPUI FormDialog render convergence

Task: `1c2dbb95-6764-4d65-80ea-e6f9065896ae` (`g18.033`)
Branch: `ns-1c2dbb95-6764-4d65-80ea-e6f9065896ae`
Base: `a27a781db37aa09c5e06c08a7b87f9c27a88e1f6` (handoff planning commit)
Runtime head: `ee31c72d512d3fcc586d90bbd023c905d4ca84b8`
Worker: implementation (Poodle GPUI renderer / specimen infrastructure)

## Summary

The `probe:gpui-specimens` non-termination is repaired. The cause was not a
renderer or layout loop inside one draw: the probe opened a fresh window per
catalogue route and never closed it, so a shard accumulated live windows and
the shared app kept redrawing an earlier route's `PreviewRoot`; a later
mount's first draw then never returned. Closing each route window after its
assertions makes the exact `form-dialog` shard, the complete 175-route probe
and the final board's native units terminate.

## Diagnosis (bounded, exact-route child processes)

Method: one prebuilt test binary, exact-route children capped at 20–30s, no
broad board during diagnosis.

1. **Reproduced.** The pre-fix shard 3
   (`canonical_catalogue_constructs_every_route_and_axis_pane_3`) printed
   `probe: mounting form-dialog` and then consumed CPU until killed. This
   matches g18.032's observation.
2. **The route itself is not the fault.** Mounting `form-dialog` (or `dialog`,
   `drawer`, `field`) alone in a fresh `TestAppContext` returned immediately
   and green. The hang only appeared after several routes shared one app.
3. **Reduction.** A temporary env-selected reduction emptied the
   `form-dialog` specimen body, its page header and its usage-docs support
   panel. The route still hung, so the FormDialog composition, submitting
   spinner and usage docs were all eliminated as causes.
4. **The loop is a redraw loop, not a stuck draw.** Instrumenting
   `PreviewRoot::render` showed thousands of renders of a *previous*
   window's root while the next route was being opened; `sample` put the
   stack under `open_route_window` → `finish_update` → `flush_effects` →
   `Window::draw`. Every mount added one more live window to redraw.
5. **Two mounts reproduce it; closing fixes it.** Mounting `form-dialog`
   twice in one app reproduced the hang (nondeterministically, consistent
   with gpui's unordered window map). `form-dialog` ×8 in one app after each
   route window was closed with `Window::remove_window()` returned in under a
   second. Repeated `button`, `dialog` and `text-input` mounts did not hang,
   which is why the leak only surfaced at the overlay-heavy `form-dialog`
   route.

Correction to the card premise: the non-termination was reached from
`open_route_window`'s first draw, but the *cause* is the probe's window
lifecycle, not shared FormDialog/Button/Spinner composition. No renderer or
node-backend behaviour change was needed.

## Repair

`packages/gpui/preview/src/specimen_probe.rs` (test-only module):

- Added `close_route_window`, which marks the route's window removed.
- The sweep closes each route's window after its assertions and then asserts
  `app.windows()` is empty for that app. That is the causal regression: on
  the pre-fix implementation the assertion fails on the first route instead
  of hanging, and on the fixed implementation it holds for all 175 routes.
- The seam-proof tests that mount through `open_route_window` close their
  windows too, so no probe path leaves a live `PreviewRoot` behind.
- The `open_route_window` doc now states the close obligation.

No route was skipped, no timeout raised, no specimen, submitting state, axis
pane, canonical route or denominator changed. `FormDialog` composition,
`packages/render`, and `packages/gpui/node-backend` are untouched.

## Provenance repin

The Nucleus M1/A1 receipt binding covers `packages/gpui/preview`, so the
test-only probe edit invalidates it by construction. Following the recorded
g18.004/g18.030 precedent, a separate commit rebinds `source_commit` in the
Nucleus manifest and its 59 receipt files from
`a797ce413d378795a8698d6782b003560878315a` to
`ee31c72d512d3fcc586d90bbd023c905d4ca84b8`. The change is identity only;
`bun scripts/parity-evidence-ledger.ts --write` regenerates
`docs/evidence/nucleus/parity-evidence-ledger.md` with zero delta, and
`bun scripts/parity-evidence-ledger.ts` validates 176 rows.

## Validation (bounded budget)

Focused selectors only during implementation, then one final broad board.

| Proof | Command | Result |
| --- | --- | --- |
| Exact `form-dialog` shard | `poodle_preview-… …and_axis_pane_3` in a 30s child | 44/44 routes, test body 1.7s, exit 0 |
| Complete specimen probe (175 routes) | `cargo test … --bin poodle-preview specimen_probe` | 9/9 tests, 175/175 routes (44+44+44+43), finished 2.27s |
| Ledger | `bun scripts/parity-evidence-ledger.ts` | 176 component rows validated, exit 0 |
| Final board | `effigy qa:board` | 556s (9m16s); all web + native units before `audit:licenses` green, including `probe:gpui-specimens` 9/9 |

Final-board detail: the first `effigy qa:board` attempt (before the repin
commit) aborted at `check:parity-evidence-ledger` because the receipt binding
requires a committed runtime head; after the repin commit the single final
board ran. Its only failure is the pre-existing, recorded `audit:licenses`
red — see below. `git diff --check` is clean.

## Pre-existing red, unchanged

`audit:licenses` fails with `deny.toml: still claims bzip2, which no lockfile
resolves`. This is recorded in `PAPERCUTS.md` (2026-09-10) and was verified
unchanged versus the dispatch head: `deny.toml`, every `Cargo.lock` and every
package manifest are byte-identical to `a27a781db`, and the task diff touches
only `packages/gpui/preview/src/specimen_probe.rs` and the 59 Nucleus
provenance files. It is the same accepted pre-existing-red disposition g18.003
used (its log records the identical failure as out of scope). The board did
not reach `audit:security`, which was not listed as red in that precedent.

The named timeout outcome g18.032 had to report is gone: `probe:gpui-specimens`
now completes inside the board in 2.8s with all 9 tests and all 175 routes
green.

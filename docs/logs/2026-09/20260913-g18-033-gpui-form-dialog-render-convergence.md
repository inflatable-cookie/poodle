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
| Final board (first) | `effigy qa:board` | 556s (9m16s); all units before `audit:licenses` green, including `probe:gpui-specimens` 9/9; stopped on the pre-existing bzip2 allowance |
| License leaf after the authorized removal | `effigy audit:licenses` | clean, 9 package manifests / 17 Cargo manifests / 4 notice surfaces, exit 0, 5s |
| Replacement final board | `effigy qa:board` | 614s (10m14s); `audit:licenses` now green; stopped on `audit:security` (`EISDIR` on the tracked symlink) |
| Symlink repair focused test | `bun test scripts/audit-repository-security.test.ts` | 19/19 pass, including both planted symlink fixtures; both fail on the pre-fix script |
| Security leaf after the symlink repair | `effigy audit:security` | repository-script step now clean (5126 files); stopped at the `bun audit` step (see below) |
| Symlink-repaired replacement board | `effigy qa:board` | 431s (7m11s); all units before `audit:security` green (ledger, `probe:gpui-specimens` 9/9, `audit:licenses` clean, repository-security script clean 5126 files); stopped at `bun audit` |

Final-board detail: the first `effigy qa:board` attempt (before the repin
commit) aborted at `check:parity-evidence-ledger` because the receipt binding
requires a committed runtime head; after the repin commit the single final
board ran. The operator then confirmed the canonical ruling on `main` at
`f7c6d44f3`, widening this task to remove the obsolete `bzip2-1.0.6`
allowance and its comment from `deny.toml` (nothing else in the license
policy changed), and authorized one replacement board because the candidate
had changed. `git diff --check` is clean.

## Baseline drift cleared, next baseline red exposed

The pre-existing `audit:licenses` red (`deny.toml: still claims bzip2, which
no lockfile resolves`) is cleared by the authorized removal of the stale
allowance. The focused leaf is green.

The replacement board then exposed the next baseline defect, which is not
caused by this change: `audit:security` failed with `EISDIR: illegal operation
on a directory, read` at `scripts/audit-repository-security.ts:63`. The audit
enumerates `git ls-files --cached --others --exclude-standard` and
`readFileSync`s every entry; the tracked symlink `.claude/skills/impeccable`
(mode `120000`, added by `54ea1ec47 chore(skills): install Impeccable` on
2026-09-11) resolves to the tracked directory `.agents/skills/impeccable`, so
the read followed it and threw.

Operator ruling on `main` at `adf6420ba` authorized a non-following, fail-closed
repair in `scripts/audit-repository-security.ts` and its focused test. The walker
now `lstat`s each tracked entry and, for a symlink, audits the link's own text
and only reads a target that is a regular file; a directory (or gitlink) target
is never read, and a dangling link still has its text audited. The intentional
Impeccable link and its tracked target are unchanged. Two planted fixtures in
`scripts/audit-repository-security.test.ts` prove the behaviour: a tracked
symlink to a tracked directory must audit clean, and a tracked symlink whose
link text carries a secret pattern must fail. Both fail against the pre-fix
script with the exact `EISDIR` crash. The focused real audit is clean
(5126 repository files).

The security leaf then exposed a further, separate baseline red that this task
is not authorized to repair:

- `effigy audit:security` now clears its repository-script step and stops at
  `bun audit`.
- `bun audit` reports two moderate vulnerabilities for GHSA-82fw-gwwq-j7x9:
  `vitest` and `@vitest/mocker` are in the advisory range `>=2.1.0 <4.1.11`,
  and this repository resolves both at `4.1.10` (`package.json` declares
  `vitest: ^4.1.10`; `bun.lock` pins `vitest@4.1.10` and
  `@vitest/mocker@4.1.10`).
- The advisory is newer than the last recorded clean run of this leaf
  (g16.053/g16.054, 2026-09-02/05) and is untouched by this change: the task
diff contains no dependency or lockfile edit. Clearing it needs a vitest bump
  to `>=4.1.11`, which the symlink ruling did not authorize.

Per the completion protocol this is reported as a blocker with the exact
evidence rather than silently widening scope or accepting a red board. The
authorized replacement board ran on the symlink-repaired candidate for 431s
(7m11s): the Nucleus ledger, `probe:gpui-specimens` (9/9), `audit:licenses`
and the repository-security script (5126 files) are green, and `audit:security`
stops at `bun audit` with the two moderate Vitest advisories above. That is
the only failure the board reports.

The named timeout outcome g18.032 had to report is gone: `probe:gpui-specimens`
now completes inside the board in 2.8s with all 9 tests and all 175 routes
green.

# 033 — GPUI FormDialog render convergence

Status: ready
Owner: Poodle GPUI renderer and specimen infrastructure
Created: 2026-09-13
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/form-dialog.md`,
`../../specs/071-fast-validation-and-npm-release-pipeline.md`,
`../../../packages/gpui/preview/src/specimen_probe.rs`
Depends on: g18.032

## Outcome

Make the GPUI `form-dialog` specimen's first draw terminate while preserving
the real submitting-state specimen. Restore `probe:gpui-specimens` across all
175 portable routes and make the complete bounded headless board pass inside
its existing fifteen-minute ceiling.

## Ready-State Rubric

- [x] The failure is isolated to the first `form-dialog` draw, before the
  probe's internal settle or body budget can run.
- [x] A process sample places the non-termination in GPUI layout/state-patch
  work reached from the real renderer.
- [x] The route, submitting example, 175-route denominator and existing
  runtime bounds are fixed acceptance constraints.
- [x] Renderer, backend and focused probe paths needed for a causal repair are
  owned.
- [x] The operator rejected timeout-only mitigation as incomplete and made
  this repair the immediate Poodle priority.

## Decisions

- A timeout is containment, not success. g18.032 remains valid release-lane
  infrastructure, but its deferred GPUI leaf is an open delivery obligation.
- Repair the non-converging render/layout lifecycle. Do not skip
  `form-dialog`, remove its submitting state, weaken the specimen probe, or
  increase a timeout.
- Treat the prior spinner explanation as a lead, not established causality.
  Preserve ordinary FormDialog behavior and prove the actual minimal cause.
- Diagnose with one prebuilt test binary and short exact-route child-process
  runs. Broad boards are completion proof, not a debugging loop.
- The first final board exposed the unrelated, pre-existing `deny.toml`
  `bzip2-1.0.6` allowance after the repaired GPUI leaf passed. Remove that
  obsolete allowance in this task rather than accepting another red board.
- The replacement board then exposed the final top-level leaf,
  `audit:security`: its repository walker follows a tracked symlink to a
  directory and crashes with `EISDIR`. Audit the symlink itself without
  following it; keep the intentional Impeccable link and its tracked target.

## Dispatch manifest

- **State:** ready; sole active Poodle queue task
- **Completion:** independently reviewed PR merged after the exact route,
  complete 175-route probe and one complete `qa:board` all pass within their
  current bounds
- **Owned mutable paths:** `packages/gpui/preview/src/specimen_probe.rs`,
  `packages/gpui/preview/src/specimens/form_dialog_specimen.rs`, focused
  test-only support under `packages/gpui/preview/`,
  `packages/render/src/form_dialog.rs`, `packages/render/src/button.rs`,
  `packages/render/src/spinner.rs`, focused renderer tests, and causal
  implementation/tests under `packages/gpui/node-backend/src/`; `deny.toml`
  only for removal of the obsolete `bzip2-1.0.6` allowance and its comment;
  `scripts/audit-repository-security.ts` and its focused test only for
  non-following, fail-closed tracked-symlink handling
- **Reserved closeout surfaces:** this task and handoff, g18 README,
  roadmap root/index/dispatch, spec 071, `PAPERCUTS.md`, and one execution log
- **Worker:** complex Rust/GPUI renderer worker able to diagnose layout and
  invalidation non-termination with bounded subprocess evidence
- **Excluded:** route or state deletion, reduced route denominator, timeout
  increases, test ignore/waiver, shipped contract changes, windowed capture,
  npm/release mutation, web changes and unrelated native cleanup
- **Escalation:** Chatterbox only if the causal repair requires an observable
  FormDialog contract change or a GPUI upstream patch beyond repository-owned
  code

## Work

1. Build the focused GPUI preview test binary once without running the suite.
   Reproduce the exact `form-dialog` route in a child process capped at thirty
   seconds. Do not start `qa:board` during diagnosis.
2. Instrument or reduce the failing node tree until the first non-converging
   renderer/backend operation is proven. Distinguish animation scheduling,
   state-patch invalidation, overlay layout and specimen composition rather
   than assuming the submitting spinner is causal.
3. Add a regression that fails closed without hanging the test process: a
   bounded child-process route proof or a lower-level convergence law that
   reproduces the causal loop.
4. Apply the smallest repository-owned repair. Preserve the submitting
   FormDialog example, its visible disabled/submitting semantics, normal
   loading animation after a committed frame, and the other 174 routes.
5. Run the exact route proof, then `probe:gpui-specimens` once. It must report
   all nine tests and all 175 routes green inside the existing child bound.
6. Run one final `qa:board`. Every required unit must pass inside the existing
   fifteen-minute board cap. A bounded failure is a failure, not completion.
7. If that board reaches the baseline `audit:licenses` failure after every
   preceding unit passes, remove only the stale unresolved `bzip2-1.0.6`
   allowance from `deny.toml`, rerun the affected license leaf, then run one
   replacement final board. This replacement is authorized because the
   candidate changed after the first board; it is not a speculative retry.
8. If the replacement reaches the final `audit:security` leaf and reproduces
   `EISDIR` on `.claude/skills/impeccable`, make the audit inspect a symlink's
   own link text rather than following its target. Add a focused symlink-to-dir
   regression, run `audit:security`, then one final replacement board. Do not
   remove or rewrite the intentional symlink. The candidate changed again, so
   this replacement proof is authorized.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| First draw converges | `form-dialog` still consumes CPU until an outer timeout kills it | exact-route subprocess exits green under 30 seconds after the binary is built |
| Real state remains | repair removes the submitting example or renders a non-submitting substitute | focused specimen assertion observes the submitting FormDialog and its disabled/submitting action semantics |
| Cause is repaired | probe special-cases the slug or suppresses draw/settle | source review plus a causal lower-level regression that fails on the pre-fix implementation |
| Catalogue proof is whole | one shard passes while another route is omitted or ignored | `probe:gpui-specimens`: 9/9 tests and 175/175 canonical routes |
| Board is genuinely healthy | timeout handling reports the hung leaf but the task calls that success | one `qa:board` run: every owned unit green under 15 minutes, with no waived or skipped required leaf |
| Loading behavior survives | all looping motion is disabled to make tests terminate | existing loading first-frame/committed-frame law plus focused renderer/backend evidence |
| Baseline license drift is cleared | the GPUI repair passes but the board remains red on an unresolved retired allowance | focused `audit:licenses` plus the replacement full board are green after removing only the stale `bzip2-1.0.6` entry |
| Security audit handles tracked symlinks | `readFileSync` follows a tracked directory link and throws `EISDIR`, or the fix simply skips all symlink evidence | planted tracked symlink-to-directory proof shows the link text is audited without traversing it; `audit:security` and the final replacement board pass |

## Stop conditions

- Stop after a capped diagnostic child identifies a required observable
  FormDialog contract change; return the exact cause and choice to Chatterbox.
- Stop rather than raising the route, child or board timeout.
- Stop if the proposed repair deletes coverage, changes the 175 denominator,
  ignores the test, or makes the specimen cease to represent submitting state.
- Stop after the single final board failure with the named remaining leaf; do
  not relabel it complete or rerun the board speculatively.

## Evidence

g18.032's final board passed 54/68 units and killed
`probe:gpui-specimens` at five minutes. Its earlier accidental baseline left
the same process at roughly 100% CPU for 2h37m. A focused shard mounts through
`drawer`, enters the initial `form-dialog` draw and never returns. `sample(1)`
places the stack under `Window::draw`, flex layout and
`poodle-gpui-node-backend` state patches. The probe's own settle and two-minute
body limit are downstream of the stuck draw and cannot contain it.

## Next task

After this repair and the genuinely green full board, resume the post-`0.4.0`
consumer/specimen sweep. Compatible fixes feed `0.4.1` through the g18.032
release path.

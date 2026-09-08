# g17.003 — Background-safe non-activation proof

Status: complete — merged in PR #230 at merge commit `583aa173935dd66ea0d8bd17196115f5b211a01b` on 2026-09-08; independently reviewed at head `62be0575a51919697f78dbeaf9789339d83ae2f6`
Type: native capture evidence correction — no component or pixel change
Opened: 2026-09-08
Depends on: none
Governing refs: `../g16/122-window-capture-cohort-fixtures.md`,
`../g16/051-icon-geometry-native-visual-admission.md`,
`../../contracts/001-working-rules.md`
Operator decision: 2026-09-08 — Poodle Lab captures must run cleanly in the
background while the operator continues using other applications.
Dispatch manifest: `../dispatch.md`

## Goal

Prove that `poodle-window-capture` never activates itself without requiring the
operator's foreground application to remain frozen for the whole batch.

The current monitor treats every application transition as evidence that the
capture process activated. That is not causal. An operator switching from
Paseo to Singlebox, or a notification application becoming frontmost, fails an
otherwise background-safe run even when the capture window remains unfocused,
non-key, and inactive. The lab then loses the whole in-memory batch.

## Fixed Boundary

- Change the native foreground monitor to sample both the frontmost process
  identity and PID. Compare the sampled PID with the capture process's own PID.
- Unrelated foreground transitions are admissible and remain recorded in the
  receipt. The verdict is publishable only when the baseline is readable,
  enough successful samples exist, no required read failed, and the capture
  process was never frontmost.
- A capture-process baseline or any later self-frontmost sample fails. Existing
  focus, key-window, application-active, exact-window, permission, scale,
  repeat, process-bounding, and atomic-publication gates remain mandatory.
- Keep the evidence auditable. If the existing receipt fields cannot establish
  which process was being tested, add the smallest explicit capturer identity /
  self-observation fields and update the schema, parsers, diagnostics, and lab
  adoption request together. Do not add a compatibility shim or silent
  fallback.
- Update transport comments, diagnostics, tests, and roadmap language that
  currently claim the entire desktop foreground was unchanged. The claim is
  narrower and stronger: the capture process did not activate itself.
- Apply the rule to every `poodle-window-capture` mode through the shared
  transport. Do not change components, scenarios, fixture rendering, pixels,
  comparison policy, or capture ordering.
- Produce a named Poodle Lab adoption request with the exact commit and receipt
  contract changes. Lab must update its external `ForegroundWatch` to reject
  its controller becoming foreground while allowing unrelated operator
  transitions, then repin Poodle before another full cohort capture.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Operator remains free | editor → browser → editor while capture never fronts | publishable foreground verdict; all identities retained |
| Native capture never fronts | one sample whose PID equals the capture PID | typed failure; no PNG or receipt published |
| Bad baseline never passes | capture PID is frontmost before its first window | typed failure |
| Evidence fails closed | unreadable baseline, failed required read, or too few samples | unprovable; no publication |
| Window proof remains independent | foreground proof passes but window is focused/key/active | capture still rejected |
| Every mode inherits | cohort, icon geometry, button, focus, and inset modes | shared transport tests/build; no local bypass |
| Lab can adopt exactly | request names commit and any schema delta | one complete cross-repo adoption request |

Plant the PID/identity samples in unit tests. No foreground capture is needed to
prove this implementation batch. A later operator-approved Lab run is the live
acceptance test after adoption.

## Validation

Use Effigy to select the narrow window-capture tests and one windowless build.
Run the relevant Rust tests, repository type/docs checks required by the
selected tasks, and `git diff --check`. Never run a windowed selector, release
mutation, or workflow edit.

## Owned Paths

`packages/gpui/preview/src/bin/window_capture/transport.rs`, its receipt modes
and diagnostics when required by the shared evidence shape, focused
window-capture tests, directly affected capture documentation, one Lab adoption
request under `docs/handoffs/`, and one execution log under
`docs/logs/2026-09/`. Reserved for coordinator closeout:
`docs/roadmaps/g17/README.md`, `docs/roadmaps/generation-index.md`, and
`docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop and report if AppKit cannot identify the frontmost process by PID, a
capture mode bypasses the shared transport, or the receipt must break consumers
beyond the named Lab adoption. Do not weaken the proof to bundle-name guessing,
permit unreadable samples, or run a windowed probe. Escalation owner:
Chatterbox.

## Continuation

After this PR merges, resume the existing queue-managed Lab `g01.006` thread.
Adopt the exact Poodle commit, correct Lab's external foreground proof to the
same non-activation semantics, then pause for one separately authorized cohort
capture.

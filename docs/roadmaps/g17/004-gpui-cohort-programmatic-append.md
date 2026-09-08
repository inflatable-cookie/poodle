# g17.004 — GPUI cohort programmatic append replay

Status: ready
Type: cross-language scenario-contract repair — no component API change
Opened: 2026-09-08
Depends on: `g17.003` complete
Governing refs: `003-background-safe-nonactivation-proof.md`,
`../g16/122-window-capture-cohort-fixtures.md`,
`../../contracts/001-working-rules.md`
Operator decision: 2026-09-08 — do whatever is required to finish the blocked
Lab cohort lane; preserve the existing queue task and close it through PR,
review, merge, and cleanup.
Dispatch manifest: `../dispatch.md`

## Goal

Make `poodle-window-capture --cohort` consume the complete pinned Nucleus A1
action vocabulary. The GPUI after-actions image for AgentTranscript must append
the declared item before capture, matching the Svelte and React hosts.

The 2026-09-08 Lab run reached 96/174 captures, then failed closed because
`cohort_capture.rs` rejected `programmatic_append`. Poodle's TypeScript contract,
scenario, extractors, and headless Rust A1 model already carry the action. The
window-capture binary has a separate stale enum and replay controller.

## Fixed Boundary

- Add the closed `programmatic_append { item }` variant to the cohort capture
  parser. Keep unknown actions and fields rejected.
- Give the cohort host explicit transcript-item state initialized from scenario
  props. Initial capture remains unchanged.
- On after-actions replay, accept the action only for AgentTranscript, validate
  the declared item through the same closed message/activity mapping used by
  the renderer, append once, remount, and wait for the normal settle frames.
- Fail closed for a malformed item, unsupported item kind, or use on any other
  component. Do not silently ignore the action.
- Keep pointer/key replay, scenario files, component APIs, pixels outside the
  intended after-actions state, foreground proof, capture ordering, scale,
  receipts, and publication law unchanged.
- Produce a named Lab adoption request with the exact merge/pin requirement.
  The existing Lab `g01.006` queue task is resumed after merge; do not create a
  replacement Lab task or discard its failure history.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Contract parses | canonical AgentTranscript scenario | production loader accepts every registered scenario |
| Initial is stable | append leaks into initial host state | initial transcript projection remains scenario props only |
| Append is real | parser accepts but renderer does not change | after-actions projection contains the declared item exactly once |
| Scope is closed | append action attached to another component | typed preparation/replay failure |
| Item shape is closed | unknown kind or missing required field | refusal before publication |
| Existing replay holds | pointer or key action | focused cohort tests remain green |
| Lab can repin | adoption request omits exact contract boundary | request names merged commit requirement and no Lab-side semantic change |

Use pure parser/state/replay tests and the existing headless renderer harness.
Do not invoke a windowed selector. The next Lab capture is separately executed
from the preserved Lab queue task after this repair merges.

## Validation

Use Effigy task inventory and test planning. Run the narrow cohort-capture Rust
tests, the relevant headless A1 checks, docs checks, `git diff --check`, and one
windowless capture-binary build. Do not run release mutations or edit workflows.

## Owned Paths

`packages/gpui/preview/src/bin/window_capture/cohort_capture.rs`, focused
cohort/headless tests when needed, one Lab adoption request under
`docs/handoffs/`, and one execution log under `docs/logs/2026-09/`. Reserved for
coordinator closeout: `docs/roadmaps/g17/README.md`,
`docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop and report if the canonical item cannot be represented by the production
AgentTranscript renderer, replay requires changing the public component API, or
the repair would weaken any fail-closed capture gate. Escalation owner:
Chatterbox.

## Continuation

After merge, resume the existing Lab `g01.006` worker. Repin the cohort manifest
to the exact Poodle merge, rebuild windowlessly, then run one fresh full cohort
capture under the operator's 2026-09-08 completion authority. On success,
validate the bundle and continue the same task through PR, independent review,
merge, closeout, and workspace/thread archival.

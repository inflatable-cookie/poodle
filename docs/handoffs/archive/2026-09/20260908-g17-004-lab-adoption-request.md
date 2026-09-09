# g17.004 — Lab adoption request: GPUI cohort programmatic append replay

Status: request — adopt the merged `g17.004` repair in PR #231 at merge commit
`8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983` so the pinned Nucleus
cohort parser accepts the complete A1 action vocabulary; unblocks the
poodle-lab `g01.006` GPUI-leg cohort lane
Date: 2026-09-08
Poodle card: `docs/roadmaps/g17/004-gpui-cohort-programmatic-append.md`
Poodle implementation commit: `2cf135d1820068f331b5ec908013dc7ba2ad0f61`
Poodle implementation branch: `ns-32f0bd22-428b-4fdf-93a6-cf257d0768df`
(request written before the PR opened; the PR head adds documentation only
and has the same runtime tree over the capture surface)
Poodle base for Lab pin: merge commit
`8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983`, which carries the g17.004
runtime repair — Lab repins to that exact commit before its next cohort
capture, per the existing pin rule. No Lab-side code or semantic change is
required.

## Why Lab must adopt

The 2026-09-08 Lab cohort run completed 96/174 captures and then failed
closed at `cohort/agent-transcript/initial [gpui]`: the pinned
`poodle-window-capture` scenario parser did not know the `programmatic_append`
action the canonical AgentTranscript scenario declares, although the
TypeScript A1 contract, the scenario, the web extractors, and the Rust
headless A1 model all already carry it. Deleting the action from the
scenario would hide the state instead of comparing it, so the repair adds
the closed variant to the capture binary's own vocabulary.

## What Lab must change before the next full cohort capture

1. Repin Poodle to g17.004 merge commit
   `8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983` on `main`.
2. Re-run Lab's cohort capture from the preserved `g01.006` queue task —
   do not create a replacement task or discard the run's failure history.

## What changed for Lab

- The GPUI after-actions image for AgentTranscript now shows the declared
  appended item once (a `message` with id `appended`), matching the Svelte
  host. Receipt schemas, foreground evidence, and every other capture
  contract are unchanged: the run still publishes the same
  `poodle.cohort-visual-capture.v2` receipt shape with the same fields.
- The scenario parser is closed on the full A1 vocabulary: unknown action
  types and malformed appended items are still rejected before publication.

## Acceptance after adoption

One fresh full cohort capture from the resumed `g01.006` task: all 174 rows
publish, the `agent-transcript` initial and after-actions images both land,
and the after-actions image contains the appended item exactly once. After
adoption, continue the same task through PR, independent review, merge,
closeout, and workspace/thread archival per the card's continuation note.

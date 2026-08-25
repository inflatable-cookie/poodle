# g15/g16 Generation Boundary Consolidation

Date: 2026-08-25
Posture: planning recovery

## Decision

The provisional g16 release-recovery and consumer-adoption runway did not begin
a new strategic programme. It implemented, corrected, and adopted the release
built by g15. Its 25 cards therefore move into g15 as one uninterrupted closeout
sequence.

## Mapping

`g16.001`–`g16.025` map in order to `g15.055`–`g15.079`. The mapping is a fixed
offset of 54; statuses, execution evidence, PR history, and card order are
unchanged.

Historical handoff, branch, and log filenames keep their original g16 labels so
point-in-time evidence remains traceable. Their canonical card IDs and roadmap
paths now point to g15. No implementation or release artifact changed.

## Result

- g15 owns the complete v0.2.x release programme: roster closure, publication,
  crates.io-GPUI recovery, v0.2.2 certification, and 16-consumer adoption.
- g15 is complete through `g15.079`; `g15.056` remains superseded and
  `g15.078` remains cancelled by repository removal.
- g16 is an empty operator-led planning slate with no ready card.

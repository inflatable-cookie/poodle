# g16.051 — Icon Geometry Native Visual Admission

Status: blocked — requires completed `g16.050`, an operational Button-only
lab, and a completed icon-geometry lab adapter/manifest extension
Type: external evidence / admission
Opened: 2026-09-01
Depends on: completed `g16.050` and every serial gate in
`visual-lab-unblock-runway.md`, including an accepted VL-1 bootstrap and a
separately planned, reviewed, authorized, and landed icon-geometry lab adapter
and manifest extension
Governing refs: the icon-geometry architecture and evidence from `g16.049`–
`g16.050`, `../../architecture/012-semantic-motion-policy.md`

## Goal

Deliver IG-06 only: curate the candidate pairs' intermediate geometry and
obtain exact-window GPUI visual evidence through the dedicated lab. Decide
admit/revise/reject. Do not create the lab, run a local Poodle windowed selector,
or add public IconMorph in this card.

## Admission Boundary

- Poodle owns fixture meaning, pair quality, geometry policy, comparison
  criteria, and the admission verdict.
- The separate lab owns one short-lived, operator-approved, non-activating GPUI
  process per fixture, exact-window capture, clean exit, and typed focus /
  permission / provenance receipts.
- The accepted lab MVP is Button-only and rejects unknown adapters and fixture
  IDs. Before this card can start, a separate lab-repository planning and
  implementation lane must add a closed icon-geometry adapter and manifest
  extension under the same allowlist, provenance, and receipt laws. Bootstrap
  alone is insufficient.
- Capture endpoint, midpoint, reverse, frozen, interruption, and teardown
  states twice. Repeats must agree. Headless/browser evidence from `g16.050`
  remains necessary but cannot substitute for native pixels.
- Missing WindowServer/permission, activation, broad capture, changed foreground,
  missing provenance, disagreeing repeats, or unavailable lab stops the card.
- Failure leaves the internal capability unadmitted. It does not authorize a
  web-only public surface or a silent static-equivalence claim.

## Ordered Work

1. Verify the exact `g16.050` head, green headless evidence, candidate pair
   inventory, fixture hashes, comparison rules, operational Button MVP, and
   the separately completed icon adapter/manifest-extension receipts.
2. Perform Poodle-owned intermediate-frame review in both directions.
3. Through separately authorized lab operations, collect two exact-window
   receipts per named native state. Do not invoke a Poodle local selector.
4. Verify transport, focus/permission/provenance, endpoint fidelity, geometry,
   color/stroke, repeat stability, foreground invariance, and teardown.
5. Record the admit/revise/reject verdict and retained digests in Poodle. Leave
   public contract admission to a later post-pass planning decision (IG-07).

## Acceptance

- Every pair admitted by this gate has complete reviewed web/headless/native
  evidence for both directions and the named states.
- Native capture is exact-window, non-activating, foreground-proved, sanitized,
  reproducible, and tied to immutable Poodle/lab/toolchain inputs.
- Endpoint geometry, intermediate quality, stroke/color, frozen output,
  interruption, and teardown satisfy the promoted architecture and budgets.
- No missing fixture/runtime is skipped or called not-applicable. Any rejected
  pair stays rejected with evidence.
- No Poodle production source, public API, visual ledger cell, release,
  workflow, consumer, local windowed route, or Jetstream surface changes.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Headless is not pixels | GPUI path assertion passes, lab absent | status stays blocked; no native visual claim |
| Button bootstrap is not icon support | operational Button MVP rejects an icon fixture ID | status stays blocked until the separately completed icon adapter/manifest extension lands |
| Capture is exact and non-activating | foreground changes or desktop fallback | typed failure; bundle is rejected |
| Repeats are deterministic | midpoint captures disagree | pair/state fails; no frame picking or averaging |
| Pair quality is human-reviewed | numeric residual passes visible twist | explicit rejection despite numeric result |
| Public scope remains gated | add IconMorph after partial evidence | scope audit fails; IG-07 remains uncompiled |

## Writable Scope

Poodle-side fixture request/criteria and sanitized admission records under the
owning execution log; this card; new papercuts. Lab code, manifests, capture,
and artifacts belong to the separately authorized lab repository/run. Do not
edit Poodle production packages, public contracts/exports, workflows, releases,
consumers, visual ledger cells, or Jetstream behavior.

## Validation

Use Poodle headless verification and docs checks plus the lab's separately
owned receipt/hash/security validation. Run `effigy docs:lint`,
`effigy docs:check`, and `git diff --check origin/main...HEAD` in Poodle. Never
run local `*-windowed` or native-visual selectors.

## Stop Conditions

Stop on absent Button-lab or icon-extension authority/capability, activation,
broad capture, unproved foreground, permission drift, incomplete provenance,
mismatched repeats, endpoint/intermediate failure, or any request to admit a
public/web-only surface from partial evidence.

## Continuation

Only an accepted admission verdict permits a later IG-07 planning batch for a
separate public IconMorph contract and curated active-cohort implementation.
That batch has no card number yet.

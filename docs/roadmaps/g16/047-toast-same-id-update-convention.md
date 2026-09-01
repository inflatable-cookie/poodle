# g16.047 — Toast Same-Id Update Convention

Status: ready
Type: implementation
Opened: 2026-09-01
Depends on: merged `g16.034`, completed `g16.043` research, and the accepted
convention in `../../triage/20260901-230403-toast-update-convention.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/toast-host.md`,
`../../contracts/components/toast-stack.md`, `../../specs/015-loading-empty-error-notification-and-remediation-rules.md`,
`../../contracts/003-native-accessibility.md`,
`../../architecture/012-semantic-motion-policy.md`

## Goal

Make host-owned same-id pending-to-settled toast updates safe. Repair timer,
announcement, focus, and native danger-role semantics without adding a promise
helper, lifecycle field, progress slot, or second creation API.

## Fixed Envelope

- One live row per `id`. Same-id updates replace copy, tone, action, and sticky
  state in place without restarting enter motion. Reuse after exit cleanup is
  a new record.
- Pending is consumer operation state plus `sticky: true`; settled is the same
  id with new host data. Toast owns no `pending | settled` field.
- Sticky rows have no clock. Sticky → non-sticky starts one fresh default
  clock. Non-sticky → sticky clears it. Copy/tone/action-only updates do not
  reset a running clock; removal clears it.
- New rows and discrete settlement announce once. Numeric progress churn never
  lives in the atomic toast row; Progress or a host-owned status owns it.
- Action replacement keeps focus. Removing the focused action moves to that
  row's dismiss control, then next, previous, then the connected entry point.
- Danger rows project `NodeRole::Alert`; other native rows stay list items.
  GPUI 0.2.2 metadata is not an AT-parity claim.
- Persistent failures remain reachable in a durable host surface under spec
  015. Consumer stores, retry, cancellation, persistence, and expiry policy
  stay outside Poodle.

## Ordered Work

1. Amend ToastHost and ToastStack contracts first with uniqueness, timer,
   announcement, focus, remediation, and native-role laws.
2. Repair `reconcileToastTimers` for become-sticky and prove the complete
   same-id timer transition table.
3. Align Svelte/React presence, announcement, and focus behavior. Align shared
   Rust danger roles without claiming GPUI accessibility support.
4. Add one pending-to-settled specimen per active construction path and biting
   focused/mounted evidence. Record one execution log.

## Acceptance

- Sticky pending schedules no timer; success settlement starts one 6000 ms
  clock; danger settlement remains sticky; copy churn never resets a clock.
- A same-id settle keeps the visual row and phase and produces one discrete
  announcement. Reorder, policy changes, and visual completion do not announce.
- Numeric progress updates cannot enter toast copy in the acceptance fixture.
- Focus survives an action label replacement and follows the exact fallback
  chain when the action disappears.
- Native danger uses `Alert`; non-danger stays `ListItem`. Evidence wording
  keeps GPUI AT unsupported and Jetstream deferred.
- Export and changed-file audits show no lifecycle field, promise helper,
  progress slot, second creation API, consumer migration, or release change.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Sticky owns no clock | pending sticky row | timer plan contains no start |
| Become-sticky clears | timed info becomes danger | old clock is cancelled before it can dismiss |
| Settlement gets one fresh clock | sticky pending becomes success | exactly one 6000 ms start |
| Same-id keeps row/phase | title and tone settle in place | no fresh enter; one announcement |
| Focus fallback is deterministic | focused action is removed | row dismiss → next → previous → entered-from |
| Native severity is honest | danger settle keeps `ListItem` | node assertion fails until role is `Alert` |
| Public surface stays API-zero | add promise, lifecycle field, or slot | export/surface audit fails |

Plant the pre-fix behavior after committing the real proof, then restore and
rerun green.

## Writable Scope

ToastHost/ToastStack contracts; core timer machinery; component-local Svelte,
React, shared Rust render, specimens, and focused tests; this card, one log,
and new papercuts. Do not edit consumer repositories, operation models,
Progress/Spinner APIs, motion policy, releases, workflows, GPUI accessibility,
visual ledger cells, or Jetstream behavior.

## Validation

Run focused timer, presence, motion-retarget, announcement, focus, Svelte,
React, Rust-node, and mounted GPUI tests; relevant drift selectors;
`effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`. Never run
`*-windowed` or native-visual selectors.

## Stop Conditions

Stop if correctness needs Toast-owned operation state, promise handling,
progress anatomy, retry/cancellation policy, a new native accessibility API,
consumer changes, or a second motion/expiry owner.

## Continuation

After accepted merge, the convention is complete in Poodle. Consumer upsert
adoption remains consumer-owned and needs separate repository authority.

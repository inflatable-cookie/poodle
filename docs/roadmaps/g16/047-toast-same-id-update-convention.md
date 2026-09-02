# g16.047 — Toast Same-Id Update Convention

Status: implemented — awaiting orchestrator review
Type: implementation
Opened: 2026-09-01
Depends on: merged `g16.034`, completed `g16.043` research, and operator
acceptance recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/toast-host.md`,
`../../contracts/components/toast-stack.md`, `../../specs/015-loading-empty-error-notification-and-remediation-rules.md`,
`../../contracts/003-native-accessibility.md`,
`../../architecture/012-semantic-motion-policy.md`
Log: `../../logs/2026-09/20260902-g16-047-toast-same-id-update-convention.md`

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
- Sticky rows have no clock. Sticky → non-sticky starts one fresh clock using
  the current configured `autoDismissMs` when positive. Non-sticky → sticky
  clears it. Copy/tone/action-only updates do not reset a running clock;
  removal clears it. A non-positive configured delay starts nothing.
- Stickiness always resolves from explicit `sticky: true` or membership in the
  current configured `stickyTones`. `6000` and `["danger"]` are default-
  configuration fixtures, not universal settlement law. Changing configuration
  preserves an existing non-sticky clock, clears it when the row becomes
  sticky, or starts the configured delay when a sticky row becomes non-sticky.
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

- Under default configuration, sticky pending schedules no timer, success
  settlement starts one 6000 ms clock, and danger settlement remains sticky.
  Custom `autoDismissMs` and `stickyTones` govern the same transitions without
  being overwritten; copy churn never resets a running clock.
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
| Default settlement gets one fresh clock | sticky pending becomes success under defaults | exactly one 6000 ms start |
| Custom delay is authoritative | sticky pending becomes success with `autoDismissMs=2500` | exactly one 2500 ms start; no 6000 ms timer |
| Custom sticky tones are authoritative | two sticky pending rows settle to danger and warning with `sticky` omitted under `stickyTones=["warning"]` | danger gets the configured clock; warning gets none |
| Disabled expiry stays disabled | sticky pending settles non-sticky with `autoDismissMs=0` | no timer starts |
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

## Evidence

- ToastHost/ToastStack contracts now own uniqueness, configured timer
  transitions, discrete announcement, action-focus fallback, spec 015
  remediation, and native `Alert` vs `ListItem` without a GPUI AT claim.
- `uniqueToastInputs` plus `reconcileToastTimers` prove the complete
  configured timer table, including custom `autoDismissMs`, custom
  `stickyTones`, disabled expiry, become-sticky clear, and copy churn.
- `nextToastVisuals` keeps phase on same-id replacement. Paired Svelte/React
  stacks keep the settled row, announce the settled copy, restore action
  focus, and keep percents out of toast copy.
- Shared Rust danger rows set `NodeRole::Alert`. Mounted GPUI
  `mounted_toast_danger_uses_alert_role` draws that tree without claiming
  assistive-technology parity.
- One pending-to-settled specimen exists on Svelte, React, and GPUI ToastHost
  and ToastStack construction paths.

## Falsification

Proofs were committed first. Restores used `git checkout --` on a clean index.

| Oracle row | Plant | Intended failure | Restore |
| --- | --- | --- | --- |
| Sticky owns no clock | existing sticky-pending case | start empty (already green pre-fix) | n/a |
| Become-sticky clears | `clear` only departed ids | `plan.clear` expected `["job"]`, received `[]` | green |
| Default 6000 ms start | existing settle case | start `["job"]`, delay 6000 | n/a |
| Custom delay 2500 | existing settle case | delay 2500, not 6000 | n/a |
| Custom sticky tones | existing two-row settle | start `["fail"]` only | n/a |
| Disabled expiry | existing `autoDismissMs=0` | start empty | n/a |
| Same-id keeps row/phase | `nextToastVisuals` always enters | phase expected settled, received enter | green |
| Focus fallback | skip `moveToastFocusFromRemovedAction` | activeElement is `body`, not dismiss | green |
| Native danger Alert | always `ListItem` | expected Alert, received ListItem | green |
| API-zero | source scan for promise/lifecycle/slot | already absent | n/a |

## Validation

Focused `packages/core/test/toast.test.ts`, `motion-runtime.test.ts`, paired
ToastHost/ToastStack/motion-family tests, `poodle-render`
`danger_uses_alert_and_other_tones`, and mounted
`mounted_toast_danger_uses_alert_role`. Required boards recorded in the
execution log.

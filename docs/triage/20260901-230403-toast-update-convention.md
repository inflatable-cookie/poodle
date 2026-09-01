# Toast Same-Id Update Convention Proposal

Status: delegate proposal awaiting operator acceptance or revision
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Scope: narrow host update convention for in-place toast settlement
Promotion authority: orchestrator after operator acceptance; merge is intake only

This packet translates the accepted consumer-owned Toast lifecycle
recommendation from
`docs/research/value-tracks/in-place-toast-lifecycle.md` (g16.043, PR #133)
into one narrow host update convention. It is not contract, roadmap,
implementation, or merge authority.

## Settled Decisions Preserved

- One stable `id` moves from sticky pending copy to settled copy by host
  update. No second toast creation API.
- Host/domain owns operation state and expiry; visuals never own timers.
- No `pending | settled` field, promise helper, or required Progress slot
  inside Toast.
- The operator authorized planning the convention, not a public lifecycle API.

## Delegate Authority

No operator conversation occurred in this delegate thread. The handoff named
the dossier recommendation accepted and instructed this lane to translate it
into the narrow convention, settling each question with evidence. Every exact
law below is a delegate recommendation; the operator accepts, revises, or
rejects the surface before canonical promotion. PR merge is intake for that
review, not acceptance of the convention.

## Proposed Convention

### 1. Identity And Uniqueness

- `ToastHostStoreItem.id` is the semantic owner of the operation row. At most
  one row per `id` in any snapshot.
- Same-id updates upsert in place: the host replaces copy, tone,
  `actionLabel`, and `sticky` on the retained row. The visual row is keyed by
  id; phase (enter/settled) is preserved — same-id updates never restart enter
  motion.
- Duplicate ids in one snapshot are undefined today (Svelte keyed `{#each}`
  and React `key={visual.id}` collapse them) and are a consumer defect under
  this convention.
- A dismissed id that left the store is a new record when supplied again.
  Reusing an id while the row is still live is an update; reusing it before
  the exit remnant clears retargets that remnant (ToastStack contract §8a);
  recreating it after exit cleanup is a fresh enter with no stale action.
- Pending is `sticky: true` plus the consumer's own operation state. Settled
  is the same id with new copy, tone, action, and sticky bit.
  `pending | settled` is never Poodle state.

Evidence: value track identity and retained-item audit [VF]; ToastHost
contract §3–§4; ToastStack contract §8a.

### 2. Clearing And Restarting The Timer

ToastHost keeps the clocks; `reconcileToastTimers` in
`packages/core/src/toast.ts` plans them. The convention timer law:

| Store transition for the same id | Timer law |
| --- | --- |
| present + non-sticky + `autoDismissMs > 0` | one clock from first non-sticky appearance |
| present + sticky | no clock |
| sticky → non-sticky (settlement) | starts one fresh default clock |
| non-sticky → sticky (e.g. info → danger) | clears the running clock — requires the small timer-plan repair |
| copy/tone/action-only update | clock never starts, stops, or resets |
| id leaves store | cleared |
| `autoDismissMs <= 0` | nothing starts |

- Pending rows are sticky, so a pending operation never auto-dismisses.
- Settlement to success/warning/info starts a fresh default clock (6000ms).
  Settlement to danger stays sticky by the default `stickyTones: ["danger"]`.
- The non-sticky → sticky hole is real today: a row that was not sticky keeps
  its clock and can auto-dismiss after becoming danger.
  `reconcileToastTimers` must clear on become-sticky before any non-sticky
  pending copy is used. Visuals never own expiry.

Evidence: value track timer audit and consequence table [VF] + [WI]; ToastHost
contract §4; architecture 012 (expiry stays with existing semantic owners).

### 3. Discrete Settlement Announcements Versus Numeric Progress

- New rows announce (polite; assertive when danger). Discrete settlement — a
  tone/title class change — announces once. Reorder, retarget, policy change,
  and visual completion do not reannounce (architecture 012).
- Numeric progress churn must never announce from the toast row. The item
  carries `aria-atomic="true"`, so any text change reannounces the whole row;
  percent must not live in toast copy.
- Percent belongs on `Progress` (`role="progressbar"`, computed `"N%"`
  `aria-valuetext`, parent-owned live region) or an inline status the host
  announces deliberately. A pending toast's copy may change with discrete
  status sentences only, never counters.
- No progress field, progress slot, or promise helper is added to Toast.
  Spinner's optional `role="status"` stays a separate, label-gated surface.

Evidence: ToastStack contract §6 (per-item `aria-atomic="true"`, polite /
assertive); Progress contract §3/§6 (percent, `aria-valuetext`, parent-owned
live region); Spinner contract §6 (status role optional and label-gated);
architecture 012 announcement law; value track announcement overlay [VF] +
[WI].

### 4. Action-Focus Replacement And Removal

- Replacement (label or tone change on a still-present action control): the
  control keeps focus.
- Removal (settlement drops the action): if the removed button owned focus,
  move focus to that row's dismiss button; if that is gone too, use the
  existing stack fallback — the next surviving non-exit row's equivalent
  control, then the previous row, then the still-connected element from which
  focus entered the stack (ToastStack §8a).
- `onAction(id)` stays the only action surface. The consumer interprets the
  id.

Evidence: value track focus audit and scenario row [WI]; ToastStack contract
§8a focus law; g16.034 log.

### 5. Durable Danger Remediation

- Settled danger is sticky by policy and may confirm the failure in place, but
  a toast is never the only record of an unresolved persistent problem.
- Spec 015: transient notifications must not become the only place a user can
  discover an unresolved persistent problem; persistent errors should prefer
  inline banner-style messaging over transient notifications alone.
- Hosts keep the failure reachable in a persistent surface — dialog, banner,
  list, or notification ledger. Underlay keeps retry on the AlertDialog;
  Nucleus/Longhorn keep the ledger and controller state. Spec 015's rule
  remains authority; this convention does not weaken it.

Evidence: value track scenario row (error persistence); spec 015 Notification
and Error And Remediation rules; consumer audit [VF].

### 6. Native Alert Semantics

- Web: item `aria-live` is polite except danger; a danger settlement escalates
  the row to assertive.
- Shared Rust: `packages/render/src/toast_stack.rs` currently sets
  `NodeRole::List` / `NodeRole::ListItem` only, while `poodle-node` already
  exposes `NodeRole::Alert` and `Status`. Native danger rows — new or settled —
  set `NodeRole::Alert`; other tones stay `ListItem`. That role is the native
  equivalent of assertive escalation.
- GPUI 0.2.2 has no accessibility API (contract 003). Metadata still flows
  through the shared renderer; the omission stays visible and deliberate. No
  fake live region, no assistive-technology parity claim from node metadata.
- Jetstream remains deferred; when admitted it binds AccessKit and must
  project the same roles. Spec 019 allows web live region versus
  platform-native announcement as a framework delta; it never allows omitting
  announcements in one runtime.

Evidence: `toast_stack.rs` role audit [VF]; `poodle-node` role surface (value
track [VF]); contract 003 GPUI/Jetstream state; spec 019 Framework Delta and
Notification rules; value track cross-runtime map.

### 7. Consumer Obligations

- Consumers doing in-place settlement must upsert by id. Underlay's
  `createToastStore.push` always appends today — that is consumer work, not
  Poodle.
- Dismissal must actually drop the id when the operation should vanish; a
  dismiss request never inerts a still-supplied row (g16.034).
- Cancellation is consumer-owned: remove the id or settle the copy; ToastHost
  never cancels work. Longhorn/Nucleus should not pretend operation progress
  is a toast phase; their ledger model stays.

Evidence: value track consumer audit [VF]; g16.034 behavior floor.

## Recommendations

- Promote all five deltas together in one bounded contract card after operator
  acceptance: snapshot uniqueness, timer clear-on-sticky, announcement split,
  action-focus fallback, and the native danger alert role. Partial promotion
  would leave runtimes choosing announcement and timer behavior.
- Keep the convention API-zero: no new fields, callbacks, promises, or slots.
  The laws are documented behavior of the existing surface plus two small
  machinery repairs (`reconcileToastTimers` clear-on-sticky; native
  `NodeRole::Alert` on danger).
- Sequence the card: contract amendments (toast-host, toast-stack) first, then
  the two machinery repairs, then paired mounted tests against the oracle
  matrix below. Underlay upsert stays outside Poodle.
- Keep Progress and Spinner contracts as the progress authority. No Toast
  anatomy change.

## Alternatives Not Selected

| Alternative | Reason |
| --- | --- |
| `pending \| settled` field or Sonner `loading` type | Duplicates consumer operation ownership; forces a field native hosts do not need in the node tree. Evidence did not support it as Poodle state. |
| Promise helper (`toast.promise`) | Hides operation ownership inside a toast helper; React-only; no shared Rust/GPUI shape; license review needed before any derivative. |
| Required Progress/Spinner slot in Toast | Progress and Spinner already own status semantics; a required slot changes focus, tokens, and native layout for a minority UX. |
| Second toast creation API | One store, one id is the entire point of the convention. |
| Hover-pause, swipe-dismiss, or expanded pending timing | Sonner product choices; Poodle timers stay host-owned, simple, and deterministic. |
| Percent in toast copy with live-region suppression | Suppressing the atomic region would hide discrete settlement too; percent belongs on Progress. |
| Always-assertive toasts | Spec 019: danger may announce assertively when it materially affects the workflow; other tones stay polite. |
| Fake GPUI live region | No API to build against in gpui 0.2.2; contract 003 forbids scheduling GPUI accessibility work. |

## Explicit Non-Goals

- Public lifecycle API: `pending \| settled` field, promise helper, progress
  slot, second creation API.
- Toast-owned expiry, timers, retry, cancellation, or progress values.
- Changing the store shape, tone vocabulary, sticky defaults, or the
  `autoDismissMs` default.
- Migrating Underlay, Longhorn, Nucleus, or Bovine consumers in this repo.
- GPUI accessibility implementation or assistive-technology parity claims.
- Jetstream admission or Jetstream implementation.
- Motion policy changes; architecture 012 stays authority.

## Required Review Oracles

| Invariant | Smallest adversarial counterexample | Expected failure or stop | Required proof |
| --- | --- | --- | --- |
| Snapshot uniqueness | One store snapshot carries two rows with the same id | Undefined today; convention requires one row per id | Contract wording plus Svelte/React keyed render audit |
| Same-id update keeps row and phase | Settle changes copy and tone and enter motion restarts | Motion trace shows a fresh enter | Presence trace with retained-item assertion in Svelte and React |
| Pending is sticky | A `sticky: true` pending row schedules a timer | Sticky ids plan no clock | `reconcileToastTimers` unit test |
| Clear on become-sticky | Non-sticky info row becomes danger; its clock survives | Danger row auto-dismisses at 6000ms | Timer unit test after the repair; revert to today's plan fails |
| Fresh clock after settle | Sticky pending → non-sticky success | Exactly one fresh default clock starts | Timer unit test asserting one start at 6000ms |
| Copy churn never resets the clock | Two same-id copy updates mid-clock | Clock restarts | Timer trace across an update sequence |
| Removal clears and exits | Id leaves the store; timer still fires | Row auto-dismisses after removal | Existing timer and presence tests stay green |
| Dismiss while pending keeps the row live | Dismiss targets a still-supplied pending row | Row inerts or loses controls | Paired Svelte/React mounted no-removal test (g16.034 row, kept) |
| Discrete settle announces once | One same-id settle changes tone/title class | Reannounce on reorder, retarget, policy change, or visual completion | Mounted live-region receipt |
| Progress churn never announces | Percent counter updates inside `message` | Every tick announces through `aria-atomic` | Live-region test; percent on Progress is not asserted from the toast row |
| Danger settle escalates on web | Tone flips to danger and `aria-live` stays polite | Web assertion catches polite-only | DOM assertion on the item `aria-live` after settle |
| Native danger alert role | Settle flips tone and the node role stays `ListItem` | Native role assertion fails | `poodle-node` role assertion for danger vs other tones |
| Action replacement keeps focus | Label-only action swap | Focus leaves the button | Mounted focus trace on a label-only update |
| Action removal moves focus to dismiss then stack | Settled row drops the focused action | Focus falls to body | Mounted trace: row dismiss → next → previous → enteredFrom |
| Reused id after removal is a new record | Id recreated after exit cleanup | Stale action/copy leaks or no fresh enter | Paired presence test; retarget only for still-exiting visuals |
| Durable remediation exists elsewhere | Host keeps the persistent failure only in the toast | Spec 015 review gate stops promotion | Contract/card review plus consumer evidence (dialog, banner, or ledger) |
| Zero new API | A change adds pending/settled, a promise, a slot, or a second creation API | Public surface audit fails | Changed-file audit plus export audit |
| Native AT is not overclaimed | GPUI evidence claims live-region parity | Wording check against contract 003 fails | Evidence wording review and ledger check |

## Proposed Canonical Destinations

| Meaning | Destination after packet acceptance |
| --- | --- |
| Uniqueness, pending = sticky definition, and the full timer law | `docs/contracts/components/toast-host.md` (§3–§4, §9) plus `packages/core/src/toast.ts` `reconcileToastTimers` |
| Announcement split, action-focus fallback, settle keeps phase | `docs/contracts/components/toast-stack.md` (§6, §8a) |
| Native danger alert role | `docs/contracts/components/toast-stack.md` §10 plus `packages/render/src/toast_stack.rs` |
| Specimens: upsert pending → settled; same-id settle | `docs/contracts/components/toast-host.md` §12 and `docs/contracts/components/toast-stack.md` §12 |
| Severity, announcement, and remediation authority | Reference `docs/specs/015`, `docs/specs/019`, `docs/architecture/012`; unchanged |
| Native accessibility boundary | Reference `docs/contracts/003-native-accessibility.md`; no GPUI work scheduled |
| Implementation sequencing | One bounded card after operator acceptance and canonical promotion; the orchestrator picks the card number and order |

The orchestrator chooses the final promotion split. No implementation card is
ready from this packet alone.

## Unresolved Questions

Operator acceptance or revision of the exact convention remains open: the
uniqueness law, timer repair, announcement split, action-focus fallback,
native alert role, oracle matrix, and promotion split. Merge is intake for
these decisions, not acceptance. Promotion may also expose integration drift
against newer `main`; the orchestrator owns that reconciliation after the
operator gate.

## Evidence Used

- `docs/research/value-tracks/in-place-toast-lifecycle.md` (g16.043, PR #133)
- `docs/roadmaps/g16/043-in-place-toast-lifecycle-research.md`
- `docs/contracts/components/toast-host.md`
- `docs/contracts/components/toast-stack.md`
- `docs/contracts/components/progress.md`
- `docs/contracts/components/spinner.md`
- `docs/contracts/003-native-accessibility.md`
- `docs/specs/015-loading-empty-error-notification-and-remediation-rules.md`
- `docs/specs/019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`
- `docs/architecture/012-semantic-motion-policy.md`
- `docs/logs/2026-09/20260901-g16-034-shared-motion-policy.md`
- `packages/core/src/toast.ts` and `packages/render/src/toast_stack.rs`
  (machinery and role facts as described by the dossier)

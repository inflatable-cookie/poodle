# Value Track: In-place toast lifecycle

Status: research complete; promotion-ready for an operator architecture decision
Created: 2026-09-01
Checked: 2026-09-01
Track: in-place pending-to-settled toast lifecycle
Origin: `docs/roadmaps/g16/043-in-place-toast-lifecycle-research.md`
Primary lead: [Sonner promise toast](https://sonner.emilkowal.ski/toast#promise)
Pinned Sonner source: `emilkowalski/sonner` commit
[`ecce1841c55e4a72dfe139a8992b56498660125e`](https://github.com/emilkowalski/sonner/commit/ecce1841c55e4a72dfe139a8992b56498660125e)
(package `2.0.7`; npm current `2.0.8` as of 2026-08-09)

This is point-in-time research, not a component contract and not permission to
change ToastHost or ToastStack. `pending | settled`, sticky pending state, and
same-id resolution remain hypotheses until promotion.

## Evidence labels

- **[VF] Verified fact** — read from the pinned Sonner source or docs, a Poodle
  contract/spec/architecture file, or inspected implementation in this
  worktree or a named consumer checkout.
- **[WI] Worker inference** — a conclusion from those facts. It is not a
  current Poodle guarantee.
- **[SAC] Source-author claim** — a Sonner docs or issue comment about intended
  UX, not independently re-run here.

## Executive summary

Poodle already has a stable toast identity (`id`) and an externally driven
item list. Same-id field updates already keep the visual row; they do not
restart enter motion. A dismiss request does not make a still-supplied row
inert. Visual lifecycle never owns expiry. Those g16.034 facts stay.

What Poodle does not have is an operation lifecycle. ToastHost maps store
rows and runs auto-dismiss timers. ToastStack renders, announces, focuses,
and animates presence. Neither owns a promise, retry, cancellation, or
progress value.

Sonner's `toast.promise` is an imperative React helper: one id, `type:
"loading"` until settle, merge-on-same-id, no auto-close while loading, then
a success/error create on that id. It is MIT-licensed. It is not a Poodle
contract shape. Importing it would hide operation ownership inside a
toast helper and would not exist on shared Rust or GPUI.

Two real async consumers already own the operation and emit **terminal**
toasts:

1. Underlay admin templates await the job, then `push` a new success or
   error row. `createToastStore.push` always appends, even when an `id` is
   supplied.
2. Nucleus, through Longhorn's notification session, toasts unseen
   warning/error/critical records under a fresh `notification-toast:${n}`
   id. Pending command state lives on the controller, not on the toast.

Recommendation: **consumer-owned** operation state, with a **host update
convention** over one stable `id`. Do not add a public `pending | settled`
field, a promise helper, or a required Progress/Spinner slot inside Toast.
Pending is the existing `sticky: true` row. Settlement is the same id with
new copy, tone, action, and sticky bit. Progress percent belongs on
Progress or inline status, not in a live region.

A later contract card may need a small timer-plan repair (clear a running
clock when an id becomes sticky) and an announcement rule that discrete
settlement is announced while progress churn is not. Those are promotion
gates, not this dossier's implementation.

---

## Method and source inventory

### Method

Read-only inspection of this worktree after worker preflight on
`research/g16-043-in-place-toast-lifecycle` at `origin/main`
`ff0909805a94b6ccf4d7c7781b241a00e104aa9e`. No ToastHost/ToastStack,
architecture, or consumer source was edited.

Sonner was pinned from the documented lead plus GitHub source at
`ecce1841`, LICENSE.md, and the public toast page. Browser automation of
the Sonner demo was not required: lifecycle, identity, and timing are in
`src/state.ts` and `src/index.tsx`. HTML attributes in `index.tsx` were
read from the raw file; Markdown fetches strip them.

Consumer evidence is from primary checkouts, not sibling worktree links:

- Underlay: `/Users/tom/Dev/projects/underlay`
- Longhorn: `/Users/tom/Dev/projects/longhorn`
- Nucleus: `/Users/tom/Dev/projects/nucleus`

### External sources

| Source | Evidence used | Authority / limitation |
| --- | --- | --- |
| [Sonner toast#promise](https://sonner.emilkowal.ski/toast#promise) | Documented loading → resolve/fail update; success/error may be functions; loading helper exists beside promise | Primary product docs. Interactive demo not driven. |
| [`src/state.ts`](https://raw.githubusercontent.com/emilkowalski/sonner/ecce1841c55e4a72dfe139a8992b56498660125e/src/state.ts) at `ecce1841` | id allocation, same-id merge, dismissed-id recreation, `promise()` loading/success/error/finally, HTTP `Response.ok` branch | Primary implementation. |
| [`src/index.tsx`](https://raw.githubusercontent.com/emilkowalski/sonner/ecce1841c55e4a72dfe139a8992b56498660125e/src/index.tsx) at `ecce1841` | 4000ms default lifetime, loading skips timer, loading disables swipe, close button hidden while loading, container `aria-live="polite"` | Primary implementation. |
| [`src/types.ts`](https://raw.githubusercontent.com/emilkowalski/sonner/ecce1841c55e4a72dfe139a8992b56498660125e/src/types.ts) | `ToastTypes` includes `loading`; `PromiseData` shape | Primary types. |
| [`LICENSE.md`](https://raw.githubusercontent.com/emilkowalski/sonner/main/LICENSE.md) | MIT, Copyright (c) 2023 Emil Kowalski | Primary licence. Legal review still appropriate before any derivative helper. |
| [npm sonner](https://www.npmjs.com/package/sonner) | MIT, homepage, 2.0.8 published 2026-08-09 | Registry metadata. Pinned git package.json at `ecce1841` is `2.0.7`. |
| [WAI-ARIA 1.2 `aria-live` / `aria-relevant` / `aria-atomic`](https://www.w3.org/TR/wai-aria/#aria-live) | Live-region update semantics | Normative a11y model. AT behavior still varies. |

No Sonner source was copied into this repository.

### Local authority

- `docs/architecture/012-semantic-motion-policy.md`
- `docs/contracts/components/toast-host.md`
- `docs/contracts/components/toast-stack.md`
- `docs/contracts/components/progress.md`, `spinner.md`, `remediation-banner.md`
- `docs/specs/015-loading-empty-error-notification-and-remediation-rules.md`
- `docs/specs/019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`
- `docs/logs/2026-09/20260901-g16-034-shared-motion-policy.md` (PR #124)

---

## Questions (card)

| Question | Answer from evidence |
| --- | --- |
| Lifecycle as ToastHost field, host update convention, or progress composition? | Host update convention. Operation stays consumer-owned. [WI] |
| Which pending/success/error/cancel/retry/progress/expiry meanings are reusable? | Reusable: one id, sticky while unfinished, discrete settle copy/tone, existing expiry/sticky rules. Not reusable as Poodle fields: promise ownership, retry, cancellation receipts, percent, HTTP mapping. [WI] |
| When copy or tone changes on one id, what is announced? | Today: item `aria-live` plus `aria-atomic="true"` will reannounce the whole row on text change, including progress churn. Motion policy only suppresses reorder/retarget/policy/visual-completion reannounce. Settlement should announce; progress churn must not. That split is a promotion gate. [VF] + [WI] |
| Action focus, dismissal, replacement, removal, auto-dismiss across settlement? | Dismiss of a still-supplied row stays live (g16.034). Timers key by id and do not restart on field changes. Sticky → non-sticky starts a timer; non-sticky → sticky does **not** clear a running timer. Action is a label + `onAction(id)` only. [VF] |
| Reduced/frozen motion vs semantic lifecycle? | Presence motion only. Policy never owns expiry, ARIA, or timers. Copy/tone updates must not restart enter. [VF] |

---

## Sonner pinned evidence

### Licensing and shape

MIT. Author Emil Kowalski. React-only Toaster + imperative `toast()`. Default
duration 4000ms, default `containerAriaLabel` `"Notifications"`. [VF]

### Identity

`getToastId` keeps an explicit `id`, else a process counter. `create` with an
existing active id **merges** props onto that toast and republishes. A
dismissed id is dropped from history before recreate so old `action` / type
do not leak (`#692`, `#777`). A pending `requestAnimationFrame` dismiss for
that id is cancelled if create races it (`#592`). [VF]

`toast.promise` captures that id from the loading `create`, then calls
`create({ id, type: "success" | "error" | "default", ... })` on settle. One
visual record. [VF]

### Lifecycle

```text
optional loading create (type=loading)
        |
   Promise.resolve(promise | promise())
        |
   +-- HTTP Response !ok or Error or catch --> error create on same id
   +-- success data defined ------------------> success create on same id
   +-- React element response ----------------> default create on same id
   +-- none of the above ---------------------> dismiss loading in finally
```

Success/error/description may be functions, including async. `data.finally`
runs after settle or dismiss. Returning `{ unwrap }` preserves the original
settlement as a Promise. [VF]

`toast.loading` is the manual equivalent: caller must later `create`/`toast()`
with the same id. `toast()` with a loading id clears loading type (`#401`).
`toast.custom` must reset type or the spinner and infinite lifetime leak
(`#652`). [VF]

### Timing and interaction while pending

In `Toast` (`index.tsx`):

- Skip the close timer when `(toast.promise && type === "loading")`,
  `duration === Infinity`, or `type === "loading"`.
- `setTimeout(..., Infinity)` is treated as 0; remaining Infinity returns
  without scheduling.
- `disabled = type === "loading"` blocks pointer swipe-dismiss.
- Close button is not rendered while `type === "loading"`.
- Hover / expand / hidden document pause a running timer; remaining time is
  kept.
- Duration changes rewrite `remainingTime.current`.
- Exit unmount waits `TIME_BEFORE_UNMOUNT` (200ms) after visual remove.

[VF]

### Announcement

The Toaster list is `aria-live="polite"`, `aria-relevant="additions text"`,
`aria-atomic="false"`. There is no per-toast assertive escalation in this
pinned source. Title text updates are therefore in-scope for polite
announcement (`text`). [VF]

Sonner does not expose a first-class progress field. Percent UX in issues
(`#201`, `#529`) is same-id `toast.info` updates; authors had to pass
`duration: Infinity` on loading or the default 4000ms closed the row. [SAC]

### What not to import

- Imperative module-level `toast()` / `toast.promise`.
- Promise ownership inside the toast library.
- `type: "loading"` as a Poodle tone. Poodle tones are
  `info | success | warning | danger`.
- HTTP `Response.ok` mapping.
- Swipe-to-dismiss, hover-pause, and JSX actions.
- React-only `unwrap`.

Those are Sonner product choices. Poodle's store is already the host API. [WI]

---

## Current Poodle audit

### Ownership split

| Concern | Owner today | Evidence |
| --- | --- | --- |
| Item list, ids, copy, tone, action label, sticky | Consumer store | ToastHost contract §3, §4 |
| Tone resolve, title fallback, timer plan | `poodle-core` `toast.ts` | `resolveToastTone`, `normalizeToast`, `reconcileToastTimers` |
| Real timers, store subscribe, `dismiss(id)` | ToastHost web adapters | `ToastHost.svelte`, `ToastHost.tsx` |
| Presence visuals, live region, focus, inert remnant | ToastStack | contract §6, §8a; `nextToastVisuals` |
| Native positioned box | `poodle-render` `toast_host` / `toast_stack` | timers still host-owned, contract §10 |
| Motion policy | architecture 012 | Toast expiry stays semantic |

ToastStack is styled-only. ToastHost is machine-backed for timers only. There
is no pending phase in either machine. [VF]

### Identity and same-id updates

`ToastItem.id` / `ToastHostStoreItem.id` is the only identity. ToastStack
keys rows by `visual.id`. `nextToastVisuals` keeps the prior phase when the
id is still live and not exiting; a new or previously-exiting id enters.
Reusing an id before exit cleanup retargets that remnant (contract §8a). [VF]

Retained item maps replace the object when the host supplies a new item for
the same id. Copy, tone, and `actionLabel` therefore update in place without
enter motion. [VF] from `ToastStack.svelte` retained-item effect and
`ToastStack.tsx` `setRetained`.

Duplicate ids in one snapshot are undefined. Svelte keyed `{#each}` and React
`key={visual.id}` collapse them. [WI]

### Timers

`reconcileToastTimers`:

- clear timers whose ids left the store
- start timers for **new** non-sticky ids when `autoDismissMs > 0`
- preserve a running timer across unrelated store changes

Default `autoDismissMs` is 6000. Default `stickyTones` is `["danger"]`.
Explicit `sticky: true` also sticks. [VF]

Consequences for in-place settlement:

| Store transition for the same id | Timer today |
| --- | --- |
| sticky pending → non-sticky success | starts (id was not running) |
| sticky pending → danger (sticky by tone) | none |
| non-sticky → sticky (e.g. info then danger) | **keeps running** — will auto-dismiss a sticky row |
| copy/progress change while non-sticky | clock continues; no reset |
| id removed | cleared |

The non-sticky → sticky hole is real. A pending row that was not sticky can
still expire after it becomes danger. [VF] + [WI]

Native `ToastHostSpec` carries `auto_dismiss_ms` and `sticky_tones` as policy
fields. `toast_host.rs` does not schedule timeouts. Contract §10: store
subscription and timer scheduling remain host-owned. [VF]

### Focus and dismiss ownership (PR #124)

A dismiss callback does not inert a row that is still in `items`. Paired
Svelte/React tests keep `data-motion="settled"`, live announcement, and
operable controls; focus still moves to the next toast's equivalent control.
Inert remnant exists only after the id leaves `items`. Log oracle row 17.
[VF]

`moveToastFocus`: if the dismissed row owns focus, next surviving non-exit
row, else previous, else `enteredFrom` if still connected. [VF]

Action replacement is not specified. The action is one optional `Button` with
`actionLabel`. `onAction(id)` fires; the consumer interprets. If settlement
removes the action while it is focused, focus restoration is whatever the
DOM does when that button unmounts — not the dismiss helper. [WI]

### Announcement

Stack: `<ul aria-live="polite" aria-atomic="false">`. Item: `aria-atomic="true"`
and `aria-live="polite"` except danger `assertive`. Exit remnant drops
`aria-live`, sets `aria-hidden`, `inert`, and `tabindex="-1"`. [VF]

Architecture 012: enter live region immediately; do not reannounce on
reorder, retarget, policy change, or visual completion; danger assertive is
mode-independent; motion never delays announcement. Spec 015: transient
toasts must not be the only place an unresolved persistent problem lives.
Spec 019: toast severity must be exposed as text; danger may be assertive;
actions and dismiss stay independently reachable. [VF]

Same-id title/message edits are not in the "do not reannounce" list. With
`aria-atomic="true"` they announce the whole toast. Progress-in-the-message
would speak every percent. [WI]

GPUI: ToastStack contract §6 requires preserving transient meaning and
dismiss/action reachability where there is no web live region. Spec 019
allows web live region vs platform announcement as a framework delta.
`poodle-node` has `NodeRole::Status` and `Alert`, but `toast_stack.rs` sets
`List` / `ListItem` only. Settlement announcement on GPUI is therefore
unmapped. [VF]

### Motion (g16.034)

Web: keyed opacity+translateY enter/exit; reduced drops translation; frozen
paints the endpoint and schedules no clock. Preloaded items are `settled` on
first paint (SSR receipt). Visual completion does not call `onDismiss`.
Owner key is `stackId:item.id`, so two stacks may share an item id. [VF]

Native render: authored items paint the settled endpoint; construction does
not attach enter (`toast_stack.rs` test `preloaded_items_do_not_enter`).
GPUI has no `toast-enter` / `toast-exit` plan in this tree. Presence motion
on native add/remove is an existing capability gap, not a lifecycle field.
[VF]

Copy/tone updates must not become a new enter. Current `nextToastVisuals`
already preserves phase. Reduced/frozen must not change sticky, timers, or
ARIA. Architecture 012 mode laws already say that. [VF]

---

## Consumer audit

### 1. Underlay admin — await, then a new toast

`createToastStore` (`underlay/ts/src/patterns/toasts.ts`): `push` always
appends. Optional `id` is stored but never used as upsert. `dismiss` filters
by id. Variants are `info | success | error` (maps to Poodle danger via
ToastHost `variant="error"`). No `sticky` field. [VF]

`SystemJobDetailPage.runAction` and `UserSessionsList.handleRevokeConfirm`:
await the mutation, then `toastStore.push({ variant: "success" | "error",
message })`. No pending toast during the await. Retry for revoke stays on
the AlertDialog (`throw error` keeps it open). Clipboard helper pushes
success or error after `navigator.clipboard.writeText`. [VF]

Songsprout, contact-patch, and underlay-reference mount
`createToastStore()` into `ToastHost`. Same store, same append. [VF]

This is a real async consumer of ToastHost. It does **not** do in-place
settlement. Same-id resolution cannot work until the store upserts. [WI]

### 2. Nucleus / Longhorn notifications — terminal records, new toast ids

Nucleus `createNotificationToastStore` projects
`session.toasts` into `ToastHostStoreItem` (`id`, `title`, `message`,
`tone`, `actionLabel`) and `dismiss` → `session.dismissToast`. Actions call
`session.invokeAction`. `stickyTones={["danger"]}` at the App mount. [VF]

`shouldToastNotification` admits warning, error, and critical only — not
in-flight operations. [VF]

Longhorn `NotificationController.install`: a toast is created for an unseen
record that is new since the previous ledger revision, passes the selector,
and does not already have a toast for that `notificationId`. Toast `id` is
`notification-toast:${++toastSequence}` — **not** the notification id, and
**not** reused when the record later changes. Toasts whose notification left
the snapshot are dropped (host removal). `pending` on the controller is
markSeen/dismiss/action in flight, not a toast phase. [VF]

Longhorn operations have their own progress/cancellation authority. They are
not this toast row. [VF]

This is a second real async consumer. It also emits terminal rows with fresh
ids. In-place pending-to-settled would fight the ledger model unless the
selector and projector were redesigned. [WI]

### Pattern that is not pending-to-settled

Bovine session recovery replaces the store with one warning toast whose id
is `session:recovery:${n}`. Identity for duplicate suppression is message
text, not the toast id. That is replace-the-stack, not same-id settle. [VF]

---

## Scenario coverage

Assumed convention for the sketch: consumer keeps one id; pending uses
`sticky: true`; settle writes new copy/tone/action/sticky; Progress stays
outside the toast.

| Scenario | Today | Recommended convention |
| --- | --- | --- |
| Progress churn (percent in `message`) | Same id updates copy; live region reannounces every change; timer unchanged | Do not put percent in the toast message. Use Progress/status. If copy must change, it is a discrete sentence, not a counter. |
| Duplicate completion (second push) | Underlay appends a second row; Longhorn skips a second toast per notificationId but still uses a new toast id if first was dismissed | Consumer must upsert by id. Two rows with one id is undefined. |
| Cancellation | Removing the id exits. Copy `"Cancelled"` is just another update. No promise to abort | Consumer cancels the operation, then either removes the id or settles copy. ToastHost does not cancel work. |
| Host removal (id leaves store) | Timer cleared; visual exit remnant; live/focus ownership dropped immediately | Keep. |
| Action replacement | Label swap in place. Focus on a removed action is unspecified | If the action control remains, keep it focused. If it goes away, move to that row's dismiss, else existing stack fallback. |
| Focused toast during settle | Row stays; phase stays; focus not stolen by motion | Keep. Do not steal focus on copy/tone change. |
| Error persistence | Danger is sticky by default; spec 015 still wants a durable inline surface for unresolved problems | Settle to danger + sticky. Do not make the toast the only record of the failure. |
| Unmount | ToastHost clears timers; ToastStack cancels presence clocks | Keep. |
| Dismiss while pending, id still supplied | Row stays live and operable (g16.034) | Keep. Consumer `dismiss` must actually drop the id if the operation should vanish. |
| Reduced / frozen | Presence only | Semantic sticky/timer/ARIA unchanged. |

---

## State-machine sketch

Not a public Poodle machine. Consumer store + existing ToastHost timer plan
+ ToastStack presence.

```text
                    host supplies id
                         |
                         v
                   +-----------+
         sticky    |  present  |   id leaves store
      (pending)    |  (row)    |------------------> exit remnant --> absent
                   +-----------+                         ^
                     |     ^                             |
         same-id     |     | same-id                     |
         field       |     | field                       |
         update      +-----+                             |
                         |                               |
                         | consumer dismiss that         |
                         | actually removes id           |
                         +-------------------------------+

Presence (ToastStack, visual only):
  initial/preloaded -> settled
  new id            -> enter -> settled
  id gone           -> exit (inert, not in live region) -> drop
  id reused on exit -> retarget enter
```

Recommended **semantic** flags on the store item, not new fields:

```text
pending  = sticky === true  && operation still running   (consumer fact)
settled  = sticky reflects tone policy (danger sticks; success does not)
```

Illegal for Poodle to invent: `pending | settled` enum, promise, retry
callback, progress value, "loading" tone.

Timer overlay on that sketch:

```text
present + non-sticky + autoDismissMs > 0 -> one clock from first non-sticky appearance
present + sticky                         -> no clock
sticky -> non-sticky                     -> start clock   (works today)
non-sticky -> sticky                     -> must clear    (does not work today)
id gone                                  -> clear         (works today)
```

Announcement overlay:

```text
new id                 -> announce (polite, or assertive if danger)
discrete settle        -> announce (tone/title class change)
progress-only text     -> do not announce
reorder / motion / policy -> do not announce (already)
```

---

## Cross-runtime map

One result: **same id, consumer-owned operation, host-updated fields,
ToastStack remains presentational.**

| Runtime | How it lands | Gap |
| --- | --- | --- |
| Svelte | Store upsert; ToastHost already maps fields; ToastStack keeps the keyed row | Announcement policy for settle vs churn; action-focus when the button disappears |
| React | Same core `normalizeToast` / `reconcileToastTimers` / `nextToastVisuals` | Same |
| Shared Rust | `Toast` / `ToastHostSpec` already have id, tone, action, sticky-tone policy. No pending field to add. Timers stay host-owned | `NodeRole` for settlement announcement (`Status` / `Alert`) is unused on toast nodes |
| GPUI | Render the updated `Toast` list. Dismiss/action stay reachable. Frozen/reduced already skip inventing clocks | No toast presence animation; no live-region equivalent wired. Spec 019 allows platform announcement, but nothing currently fires it on copy change |

Jetstream stays deferred.

---

## Recommendation

**Consumer-owned**, plus a documented **host update convention**.

| Option | Verdict |
| --- | --- |
| **Extend** ToastHost with `pending \| settled` (or Sonner `loading` type) | Reject as public contract. Duplicates Underlay/Longhorn/Nucleus operation ownership. Forces a field GPUI and native hosts do not need in the node tree. `pending \| settled` was a hypothesis; evidence did not support it as Poodle state. |
| **Compose** Progress/Spinner inside Toast | Reject as required anatomy. Progress already owns determinate/indeterminate status. Spinner is decorative or optionally announced. Toast copy is title+message. A required slot would change focus, tokens, and native layout for a minority UX. A later specimen may *show* a Spinner beside pending copy without making it contract anatomy. |
| **Consumer-owned** same-id updates | **Accept.** Sticky pending, settle by mutating the row, keep `onAction(id)`, keep host timers and ToastStack presence. Underlay must upsert; Longhorn/Nucleus should not pretend operation progress is a toast phase. |
| **Reject** in-place lifecycle entirely | Too strong. Same-id in-place updates already exist and match the product need (one visible record). The missing work is convention + announcement/timer gates, not a new component. |

Reject importing Sonner's promise helper. Reject a second toast creation
API. Reject visual lifecycle owning expiry.

### Smallest later contract deltas (not done here)

1. Snapshot uniqueness: at most one row per `id`.
2. Timer plan: when an existing id becomes sticky, **clear** its clock.
   Document sticky-while-pending so success settlement starts a fresh default
   clock.
3. Announcement: discrete identity/tone/title settlement announces; numeric
   progress and motion do not. Likely means pending progress copy must not
   live in the atomic live region, or progress must not be toast copy.
4. Action focus: replacement keeps focus; removal uses dismiss-then-stack
   fallback.
5. Native: map danger settlement to `NodeRole::Alert` or platform
   announcement if web assertive is required for parity. Do not fake a live
   region in GPUI.

Until those are accepted, implementations must not add fields, callbacks,
promises, or a public state machine.

---

## Unresolved promotion gates

Operator (and then a contract card) must accept:

1. **Identity** — `id` is the semantic owner. Duplicate ids are a consumer
   defect. Reusing an id after dismiss is a new record (Sonner now does this;
   Poodle already drops visuals when the id leaves).
2. **Announcement** — settlement yes, progress churn no. Current markup does
   not implement that split. Native announcement is unset.
3. **Focus** — dismiss-while-focused already specified. Action disappearance
   on settle is not.
4. **Timer ownership** — ToastHost keeps clocks. Pending must be sticky.
   Clear-on-become-sticky is required before non-sticky pending is safe.
5. **Remediation** — a settled danger toast is not enough for a persistent
   failure (spec 015). Underlay/Nucleus already keep dialogs, lists, or the
   notification ledger.

g16.034 behavior remains the floor: items externally controlled; dismiss
request does not inert a still-supplied row; visual lifecycle never owns
expiry.

---

## Related

- Card: `../../roadmaps/g16/043-in-place-toast-lifecycle-research.md`
- Motion baseline: `../../architecture/012-semantic-motion-policy.md`,
  `../../logs/2026-09/20260901-g16-034-shared-motion-policy.md`
- Notification rules: `../../specs/015-loading-empty-error-notification-and-remediation-rules.md`
- Overlay motion catalogue (queue/timer questions only):
  `./transitions-dev-catalogue.md`

## Follow-up

Orchestrator: accept or reject the consumer-owned convention and the five
gates. If accepted, a later contract card amends ToastHost/ToastStack (timer
clear-on-sticky, announcement, uniqueness, action focus) without adding a
promise API. Underlay upsert is consumer work, not Poodle. Do not schedule
implementation on this research PR.

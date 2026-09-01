# Transcript-inline agent approval

Status: research complete
Disposition: reject a new shared transcript-inline approval semantic; compose the existing question and plan paths; keep novel workflows consumer-owned
Research date: 2026-09-01
Card: [g16.037 — Transcript-Inline Agent Approval Research](../../roadmaps/g16/037-transcript-inline-agent-approval-research.md)
Baseline: Poodle HEAD 06a42e3cc36b865344d0bf9e3b5c81bbd1c0a32d

## Executive decision

Poodle should not add a generic TranscriptInlineApproval component, transcript
approval item, or Approval variant union at this time.

The evidence supports three narrower decisions:

- Compose [AgentQuestion](../../contracts/components/agent-question.md) and
  [AgentPlan](../../contracts/components/agent-plan.md) in their existing
  [AgentChatInput](../../contracts/components/agent-chat-input.md) slots.
  Their placement, blocking model, editor ownership, focus behavior, and
  settled-record handoff are deliberate contract boundaries.
- Keep [ConfirmAction](../../contracts/components/confirm-action.md) as the
  dialog-only convenience composite for destructive or significant application
  actions. Do not turn it into an inline agent protocol surface.
- Keep a genuinely new command, recommendation, confidence, alternative,
  refusal, expiry, or undo workflow consumer-owned until a real downstream
  consumer demonstrates repeated demand for the same semantics across the
  active runtimes.

This is a reject/compose/consumer-owned disposition, not an implementation
proposal. The external examples establish useful interaction patterns, but
they do not establish one reusable Poodle meaning. There is no downstream
product use of the current agent surfaces in the release roster; the live
Poodle preview specimens are the available host consumers.

No contracts, source, specimens, packages, roadmaps, triage files, or
consumers were changed. This dossier is the only changed file.

## Evidence method and source limits

Evidence labels:

- Local fact: a current repository contract, implementation, test, or
  consumer observation.
- Source fact: an observation from a named external source.
- Worker inference: a conclusion drawn from the local and source facts. It is
  not a Poodle contract until promoted through the repository authority.

External sources were accessed on 2026-09-01. Mutable pages and registries are
recorded as observations, not as durable API authority. Where a public source
repository exists, the commit pin is recorded. No external code, CSS, icons,
assets, copy, or vendor vocabulary is proposed for reuse.

| Source | Durable capture | Useful observation | Limit |
| --- | --- | --- | --- |
| [Beautiful UI showcase](https://www.beautifului.dev/) | Official page, accessed 2026-09-01; retrieved HTML SHA-256 03663cef0f201febce60a152b4a4f5fc9f3541ad7208ab4d42569984c6496387; live page is mutable | The Approval Card shows a human-in-the-loop choice with several options, an Other input, progress, Skip, and Continue. The Recommendation Card adds confidence, alternatives, and Accept. | The showcase is not a pinned source snapshot. It is evidence of visual/product patterns, not a Poodle contract. |
| [Beautiful UI license](https://www.beautifului.dev/license) | Official license page, accessed 2026-09-01; retrieved HTML SHA-256 de180f0aad37d4326ef065dd7550d828b5836e55b22e34aab9cfeff5da9cd17d | The site publishes an MIT license with notice and disclaimer requirements. | The page is mutable, and this research does not infer that every linked or inspired artifact has the same provenance. |
| [Pinned MIT-inspired approval-card implementation](https://github.com/ithmz/beautiful-ui/tree/31870d86820da6edfdf8108686c4c93bd31611f1) | Public repository commit 31870d86820da6edfdf8108686c4c93bd31611f1, resolved 2026-09-01; [pinned source](https://github.com/ithmz/beautiful-ui/blob/31870d86820da6edfdf8108686c4c93bd31611f1/components/ui/approval-card.tsx) | The implementation has pending, approved, rejected, and Undo states. Its icons are decorative and it has no demonstrated async/in-flight guard, focus model, dialog model, or live-region contract. | The README says it is inspired by Beautiful UI and is not affiliated or endorsed. It is a corroborating implementation, not the official Beautiful UI source. Its MIT license permits that repository's code under notice conditions; it does not authorize copying into Poodle without review. |
| [AICSS Approval Card](https://www.aicss.dev/components/approval-card) | Official page and registry URL accessed 2026-09-01; registry retrieval SHA-256 43c44640d310a79dfe336662382858625add7cd9aeb2a83484acc8997e0f0e8e; registry is [mutable](https://www.aicss.dev/r/approval-card.json) | One visual component exposes Questions, Command, and Plan variants. The page shows plan preview, a shell command, up to three questions, an auto-approve countdown, View Plan, and Approve. The registry code includes radio/checkbox semantics, question viewport live announcements, focus for a custom answer, disabled incomplete actions, reduced-motion handling, and an auto-approve timer. | The page/registry is mutable. The public [AICSS repository](https://github.com/kvnkld/aicss/tree/4556a918fd8c9358d42d2b24a3866301b8ea10a2) is pinned at commit 4556a918fd8c9358d42d2b24a3866301b8ea10a2 and is MIT, but its README says the website is private and distinguishes free public components from Pro components. The repository license cannot be assumed to cover the mutable website registry. |
| [Fluid Functionalism AskUserQuestions](https://www.fluidfunctionalism.com/docs/ask-user-questions) | Official page and [registry artifact](https://www.fluidfunctionalism.com/r/ask-user-questions.json) fetched 2026-09-01; registry retrieval SHA-256 5c97c08e878f84ba474ed9c17eb23223ad277654600a02fecf9206f6b51a2082; ETag 9a947948053e1b18cb12a8eeeeb723a9; Last-Modified Tue, 01 Sep 2026 20:52:30 GMT; registry response is mutable | A stepped question flow supports 2–5 options, single/multi-select, numbered 1–9 shortcuts, inline Other text, optional Skip, question navigation, single-select auto-advance, roving focus, validation, reduced motion, and keyboard-aware focus restoration. | No explicit license was found on the page or registry artifact. Open-source metadata is not a license. No pinned public source repository was found from the official page. Do not copy the registry source, CSS, assets, or text. |

Source synthesis: all three references treat several workflows as an
approval-like visual family, but their public behavior differs materially:
question collection, command authorization, plan review, recommendation
acceptance, countdown policy, Undo, and multi-question navigation are not one
state machine. [Worker inference] The reusable signal is bounded decision
interaction and careful focus/state handling; the payload and policy remain
domain-owned.

## Live Poodle contract audit

### Existing semantics

| Surface | Placement and owner | Blocking | Terminal path | Accessibility/focus contract |
| --- | --- | --- | --- | --- |
| AgentQuestion | Composite inside AgentChatInput, above the editor. The composer owns the free-text override; AgentQuestion owns option selection and answer resolution. | Blocks the turn, not the UI. Ordinary sends are unavailable while the batch is unresolved. | Selection, override, or explicit dismissal resolves an answer. The host advances the batch and appends a read-only AgentQuestionRecord. Stop abandons the turn. | Plain group/radiogroup, real prompt label, radio/checkbox checked state, explicit dismiss control, no scrim or focus trap. The editor keeps focus; Escape is intentionally a no-op. See the [contract](../../contracts/components/agent-question.md), especially its placement, lifecycle, and accessibility sections. |
| AgentPlan | Composite inside AgentChatInput, above the editor. It owns decision controls and no text input. | Blocks the next operator action, not the turn. The editor stays available as the revision channel. | Accept, revise, or dismiss settles the plan. The host returns to idle and appends a read-only AgentPlanRecord. A settled AgentPlan only shows a transitional badge. | Plain region, ordinary labelled buttons, no dialog semantics, no scrim or focus trap. Revise asks the host to focus the composer; Escape is a no-op. See the [contract](../../contracts/components/agent-plan.md). |
| AgentQuestionRecord | Transcript-owned read-only settled record. | None. | Chosen, overridden, or declined answer is history; it cannot be re-answered. | Plain region, no interactive decision affordance, chosen state is conveyed in text. See the [record contract](../../contracts/components/agent-question-record.md). |
| AgentPlanRecord | Transcript-owned read-only settled plan record. Disclosure can reveal the full plan, but it is not a decision control. | None. | Accepted, revised, or dismissed plan is history; it cannot be re-decided. | Plain region, status text, and disclosure state when used. See the [record contract](../../contracts/components/agent-plan-record.md). |
| ConfirmAction | Application-owned trigger paired with AlertDialog. | Blocks the dialog interaction while open, with modal focus behavior. It is not an agent turn protocol. | Confirm or cancel closes the dialog; async application work is owned by the callback/consumer. | Alertdialog, labelled title/description, focus enters the dialog and returns to the trigger, Escape and Tab delegated to dialog behavior. Inline confirmation is explicitly out of scope. See the [contract](../../contracts/components/confirm-action.md). |
| AgentTranscript | Append-oriented output log with its own scroll viewport. It renders settled records, not live decisions. | None. | Host appends terminal records. Detached readers keep their scroll position; following readers may follow new blocks. | One role=log with aria-live=polite. Appending must not move composer focus or caret. Streaming message bodies do not get token-level live regions. Virtualization can remove blocks from the accessibility tree. See the [contract](../../contracts/components/agent-transcript.md). |

Local fact: the existing contracts answer the card's placement and blocking
questions already. Moving AgentQuestion or AgentPlan into the transcript
would either duplicate the editor/override path, change a turn or action
boundary, or put live decision controls into a surface whose contract is
append-only history.

### Live implementation and parity facts

- The shared headless question core resolves overrides, selected options,
  ordered answers, dismissal, progress, and immutable question records in
  [agent-question.ts](../../../packages/core/src/agent-question.ts). The
  question component has no general disabled or in-flight prop; its host must
  transition it out after resolution. Svelte and React both implement the
  editor-as-override rule and guard digit shortcuts while an input has focus
  ([Svelte](../../../packages/svelte/components/src/AgentQuestion.svelte),
  [React](../../../packages/react/components/src/AgentQuestion.tsx)).
- The shared plan core only decides a pending plan and returns no decision for
  an already-settled plan in
  [agent-plan.ts](../../../packages/core/src/agent-plan.ts). The live controls
  are ordinary buttons and have no explicit async/in-flight state. The host
  owns the transition to the settled record
  ([Svelte](../../../packages/svelte/components/src/AgentPlan.svelte),
  [React](../../../packages/react/components/src/AgentPlan.tsx)).
- AgentChatInput selects only the active question or plan child by status. It
  keeps one editor: the question override is that editor, while a plan leaves
  it as the ordinary revision channel. The contract records this in
  [AgentChatInput](../../contracts/components/agent-chat-input.md), and the
  Svelte and React implementations seat the child above the editor
  ([Svelte](../../../packages/svelte/components/src/AgentChatInput.svelte),
  [React](../../../packages/react/components/src/AgentChatInput.tsx)).
- Svelte renders both settled question and plan records in
  [AgentTranscript.svelte](../../../packages/svelte/components/src/AgentTranscript.svelte).
  React renders answered-question blocks but currently has no decided-plan
  branch in [AgentTranscript.tsx](../../../packages/react/components/src/AgentTranscript.tsx).
  The shared Rust transcript renderer also currently skips the visual plan
  block because it has no plan-card primitive in
  [agent_transcript.rs](../../../packages/render/src/agent_transcript.rs).
  These are existing transcript parity gaps, not evidence for adding a new
  live approval item.
- The native path is real: shared Rust accepts question and plan child
  vectors, and the GPUI preview mounts AgentChatInput, AgentQuestion, and
  AgentPlan nodes. The current native renderer carries roles, labels,
  focusability, and activation handlers, but GPUI 0.2.2 has no accessibility
  tree/announcement API. The accepted runtime delta is recorded in
  [003-native-accessibility](../../contracts/003-native-accessibility.md).

## Real consumer workflows

### AgentChatInput question and plan specimens

The Svelte [AgentChatInput specimen](../../../packages/svelte/preview/src/specimens/AgentChatInputSpecimen.svelte)
and React [AgentChatInput specimen](../../../packages/react/preview/src/gallery/specimens/AgentChatInputSpecimen.tsx)
are paired live host consumers of the contract:

1. In questioning state, AgentQuestion owns selections. The composer editor
   is the only free-text override. AgentChatInput's submit request routes to
   the question ref; the host resolves the answer, clears selection/override
   state, and can append the answered-question record.
2. In reviewing-plan state, AgentPlan owns Accept/Revise/Dismiss. The composer
   editor remains an ordinary revision message path. The host changes the
   status and can append the decided-plan record after the one-shot decision.

These workflows demonstrate that the same visual location does not imply the
same semantics: question submission is a turn answer, while plan revision is
an ordinary message after the plan-mode turn. An inline transcript control
would need a third blocking and focus model rather than merely relocating one
of these children.

### Destructive/significant application actions

The paired Svelte and React [LicenceSeats](../../../packages/svelte/components/src/LicenceSeats.svelte)
consumers use ConfirmAction for releasing a seat and disable or show pending
state on the row action while the callback is active. The paired
[UpdateStatus](../../../packages/svelte/components/src/UpdateStatus.svelte)
consumers use an AlertDialog for Install and restart, with application
pending state. HistoryCenter uses a single AlertDialog for delete and restores
focus to its trigger after the modal closes.

These are real application workflows with different ownership and risk
policies. They support keeping ConfirmAction dialog-only and do not justify
an agent-specific transcript semantic.

### Downstream demand check

The [g15 release baseline roster](../../roadmaps/g15/release-baseline-roster.md)
records no downstream consumer use found for AgentPlan, AgentPlanRecord,
AgentQuestion, AgentQuestionRecord, or AgentTranscript. This is not a release
failure, but it is important research evidence: there is no product workflow
currently demonstrating that a shared transcript-inline approval has repeated
cross-product demand. The paired previews and native specimens are contract
consumers and parity probes, not downstream product adoption.

## Lifecycle trace

| Concern | Existing Poodle behavior | Consequence for a new transcript-inline approval |
| --- | --- | --- |
| Focus entry | Question and plan mount above the editor but do not steal focus. ConfirmAction enters a dialog and traps focus. Transcript append does not focus the log. | An inline surface cannot choose a focus rule by visual analogy. It must preserve the current caret or explicitly replace it, and must not turn the log into a modal. |
| Focus return | Plan Revise asks the host to focus the editor. Question submit is routed through the editor/action and host state. Transcript append leaves focus unchanged. Dialog close returns focus to its trigger. | Placement, return target, and cancellation must be semantic inputs, not inferred from a card shape. |
| Repeated activation | Question single-select can call its submit callback immediately; the component has no in-flight guard. Plan core rejects an already-settled status, but a render can receive repeated activation before host state changes. | Any future shared action needs an explicit one-shot/working guard and callback ownership. |
| Replacement | Question batches advance without a back/re-answer path. Plan pending becomes settled and then a record. | A live decision must be removed or replaced by a read-only terminal record; a transcript record must never retain active controls. |
| Cancellation | Question Escape is a no-op; explicit dismiss resolves declined. Plan Escape is a no-op; explicit dismiss is a decision. Stop abandons a questioning turn. Dialog Escape/backdrop cancel belongs to the dialog. | A new surface needs explicit cancel, abort, expiry, and terminal-result semantics. “Dismiss” cannot be a generic label with inferred meaning. |
| Disabled state | AgentChatInput disables its action when canSubmit is false. AgentQuestion and AgentPlan do not expose general disabled/in-flight props. ConfirmAction consumers often disable their trigger while application work is pending. | Disabled, pending, failure, retry, and already-settled behavior must be contract data if a new workflow is ever shared. |
| Decision to record | Host callback resolves the question/plan, returns the composer to the appropriate state, and appends a settled record. Svelte supports both record kinds; React and shared Rust currently have decided-plan rendering gaps. | The host must own persistence, provenance, ordering, and record append timing. Cross-runtime proof must include the record transition, not just a card click. |
| Scroll and announcement | AgentTranscript is a detached-scroll-aware role=log with polite append announcements. It does not announce every streaming token and may virtualize old blocks. | Live controls inside the log risk duplicate announcements, disappearing focus targets, and history mutation. A new live region is not justified by the external card visuals. |

## Reusable semantics versus domain payload

The following is a research boundary, not a public API proposal.

Potentially reusable only after proof:

- a stable decision identity and host-owned placement;
- an explicit user-visible purpose and action label;
- pending, working, and terminal states;
- named callbacks with one-shot behavior;
- explicit cancellation and expiry behavior, if the workflow has them;
- a host-owned immutable terminal record with outcome, time, and provenance;
- keyboard, focus, announcement, and disabled-state rules that are identical
  across every admitted runtime.

Keep consumer or protocol-owned:

- command text, cwd, recipients, permissions, risk, and authorization
  policy;
- plan steps, plan diffs, revision protocol, and plan-specific status;
- recommendation confidence, alternatives, rationale, ranking, and refusal;
- expiry duration, auto-approve policy, undo window, retry, error recovery,
  transport, authentication, persistence, and audit provenance;
- agent/vendor language, domain copy, icons, and visual treatment.

Do not create a broad union whose optional fields make Questions, Command,
Plan, and Recommendation look like one semantic. That shape would move
domain policy into Poodle and make “Approve” ambiguous across turn blocking,
action blocking, and nonblocking recommendation review.

## Disposition matrix

| Candidate | Disposition | Reason |
| --- | --- | --- |
| Extend AgentQuestion into the transcript | Reject | Its free-text override belongs to the one composer editor; its turn-blocking batch and answer-record handoff are not transcript semantics. |
| Extend AgentPlan into the transcript | Reject | Its action-blocking review and ordinary revision channel are composer-owned; the transcript plan record is intentionally read-only. |
| Add a generic Approval or TranscriptInlineApproval semantic | Reject for this lane | No stable shared state machine, no downstream demand, and current runtime/record parity is incomplete. External variants conflate distinct payloads and policies. |
| Compose existing AgentQuestion/AgentPlan in AgentChatInput | Accept | This is the current contract and is demonstrated by paired Svelte/React specimens plus native composition. |
| Compose existing primitives for a novel bounded consumer workflow | Accept provisionally | A product may own a one-off command/recommendation surface using existing primitives while collecting real usage and accessibility evidence. |
| Promote a novel workflow to Poodle | Hold behind gates | Require repeated demand, a stable semantic boundary, a decided blocking/placement model, record ownership, active-runtime proof, accessibility proof, and licensing clearance. |
| Use ConfirmAction for agent approval | Consumer decision, not a default | Use it when the product action is a destructive/significant application confirmation. Do not infer that an agent command is a ConfirmAction merely because the button says Approve. |

## Explicit gates

### Runtime gate

No implementation card for a new shared semantic until the smallest proof
passes in every admitted active runtime:

- Svelte and React mounted fixtures must cover one question and one plan
  through pending, activation, repeated activation, replacement, and settled
  record. Assert the same accessible roles/names, tab order, callback count,
  editor/caret focus, disabled/working behavior, and terminal rendering.
- Shared Rust must have headless vectors for the same state transitions and
  node-tree assertions for role, label, focusability, action identity, and
  absent handlers after settlement or disablement.
- GPUI must have a mounted probe for pointer activation, Tab and Shift+Tab
  order, Escape behavior, and host focus request/return. Do not claim assistive
  technology announcement parity while GPUI 0.2.2 has no accessibility API;
  document that accepted delta.
- Jetstream remains deferred. If admitted later, its AccessKit tree and
  announcement behavior must be compared with the same semantic vectors.
- The current React and shared-Rust decided-plan gaps must be repaired or
  explicitly excluded before transcript parity can be claimed. They are
  prerequisites, not a reason to add a parallel item.

The smallest cross-runtime proof is two existing lanes, not a generic
command/plan/recommendation union:

1. AgentQuestion in AgentChatInput -> answer -> AgentQuestionRecord.
2. AgentPlan in AgentChatInput -> decision -> AgentPlanRecord.

If a novel workflow later earns admission, its first proof should be at the
owning consumer boundary. It should not expand Poodle's public surface before
the workflow repeats across products.

### Accessibility gate

Any future inline decision surface must prove all of the following:

- no scrim, modal focus trap, or hidden focus target when the intended model is
  inline;
- no second text input when the composer is the override/revision owner;
- one clear live-region strategy: transcript role=log with polite terminal
  announcement, no token-level live region, and no duplicate announcement
  from the decision child;
- real labels and group semantics, radio/checkbox checked state where
  applicable, visible text for status, and action names synchronized with
  state;
- Enter, Space, Tab, arrow, and Escape behavior is explicit; repeated
  activation is single-shot; disabled actions expose disabled state and do
  not call the host;
- append, replacement, cancellation, and settlement never steal the editor
  caret or move a detached transcript reader unexpectedly;
- virtualized transcript accessibility limits are recorded;
- Svelte and React have mounted accessibility assertions, Rust has semantic
  node assertions, and GPUI documents its accepted no-AT-tree delta.

### Licensing and provenance gate

- Use external work as behavior/layout evidence only. Do not copy code, CSS,
  icons, assets, text, or vendor-specific vocabulary into Poodle.
- Treat Beautiful UI's live showcase and license page as mutable. If code or
  assets were ever considered, pin an official source and verify the exact
  license scope; the inspected MIT-inspired repository is not official.
- Treat AICSS's mutable registry separately from the pinned MIT public
  repository. The repository pin does not establish the registry or Pro
  component's license scope.
- Treat Fluid Functionalism's registry as unlicensed for reuse until an
  explicit license and immutable source are identified. “Open Source” page
  metadata is insufficient.
- Any future dependency or copied artifact requires legal/license review,
  attribution and notice handling where applicable, and a pinned immutable
  source or digest. There is no third-party dependency in this research.

### Promotion gate

Do not create an implementation card until an operator accepts all of:

- placement: composer, transcript, or owning consumer;
- blocking model: turn-blocking, action-blocking, or nonblocking;
- public semantic boundary and owner;
- terminal record owner, append timing, provenance, and re-decision policy;
- action identity, one-shot/async behavior, cancellation, expiry, failure,
  retry, and undo semantics;
- whether command approval is a destructive application confirmation or an
  agent protocol response;
- active runtime scope, including the GPUI accessibility delta and any future
  Jetstream admission;
- source ownership, versioning, and third-party reuse policy.

## Unresolved operator decisions

These are recorded for promotion review; no decision is requested in this
research lane:

1. Whether a future real product workflow belongs in the composer, transcript,
   or the owning product.
2. Whether a command-like action is ConfirmAction territory, AgentQuestion/
   AgentPlan protocol territory, or a consumer-owned workflow.
3. Whether any novel command or recommendation semantics recur enough across
   products to justify a Poodle contract.
4. The exact public boundary for identity, payload, provenance, terminal
   records, and async/cancellation/expiry/undo behavior.
5. Whether current React and shared-Rust decided-plan rendering gaps are
   prerequisites for any future transcript decision work or are explicitly
   out of scope.
6. The active-runtime proof set and the acceptable GPUI announcement delta;
   Jetstream admission remains governed by its current status.
7. The licensing and provenance policy for future external inspiration or
   dependencies.

## Result

The research answer is: reject a new generic transcript-inline approval
semantic now; compose the existing composer-owned AgentQuestion and AgentPlan
contracts; keep novel bounded workflows consumer-owned until demand and
promotion gates are met. The external sources inform interaction questions,
but the live Poodle contracts, consumers, runtime gaps, and absence of
downstream adoption do not support a new shared transcript item.

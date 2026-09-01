# g16.039 — Agent Task List

Status: research complete; awaiting operator acceptance

Card: `docs/roadmaps/g16/039-agent-task-list-research.md`

Captured: 2026-09-01

Poodle baseline: `06a42e3cc36b865344d0bf9e3b5c81bbd1c0a32d`

## Verdict

Primary disposition: **consumer-owned**.

Composition disposition: **compose locally from existing Poodle primitives when
the host already has the semantics**. A consumer may use `Progress` for a real
scalar, `Stepper` for a real linear route, `AgentTranscript` for settled
conversation evidence, and the existing tool/file cards for their own evidence.
That composition is not a new Poodle task-list contract.

Extend `ToolCallGroup`, extend `AgentPlan`, or add a Poodle `AgentTaskList`:
**reject for this lane**. The evidence shows three different ownership models,
not one reusable Poodle-owned task model. A task-list composite would have to
choose meanings for identity, replacement, retry, selection, hierarchy,
history, and authority that current consumers deliberately keep in their own
domains.

This is a research disposition, not a promotion. No contract, package,
consumer, or roadmap change is proposed here.

## Evidence discipline

- **[LF] Local fact** — read from the Poodle baseline or a captured consumer
  checkout.
- **[SF] Source fact** — read from a primary external source or official
  documentation.
- **[WI] Worker inference** — a conclusion from the evidence.
- **[G] Gate** — a condition that must pass before a future promotion.

The source pass did not copy external code, CSS, icons, or assets. Live web
pages are identified by capture date and response hash where available. The
consumer checkouts were read at pinned Git commits; their working trees are
not Poodle authority.

## Why this is not one thing

| Surface | Meaning | Authority | Interaction boundary |
| --- | --- | --- | --- |
| `AgentPlan` | A proposed plan awaiting one operator decision | Host and plan record | Accept, revise, or dismiss; no task mutation |
| Provider task-list snapshot | Provider-authored ordered work evidence | Host adapter persists a replacement snapshot | Read-only display; no promotion into tasks |
| `ToolCallGroup` | A contiguous run of executed tool calls | Transcript host feeds calls | Disclosure only; no retry or task lifecycle |
| `Progress` | One determinate or indeterminate scalar | Parent supplies value and meaning | Display-only; no events |
| `Stepper` | A route through an explicit multi-step process | Consumer supplies step state and current value | Step selection and optional rerun |
| Nucleus Task/Goal | Durable planning and execution domain records | Nucleus server | Selection, admitted transitions, work-item and review authority |
| Figmatic operation | Host-admitted finite product work with phase progress | Figmatic and Longhorn | Host-owned admission, execution, retry, recovery, and retention |

[WI] “Live agent-maintained checklist” is therefore a presentation shape, not
yet a shared semantic category. A plan step, a provider observation, a tool
execution, a scalar meter, and a durable task can all look row-like while
having incompatible authority.

## Sources inspected

### Poodle authority and live implementation

| Source | [LF] Finding |
| --- | --- |
| [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md), [`docs/architecture/001-poodle-system-shape.md`](../../architecture/001-poodle-system-shape.md), [`docs/architecture/product-guardrails.md`](../../architecture/product-guardrails.md) | Contracts own semantics, states, behavior, accessibility, layout, tokens, and active-cohort parity. Poodle owns generalized primitives and composites; host products own routing, data, persistence, workflow, and domain actions. |
| [`docs/contracts/components/agent-plan.md`](../../contracts/components/agent-plan.md), [`agent-plan-record.md`](../../contracts/components/agent-plan-record.md) | `AgentPlan` is pending decision UI in the composer. Its four statuses are one-way decision state; a settled plan becomes a read-only record. It has no task identity, retry, reorder, or execution state. |
| [`docs/contracts/components/tool-call-group.md`](../../contracts/components/tool-call-group.md), [`tool-call.md`](../../contracts/components/tool-call.md) | A group is adjacency-derived executed tool evidence. Its id is the first call id, its status is error over running over success, and its only group action is disclosure. It must not decide grouping, retry, or task lifecycle. |
| [`docs/contracts/components/progress.md`](../../contracts/components/progress.md) | `Progress` is a parent-owned scalar with `value`, `max`, or indeterminate mode. Its progressbar semantics do not describe ordinal checklist state. |
| [`docs/contracts/components/stepper.md`](../../contracts/components/stepper.md) | `Stepper` requires explicit per-step status rather than deriving status from position. It owns a route/current-step interaction and optional rerun, and explicitly excludes hierarchy, content panels, branching, and percentages. |
| [`docs/contracts/components/agent-transcript.md`](../../contracts/components/agent-transcript.md), [`changed-files.md`](../../contracts/components/changed-files.md), [`agent-subagent.md`](../../contracts/components/agent-subagent.md) | The transcript is append-only host-fed output; changed files are settled evidence; subagent work is observational child topology. None is a generic mutable task list. |
| [`packages/core/src/agent-transcript.ts`](../../../packages/core/src/agent-transcript.ts), [`agent-plan.ts`](../../../packages/core/src/agent-plan.ts) | Shared data currently models transcript item unions, adjacency grouping, windowing, plan decisions, and settled records. There is no `AgentTaskList` type or transcript item. |
| `packages/svelte/components/src/{AgentPlan,AgentTranscript,ToolCallGroup,Progress}.svelte` and `packages/react/components/src/{AgentPlan,AgentTranscript,ToolCallGroup,Progress}.tsx` | Svelte and React consume the same current semantics through native HTML: log, list, disclosure button, and progressbar paths. Their task-list gap is semantic, not a missing row stylesheet. |
| `packages/contracts/components/src/{agent_plan,agent_transcript,tool_call_group,progress}.rs`, `packages/render/src/{agent_plan,agent_transcript,tool_call_group,progress}.rs`, and `packages/gpui/preview/src/node_compat.rs` | Shared Rust composition and the GPUI specimen wrappers exist for the current surfaces. No task-list spec or renderer exists. |
| [`docs/contracts/003-native-accessibility.md`](../../contracts/003-native-accessibility.md) | Svelte/React consume web accessibility metadata. GPUI 0.2.2 has no accessibility tree or role/label API; shared metadata is carried but cannot become mounted assistive-technology proof. Jetstream is deferred by the current working rules. |

[LF] A repository file search at the baseline found no `AgentTaskList`
contract, spec, package component, or specimen.

### Live host models

#### Nucleus provider snapshot

Pinned checkout: Nucleus `9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0`.

- [`AgentChatActivity`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/apps/desktop/src/lib/control/agentChat.ts)
  carries an optional ordered `task_list`. Each item has content, one of the
  provider's pending/in-progress/completed values, and optional priority. The
  item has no stable id.
- [`assembleTurnActivity`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/apps/desktop/src/lib/agentChatTranscript/turn.ts)
  retains a replacement snapshot for one activity identity. A null field means
  “no new snapshot”; an empty array clears the snapshot. The presentation is a
  read-only transcript message, not a Nucleus Task.
- [`019 Conversation Timeline`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/docs/contracts/019-conversation-timeline-contract.md)
  makes the boundary explicit: provider task-list snapshots retain ordered
  content, status, and optional priority, but do not create, mutate, promote,
  dispatch, or complete Nucleus Tasks.
- The provider path therefore has an ordered list and replacement semantics,
  but no task identity, retry authority, selection, hierarchy, or settled task
  history. Those omissions are intentional, not gaps for Poodle to fill.

#### Nucleus durable Task/Goal model

The same pinned checkout also has a separate server-owned task domain:

- [`Task` and `TaskActivityState`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/crates/nucleus-tasks/src/lib.rs)
  carry stable task id, project id, title, description, acceptance criteria,
  action type, assignment, agent readiness, history, timestamps, and explicit
  proposed/ready/active/blocked/done/archived activity.
- [`ControlTaskRecordDto`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/crates/nucleus-server/src/control_envelope_dto/tasks.rs)
  is display-ready but still includes domain fields such as revision,
  acceptance criteria, validation commands, blocked reason, and readiness.
- [`TaskListPanel.svelte`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/apps/desktop/src/lib/TaskListPanel.svelte)
  renders host-supplied goals and tasks. Selection and refresh are host-facing
  callbacks; the detail panel is read-only and transition controls are limited
  to admitted host commands.
- [`023 Task Backed Agent Workflow`](https://github.com/inflatable-cookie/nucleus/blob/9dde0e573e6bc5f9de4261ce8e3dd0655dfac7f0/docs/contracts/023-task-backed-agent-workflow-contract.md)
  keeps runtime completion, review acceptance, and parent task completion
  separate. A goal snapshots ordered task membership at admission, while a
  provider checklist has no durable task identity.

[WI] This is a second, materially different host task model in the same live
consumer: durable work with revisions and authority versus provider display
evidence without task authority. A Poodle composite cannot safely collapse
them.

#### Figmatic host operation model

Pinned checkout: Figmatic `7deb780ea44dafb38a649b8805106ff709706bfc`.

- [`Repository Authority Map`](https://github.com/inflatable-cookie/figmatic/blob/7deb780ea44dafb38a649b8805106ff709706bfc/docs/architecture/repo-authority-map.md)
  assigns workflow, task admission, scheduler, executors, progress meaning,
  wording, actions, and recovery to Figmatic. Poodle supplies controls and
  layout, not enabled-state rules.
- [`006 Operations and Retained Notifications`](https://github.com/inflatable-cookie/figmatic/blob/7deb780ea44dafb38a649b8805106ff709706bfc/docs/contracts/006-operations-and-retained-notifications.md)
  defines finite host kinds with scope, phase labels, determinate or
  indeterminate progress, terminal outcomes, retry lineage, and recovery. The
  MessageCenter rows are read, non-selectable, non-removable projections; retry
  remains a fresh host admission.
- [`activity/capture.rs`](https://github.com/inflatable-cookie/figmatic/blob/7deb780ea44dafb38a649b8805106ff709706bfc/crates/figmatic-studio/src/activity/capture.rs)
  reports a host-owned 0..16 unit projection and phase labels. The renderer
  does not invent the meaning of a unit or its terminal state.

[WI] Figmatic confirms the broader boundary: even when a host has many
“task-like” rows and detailed progress, operation identity, retry, recovery,
and meaning stay outside Poodle.

## External examples

### Beautiful UI — Task Rows

Primary source: [Beautiful UI](https://www.beautifului.dev/), checked
2026-09-01. The live response was 627,682 bytes with SHA-256
`03663cef0f201febce60a152b4a4f5fc9f3541ad7208ab4d42569984c6496387`.

[SF] The page presents three compact rows with completed, running, and
sequence-driven states. The embedded source defines a `TaskRow` with a stable
`key`, label, amount, status (`done`, `running`, or `sequence`), optional step,
and detail rows. A row header is a disclosure button with `aria-expanded`;
details are label/metadata pairs. The sequence internally advances from
pending to failed to completed and shows a retry icon during the failure.

[WI] Transferable presentation evidence:

- a stable row key is useful for preserving expansion and visual identity;
- a concise primary label, secondary amount, and optional detail disclosure
  fit the compact row shape;
- failure must be visible in the row, not hidden behind a successful latest
  update;
- details can be richer than the primary status without becoming a tool-call
  transcript.

[WI] Limits:

- the public example's lifecycle is scripted by timers, not supplied by an
  authoritative host model;
- `sequence` is a presentation mode, not a portable task status;
- the supplied rows do not demonstrate insertion, removal, restore, retry
  authority, hierarchy, or settled history;
- the reduced-motion CSS removes visual animation, but the source's timers
  still advance the sequence. A production component cannot equate reduced
  motion with reduced lifecycle work.

License: [Beautiful UI license](https://www.beautifului.dev/license), checked
2026-09-01, states MIT and requires the copyright and permission notice in
copies or substantial portions. The page source is live and no immutable
repository commit for this exact Task Rows implementation was found. Use the
behavior as reference; do not copy its source, CSS, icons, or assets without
retaining the notice and recording provenance.

### AICSS — To-do List

Primary source: [AICSS Task List](https://www.aicss.dev/components/task-list),
checked 2026-09-01. The page describes a collapsible Cursor-style list with
done, in-progress, and pending states. The registry payload at
[`/r/task-list.json`](https://www.aicss.dev/r/task-list.json) was captured the
same day with SHA-256
`cee86a84527518bd2e506be96b6ce9a297944c2098a7b1b94fa6a340c0230083`.

[SF] The payload's `TodoList` uses five fixed labels and an internal `current`
index. Pending, active, and completed are derived from that index; the header
shows a completed/total count; the header is a disclosure button with
`aria-expanded`; rows are an unordered list; and reduced motion jumps the
animation to all complete. There are no external item ids, status inputs,
failure/blocked/skipped states, detail fields, selection callbacks, retry
callbacks, or replacement snapshots.

[WI] Transferable presentation evidence:

- a collapsible header and completed/total summary are useful optional chrome;
- list semantics and text labels are the right baseline for non-interactive
  checklist rows;
- reduced-motion behavior must be specified as state/render behavior, not only
  as a CSS animation switch.

[WI] Limits:

- index-derived status is unsuitable for a live host list because insertion,
  reordering, failure, and restore change the meaning of an index;
- fixed labels and timers are a demo model, not a reusable contract;
- completion count is a summary, not evidence that the host task completed;
- the example supplies no authority boundary for retry, selection, or history.

License boundary: [AICSS license](https://www.aicss.dev/license), checked
2026-09-01, says free components and the CLI are MIT-licensed on GitHub and
npm, while the website is private and Pro components are not in the public git
tree. The public package source was pinned to commit
[`4556a918fd8c9358d42d2b24a3866301b8ea10a2`](https://github.com/kvnkld/aicss/tree/4556a918fd8c9358d42d2b24a3866301b8ea10a2);
its [MIT notice](https://github.com/kvnkld/aicss/blob/4556a918fd8c9358d42d2b24a3866301b8ea10a2/LICENSE)
and [README](https://github.com/kvnkld/aicss/blob/4556a918fd8c9358d42d2b24a3866301b8ea10a2/README.md)
identify that repository as public package source and the website as private.
The mutable registry payload is not treated as covered by that pinned source
license. Reference the interaction shape only; do not copy the payload's
code into Poodle without provenance and license review.

## Candidate dispositions

| Candidate | Disposition | Evidence and boundary |
| --- | --- | --- |
| Extend `ToolCallGroup` | **Reject** | Its contract groups adjacent executed calls and reports aggregate run failure. A task row can fail without being a tool call, can persist after a run, and can have host-owned retry or selection. Extending it would falsify execution evidence. |
| Extend `AgentPlan` | **Reject** | A plan is a pending operator decision with one-way decision state. Plan prose may contain numbered steps, but accepting it does not execute or complete those steps. |
| Reuse `Progress` as each task row | **Reject** | A row's ordinal status is not a scalar fraction. Use `Progress` only when the host supplies meaningful numeric progress with min/max semantics. |
| Extend `ChangedFiles` or `AgentTranscript` | **Reject** | Changed files are settled output; transcript grouping and append-only history are already bounded. A new mutable task item would need a new item kind and authority model, not a disguised existing block. |
| Use `Stepper` directly | **Conditional compose** | Fits only when the consumer has a true route/current step and accepts Stepper's selection/rerun semantics. A parallel checklist with independent failures, no navigation, or host-owned detail is not a Stepper. |
| Add `AgentTaskList` to Poodle | **Reject for now** | No two independent consumers expose the same stable-id, lifecycle, action, history, hierarchy, and announcement contract. The two design references are presentation demos, not sufficient semantic evidence. |
| Compose at the consumer boundary | **Recommended technique** | Keep the host snapshot, ownership, labels, actions, and history in the consumer. Compose existing Poodle primitives only for the portions whose semantics already match. |
| Keep the list consumer-owned | **Recommended ownership** | Nucleus provider evidence, Nucleus durable tasks, and Figmatic operations all keep authority outside Poodle. This is the only disposition that preserves those boundaries without a speculative contract. |

## Bounded semantic model if evidence changes

This is a future host-side shape, not a Poodle API proposal. It is the smallest
model that would make a later cross-consumer claim testable:

```text
TaskListSnapshot {
  scopeKey: stable host-owned scope
  revision: host-owned replacement/version marker
  items: ordered TaskListItem[]
}

TaskListItem {
  id: stable within the scope (synthetic ids marked by the adapter)
  label: full primary text
  status: pending | running | complete | failed
  detail: optional host-authored disclosure content
  progress: optional { value, max } only when numeric progress is real
}
```

Status extensions are evidence-gated, not assumed:

- `pending`, `running`, `complete`, and `failed` are the minimum useful set.
  The examples and live hosts demonstrate these meanings, though with different
  names and authority.
- `blocked` is distinct from `pending` when the host can state the dependency,
  approval, or input that prevents progress. It should not be invented by a
  renderer from elapsed time.
- `skipped` is distinct from `failed` only when a host domain deliberately
  records “not run”; current evidence does not justify making it required for a
  shared Poodle surface.
- Counts are summaries, not statuses. A completed/total count may be shown
  when the snapshot is complete; it must not grant completion authority.
- Details are content, not status. Failure detail must remain available and
  associated with the failed item; it must not be collapsed into a generic run
  error.

The list is flat. Nucleus's Goal → ordered task references are a domain
relationship, not evidence for arbitrary nested UI. Beautiful UI's detail rows
are disclosure content, and AICSS's rows are flat. Do not generalize
`children`, `parentId`, drag reorder, or nested progress until two independent
consumers require the same semantics.

Product fields such as action type, readiness, priority, assignment, and
workflow state stay in the consumer. A future Poodle surface would receive
normalized semantic state plus host-supplied labels; it would not hard-code
Nucleus, Figmatic, provider, or vendor status vocabulary.

## Semantics probes

These are research probes against the captured examples and live consumer
models, not implementation tests. They answer whether the proposed semantics
survive the card's edge cases.

| Probe | Observed evidence | Result |
| --- | --- | --- |
| Rapid status changes | Beautiful UI advances one demo row from pending to failed to completed on timers. AICSS advances every row from an index. Nucleus retains the latest replacement snapshot for one activity identity. | A live model needs explicit status plus stable identity and host revision. Timer/index state is presentation-only; Poodle must not own it. |
| Insertion, removal, and reorder | Neither design example exercises it. Nucleus defines replacement order and explicit empty clearing; the durable Task panel keys task rows by stable ids and treats stale selection as host-driven. | The list must be keyed by host id, with atomic replacement and deterministic focus fallback. Reorder is host data, not a drag interaction. |
| Long labels | Beautiful's row label is visually truncated; neither external example proves a complete accessible name at narrow width. Poodle's existing disclosure/list contracts require accessible names and bounded layout. | Full text must remain nameable even when visual layout truncates or wraps. This remains a promotion gate, not evidence that either vendor example passes it. |
| Failure detail | Beautiful exposes a failed badge and retry glyph but no retry authority. Nucleus durable tasks expose a blocked reason; provider snapshots do not carry failure detail. | Failure state and detail are separate host fields. Display the detail; expose retry only through a host command. Do not use `ToolCallGroup`'s aggregate call failure as a substitute. |
| Empty list | Nucleus distinguishes omitted snapshot (retain) from empty replacement (clear). AICSS always renders its fixed five rows. Poodle `ChangedFiles` intentionally renders no card for empty files. | Empty/omitted behavior belongs to the host projection. A consumer must not synthesize pending rows or silently treat omission as clear. |
| Restored history | Nucleus persists the final provider snapshot and durable task revision. Figmatic's v1 live operation state is process-local and does not resume after relaunch. Neither animated reference is a history model. | Restore settled host data read-only, without timers or initial live announcements. History and recovery remain host-owned. |

## Ownership rules

| Concern | Owner in the recommended shape |
| --- | --- |
| Snapshot identity, revision, replacement, omission, and clear | Host adapter/domain; the renderer receives the current snapshot |
| Stable item ids and order | Host; array position is never identity |
| Status transitions and terminal truth | Host runtime or domain; UI displays the latest admitted state |
| Reorder, insertion, removal, and dependency resolution | Host; the UI preserves surviving row identity and does not infer authority |
| Retry, cancel, resume, accept, or promote | Host command boundary; no generic row action |
| Selection and navigation | Consumer shell; only add controls when the host has a target and action |
| Expansion | Local presentation state unless the host must persist/restore it; any controlled persistence is host-owned |
| Settled history and restored snapshots | Host persistence/transcript; initial restore is read-only and must not replay live announcements |
| Labels and product wording | Consumer or adapter; Poodle supplies structure and semantic slots |
| Numeric progress meaning | Parent host; Poodle `Progress` only renders a valid scalar |

Replacement snapshots should be applied atomically. A stale revision must not
overwrite a newer one. A consumer that lacks stable provider item ids may
preserve only the list as a whole; it must not pretend that content hashes or
array indexes are durable identity.

## Required behavior probes

| Scenario | Required result for a future consumer or promotion |
| --- | --- |
| Rapid status changes | Key rows by stable id. Render the latest host snapshot. Keep focus and expansion on surviving ids. Coalesce announcements to meaningful status/detail changes; do not announce every timer tick or replay the whole list. |
| Insertion, removal, and reorder | Treat the replacement revision as authoritative. Do not use array indexes as keys. New rows may enter without moving focus; removed focused rows need a consumer-defined focus fallback. Reorder is data, not a drag affordance. |
| Long labels | Wrap or use a visual truncation strategy without losing the full accessible name. Amounts and status text must not force the label out of the row. Fixed-width clipping is a failed case. |
| Failure detail | Show an explicit failure state in text and non-color form. Keep the host detail available through a disclosure or visible body. Retry is absent unless a host callback and admission rule exist. |
| Empty list | Distinguish “no snapshot supplied” from an explicit empty replacement when the host contract does. Do not render phantom pending rows. Nucleus's explicit empty replacement clears; omission retains the prior snapshot. |
| Restored history | Restore the host's settled order/status/detail as read-only. Do not run timers, retry actions, or initial live announcements. Any navigation or mutation remains a host command. |

## Active-cohort rendering and accessibility map

### Web: Svelte and React

[LF] The current web components already establish the useful grammar:

- use a real `ul`/`ol` and `li` for non-interactive rows;
- use a native button for disclosure, with `aria-expanded` and an optional
  `aria-controls` target, matching the [W3C disclosure pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/);
- expose status as visible text and make icons decorative when they repeat that
  text; never make color or a spinner the only state signal;
- use `Progress`/`role="progressbar"` only for real `value`/`max` progress, not
  for “three of five rows are complete” unless the host explicitly defines that
  aggregate as meaningful;
- keep row labels full in the accessibility tree even if a narrow layout uses
  visual ellipsis; and
- preserve the focused control when a snapshot changes, instead of moving focus
  to the list or announcing a complete-list replacement.

The current `AgentTranscript` is already `role="log"` with polite live output.
It must not be nested with a second whole-list live region that repeats every
task update. A structured task list inside a transcript would need one
announcement policy: update one stable item or emit one concise host-owned
status message, not append a new row for every transition.

WAI-ARIA 1.2 defines `status` as a polite, atomic live region and advises that
it should not receive focus. Therefore a future implementation should put only
the concise changed-item message in a dedicated status region; putting the
entire task list in `role="status"` would cause full-list announcements on
updates. Use `alert` only for genuinely urgent, time-sensitive failures, not as
the default for every failed row. See the [WAI-ARIA 1.2 status role](https://www.w3.org/TR/wai-aria-1.2/#status),
[live-region definition](https://www.w3.org/TR/wai-aria-1.2/#aria-live), and
[progressbar role](https://www.w3.org/TR/wai-aria-1.2/#progressbar).

### Shared Rust composition and GPUI

[LF] `poodle-specs` and `poodle-render` carry the current component semantics
into GPUI's preview wrappers. A future Poodle task-list promotion would need a
new spec, headless behavior vectors, shared renderer, Svelte implementation,
React implementation, and GPUI specimen/interaction coverage. A web-only list
would be a consumer feature, not a Poodle active-cohort component.

[LF] The native accessibility contract is a hard gate. GPUI 0.2.2 currently
cannot expose the required roles, names, state changes, or live announcements
through a native accessibility tree. Native construction or visual parity is
not assistive-technology proof. Until that runtime changes or the operator
explicitly accepts a bounded native exception under contract 003, a new shared
Poodle task-list component cannot claim full accessibility parity.

### No product status names in Poodle

[G] A future shared component may normalize host values into semantic visual
categories, but it must take labels and detail copy from the consumer. Strings
such as a product's readiness, workflow, priority, or provider lifecycle names
must not become a Poodle vocabulary. The host owns the mapping from its domain
states to the shared semantic inputs.

## Runtime, accessibility, licensing, and promotion gates

### Runtime gates

1. **Ownership gate:** two independent live consumers must demonstrate the same
   meaning for identity, replacement, statuses, detail, history, and actions.
   Current evidence fails this gate: Nucleus provider snapshots, Nucleus Tasks,
   and Figmatic operations intentionally differ.
2. **Model gate:** stable ids, atomic replacement/version behavior, explicit
   status semantics, empty/omission rules, long-label behavior, failure detail,
   and restored-history behavior must be written before an implementation card.
3. **Web gate:** Svelte and React must pass shared vectors for all six probes,
   with keyed updates, controlled disclosure, no timer-owned lifecycle, and
   host callbacks for every action.
4. **Native gate:** shared Rust and GPUI must cover the same semantic states and
   interaction outcomes. GPUI accessibility remains blocked by the current
   runtime capability described in contract 003; construction evidence cannot
   waive that fact.
5. **Parity gate:** the component must remain a generalized contract. A
   consumer-specific field, product status, transcript adapter, or task-domain
   action is a reason to keep the surface consumer-owned.

### Accessibility gates

- list and item semantics survive empty, insert, remove, reorder, and restore;
- status is conveyed in text, not color, icon, motion, or position alone;
- labels remain fully nameable at narrow widths;
- disclosure is keyboard-operable and reports expanded state;
- focus does not jump when rows update; removed-row fallback is deterministic;
- announcements are concise, polite, coalesced, and scoped to changed content;
- the whole list is not a live region by default, and it is not redundantly
  nested inside the transcript log;
- `Progress` is used only for numeric progress with correct value attributes;
- failure detail is reachable without requiring a retry or a destructive action;
- Svelte/React automated accessibility checks pass, while native claims are
  withheld on GPUI until its runtime gate passes.

### Licensing gates

- use Beautiful UI and AICSS as behavior/presentation references only;
- do not copy source, CSS, icons, or assets from the mutable Beautiful page;
- if exact Beautiful source is ever copied, preserve the MIT copyright and
  permission notice and retain the capture/provenance record;
- treat AICSS's public MIT repository license as applying to the pinned public
  package source, not automatically to mutable website or registry payloads;
- do not import AICSS website/Pro code into Poodle without an explicit license
  review; and
- keep Poodle's implementation original and under Poodle's existing license
  and contribution policy.

### Promotion gates and unresolved operator decisions

This research lane does not request a decision. These items remain recorded for
the operator-owned promotion review:

- whether any future task-list need belongs in Poodle at all, or remains a
  consumer composition;
- if a shared surface is accepted, whether it is a foundation primitive,
  `Stepper` composition, or a distinct composite rather than an extension of
  `AgentPlan` or `ToolCallGroup`;
- which host owns the normalized snapshot, status mapping, replacement cursor,
  retry/selection actions, and settled history;
- whether `blocked` and `skipped` need shared semantics or remain consumer
  fields;
- whether flat rows remain sufficient or two independent consumers truly need
  hierarchy; and
- whether GPUI's current accessibility limitation is an accepted documented
  exception or a hard deferral until the runtime supplies a native tree.

Until those questions have an accepted owner and the runtime gates pass, the
promotion disposition is **hold as consumer-owned; no Poodle component**.

## Conclusion

[WI] The current evidence supports a durable boundary, not a new component:
hosts own task meaning and lifecycle; consumers own task-list composition;
Poodle supplies only the already-contracted pieces whose semantics match.
Beautiful UI contributes row/disclosure and failure-presentation ideas. AICSS
contributes compact count/list and reduced-motion observations. Neither source
justifies importing a vendor task model, and neither resolves the ownership
conflict between provider evidence, accepted plans, executed calls, scalar
progress, and durable tasks.

The next valid promotion input is a second independent consumer with the same
stable-id snapshot semantics and an operator-accepted ownership boundary. No
implementation is admitted by this dossier.

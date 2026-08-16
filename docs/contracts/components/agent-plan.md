# AgentPlan

Status: draft
Updated: 2026-08-07

## 1. Purpose

- Component name: `AgentPlan`
- Layer: `composites`
- Summary: the plan an agent proposes at the end of a plan-mode turn — the
  proposed plan rendered as markdown, with Accept, Revise and Dismiss controls
- In scope: the plan body, the decision controls, the settled badge, the
  decision payload
- Out of scope: any text input (revision feedback is typed in the composer's
  editor, as an ordinary message), the transcript record of a decided plan
  (that is `AgentPlanRecord`), transport, deciding *when* to propose a plan

`AgentPlan` is not a standalone surface. It renders inside `AgentChatInput`'s
field, above the editor, like `AgentQuestion` does.

That placement is the point. A proposed plan is input requiring the operator's
attention. Rendered as a transcript item it would sit in the output stream
while the composer kept accepting messages — two live regions with different
semantics, and the one asking for a decision would scroll away. The transcript
gets the settled record instead (`AgentPlanRecord`), so there is never both a
live plan and a second decision surface on screen.

The component owns no text input and no free-form field. Revise is a signal to
the host to focus the composer's editor, where the operator types revision
feedback as an ordinary message — the same philosophy as the question's
override, one level up.

## 2. Blocking

A proposed plan blocks the **next operator action**, not the turn.

Unlike `AgentQuestion`, a plan appears after the plan-mode turn completes — the
turn is already finished, and the transcript, scrollback and composer all stay
live. What waits is the operator's decision: accept it, revise it, or dismiss
it. Sending an ordinary message while a plan is pending is the revise channel,
not an error — the composer keeps its usual submit semantics under
`status="reviewing-plan"`.

There is no scrim, no focus trap, and no `Dialog`, for the same reason as the
question: the operator deciding a plan about their own codebase may need to
scroll back through the turn that produced it.

## 3. Anatomy

```text
[Root .agent-plan] <div>  (carries data-status/data-size/data-density)
  ├── [Body .agent-plan__body] AgentMessage  (the plan, rendered as markdown)
  ├── [Actions .agent-plan__actions] <div>  (conditional: status="pending")
  │   ├── [Accept .agent-plan__action] <button type="button" data-variant="primary">
  │   ├── [Revise .agent-plan__action] <button type="button" data-variant="secondary">
  │   └── [Dismiss .agent-plan__action] <button type="button" data-variant="ghost">  (conditional: dismissible)
  └── [Badge .agent-plan__badge] <span data-status>  (conditional: a settled status)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | stacks the plan body over the controls above the composer's editor | `--poodle-space-stack-sm` |
| Body | yes | the proposed plan, rendered by `AgentMessage` from raw markdown | (AgentMessage contract) |
| Actions | no | the decision controls; present only while `pending` | `--poodle-space-stack-sm` |
| Accept | yes (while pending) | the primary decision — filled, so the eye finds it first | `--poodle-color-accent-base`, `--poodle-color-text-inverse` |
| Revise | yes (while pending) | asks the host for the composer; the component owns no input | `--poodle-color-text-secondary` |
| Dismiss | no | settles the plan as dismissed | `--poodle-color-text-secondary` |
| Badge | no | the settled status in words; the transitional render before the host swaps in the record | `--poodle-color-text-secondary` |

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `plan` | `string` | `""` | yes | raw markdown of the proposed plan; rendered, never pre-rendered by the host |
| `status` | `AgentPlanStatus` | `"pending"` | no | settled statuses hide the controls and show the badge |
| `dismissible` | `boolean` | `true` | no | dismiss is a first-class decision for a plan, so it renders by default |
| `dismissLabel` | `string` | `"Dismiss plan"` | no | dismiss control label |
| `acceptLabel` | `string` | `"Accept plan"` | no | accept control label |
| `reviseLabel` | `string` | `"Revise"` | no | revise control label |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onAccept` | `(() => void) \| null` | `null` | no | the operator accepted the plan |
| `onRevise` | `(() => void) \| null` | `null` | no | the operator wants to revise; the host focuses the composer |
| `onDismiss` | `(() => void) \| null` | `null` | no | the operator dismissed the plan |

### Shared Types

Defined in `@inflatable-cookie/poodle-core` (`agent-plan.ts`), mirrored in
`poodle-headless::agent_plan` (snake_case).

```typescript
/** Where a proposed plan stands. `pending` is the only undecided state. */
type AgentPlanStatus = "pending" | "accepted" | "revised" | "dismissed";

/** The states a decision can settle a plan into. */
type AgentPlanSettledStatus = "accepted" | "revised" | "dismissed";

/** The operator's decision. Data-only: provenance is the host's to persist. */
type AgentPlanDecision = {
  status: AgentPlanSettledStatus;
  /** ISO timestamp, formatted by the host. */
  decidedAt?: string;
};
```

### Computed Values

| Name | Formula |
|------|---------|
| `isPending` | `canDecidePlan(status)` — `status === "pending"` |
| `badge` | `planStatusLabel(status)` |

## 5. Decision Lifecycle

Three settled states, entered once:

1. **Accepted.** The plan is approved as proposed.
2. **Revised.** The operator wants changes. The component carries no text —
   the callback is the host's cue to focus the composer, and the feedback
   travels as an ordinary message.
3. **Dismissed.** The plan is set aside without acting on it.

`decidePlan(status, next, decidedAt?)` settles a pending plan and returns
`null` for an already-settled one: a decision the host has acted on cannot be
re-decided, which is also why the transcript record carries no re-decide
affordance.

### Dismiss Is A First-Class Decision

`AgentQuestion` hides dismissal behind an opt-in, because skipping a question
the turn is waiting on is the exceptional path. A plan is different: the turn
is complete and "don't do this" is an ordinary answer, so the dismiss control
renders by default. A host can still withhold it with `dismissible={false}`.

### Settled Display Belongs To The Record

A settled `AgentPlan` renders the badge in place of the controls. That render
exists to cover the moment between the operator's click and the host appending
the transcript record — it is not the settled presentation. Hosts swap
`AgentPlan` out of the composer and append a `decided-plan` item rendered by
`AgentPlanRecord`, the same split as the question pair.

## 6. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| pending | `status="pending"` | plan body plus the decision controls |
| accepted | `status="accepted"` | controls hidden; badge reads "Accepted" |
| revised | `status="revised"` | controls hidden; badge reads "Revised" |
| dismissed | `status="dismissed"` | controls hidden; badge reads "Dismissed" |
| not dismissible | `dismissible={false}` | no dismiss control |

## 7. Events

| Event | Payload | When |
|-------|---------|------|
| `onAccept` | — | the accept control was used |
| `onRevise` | — | the revise control was used |
| `onDismiss` | — | the dismiss control was used |

The events carry no payload: the decision is one of three, the plan is the
host's, and the timestamp is the host's to take. `decidePlan` is there for
hosts that want the settled shape rather than assembling it.

## 8. Accessibility

### Semantics

- The root is a plain region, not a dialog: no `aria-modal`, no focus trap.
- The controls are ordinary buttons with their labels as accessible names.
- The badge is text, not colour: the settled status is announced in words.
- A live plan does not steal focus. The composer's editor keeps it, because
  typing revision feedback is a first-class path, not an escape hatch.

### Keyboard

| Key | Action |
|-----|--------|
| `Tab` | moves through the decision controls |
| `Enter` / `Space` on a control | decides |
| `Escape` | nothing — dismissal is a decision, and decisions are explicit |

## 9. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| width | fills the composer's field |
| body-to-actions gap | `--poodle-space-stack-sm` |
| action gap | `--poodle-space-stack-sm` |

### Composition

Rendered inside `AgentChatInput`'s field, above the attachments and editor,
while `status="reviewing-plan"`. The composer keeps its editor, toolbar and
submit control with their ordinary semantics; see `agent-chat-input.md` §Plan
Region.

## 10. Token Usage

| Property | Token |
|----------|-------|
| plan body | (AgentMessage contract) |
| accept fill | `--poodle-color-accent-base` |
| accept label | `--poodle-color-text-inverse` |
| accept fill, hovered | `--poodle-color-accent-hover` |
| secondary label | `--poodle-color-text-secondary` |
| secondary border | `--poodle-color-border-subtle` |
| control radius | `--poodle-radius-control` |
| badge | `--poodle-color-text-secondary` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-status` | `pending`/`accepted`/`revised`/`dismissed` | root, badge |
| `data-size` / `data-density` | the ladders | root |
| `data-variant` | `primary`/`secondary`/`ghost` | each action |

## 11. Svelte Notes

- The plan body is `AgentMessage` with the raw markdown as its `markdown` prop;
  there is no second markdown path to keep in step.
- `isPending` is `$derived(canDecidePlan(status))`; there is no stored
  decision state in the component, because the decision is the host's.

## 12. GPUI And Jetstream Notes

Deferred. Svelte is the reference implementation and the React mirror landed
in `g15.006`; the shared headless core, the conformance vectors, the spec
structs and the `poodle-render` functions (`agent_plan`, `agent_plan_record`)
already land, so a native variant is wiring, not design. See §14.

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] controls render only while `pending`
- [ ] a settled status shows the badge and no controls
- [ ] dismissal renders by default and can be withheld
- [ ] the component owns no text input anywhere
- [ ] the badge wording matches `planStatusLabel` exactly
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] accept is the one filled control
- [ ] the plan body renders through the same markdown path as agent prose
- [ ] gaps match per size and density
- [ ] density never changes control height

### Tier 3: Implementation Freedom

- [ ] key handling beyond the platform defaults is host-owned
- [ ] transition timing is platform-owned

## 14. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI/Jetstream variants deferred — Svelte is the reference and the React mirror landed in `g15.006` | the consumer integrating plan mode (nucleus) is Svelte-only today; the headless core, vectors, specs and shared renderer already landed, so each native variant is wiring | pending review | build the native variants when a second target needs them |
| Settled statuses render on `AgentPlan` at all | the badge covers the moment between the decision and the host appending the record; forbidding it would make that gap a blank flash | accepted (by design) | none |
| `dismissible` defaults to `true`, unlike `AgentQuestion` | the turn is complete, so "don't do this" is an ordinary answer rather than an escape from a blocking prompt | pending review | none |

## 15. Approval And Adoption Notes

- contract status: `draft`
- approvers: pending review
- downstream adopters: nucleus (Codex plan mode), future agent surfaces
- future follow-up: the native variants, diff view of a revised
  plan against its original

## 16. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a pending plan
hosted by the composer under `status="reviewing-plan"`; pending standalone;
each settled status showing the badge; not dismissible; full size ladder;
density variants.

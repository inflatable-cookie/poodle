# AgentSubagent

Status: detailed contract
Updated: 2026-09-02

## 1. Purpose

- Component name: `AgentSubagent`
- Layer: `composites`
- Summary: an inline group for a provider-owned child agent's (sub-agent's)
  work in the transcript — identity and status in the header, a host-supplied
  one-line activity while the child is non-terminal, expandable detail, and a
  click-through to the child's work
- In scope: the header (label + status badge), the live activity line, the
  terminal summary, the expanded detail region, the disclosure, the
  click-through action, the status vocabulary
- Out of scope: any control affordance over the child — no stop, cancel or
  steer, because the underlying model (Swallowtail contract 045) is
  observation-only; transport; deciding *when* to spawn or report a child;
  the child's own output surface

The child's work renders inline in the transcript as it happens. The host
feeds observations; the component never asks the child for anything. The
status vocabulary is Swallowtail's `SubagentStatus` exactly
(`swallowtail-runtime/src/activity/subagent.rs`), so a badge never says
something the provider never said.

The active implementation surfaces are the Svelte and React web shells, the
shared headless model/spec/render path, and the GPUI preview adapter. They all
render the current static group semantics below. Jetstream remains deferred at
the program level and is not an implementation or effect target of this
contract.

`unknown` renders literally as "Unknown". It means "no portable status was
supplied", and any prettier word would be a fact the provider did not give —
so the badge shows exactly the vocabulary value.

## 2. Blocking

Nothing blocks. A child runs alongside the turn, and the transcript must keep
scrolling and reading while it works. There is no scrim, no focus trap, no
`Dialog`, and no pause — the child is provider-owned and the transcript is a
window on it, not its controller.

## 3. Anatomy

```text
[Root .poodle-agent-subagent] <div>  (carries data-status/data-expanded/data-size/data-density)
  ├── [Header .poodle-agent-subagent__header] <div>
  │   ├── [Label .poodle-agent-subagent__label] <span>
  │   └── [Badge .poodle-agent-subagent__badge] <span data-status>
  ├── [Activity .poodle-agent-subagent__activity] <div>  (conditional: non-terminal)
  │   ├── [Spinner .poodle-agent-subagent__spinner] Spinner (variant="dots")  (conditional: status="running")
  │   └── [Activity Line .poodle-agent-subagent__activity-line] <span>  (conditional: activityLine set)
  ├── [Summary .poodle-agent-subagent__summary] <p>  (conditional: terminal and summary set)
  ├── [Detail .poodle-agent-subagent__detail] <ul>  (conditional: expanded and detailLines non-empty)
  │   └── [Detail Line .poodle-agent-subagent__detail-line] <li> (repeated)
  └── [Actions .poodle-agent-subagent__actions] <div>
      ├── [Toggle .poodle-agent-subagent__action] <button type="button" data-kind="toggle">  (conditional: detailLines non-empty)
      └── [Open Child .poodle-agent-subagent__action] <button type="button" data-kind="open">  (web conditional on handler; native renderer emits the labeled action)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | the group card: stacks header, body, detail and actions | `--poodle-color-background-surface`, `--poodle-color-border-subtle`, `--poodle-radius-control` |
| Header | yes | identity and status at a glance | `--poodle-space-stack-sm` |
| Label | yes | the child's short label, at full strength | `--poodle-color-text-primary` |
| Badge | yes | the status in words — never colour alone | per status: `--poodle-color-accent-base` (running), `--poodle-color-status-danger` (failed), `--poodle-color-status-success` (completed), `--poodle-color-text-secondary` otherwise — `unknown` claims nothing, so it reads at meta strength |
| Activity | no | the host-supplied one-line activity while the child is non-terminal | `--poodle-color-text-secondary` |
| Spinner | no | the running indicator; `dots` is the quietest variant, matching the transcript's activity footer | (Spinner contract) |
| Activity Line | no | one line of what the child is doing right now | `--poodle-color-text-secondary` |
| Summary | no | what the child accomplished, once terminal | `--poodle-color-text-secondary` |
| Detail | no | the child's recent activity lines, as plain host-supplied strings | `--poodle-color-text-secondary` |
| Detail Line | no | one recent activity line | `--poodle-color-text-secondary` |
| Actions | yes | the disclosure and the click-through | `--poodle-space-stack-sm` |
| Toggle | no | opens and closes the detail region; exists only when there is detail to reveal | `--poodle-color-text-tertiary` |
| Open Child | yes | the labeled click-through action — "Open child work"; web shells omit it without a handler, while the native renderer currently keeps the unlatchable action in the tree | `--poodle-color-text-tertiary` |

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `item` | `AgentSubagentItem` | — | yes | the child work this group renders; the React and native surfaces require it, while the Svelte adapter accepts omission as an empty render |
| `expanded` | `boolean` | `false` | no | bindable disclosure state; the detail region shows while expanded |
| `detailLines` | `string[]` | `[]` | no | recent activity lines shown when expanded; a simple block list is enough for v1 |
| `expandLabel` | `string` | `"Show activity"` | no | collapsed disclosure label |
| `collapseLabel` | `string` | `"Hide activity"` | no | expanded disclosure label |
| `openChildLabel` | `string` | `"Open child work"` | no | click-through action label |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToggle` | `((expanded: boolean) => void) \| null` | `null` | no | the disclosure was used |
| `onOpenChild` | `(() => void) \| null` | `null` | no | the web click-through was used; the host owns what "open" means. Web shells omit the action when unset; native handlers are supplied separately to `poodle-render` and an absent native handler leaves its labeled action inert |

### Shared Types

Defined in `@inflatable-cookie/poodle-core` (`agent-subagent.ts`), mirrored in
`poodle-headless::agent_subagent` (snake_case). The status vocabulary mirrors
Swallowtail's `SubagentStatus` exactly:

```typescript
type AgentSubagentStatus =
  | "unknown"
  | "pending"
  | "running"
  | "waiting"
  | "completed"
  | "failed"
  | "interrupted"
  | "shutdown";

type AgentSubagentItem = {
  id: string;
  label: string;
  status: AgentSubagentStatus;
  /** One host-supplied activity line while the child is non-terminal. */
  activityLine?: string;
  /** What the child accomplished, once terminal. */
  summary?: string;
};
```

### Computed Values

| Name | Formula |
|------|---------|
| `isTerminal` | `isTerminalSubagentStatus(item.status)` |
| `spins` | `subagentStatusSpins(item.status)` — `status === "running"` |
| `badge` | `subagentStatusLabel(item.status)` |
| `showsToggle` | `detailLines.length > 0` |

## 5. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| running | `item.status="running"` | badge reads "Running" in accent; dots spinner plus the activity line |
| pending | `item.status="pending"` | badge reads "Pending"; activity line without a spinner — the child is not actively working |
| waiting | `item.status="waiting"` | badge reads "Waiting"; activity line without a spinner |
| unknown | `item.status="unknown"` | badge reads literally "Unknown", at meta strength; activity line without a spinner — the provider never said the child stopped |
| completed | `item.status="completed"` | badge reads "Completed" in success; the summary replaces the activity line; no spinner |
| failed | `item.status="failed"` | badge reads "Failed" in danger; the summary replaces the activity line; no spinner |
| interrupted | `item.status="interrupted"` | badge reads "Interrupted"; the summary replaces the activity line; no spinner |
| shutdown | `item.status="shutdown"` | badge reads "Shutdown"; the summary replaces the activity line; no spinner |
| expanded | `expanded` and `detailLines` non-empty | the detail region lists the recent activity lines |
| no detail | `detailLines` empty | no disclosure — an expand control opening an empty region is noise |

### Behavior Machine

Behavior classification: styled-only (no machine). The only interactive parts
are the disclosure and the click-through: the web disclosure flips local
`expanded` state and the click-through signals the host; native composition
receives its state and handlers through the shared spec/render boundary. There
is no child-control or transport behavior here.

## 6. Events

| Event | Payload | When |
|-------|---------|------|
| `onToggle` | `expanded: boolean` | the disclosure was used |
| `onOpenChild` | — | the click-through was used |

No event carries a payload beyond the toggle's next state: the child's status
and lines are the host's data, and the host persists them. The click-through
carries nothing because the component renders exactly one child — "open" is a
signal, and the host decides what that means for its session. Native handlers
use the same two actions through `AgentSubagentHandlers`.

## 7. Accessibility

### Semantics

- The root is a plain group inside the transcript's `log` region; Svelte and
  React `AgentTranscript` own `role="log"` and `aria-live="polite"`, so the
  group adds no live region of its own. The shared native transcript renderer
  owns the surrounding transcript surface as well.
- The badge is text, not colour: the status is announced in words, and the
  `data-status` colours are a second channel, never the only one.
- The spinner is decorative (`aria-hidden`), exactly as in the transcript's
  activity footer: the label next to it is the announcement.
- The disclosure button carries `aria-expanded`, so the region's state is
  announced rather than inferred from the label swap.
- Web actions are real `<button>`s with accessible names from their labels;
  native composition emits button nodes with the same labels.
- A running child does not steal focus and is not announced on every line —
  the transcript's `polite` region announces when it settles.

### Keyboard

| Key | Action |
|-----|--------|
| `Tab` | moves through the disclosure and the click-through |
| `Enter` / `Space` on the disclosure | toggles the detail region |
| `Enter` / `Space` on the click-through | opens the child's work |
| `Escape` | nothing — there is no overlay to dismiss |

## 8. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| width | fills the transcript block |
| inset | `--poodle-space-inset-sm` per density (compact 0.5rem, default 0.75rem, comfortable 1rem) |
| block gap | `--poodle-space-stack-sm` per density |
| header gap | `--poodle-space-stack-sm` |

### Composition

Rendered as a transcript block (`data-kind="subagent-group"`) between the
turn's other blocks, sized and densitied by the transcript's own ladder. The
web group owns its disclosure state; the web transcript does not hold an
expansion list for it. The native transcript spec can supply expanded group
ids to the shared renderer. The host feeds observations — a group appears, its
status and lines update, and terminal statuses settle it in place.

## 9. Token Usage

| Property | Token |
|----------|-------|
| card fill | `--poodle-color-background-surface` |
| card border | `--poodle-color-border-subtle` |
| card radius | `--poodle-radius-control` |
| label | `--poodle-color-text-primary` |
| badge, running | `--poodle-color-accent-base` |
| badge, failed | `--poodle-color-status-danger` |
| badge, completed | `--poodle-color-status-success` |
| badge, other statuses | `--poodle-color-text-secondary` |
| activity line / summary / detail | `--poodle-color-text-secondary` |
| action labels | `--poodle-color-text-tertiary` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-status` | the eight-status vocabulary | root, badge |
| `data-expanded` | `true`/`false` | root |
| `data-size` / `data-density` | the ladders | root |
| `data-kind` | `toggle`/`open` | each action |

## 10. Web Notes

- The spinner is the poodle `Spinner` with `variant="dots"` and `tone="muted"`,
  the same quiet variant the transcript's activity footer uses.
- `isTerminal`, `spins` and `badge` are `$derived` from `item`; there is no
  stored status state in the component, because the status is the host's.
- `expanded` is `$bindable`; unbound, the component owns it as local state.
- Rendered without an `item`, the component renders nothing — an empty group
  would reserve space for a child that is not there.
- React requires `item`, uses local uncontrolled disclosure state unless
  `expanded` is supplied, and reports the same two callbacks.
- Both web shells consume the shared status helpers and AgentSubagent CSS. Both
  render the activity line only for non-terminal statuses, the summary only for
  terminal statuses, and a spinner only for `running`.

## 11. Shared Rust, GPUI, And Jetstream Notes

The shared headless core owns the status vocabulary, terminal mapping, and
spinner mapping. `AgentSubagentSpec` carries the item, disclosure state, detail
lines, labels, size and density. `poodle-render::agent_subagent` renders the
same static header/body/detail/action structure and accepts native handlers.
Its current native behavior emits the labeled open action even without an open
handler; without a handler it has no activation callback.

The GPUI preview uses `node_compat::AgentSubagent::from_spec` and the shared
renderer for its running, waiting, completed, failed, unknown, size and
density specimens. It is a shipped static composition path, not a deferred
variant.

Jetstream remains program-deferred. This contract makes no Jetstream variant,
effect, or admission claim.

## 12. Parity Checklist

### Tier 1: Strict Parity

- [ ] the badge wording matches `subagentStatusLabel` exactly
- [ ] the terminal mapping matches `isTerminalSubagentStatus` exactly
- [ ] the spinner shows only while `running`, and never for a terminal status
- [ ] `unknown` renders literally as "Unknown", never prettified
- [ ] `activityLine` renders only while non-terminal; `summary` only once terminal
- [ ] the component owns no control over the child — no stop, cancel or steer anywhere
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] the badge colours follow the same per-status mapping on every target
- [ ] the group card reads as one unit with the same inset and radius
- [ ] the disclosure and click-through sit in one action row

### Tier 3: Implementation Freedom

- [ ] the detail region's list styling is implementation-owned
- [ ] transition timing is platform-owned

## 13. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream remains program-deferred; Svelte, React, shared Rust composition and GPUI preview are active static surfaces | Jetstream follows the current program admission posture; the active cohort already uses the shared AgentSubagent model and rendering path | accepted for this lane | any Jetstream admission is a separate program decision |
| Web shells omit the open action without a handler; the native renderer emits an inert labeled action when its handler is absent | the adapters currently expose different handler boundaries, and the observable output must remain documented until a parity change is separately approved | current implementation delta | reconcile action presence only in a scoped parity change |
| The disclosure exists only when `detailLines` is non-empty | an expand control opening an empty region is noise; the host decides when there is detail to reveal | pending review | none |
| The Svelte transcript passes no expansion state for groups | a child's expansion is local state the transcript host does not need to hold; the native spec carries `expanded_subagent_groups` for hosts that want to drive it | pending review | none |
| Svelte accepts an omitted item and renders no group; React and native require the semantic item | the web adapter surfaces have different type/runtime boundaries today, while a present item has the same shared shape | current implementation delta | reconcile input optionality only in a scoped API migration |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending review
- downstream adopters: nucleus (provider sub-agent work in the transcript),
  future agent surfaces
- future follow-up: native/web action-presence parity and richer detail than
  plain activity lines

## 15. Specimen Definitions

Required specimen coverage (Svelte, React and GPUI preview surfaces): running
with a spinner and a host-supplied activity line; waiting; completed with a
summary; failed; unknown reading literally as "Unknown"; the expanded detail
region; the size ladder; density variants.

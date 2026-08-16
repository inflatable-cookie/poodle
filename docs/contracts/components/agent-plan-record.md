# AgentPlanRecord

Status: draft
Updated: 2026-08-07

## 1. Purpose

- Component name: `AgentPlanRecord`
- Layer: `composites`
- Summary: the read-only record a decided plan leaves in the transcript — the
  decision badge, a one-line summary, and the full plan behind a disclosure
- In scope: the settled presentation of a proposed plan — accepted, revised,
  dismissed
- Out of scope: deciding anything (`AgentPlan`), re-deciding, any provenance
  beyond what the host hands it

The pending plan lives in the composer, because deciding it is input requiring
the operator's attention. This is what it leaves behind once decided.

Without it the transcript has a hole exactly where the operator steered the
agent: the plan-mode turn ends, the next turn behaves differently, and nothing
on screen says why. With it, the conversation still reads correctly weeks
later.

## 2. Read-Only By Construction

This component has no decision affordance. Not disabled controls — *no*
controls, beyond the disclosure.

That is what makes hosting the live plan in the composer safe. If the record
carried a re-decide affordance, there would be two decision surfaces again,
and the transcript one would let you change a decision the agent has already
acted on.

The disclosure is the one exception, and it decides nothing: it trades the
summary for the full plan and back.

## 3. Anatomy

```text
[Root .agent-plan-record] <div>  (carries data-status/data-expanded/data-size/data-density)
  ├── [Header .agent-plan-record__header] <div>
  │   ├── [Badge .agent-plan-record__badge] <span data-status>
  │   └── [Meta .agent-plan-record__meta] <span>  (conditional: decidedAt set)
  ├── [Summary .agent-plan-record__summary] <p>  (conditional: collapsed)
  ├── [Body .agent-plan-record__body] AgentMessage  (conditional: expanded)
  └── [Toggle .agent-plan-record__toggle] <button type="button" aria-expanded>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | the record; carries which decision it holds | `--poodle-color-background-surface`, `--poodle-radius-control`, `--poodle-color-border-subtle` |
| Header | yes | the decision at a glance | `--poodle-space-inline-sm` |
| Badge | yes | what was decided, in words | `--poodle-color-accent-base` |
| Meta | no | when it was decided, formatted and supplied by the host | `--poodle-color-text-tertiary` |
| Summary | no | one line of the plan, whitespace-collapsed and truncated to budget | `--poodle-color-text-secondary` |
| Body | no | the full plan, rendered by `AgentMessage` from raw markdown | (AgentMessage contract) |
| Toggle | yes | the disclosure between summary and plan | `--poodle-color-text-tertiary` |

### Why The Summary And The Plan Never Show Together

The summary is a stand-in for exactly the content the disclosure hides.
Showing both says one thing twice, and the reader cannot tell which to trust
when the truncated line and the full text disagree in emphasis.

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `plan` | `string` | — | yes | raw markdown of the plan that was decided |
| `status` | `AgentPlanSettledStatus` | — | yes | `accepted` \| `revised` \| `dismissed`; `pending` never reaches the record |
| `decisionLabel` | `string \| undefined` | `undefined` | no | overrides the badge wording; defaults to the status label |
| `decidedAt` | `string \| undefined` | `undefined` | no | when the decision was made, formatted by the host |
| `summaryMaxLength` | `number` | `160` | no | character budget for the summary, ellipsis included |
| `expanded` | `boolean` | `false` | no | disclosure state; controlled when bound |
| `expandLabel` | `string` | `"Show plan"` | no | collapsed toggle label |
| `collapseLabel` | `string` | `"Hide plan"` | no | expanded toggle label |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToggle` | `((expanded: boolean) => void) \| null` | `null` | no | the disclosure was used |

Provenance stops at `decisionLabel` and `decidedAt`. Who decided and why is
the host's to persist — nucleus keeps that in its own store; the record
renders what it is given.

### Computed Values

| Name | Formula |
|------|---------|
| `badge` | `decisionLabel ?? planStatusLabel(status)` |
| `summary` | `planRecordSummary(plan, summaryMaxLength)` |

### Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onToggle` | The record's disclosure toggles | `boolean` | The next expanded state. Controlled when `expanded` is supplied: the host drives state and the component reports without mutating it. Otherwise component-owned, and the host does not have to echo the value back |

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| accepted | `status="accepted"` | badge at accent strength |
| revised | `status="revised"` | badge at accent strength, wording overridable |
| dismissed | `status="dismissed"` | badge at meta strength — a non-event reads as one |
| collapsed | `expanded={false}` | summary line plus the disclosure |
| expanded | `expanded={true}` | full plan, rendered as markdown |
| truncated | plan over budget | summary ends in an ellipsis counted against the budget |

## 6. Accessibility

- The disclosure is the only focusable part. A transcript may contain hundreds
  of records, and a card with no controls never enters the tab order.
- The decision is conveyed in text, not by the badge's colour: the badge is
  the status label in words.
- The toggle carries `aria-expanded`, so the state of the disclosure is
  announced rather than inferred from what appears.
- The summary is plain text — markdown markers collapse to whitespace rather
  than being read as syntax.

## 7. Layout

| Aspect | Rule |
|--------|------|
| width | fills the transcript block |
| card inset | per density |
| header gap | `--poodle-space-inline-sm` |

## 8. Token Usage

| Property | Token |
|----------|-------|
| card fill | `--poodle-color-background-surface` |
| card border | `--poodle-color-border-subtle` |
| card radius | `--poodle-radius-control` |
| badge | `--poodle-color-accent-base` |
| badge, dismissed | `--poodle-color-text-tertiary` |
| meta | `--poodle-color-text-tertiary` |
| summary | `--poodle-color-text-secondary` |
| toggle | `--poodle-color-text-tertiary` |

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-status` | `accepted`/`revised`/`dismissed` | root, badge |
| `data-expanded` | `true`/`false` | root |
| `data-size` / `data-density` | the ladders | root |

## 9. Parity Checklist

### Tier 1: Strict Parity

- [ ] nothing inside decides or re-decides anything
- [ ] collapsed shows the summary, expanded shows the full plan, never both
- [ ] the badge wording falls back to `planStatusLabel` exactly
- [ ] the summary matches `planRecordSummary` exactly, ellipsis in budget
- [ ] `pending` never renders — the record is what a decision leaves behind
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] a dismissed plan's badge drops to meta strength
- [ ] card fill, border and radius match per size and density
- [ ] the expanded plan renders through the same markdown path as agent prose

## 10. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI/Jetstream variants deferred — Svelte is the reference and the React mirror landed in `g15.006` | the consumer integrating plan mode (nucleus) is Svelte-only today; the spec structs and the shared `poodle-render` function already landed | pending review | build the native variants when a second target needs them |
| No re-decide affordance on any target | a second decision surface would let the reader change a decision the agent has already acted on | accepted (by design) | none |

## 11. Approval And Adoption Notes

- contract status: `draft`
- approvers: pending review
- downstream adopters: nucleus (Codex plan mode), future agent surfaces

## 12. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): accepted, revised
and dismissed records; an expanded record showing the full plan; a record with
`decisionLabel` overriding the badge; a record with `decidedAt`; a long plan
truncating to budget; full size ladder; density variants.

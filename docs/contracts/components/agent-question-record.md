# AgentQuestionRecord

Status: detailed contract
Updated: 2026-07-30

## 1. Purpose

- Component name: `AgentQuestionRecord`
- Layer: `composites`
- Summary: the read-only record an answered question leaves in the transcript —
  what was asked, what was on offer, and what was chosen
- In scope: the answered presentation of a question, the chosen option, an
  override answer, a declined question
- Out of scope: asking anything (`AgentQuestion`), any input at all, re-answering

The pending question lives in the composer, because its free-text override *is*
the composer's editor. This is what it leaves behind once answered.

Without it the transcript has a hole exactly where a decision was made: the
agent's behaviour changes course and nothing on screen says why. With it, the
conversation still reads correctly weeks later.

## 2. Read-Only By Construction

This component has no interactive parts. Not disabled inputs — *no* inputs.

That is what makes hosting the live question in the composer safe. If the record
carried a re-answer affordance, there would be two answering surfaces again, and
the transcript one would let you change an answer the agent has already acted
on.

## 3. Anatomy

```text
[Root .agent-question-record] <div>  (carries data-outcome/data-size/data-density)
  ├── [Header .agent-question-record__header] Eyebrow  (conditional: header set)
  ├── [Prompt .agent-question-record__prompt] <p>
  ├── [Options .agent-question-record__options] <ul>  (conditional: outcome is "selected")
  │   └── [Option .agent-question-record__option] <li>  (repeated; data-chosen)
  │       ├── [Option Mark .agent-question-record__option-mark] Icon (icon="check"; chosen only)
  │       └── [Option Label .agent-question-record__option-label] <span>
  └── [Answer .agent-question-record__answer] <p>  (conditional: outcome is "override" or "declined")
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | the record; carries which outcome it holds | `--poodle-color-background-surface`, `--poodle-radius-control`, `--poodle-color-border-subtle` |
| Header | no | the question's short label | (Eyebrow contract) |
| Prompt | yes | what was asked, one step down from live prose — it is history, not the current subject | `--poodle-color-text-secondary` |
| Options | no | what was on offer, so "why did it pick that" has its alternatives | `--poodle-space-stack-xs` |
| Option | no | one option; the chosen one carries `data-chosen="true"` | `--poodle-color-text-tertiary` |
| Option Mark | no | the tick on the chosen option | `--poodle-color-accent-base` |
| Option Label | yes | the option text; chosen at full strength, the rest dimmed | `--poodle-color-text-primary` / `--poodle-color-text-tertiary` |
| Answer | no | the typed answer, or "Declined" | `--poodle-color-text-primary` |

### Why The Unchosen Options Stay

An answered question shows every option, not just the one taken.

"Why did the agent do that" is usually answered by what it *didn't* do. A record
showing only the chosen option loses the alternatives, and the reader cannot
tell whether the choice was between three reasonable things or the only one
offered.

They are dimmed rather than dropped, so the chosen one still reads first.

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `question` | `AgentQuestionItem` | — | yes | what was asked |
| `answer` | `AgentQuestionAnswer` | — | yes | what came back |
| `showOptions` | `boolean` | `true` | no | when false only the chosen answer shows |
| `declinedLabel` | `string` | `"Declined"` | no | wording for a declined question |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |

### Computed Values

| Name | Formula |
|------|---------|
| `summary` | `answeredQuestionSummary({ question, answer })` |
| `showsOptions` | `showOptions && answer.outcome === "selected"` |
| `isChosen(value)` | `isChosenOption({ question, answer }, value)` |

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| selected | `outcome === "selected"` | options listed, chosen ones ticked and at full strength |
| override | `outcome === "override"` | the typed answer; no option list, because none was taken |
| declined | `outcome === "declined"` | `declinedLabel`; no option list |
| multiple chosen | several values | every chosen option ticked |

## 6. Accessibility

- The root is a plain region, not a list of controls. Nothing inside is
  focusable, so the record never appears in the tab order of a transcript that
  may contain hundreds of them.
- The chosen option is conveyed in text, not by the tick alone: chosen options
  are prefixed in their accessible name with "chosen".
- The prompt is a real paragraph, so a screen reader reading the transcript
  linearly encounters the question before the answer.

## 7. Layout

| Aspect | Rule |
|--------|------|
| width | fills the transcript block |
| option gap | `--poodle-space-stack-xs` |
| card inset | `--poodle-space-inline-md` |

## 8. Token Usage

| Property | Token |
|----------|-------|
| card fill | `--poodle-color-background-surface` |
| card border | `--poodle-color-border-subtle` |
| card radius | `--poodle-radius-control` |
| prompt colour | `--poodle-color-text-secondary` |
| chosen option | `--poodle-color-text-primary` |
| unchosen option | `--poodle-color-text-tertiary` |
| tick | `--poodle-color-accent-base` |
| answer colour | `--poodle-color-text-primary` |

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-outcome` | `selected`/`override`/`declined` | root |
| `data-chosen` | `true`/`false` | each option |
| `data-size` / `data-density` | the ladders | root |

## 9. Parity Checklist

### Tier 1: Strict Parity

- [ ] nothing inside is focusable or interactive
- [ ] every option renders for a `selected` outcome, not only the chosen ones
- [ ] `override` and `declined` render no option list
- [ ] chosen options carry "chosen" in their accessible name, not only a tick
- [ ] the summary matches `answeredQuestionSummary` exactly
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] chosen options at full strength, unchosen dimmed
- [ ] card fill, border and radius match per size and density
- [ ] density never changes option height

## 10. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| No re-answer affordance on any target | a second answering surface would let the reader change an answer the agent has already acted on | accepted (by design) | none |

## 11. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces

## 12. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a selected answer
with three options; several chosen in a multi-select question; an override
answer; a declined question; `showOptions={false}`; a question with a header
and one without; full size ladder; density variants.

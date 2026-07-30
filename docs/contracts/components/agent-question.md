# AgentQuestion

Status: detailed contract
Updated: 2026-07-30

## 1. Purpose

- Component name: `AgentQuestion`
- Layer: `composites`
- Summary: the question an agent asks mid-turn — a prompt, a set of options, and
  the composer's own editor as a free-text override
- In scope: the prompt and option list, single- and multi-select, keyboard
  shortcuts, override semantics, sequential progress through a batch, optional
  dismissal, the resolved answer payload
- Out of scope: the text input and the submit control (both belong to
  `AgentChatInput`), the transcript record of an answered question (that is
  `AgentQuestionRecord`), transport, deciding *when* to ask

`AgentQuestion` is not a standalone surface. It renders inside
`AgentChatInput`'s field, above the editor, and the composer supplies the input
and the submit button.

That is the whole point. The free-text override *is* the composer's editor. Put
the question anywhere else — a transcript block, a popover — and there are two
text inputs on screen with different submit semantics: one sends a message, one
answers a question. No arrangement of those two is unconfusing, and coalescing
them is worse.

## 2. Blocking

A pending question blocks the **turn**, not the UI.

There is no scrim, no focus trap, and no `Dialog`. The reader can still scroll
the transcript back to decide, which is exactly what someone answering a
question about their own codebase needs to do. What blocks is the composer: it
refuses to send anything but an answer while a question is live.

A turn cannot complete until every question in the batch resolves. Dismissal is
a resolution (see §5), not an escape from one. Abandoning the whole turn stays
with the composer's existing stop action.

## 3. Anatomy

```text
[Root .agent-question] <div>  (carries data-size/data-density/data-multi-select)
  ├── [Progress .agent-question__progress] <div>  (conditional: more than one question)
  │   ├── [Progress Dot .agent-question__progress-dot] <span> (repeated; data-state="answered"|"current"|"pending")
  │   └── [Progress Label .agent-question__progress-label] <span>  ("2 of 4")
  ├── [Header .agent-question__header] Eyebrow  (conditional: header set)
  ├── [Prompt .agent-question__prompt] <p id="{id}-prompt">
  ├── [Options .agent-question__options] <div role="radiogroup"|"group" aria-labelledby="{id}-prompt">
  │   └── [Option .agent-question__option] <button type="button" role="radio"|"checkbox"> (repeated)
  │       ├── [Option Check .agent-question__option-check] Checkbox  (multi-select only; decorative)
  │       ├── [Option Label .agent-question__option-label] <span>
  │       ├── [Option Description .agent-question__option-description] <span>  (conditional)
  │       └── [Option Shortcut .agent-question__option-shortcut] <kbd>  (conditional: index < 9)
  └── [Dismiss .agent-question__dismiss] <button type="button">  (conditional: dismissible)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | stacks progress, header, prompt and options above the composer's editor | `--poodle-space-stack-sm` |
| Progress | no | position in the batch; present only when there is more than one question | `--poodle-color-text-tertiary` |
| Progress Dot | no | one per question, carrying `answered` / `current` / `pending` | `--poodle-color-accent-base`, `--poodle-color-border-subtle` |
| Header | no | the question's short label, as an `Eyebrow` | (Eyebrow contract) |
| Prompt | yes | the question itself, at full strength — it is the thing being asked | `--poodle-color-text-primary` |
| Options | yes | the option list; `radiogroup` when single-select, `group` when multi | `--poodle-space-stack-xs` |
| Option | yes | one option; the whole row is the hit target | `--poodle-color-background-elevated`, `--poodle-radius-control` |
| Option Check | no | a `Checkbox` shown only in multi-select, so the mode is visible before you click | (Checkbox contract) |
| Option Label | yes | the choice, at full strength | `--poodle-color-text-primary` |
| Option Description | no | why you would pick it | `--poodle-color-text-secondary` |
| Option Shortcut | no | the digit that selects it | `--poodle-color-text-tertiary` |
| Dismiss | no | resolves the current question as declined | `--poodle-color-text-secondary` |

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `questions` | `AgentQuestionItem[]` | `[]` | yes | answered in order; one is live at a time |
| `activeIndex` | `number` | `0` | no | which question is live; controlled when bound |
| `selections` | `string[]` | `[]` | no | selected option values for the live question |
| `override` | `string` | `""` | no | the composer's editor text, passed in so the component can resolve the answer |
| `dismissible` | `boolean` | `false` | no | renders the dismiss control |
| `dismissLabel` | `string` | `"Skip this question"` | no | dismiss control label |
| `progressLabel` | `(current: number, total: number) => string` | `` (c, t) => `${c} of ${t}` `` | no | batch progress wording |
| `showShortcuts` | `boolean` | `true` | no | digit hints on the first nine options |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onSelectionChange` | `((values: string[]) => void) \| null` | `null` | no | an option was picked or unpicked |
| `onSubmit` | `((answer: AgentQuestionAnswer) => void) \| null` | `null` | no | the live question resolved |
| `onDismiss` | `((id: string) => void) \| null` | `null` | no | the live question was declined |

### Shared Types

Defined in `@poodle/headless` (`agent-question.ts`), mirrored in
`poodle-headless::agent_question` (snake_case).

```typescript
type AgentQuestionOption = {
  value: string;
  label: string;
  description?: string;
};

type AgentQuestionItem = {
  id: string;
  /** Short label shown as an eyebrow above the prompt. */
  header?: string;
  prompt: string;
  options: AgentQuestionOption[];
  /** Opt-in. Single-select is the default because it can submit on one click. */
  allowMultiple?: boolean;
};

type AgentQuestionOutcome = "selected" | "override" | "declined";

type AgentQuestionAnswer = {
  questionId: string;
  outcome: AgentQuestionOutcome;
  /** Chosen option values. Empty for `override` and `declined`. */
  values: string[];
  /** The free-text answer. Empty unless `outcome` is `override`. */
  text: string;
};
```

### Computed Values

| Name | Formula |
|------|---------|
| `activeQuestion` | `questions[activeIndex] ?? null` |
| `isMultiSelect` | `activeQuestion?.allowMultiple === true` |
| `hasOverride` | `override.trim().length > 0` |
| `canSubmit` | `hasOverride \|\| selections.length > 0` |
| `answer` | `resolveQuestionAnswer(activeQuestion, selections, override)` |
| `isLastQuestion` | `activeIndex >= questions.length - 1` |

## 5. Answer Resolution

Three outcomes, in priority order:

1. **Override.** Non-empty editor text wins, and the answer carries the text
   with no values.
2. **Selected.** Otherwise the chosen option values.
3. **Declined.** Dismissal, carrying neither.

### Typing Clears The Selection

Entering override text clears any selections rather than disabling the option
list.

The alternative — locking the editor once an option is picked — traps the
reader: they tick a box, realise none of the options fit, and now have to untick
before they can type. Clearing is forgiving, and it keeps the state honest,
because what is on screen is exactly what will be sent. Clearing the text again
does not restore the old selections; there is only ever one answer in flight.

This rule barely matters in single-select, where a click submits immediately.
It is really a multi-select rule.

### Dismissal Is A Resolution

A turn cannot finish with an unanswered question, so dismissal has to send
something. It resolves the live question as `declined` and advances to the next
one. It does not abandon the batch or the turn — that is the composer's stop
action.

Without this, "dismiss" on question 2 of 4 has no defined meaning.

## 6. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| single-select | `allowMultiple` unset | no checkboxes; one click selects *and* submits |
| multi-select | `allowMultiple` | a `Checkbox` on every option, so the mode is visible before the first click; Submit is always explicit |
| selected | option in `selections` | option carries `data-selected="true"` |
| overridden | editor text present | selections cleared; options render unselected |
| batch | more than one question | progress dots and count above the header |
| dismissible | `dismissible` | the dismiss control renders |

### Why Multi-Select Costs Submit-On-Click

With one answer, the first click is also the last, so the question can resolve
on it and the reader never touches Submit. With several, a click cannot be
distinguished from a first-of-several, so Submit is always explicit.

That is a consequence of allowing multi-select, not a free addition, and it is
why the mode is opt-in per question rather than a component-wide setting: most
questions get the faster interaction.

### Progress Is Not Navigation

The dots report position. They are not controls, and there is no way back to an
answered question.

Going back would mean changing an answer the agent already has. `Stepper` is
deliberately *not* reused here for the same reason — its steps are triggers, it
is a full-width bordered track sized to be a wizard's primary navigation, and
suppressing its interaction and layout to fit inside a composer card would leave
nothing of it but the status vocabulary.

## 7. Events

| Event | Payload | When |
|-------|---------|------|
| `onSelectionChange` | option values | an option was picked or unpicked |
| `onSubmit` | `AgentQuestionAnswer` | single-select click, or the composer's submit |
| `onDismiss` | question id | the dismiss control was used |

## 8. Accessibility

### Semantics

- The option list is `radiogroup` in single-select and `group` in multi-select,
  labelled by the prompt through `aria-labelledby`.
- Options are `role="radio"` or `role="checkbox"` with `aria-checked`, so the
  mode is announced rather than inferred from the presence of a tick.
- The `Checkbox` in a multi-select option is decorative (`aria-hidden`); the
  option itself carries the state. Two announcements of one state is worse than
  one.
- The prompt is a real element referenced by the group, not a placeholder or a
  visually-positioned heading.
- Batch progress is announced through the progress label's text, not through the
  dots, which are `aria-hidden`. "2 of 4" is the fact; the dots are its picture.
- A live question does not steal focus. The composer's editor keeps it, because
  the override is the fastest path for a reader who already knows their answer.

### Keyboard

| Key | Action |
|-----|--------|
| `1`–`9` | selects the option at that position, when focus is not in the editor |
| `Tab` | moves through the options, then the dismiss control |
| `Enter` / `Space` on an option | selects it; submits too in single-select |
| arrow keys within the group | moves between options (radiogroup semantics) |
| `Escape` | nothing — a question is not dismissible by default, and where it is, the control is explicit |

`Escape` doing nothing is deliberate. The surface looks dismissible and is not;
silently discarding a question the turn is waiting on would strand the agent.

## 9. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| width | fills the composer's field |
| option gap | `--poodle-space-stack-xs` |
| block gap | `--poodle-space-stack-sm` between progress, header, prompt and options |

### Composition

Rendered inside `AgentChatInput`'s field, above the attachments and editor. The
composer owns the editor, the toolbar and the submit control; see
`agent-chat-input.md` §Question Region.

## 10. Token Usage

| Property | Token |
|----------|-------|
| prompt colour | `--poodle-color-text-primary` |
| option label | `--poodle-color-text-primary` |
| option description | `--poodle-color-text-secondary` |
| option fill | `--poodle-color-background-elevated` |
| option fill, selected | `--poodle-color-accent-base` at 10% |
| option border, selected | `--poodle-color-accent-base` |
| option radius | `--poodle-radius-control` |
| shortcut colour | `--poodle-color-text-tertiary` |
| progress dot, answered | `--poodle-color-accent-base` |
| progress dot, pending | `--poodle-color-border-subtle` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-size` / `data-density` | the ladders | root |
| `data-multi-select` | `true`/`false` | root |
| `data-selected` | `true`/`false` | each option |
| `data-state` | `answered`/`current`/`pending` | each progress dot |

## 11. Svelte Notes

- `answer` is `$derived` from the live question, selections and override; there
  is no stored answer to fall out of step.
- Digit shortcuts are bound on the root and ignored while focus is in the
  composer's editor, so typing "1" into an override does not select an option.

## 12. GPUI And Jetstream Notes

- Render the state the spec describes; selection and submission are host-driven,
  matching the render-only posture of every other native component.
- No digit shortcuts: key handling is host-event-loop work on both natives.

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] override text wins over selections, and clears them
- [ ] clearing the override does not restore cleared selections
- [ ] single-select resolves on one click; multi-select never does
- [ ] multi-select shows checkboxes, single-select shows none
- [ ] dismissal resolves as `declined` and advances
- [ ] progress renders only when there is more than one question
- [ ] progress is inert — no navigation back to an answered question
- [ ] the answer payload carries exactly one of values or text
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] selected options carry the accent fill and border
- [ ] the prompt is at full strength and descriptions one step down
- [ ] shortcuts render on the first nine options only
- [ ] option gap, block gap and radius match per size and density
- [ ] density never changes option height

### Tier 3: Implementation Freedom

- [ ] keyboard shortcut binding is platform-owned
- [ ] transition timing is platform-owned

## 14. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Natives do not handle digit shortcuts | key handling is host-event-loop work, as with every native control | accepted | host wires keys |
| Natives render selection state without driving it | shared render-only posture | accepted | host drives selection |
| Progress is a dot row rather than `Stepper` | `Stepper` is an interactive full-width wizard track; only its status vocabulary transfers | accepted (by design) | revisit if a second consumer needs richer batch progress |

## 15. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: answering several questions at once rather than in sequence,
  richer option content, per-option icons

## 16. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a single-select
question with three options; the same with descriptions; multi-select showing
checkboxes; a selection made; an override typed over a selection, showing the
selection cleared; a batch of four showing progress at the second; the last
question of a batch; dismissible; not dismissible; a question with more than
nine options, showing shortcuts on the first nine only; full size ladder;
density variants.

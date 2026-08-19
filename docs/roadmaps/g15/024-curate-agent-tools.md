# g15.024 — Overloaded Examples: agent and tools

Status: **paused in PR #48 review** — paired-web corrections are sound, but the
GPUI AgentTranscript page exposed a real detached-scroll/jump capability gap;
resume only after `g15.037`
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Consumes: `g15.011` partial screening baseline
Depends on: `g15.015` and `g15.023` (complete), `g15.037` (planned prerequisite)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`, and
the six component contracts named below

## Scope

Agent surfaces whose useful stories are obscured by markdown, state, or edge-
case lists.

Catalogue family: `agent-tools`.

### Pages this card owns (6)

- `AgentChatInput`
- `AgentMessage`
- `AgentQuestion`
- `AgentQuestionRecord`
- `AgentTranscript`
- `ChangedFiles`

This list is exact and exhaustive. No other card owns these pages, and this
card owns no others. No component behavior, contract, or public API change is
authorised.

## Remeasured Baseline

Counts are visible `Examples` captions on current `main` after `g15.023`.
Dedicated size and density panes are excluded.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| AgentChatInput | 9 | 9 | 8 | combine state and submission matrices; converge embedded flows |
| AgentMessage | 8 | 8 | 8 | group markdown features by reader task |
| AgentQuestion | 6 | 6 | 3 | preserve hosted use; converge optional modes |
| AgentQuestionRecord | 6 | 6 | 5 | combine answer and presentation variants |
| AgentTranscript | 6 | 4 | 5 | restore the worked turn and streaming to React; converge run teaching |
| ChangedFiles | 7 | 7 | 6 | combine count/action variants; remove the blank empty example |

The prerequisite caption repair is complete. The mismatched counts above are
real runtime specimen drift, not a measurement artefact.

## Target Teaching Outline

Use these sections in this order. Svelte and React captions and explanatory
copy stay verbatim. GPUI teaches the same ordered intent. A section may contain
several component instances when they answer one reader question.

### AgentChatInput (6)

1. `Default composer` — composing, model picker, submit feedback, and context
   below the warning threshold.
2. `Questions and plans` — embedded `AgentQuestion` and `AgentPlan` states.
3. `Busy and unavailable` — busy/high-context, read-only, and disabled.
4. `Attachments and footer` — image/file attachments, removal, and footer
   content.
5. `Submission rules` — empty disabled, `allowEmptySubmit`, no context ring,
   and `submitOnEnter=false` without toolbar dividers.
6. `Editor growth` — the `maxRows` ceiling.

### AgentMessage (6)

1. `Assistant and user messages` — plain assistant copy, user role, and the
   long prose measure.
2. `Inline formatting and headings` — code, emphasis, link, strikethrough, and
   every heading level.
3. `Code blocks` — annotated and unannotated fences.
4. `List structures` — tight, loose, ordered-offset, nested, and fenced-item
   lists.
5. `Quotes, rules and fallback` — blockquote, thematic break, and unsupported
   markdown degrading to text.
6. `Streaming` — the aria-hidden progress caret.

### AgentQuestion (5)

1. `Hosted by the composer` — the primary arrangement, including selection
   cleared by a typed override.
2. `Choice modes` — single and multiple selection, descriptions, and a made
   selection.
3. `Batch progress` — the second and final questions in a batch.
4. `Dismissal` — dismissible and non-dismissible questions.
5. `Shortcut limits` — more than nine options and the shortcut-free posture.

### AgentQuestionRecord (4)

1. `Selected answers` — one and several selected values.
2. `Free-text override`.
3. `Declined`.
4. `Presentation options` — `showOptions=false`, with-header, and no-header.

### AgentTranscript (5)

1. `A worked turn` — the full realistic message/run/changed-files/activity
   sequence.
2. `Tool run states` — a simple message and run, thirty calls collapsed and
   expanded, and a failure.
3. `Streaming and detached scroll` — streaming content and jump-to-latest.
4. `Long transcript rendering` — the same mixed-height content windowed and
   unwindowed.
5. `Empty`.

### ChangedFiles (4)

1. `Worked change set` — the nine-file example collapsed and expanded.
2. `Paths and scopes` — deep chain collapse and files across several scopes.
3. `Count variations` — single-file, additions-only, and deletions-only.
4. `Overflow and actions` — long-name truncation, `chipLimit`, and
   `showOpenDiff=false`.

The empty `ChangedFiles` case leaves Examples because the component correctly
renders nothing. Its focused tests retain the no-output contract; a blank
teaching surface is not useful.

## Evidence Rules

- Add `test/parity/g15-024-agent-tools-specimens.test.tsx` to pin the exact
  caption order, section budget, paired-web equality, and representative live
  behavior.
- Name every removed or combined story in the August batch log and point to
  its retained section or focused component evidence.
- Preserve all behavior required by each contract's specimen-definition
  section. Several required cases may share one captioned section.
- Keep interactive feedback visible: composer submit/stop/removal, question
  selection/override, transcript disclosure/scroll, and changed-file
  disclosure/action states must not become inert pictures.
- `g15.026` owns mounted native page probing. This card uses deterministic
  GPUI source assertions and existing renderer tests; it does not build a new
  harness.

## Writable Scope

- `packages/svelte/preview/src/specimens/{AgentChatInput,AgentMessage,AgentQuestion,AgentQuestionRecord,AgentTranscript,ChangedFiles}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{AgentChatInput,AgentMessage,AgentQuestion,AgentQuestionRecord,AgentTranscript,ChangedFiles}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{agent_chat_input_specimen,agent_message,agent_question,agent_question_record,agent_transcript,changed_files}.rs`
- `test/parity/g15-024-agent-tools-specimens.test.tsx`
- one August batch log

Do not edit component implementations, contracts, shared specimen shells,
catalogue generation/navigation, scenes, another card's pages, Jetstream, or
release surfaces. Stop if the target outline requires any of those changes.

## Acceptance

- [ ] The six pages use the exact ordered teaching outline above.
- [ ] Svelte and React captions/copy match; GPUI teaches the same intent.
- [ ] Every contract-required specimen behavior remains visible inside the
      target sections or has named focused evidence.
- [ ] Every first section is a realistic normal use, not an edge case.
- [ ] No page exceeds six captioned Examples sections.
- [ ] Removed and combined examples have named dispositions in the batch log.
- [ ] The changed Svelte and React pages receive operator live review before
      the card is called complete.

## Validation

- focused `g15.024` parity regression
- `effigy test:parity`
- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, conformance, Jetstream, or
release selectors.

## Stop Conditions

- A target section cannot preserve a required contract behavior without a
  component or contract change.
- Runtime capability differs enough that the ordered teaching intent would be
  dishonest.
- Curation becomes an exhaustive reference view or grows outside these six
  pages.
- Another active lane touches a writable specimen file.

## Review Pause — 2026-08-19

PR #48 correctly curated the six pages and repaired paired-web hosted-question
reset drift. Review found that GPUI cannot honestly render the target
`Streaming and detached scroll` story: the shared native renderer emits no
tracked viewport or jump control. A follow-up attempt added a ScrollShell and a
button that only incremented a counter; it did not change scroll state and was
rejected as fabricated evidence.

`g15.037` now owns the missing native capability. PR #48 must remove the
counter-button workaround, pause, then rebase after that prerequisite lands and
bind the specimen to the real behavior. No roadmap wording change turns the
absence into parity.

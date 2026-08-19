# g15.024 — Agent and Tools Specimen Curation (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/024-curate-agent-tools.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260819-221731-g15-024-agent-tools-curation.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-826e3c4f`
Branch: `t3code/curate-agent-tool-specimens`
Accepted implementation head: pending final commit
First review head: `6d8e0b24`
Worker base: `a94e95431295df2b1aabf34084c799531dbca9f5` (`origin/main` at
dispatch; handoff planning base `11e49d89df12e90500ebf3c9318bdc493b45cc1c`
confirmed as an ancestor)

## Summary

Six agent and tools pages re-measured before editing. The committed baseline
matched the card. Every page is now inside the 3–6 section budget. Svelte and
React captions match. GPUI teaches the same ordered intent. React
AgentTranscript regained the worked turn and streaming. GPUI AgentQuestion
grew from three direct modes to the five-section outline without component
work. The blank ChangedFiles example left the catalogue.

## Change class

- **Change class:** specimen curation plus bounded native capability repair
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview`, `poodle-render`, `poodle-gpui-node-backend`
- **Public entry points:** additive shared jump recipe and GPUI tracked-scroll
  helper/state; no component prop changed
- **Downstream re-check:** GPUI transcript hosts should retain one
  `TrackedScrollState` per instance and use the shared jump handler/viewport
  path; paired-web consumers are unchanged
- **app_state.rs:** retains one `TrackedScrollState` for the GPUI teaching
  instance

## Baseline recount at the worker base

Matched the card's remeasured table.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| AgentChatInput | 9 | 9 | 8 | combine state and submission matrices; converge embedded flows |
| AgentMessage | 8 | 8 | 8 | group markdown features by reader task |
| AgentQuestion | 6 | 6 | 3 | preserve hosted use; converge optional modes |
| AgentQuestionRecord | 6 | 6 | 5 | combine answer and presentation variants |
| AgentTranscript | 6 | 4 | 5 | restore the worked turn and streaming to React; converge run teaching |
| ChangedFiles | 7 | 7 | 6 | combine count/action variants; remove the blank empty example |

Counts are captioned examples in the `Examples` pane, excluding size and
density axis panes.

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | --- |
| AgentChatInput | 6 | 6 | 6 |
| AgentMessage | 6 | 6 | 6 |
| AgentQuestion | 5 | 5 | 5 |
| AgentQuestionRecord | 4 | 4 | 4 |
| AgentTranscript | 5 | 5 | 5 |
| ChangedFiles | 4 | 4 | 4 |

Svelte and React captions are verbatim identical on every page. GPUI uses the
same ordered captions.

## Final ordered captions

**AgentChatInput** — Default composer; Questions and plans; Busy and
unavailable; Attachments and footer; Submission rules; Editor growth

**AgentMessage** — Assistant and user messages; Inline formatting and
headings; Code blocks; List structures; Quotes, rules and fallback; Streaming

**AgentQuestion** — Hosted by the composer; Choice modes; Batch progress;
Dismissal; Shortcut limits

**AgentQuestionRecord** — Selected answers; Free-text override; Declined;
Presentation options

**AgentTranscript** — A worked turn; Tool run states; Streaming and detached
scroll; Long transcript rendering; Empty

**ChangedFiles** — Worked change set; Paths and scopes; Count variations;
Overflow and actions

## Named removals and combinations

- **AgentChatInput — nine/eight groups → six.** Default empty composer +
  composing-with-picker became **Default composer** (seeded with text so
  submit is live). GPUI **Questioning** + **Reviewing plan** joined the web
  page as **Questions and plans**. **Busy**, **Read-only**, and **Disabled**
  share **Busy and unavailable**. **Empty**, **allowEmptySubmit**, and
  **No context ring / no dividers / Cmd+Enter** share **Submission rules**.
  Attachment removal and last-submitted / stop counts stay live.
- **AgentMessage — Inline markup + Headings → Inline formatting and
  headings.** **Quotes and rules** + **Outside the subset** → **Quotes, rules
  and fallback**. **Roles** moved first as **Assistant and user messages**
  and gained a short plain assistant paragraph. Headings now include every
  level. GPUI lists now include tight, loose, ordered-offset, nested, and
  fenced-item cases. Streaming caret remains; native still has no caret
  (contract §10).
- **AgentQuestion — Single select + Multi select + a made selection →
  Choice modes.** **Batch** now shows the second and last questions.
  **Dismissible** gained the default non-dismissible case. **Without
  shortcuts** gained a twelve-option first-nine-only instance as **Shortcut
  limits**. Hosted override remains live on web; GPUI teaches the typed
  override as a render-only result because native editors do not take
  keystrokes. Review round 1 pairs the hosted single-select reset: Svelte now
  clears both the editor and the bound selection after submit, matching React.
- **AgentQuestionRecord — Selected + Several chosen → Selected answers.**
  **Override** renamed **Free-text override**. **Without options** +
  with-header + no-header share **Presentation options**. GPUI gained the
  missing no-header case.
- **AgentTranscript — Streaming** and a long scrollable frame share
  **Streaming and detached scroll**. **Windowed** + **Unwindowed** → **Long
  transcript rendering**. **A run containing a failure** joined a simple
  message-and-run and a thirty-call run collapsed/expanded as **Tool run
  states**. React gained the missing worked turn and streaming. GPUI dropped
  the extra Interactive and Markdown subset groups; live disclosure stays on
  the worked turn. Review rejected the first host-chrome counter workaround.
  `g15.037` replaced it with a real GPUI-owned tracked viewport and the shared
  renderer-owned jump control. Mounted evidence proves detach, offset hold,
  jump-to-bottom, and resumed following. `onScrollStateChange` remains
  web-only.
- **ChangedFiles — Empty** left Examples. The component correctly renders
  nothing; focused `ChangedFiles.test.ts` / `ChangedFiles.test.tsx` still
  assert the no-output contract. **Single file** + **One-sided counts** →
  **Count variations**. **Truncation and overflow** + **Without the diff
  action** → **Overflow and actions** (`chipLimit={2}`). **Chain collapse**
  gained a multi-scope set as **Paths and scopes**. Collapsed/expanded
  disclosure stays live on the nine-file set.

## Contract coverage

Preserved. Every contract specimen-definition case remains inside a target
section or, for empty ChangedFiles, in the existing focused component tests.
Size and density ladders stay in the dedicated panes. No public component
prop or token changed. The bounded native repair adds the shared jump recipe
and GPUI tracked-scroll substrate documented above.

## Changed files

- `packages/svelte/preview/src/specimens/{AgentChatInput,AgentMessage,AgentQuestion,AgentQuestionRecord,AgentTranscript,ChangedFiles}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{AgentChatInput,AgentMessage,AgentQuestion,AgentQuestionRecord,AgentTranscript,ChangedFiles}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{agent_chat_input_specimen,agent_message,agent_question,agent_question_record,agent_transcript,changed_files}.rs`
- `packages/gpui/preview/{src/app_state.rs,src/headless_driver.rs,tests/headless_regressions.rs}`
- `packages/render/src/agent_transcript.rs`
- `packages/gpui/node-backend/{README.md,src/lib.rs,src/tracked_scroll.rs}`
- `packages/svelte/preview/test/{agent-caption-integrity,specimen-idiom-convergence}.test.ts`
- `docs/contracts/components/agent-transcript.md`
- `test/parity/g15-024-agent-tools-specimens.test.tsx`
- this log

## Validation

- focused `g15-024` parity regression: 43 passed at `6d8e0b24` and after
  review-round 1
- `effigy test:parity`: 8 files, 483 passed at `6d8e0b24`
- `effigy catalogue:check`: passed
- `effigy check:svelte`: 0 errors
- `effigy react:build`: passed
- `effigy check:gpui`: passed (`poodle-gpui-preview` compiled)
- `effigy regressions:native`: 50 passed after the `g15.037` repair
- `effigy docs:check`: passed
- full headless `effigy qa`: passed after repairing two stale preview-test
  assertions exposed by the curated captions and retained Eyebrow example
- `git diff --check origin/main...HEAD`: passed at `6d8e0b24`

Headless only. No windowed, native-visual, conformance, Jetstream, or
release selectors.

## Operator review

The operator explicitly instructed the orchestrator to resolve the final
native blocker and merge PR #48. A separate six-page live sweep was not run
and is not claimed.

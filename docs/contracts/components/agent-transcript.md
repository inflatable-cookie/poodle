# AgentTranscript

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `AgentTranscript`
- Layer: `composites`
- Summary: the output surface for an AI agent conversation — an append-only
  sequence of messages, tool-call runs and changed-file summaries in a
  bottom-anchored scroll region
- In scope: the transcript item model, contiguous tool-run grouping, windowed
  rendering of variable-height blocks, bottom anchoring with a jump-to-latest
  affordance, the live activity footer, empty state
- Out of scope: the composer (`AgentChatInput`), transport and token streaming,
  model invocation, diff viewing, message editing or retry, threading and
  branching, persistence, vendor vocabulary

`AgentChatInput` owns the composer and explicitly scopes out the transcript.
This is the other side of that boundary: the two compose into a chat surface,
and neither knows about the other.

The host feeds a flat, append-only list of items. Poodle decides how they group
and which of them are worth rendering. The host does not pre-group, because the
events arrive flat from an agent stream and every consumer would otherwise write
the same adjacency scan.

## 2. Anatomy

```text
[Root .agent-transcript] <div>  (carries data-size/data-density/data-empty)
  ├── [Viewport .agent-transcript__viewport] <div role="log" aria-live="polite">
  │   ├── [Runway .agent-transcript__runway] <div>  (conditional: virtualized; holds total scroll height)
  │   │   └── [Slice .agent-transcript__slice] <div>  (translated to the window's offset)
  │   │       └── [Block .agent-transcript__block] <div>  (repeated; data-kind)
  │   │           ├── AgentMessage         (data-kind="message")
  │   │           ├── ToolCallGroup        (data-kind="tool-run")
  │   │           ├── ChangedFiles         (data-kind="changed-files")
  │   │           ├── AgentQuestionRecord  (data-kind="answered-question")
  │   │           └── AgentPlanRecord      (data-kind="decided-plan")
  │   ├── [Activity .agent-transcript__activity] <div>  (conditional: an activity item is present)
  │   │   ├── [Activity Spinner .agent-transcript__activity-spinner] Spinner (variant="dots")  (omitted when the activity item sets spinning: false)
  │   │   └── [Activity Label .agent-transcript__activity-label] <span>
  │   └── [Empty .agent-transcript__empty] EmptyState  (conditional: no items)
  └── [Jump .agent-transcript__jump] <button type="button">  (conditional: not pinned to bottom)
      ├── [Jump Icon] Icon (icon="arrow-down")
      └── [Jump Label] <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | positioning context for the jump button; carries the presentation axes | — |
| Viewport | yes | the scroll container; `role="log"` so appended output is announced without stealing focus | `--poodle-space-stack-md` |
| Runway | no | full-height spacer that gives the scrollbar its range while only a window of blocks exists | — |
| Slice | no | the rendered window, offset to sit where its blocks belong | — |
| Block | yes | one grouped block; `data-kind` carries which | `--poodle-space-stack-lg` (between blocks) |
| Activity | no | the live footer — spinner plus "Working for 1h 1m"; terminal states ("Turn cancelled") reuse the strip with `spinning: false`, which omits the spinner so nothing signals ongoing work | `--poodle-color-text-secondary` |
| Empty | no | `EmptyState` shown when there are no items at all | (EmptyState contract) |
| Jump | no | returns the reader to the bottom and re-arms following | `--poodle-color-background-elevated`, `--poodle-radius-pill`, `--poodle-elevation-overlay` |

Runway and Slice exist only when `virtualized` is true. Unwindowed, blocks are
children of the viewport directly — the extra wrappers would change nothing and
their absence keeps the DOM honest about what is happening.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `TranscriptItem[]` | `[]` | yes | flat, append-only; Poodle groups it |
| `virtualized` | `boolean` | `true` | no | when false every block renders; correctness is identical, only cost differs |
| `estimatedBlockHeight` | `number` | `120` | no | height assumed for unmeasured blocks, in px |
| `overscan` | `number` | `3` | no | blocks rendered beyond each edge of the viewport |
| `autoScroll` | `boolean` | `true` | no | follow new output while the reader is at the bottom |
| `pinThreshold` | `number` | `32` | no | px of slack that still counts as "at the bottom" |
| `jumpLabel` | `string` | `"Jump to latest"` | no | accessible name and label for the jump button |
| `ariaLabel` | `string` | `"Conversation"` | no | accessible name for the log region |
| `emptyLabel` | `string` | `"No messages yet"` | no | empty-state title |
| `expandedToolRuns` | `string[]` | `[]` | no | run ids currently expanded; controlled when bound |
| `expandedToolCalls` | `string[]` | `[]` | no | call ids whose output is open; controlled when bound |
| `expandedChangedFiles` | `string[]` | `[]` | no | changed-files ids showing their tree |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToolRunToggle` | `((id: string) => void) \| null` | `null` | no | a run's "+N previous tool calls" was used |
| `onToolCallToggle` | `((id: string) => void) \| null` | `null` | no | a call's output was opened or closed |
| `onChangedFilesToggle` | `((id: string) => void) \| null` | `null` | no | a changed-files card was opened or closed |
| `onOpenDiff` | `((id: string) => void) \| null` | `null` | no | "Open diff" was used; the host owns what that means |
| `onFileSelect` | `((path: string) => void) \| null` | `null` | no | a file in the tree was chosen |
| `onScrollStateChange` | `((pinned: boolean) => void) \| null` | `null` | no | fires when following starts or stops |

### Slots / Children

| Slot | Renderer form | Purpose |
|------|---------------|---------|
| `blockOverride` | Svelte `Snippet`, React `ReactNode` | replaces the rendering of a block by kind, for hosts with a bespoke item type |
| `empty` | Svelte `Snippet`, React `ReactNode` | replaces the default `EmptyState` |

### Naming Rules

Follows Poodle conventions: `camelCase` multi-word props, `on*` handlers,
`size`/`density`/`sizeRole` for the shared presentation axes. The Rust spec
keeps `is_virtualized` / `is_auto_scroll`.

### Shared Types

Defined in `@inflatable-cookie/poodle-core` (`agent-transcript.ts`), re-exported from the
component packages, mirrored in `poodle-headless::agent_transcript` (snake_case).

```typescript
type TranscriptRole = "user" | "assistant";
type ToolCallStatus = "running" | "success" | "error";

type TranscriptMessage = {
  kind: "message";
  id: string;
  role: TranscriptRole;
  /** Raw markdown. Parsed by the renderer, never pre-rendered by the host. */
  markdown: string;
  isStreaming?: boolean;
};

type TranscriptToolCall = {
  kind: "tool-call";
  id: string;
  /** What kind of work this was — "Ran command", "File change". */
  label: string;
  /** The argument line, truncated to one line when collapsed. */
  detail?: string;
  status: ToolCallStatus;
  icon?: string;
  output?: string;
};

type ChangedFile = {
  path: string;
  additions: number;
  deletions: number;
  status?: "added" | "modified" | "deleted" | "renamed";
};

type TranscriptChangedFiles = { kind: "changed-files"; id: string; files: ChangedFile[] };
type TranscriptActivity = { kind: "activity"; id: string; label: string; spinning?: boolean };

// The records live items leave behind; both render read-only. See
// agent-question-record.md and agent-plan-record.md.
type TranscriptAnsweredQuestion = {
  kind: "answered-question";
  id: string;
  question: AgentQuestionItem;
  answer: AgentQuestionAnswer;
};
type TranscriptDecidedPlan = {
  kind: "decided-plan";
  id: string;
  /** Raw markdown of the plan that was decided. */
  plan: string;
  status: "accepted" | "revised" | "dismissed";
  decidedAt?: string;
};

type TranscriptItem =
  | TranscriptMessage
  | TranscriptToolCall
  | TranscriptChangedFiles
  | TranscriptAnsweredQuestion
  | TranscriptDecidedPlan
  | TranscriptActivity;

/** What grouping produces. */
type TranscriptToolRun = { kind: "tool-run"; id: string; calls: TranscriptToolCall[] };
type TranscriptBlock =
  | TranscriptMessage
  | TranscriptToolRun
  | TranscriptChangedFiles
  | TranscriptAnsweredQuestion
  | TranscriptDecidedPlan
  | TranscriptActivity;
```

### Controlled And Uncontrolled

- Controlled: bind `expandedToolRuns` / `expandedToolCalls` /
  `expandedChangedFiles`; every toggle is mirrored through the matching handler
- Uncontrolled: omit them and the component owns expansion state
- `items` is always host-owned; the transcript never mutates it

### Computed Values

| Name | Formula |
|------|---------|
| `blocks` | `groupTranscriptItems(items)` |
| `isEmpty` | `items.length === 0` |
| `isPinned` | `isPinnedToBottom(scrollTop, scrollHeight, clientHeight, pinThreshold)` |
| `window` | `transcriptWindow(measuredHeights, estimatedBlockHeight, scrollTop, viewportHeight, overscan)` |
| `showsJump` | `!isPinned && !isEmpty` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no items | `EmptyState` centred in the viewport; `data-empty="true"` |
| following | pinned to bottom | new blocks scroll into view; no jump button |
| detached | reader scrolled up | scroll position holds as blocks append; jump button appears |
| live | an activity item is present | dots and elapsed label pinned under the last block |
| streaming | last message `isStreaming` | that message shows its caret (see `agent-message.md`) |

### Component States

- `virtualized` — a window of blocks renders; the runway carries total height
- `unwindowed` — every block renders; used for short transcripts and for tests
  that need the whole DOM

### Behavior Machine

Grouping, windowing and the anchoring predicate live in `@inflatable-cookie/poodle-core`
`agent-transcript.ts`, mirrored in `poodle-headless::agent_transcript`, with
shared vectors in `packages/contracts/headless/vectors/agent-transcript.json`
run by both runtimes.

#### Grouping

Contiguous tool calls collapse into one run. Adjacency is the whole rule:
anything that is not a tool call ends the run. A changed-files card between two
commands therefore splits them into two runs, which is what the transcript
should say — those commands happened either side of an edit, not as one stretch
of work.

Grouping is a pure function of the whole list rather than an incremental
accumulator, so a run that is still growing regroups correctly on every append
and there is no partial state to get out of step.

#### Bottom Anchoring

Anchoring is a latch, not a computation. Once the reader scrolls up, the
transcript must stop dragging them back down; it re-arms only when they return
to the bottom themselves, or use the jump button.

`pinThreshold` exists because "at the bottom" is never exact — subpixel scroll
positions, and a block growing by a line while the reader sits at the end, both
leave a few pixels of slack that should still count as following.

Following is implemented by scrolling after the append is laid out, not by
holding `scrollTop` at `scrollHeight` continuously. The distinction matters
while a message streams: its block grows every frame, and a continuous clamp
fights the reader's own wheel events instead of losing to them.

#### Windowing

`transcriptWindow` walks measured heights rather than dividing by a uniform row
height, because a transcript has none — a one-line message, a forty-row tool run
and a file tree differ by an order of magnitude.

Unmeasured blocks use `estimatedBlockHeight`, which makes total height an
estimate that changes as blocks are measured. That is unavoidable for variable
heights and is why the scrollbar settles rather than being right immediately;
the alternative is measuring every block up front, which defeats the point.

Overscan is applied after the scan, never during it, so the window's offset
cannot fall out of step with the index it describes. An offset that disagrees
with its index renders every block a fixed distance from where it belongs, which
reads as a scroll bug rather than a spacer one.

Measurement observes the rendered blocks, and re-measuring changes what is
rendered. Implementations must not let a measurement pass re-enter itself; see
§9.

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `onToolRunToggle` | run id | "+N previous tool calls" or "Show fewer tool calls" used |
| `onToolCallToggle` | call id | a call's output opened or closed |
| `onChangedFilesToggle` | changed-files id | the file tree opened or closed |
| `onOpenDiff` | changed-files id | "Open diff" used |
| `onFileSelect` | file path | a file row chosen in the tree |
| `onScrollStateChange` | `pinned: boolean` | following started or stopped |

## 6. Accessibility

### Semantics

- Viewport is `role="log"` with `aria-live="polite"` and `aria-label` from
  `ariaLabel`. `log` is the role for append-only output, and `polite` means a
  finished response is announced without interrupting whatever the reader is
  doing.
- `aria-live` is **not** applied to a streaming message's own body. A token-level
  live region would announce a partial sentence on every frame, which is worse
  than silence; the message is announced when streaming ends and its text is
  final.
- The jump button is a real `<button>` with an accessible name from `jumpLabel`.
- Windowing removes blocks from the DOM. That is a real accessibility
  consequence: a screen reader cannot navigate to a block that is not rendered.
  Hosts needing the full transcript available to assistive technology should set
  `virtualized={false}`, and the contract records this rather than pretending
  windowing is free.

### Keyboard

| Key | Action |
|-----|--------|
| `Tab` | moves through the interactive controls in the rendered window |
| `Space` / `PageDown` | scrolls the viewport (native) |
| `End` | scrolls to the latest block and re-arms following |
| `Enter` / `Space` on the jump button | scrolls to latest and re-arms following |

### Focus And Announcement

- The transcript never takes focus on append; output arriving must not move the
  reader's caret out of the composer.
- Expanding a tool run keeps focus on the toggle that did it, and the toggle's
  `aria-expanded` flips.

## 7. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| height | the host's; the transcript fills its container and scrolls internally |
| block gap | `--poodle-space-stack-lg`, from the density ladder |
| max prose width | `--poodle-agent-message-measure` on message bodies only, so tool rows and file trees still use the full width |

### Composition

Composes with `AgentChatInput` below it in a chat surface. The transcript owns
its own scrolling; the composer never scrolls with it.

## 8. Token Usage

| Property | Token |
|----------|-------|
| viewport padding | `--poodle-space-inline-lg` |
| block gap | `--poodle-space-stack-lg` |
| activity label | `--poodle-color-text-secondary` |
| activity font size | `--poodle-typography-label-size` |
| jump background | `--poodle-color-background-elevated` |
| jump border | `--poodle-color-border-subtle` |
| jump radius | `--poodle-radius-pill` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |
| jump shadow | `--poodle-elevation-overlay` |

### Size Variants

Size controls the block gap floor and the activity row's type scale. Density
controls viewport inset and the gap between blocks. Density never changes a
block's own height.

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-size` | `xs`…`xl` | root |
| `data-density` | `compact`/`default`/`comfortable` | root |
| `data-empty` | `true` | root, when there are no items |
| `data-pinned` | `true`/`false` | root |
| `data-virtualized` | `true`/`false` | root |
| `data-kind` | `message`/`tool-run`/`changed-files`/`answered-question`/`decided-plan` | each block |

## 9. Svelte Notes

- Grouping is derived, not stored: `$derived(groupTranscriptItems(items))`. A
  stored copy would drift from `items` on append.
- Block heights are measured with a `ResizeObserver` over the rendered blocks.
  The observer watches what the window renders, and choosing a window changes
  what is rendered — so a measurement pass must guard against re-entering
  itself, and the guard must be released a frame later rather than
  synchronously, because the observer fires asynchronously and a synchronous
  release lets the resulting notification straight back in. `Tabs` hit exactly
  this shape in its shed ladder; see `tabs.md` §Graded Overflow.
- Following is applied in an effect after the DOM updates, so the scroll lands
  on the laid-out height rather than the previous one.

## 10. GPUI Notes

- Windowing uses GPUI's own variable-height list rather than the shared
  `transcriptWindow`, which assumes a scroll container the component owns. The
  grouping and status logic are shared unchanged.
- Markdown renders from `poodle-markdown` blocks, not from HTML.
- `AgentTranscript::from_spec(spec, theme)` forwards the same four events as
  Jetstream — `.on_tool_run_toggle(...)`, `.on_tool_call_toggle(...)`,
  `.on_changed_files_toggle(...)`, `.on_file_select(...)` — into whichever
  block raises them. A host attaches at the transcript, not the blocks.

## 10a. Jetstream Notes

- `AgentTranscript::from_spec(spec, theme)` then `.on_tool_run_toggle(...)`,
  `.on_tool_call_toggle(...)`, `.on_changed_files_toggle(...)`,
  `.on_file_select(...)`.
- The transcript forwards handlers into the block that raises each event rather
  than re-deriving which block was clicked. It is the only level that sees every
  block, and the host holds all the expansion state, so this is where a host
  attaches.
- `onScrollStateChange` and `onOpenDiff` are web-only. The first belongs to a
  scroll container the component does not own here; the second has no native
  affordance to fire it.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] contiguous tool calls group into runs; any non-tool item splits them
- [ ] a run's id is its first call's id, stable across appends
- [ ] collapsed runs show the newest call, with the rest behind "+N previous"
- [ ] one failed call anywhere makes the whole run read as failed
- [ ] changed-files totals sum additions and deletions across all files
- [ ] following stops when the reader scrolls up and re-arms only at the bottom
- [ ] the jump button appears exactly when not pinned and not empty
- [ ] expansion state is controlled when bound and component-owned otherwise
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] block gap, viewport inset and prose measure match per size and density
- [ ] the activity row renders the dots spinner plus label in the secondary text colour
- [ ] the jump button is a pill on the elevated surface with the overlay shadow
- [ ] the empty state is centred in the viewport
- [ ] density never changes a block's own height

### Tier 3: Implementation Freedom

- [ ] measurement technique is platform-owned
- [ ] windowing implementation is platform-owned, so long as the rendered result
      matches the unwindowed one
- [ ] scroll physics and transition timing are platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Windowed blocks are absent from the accessibility tree | inherent to virtualization on every platform; `virtualized={false}` is the documented escape | accepted | revisit if a platform gains a virtualized-a11y API |
| Jetstream renders unwindowed | `jetstream-ui` materializes every child of a scroll container; windowing needs engine support | accepted, tracked | engine work scoped separately |
| GPUI uses its own list rather than `transcriptWindow` | GPUI owns its scroll container and already has variable-height list machinery | accepted (by design) | none |
| Natives do not stream | neither native re-renders per token during spec resolution; unrelated to interaction, which GPUI does support | accepted | host drives re-render |

## 13. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: message retry and edit affordances, branching, per-message
  copy actions — all deliberately out of v1

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): empty; a single
message; a message plus one tool run; the full worked turn (message, run,
message, run, changed files, run, activity); a run of thirty calls collapsed;
the same run expanded; a run containing a failure; a streaming message; the
activity footer; detached scroll showing the jump button; a long transcript
demonstrating windowing; `virtualized={false}` rendering the same content; full
size ladder; density variants.

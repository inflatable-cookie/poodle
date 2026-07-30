# ToolCallGroup

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `ToolCallGroup`
- Layer: `composites`
- Summary: a contiguous run of tool calls presented as one unit — the newest
  call visible, the rest behind a count that expands
- In scope: collapsed and expanded presentation, the toggle and its wording,
  run-level status, ordering
- Out of scope: deciding what belongs in a run (`AgentTranscript` owns
  grouping), the rows themselves (`ToolCall`), running anything

A turn can contain dozens of tool calls, and a transcript that lists all of them
is unreadable. A run compresses to a single row plus a count, which is enough to
skim past — or to notice that something in it failed.

## 2. Anatomy

```text
[Root .tool-call-group] <div>  (carries data-expanded/data-status/data-size/data-density)
  ├── [List .tool-call-group__list] <ul>
  │   └── [Item] <li>  (repeated)
  │       └── ToolCall  (collapsed: the newest call only. expanded: every call, in order)
  └── [Toggle .tool-call-group__toggle] <button type="button" aria-expanded aria-controls>
      ├── [Toggle Icon] Icon (icon="chevron-down", rotated when expanded)
      └── [Toggle Label] <span>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | the run; carries expansion and run status. A bordered container on the *run*, so a thirty-call run is one box | `--poodle-color-background-surface`, `--poodle-color-border-subtle`, `--poodle-radius-control` |
| List | yes | the rendered calls | — |
| Toggle | no | present only when there is more than one call | `--poodle-color-text-secondary`, `--poodle-typography-label-size` |

The toggle is always the **last** child, in both states. Collapsed it reads
"+N previous tool calls"; expanded, "Show fewer tool calls".

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | the run id, from grouping |
| `calls` | `TranscriptToolCall[]` | `[]` | yes | in chronological order |
| `expanded` | `boolean` | `false` | no | controlled when bound |
| `expandedCalls` | `string[]` | `[]` | no | call ids whose output is open |
| `moreLabel` | `(count: number) => string` | `` (n) => `+${n} previous tool calls` `` | no | collapsed toggle wording |
| `fewerLabel` | `string` | `"Show fewer tool calls"` | no | expanded toggle wording |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToggle` | `((id: string) => void) \| null` | `null` | no | the run was expanded or collapsed |
| `onCallToggle` | `((id: string) => void) \| null` | `null` | no | a call's output was opened or closed |

### Computed Values

| Name | Formula |
|------|---------|
| `leadCall` | `calls[calls.length - 1]` |
| `hiddenCount` | `max(0, calls.length - 1)` |
| `status` | `error` if any call failed, else `running` if any is running, else `success` |
| `showsToggle` | `hiddenCount > 0` |
| `renderedCalls` | `expanded ? calls : [leadCall]` |

### Ordering

The collapsed row is the run's **newest** call, not its oldest. The newest call
is the one still telling you something; the older ones are history you can ask
for.

Expanded, the run lists every call chronologically and therefore **ends** on the
same call that was visible while collapsed. That is what makes expanding safe to
do while reading: the row under the cursor does not move, the rest appears above
it.

### Run Status

One failed call anywhere makes the whole run read as failed, outranking any
number of successes and outranking `running` too. The summary exists to answer
"do I need to open this?", and one failed command inside eight successful ones
is exactly when the answer is yes. A run that already broke is not "in progress"
in any sense the reader cares about.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| single call | one call in the run | the row alone; no toggle at all |
| collapsed | more than one call | newest row, then "+N previous tool calls" |
| expanded | `expanded` | every row in order, then "Show fewer tool calls" |
| failed run | any call failed | the toggle carries the danger colour so a collapsed run advertises the failure |
| running run | any call running, none failed | the toggle carries the running indicator |

A collapsed run whose failure is not the newest call is the case this state
exists for. Without run status, a failure three calls back is completely
invisible until someone expands.

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `onToggle` | run id | the run expanded or collapsed |
| `onCallToggle` | call id | a call's output opened or closed |

## 6. Accessibility

### Semantics

- The list is a `<ul>`, and the group owns the `<li>` wrappers so `ToolCall`
  stays valid on its own.
- The toggle is a `<button>` with `aria-expanded` and `aria-controls` pointing
  at the list.
- The toggle's accessible name includes the run status when it is not
  `success` — "+8 previous tool calls, contains a failure" — because otherwise a
  collapsed failing run is announced identically to a passing one.

### Keyboard

| Key | Action |
|-----|--------|
| `Enter` / `Space` on the toggle | expands or collapses the run |
| `Tab` | moves through the rendered rows, then the toggle |

### Focus

Expanding keeps focus on the toggle. Since the toggle stays the last child and
new rows appear above it, focus does not move on screen either.

## 7. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| row gap | `--poodle-space-stack-xs` between calls |
| toggle inset | aligned to the rows' icon column, so the count sits under the labels |

## 8. Token Usage

| Property | Token |
|----------|-------|
| container fill | `--poodle-color-background-surface` |
| container border | `--poodle-color-border-subtle` |
| container radius | `--poodle-radius-control` |
| toggle colour | `--poodle-color-text-secondary` |
| toggle colour, failed run | `--poodle-color-status-danger` |
| toggle font size | `--poodle-typography-label-size` |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, offset outline |
| row gap | `--poodle-space-stack-xs` |
| group gap | `--poodle-space-stack-sm` |

#### The Container Is On The Run

A contiguous run sits in its own bordered box. The container belongs to the run
and not to each row: a thirty-call run has to read as one thing you can skim
past, and thirty boxes is a cage.

Together with the row's secondary label this is what separates the agent's work
from its prose. The prose sits outside the boxes at full strength; the rows are
walled off and dimmer. Without the container, a transcript of alternating
messages and rows reads as one undifferentiated column.

The container has block padding only. Inline inset is left to the rows, whose
own padding already provides it.

### Size Variants

Size sets the toggle's type scale, matching the rows. Density sets the gaps and
the container's block padding. Density never changes row height.

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-expanded` | `true`/`false` | root |
| `data-status` | `running`/`success`/`error` | root |
| `data-count` | number of calls | root |
| `data-size` / `data-density` | the ladders | root |

## 9. Svelte Notes

- `renderedCalls` is `$derived`; there is no separate "collapsed list" state to
  keep in sync.
- The toggle is omitted entirely rather than hidden when `hiddenCount` is zero,
  so a single-call run has no stray tab stop.

## 10. GPUI Notes

- Expansion is host-driven through the spec; the group renders whichever state
  the spec describes.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] collapsed shows the newest call, not the oldest
- [ ] expanded lists chronologically and ends on the previously visible call
- [ ] a single-call run renders no toggle and no tab stop
- [ ] one failure anywhere makes the run read as failed
- [ ] error outranks running in run status
- [ ] the toggle's accessible name carries a non-success run status
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] the toggle aligns to the rows' icon column
- [ ] a failed run's toggle takes the danger colour while collapsed
- [ ] row gap and group gap match per size and density
- [ ] density never changes row height

### Tier 3: Implementation Freedom

- [ ] expand/collapse animation is platform-owned
- [ ] chevron rotation is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream renders the expansion state without driving it | no Poodle Jetstream component wires a click yet. No engine work is needed — the runtime dispatches clicks and the preview already feeds pointer state; only the handler shape is undecided | accepted, tracked | g12.017 |
| `moreLabel` is an optional resolved string on the native spec, not a formatter | a Rust spec holds data, not closures; `None` uses the default phrasing | accepted | none |

## 13. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: partial expansion (show 5 more), per-run copy

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a single-call run;
a three-call run collapsed; the same expanded; a thirty-call run collapsed; a
run whose failure is not the newest call, collapsed, showing the danger toggle;
a running run; a run with mixed statuses; a run with one call's output expanded;
full size ladder; density variants.

# AgentMessage

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `AgentMessage`
- Layer: `composites`
- Summary: one turn of prose in an agent conversation — markdown rendered into
  the shared block model, with a streaming caret while tokens are still arriving
- In scope: the markdown subset and how it renders, role presentation, the
  streaming caret, prose measure, code blocks and inline code, plain-text
  extraction for accessible names
- Out of scope: markdown editing (`MarkdownEditor`), syntax highlighting,
  message actions (copy, retry, edit), avatars, timestamps, transport

Poodle owns how agent prose looks and what markdown means. The host owns the
text and when it changes.

## 2. Anatomy

```text
[Root .agent-message] <div>  (carries data-role/data-streaming/data-size/data-density)
  └── [Body .agent-message__body] <div>
      ├── [Paragraph .agent-message__paragraph] <p>
      ├── [Heading .agent-message__heading] <h1>…<h6>
      ├── [Code Block .agent-message__code] Code  (block variant)
      ├── [List .agent-message__list] <ul> / <ol>
      │   └── [List Item .agent-message__list-item] <li>
      ├── [Blockquote .agent-message__quote] <blockquote>
      ├── [Rule .agent-message__rule] Separator
      └── [Caret .agent-message__caret] <span aria-hidden="true">  (conditional: isStreaming)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | carries role and streaming state; sets the prose measure | `--poodle-agent-message-measure` |
| Body | yes | the rendered block sequence | `--poodle-space-stack-md` |
| Paragraph | no | prose paragraph | `--poodle-typography-body-size`, `--poodle-typography-body-lineHeight`, `--poodle-color-text-primary` |
| Heading | no | `h1`–`h6` from the block's level | `--poodle-typography-label-weight`, heading size ramp |
| Code Block | no | the `Code` component in its block form, language from the fence | (Code contract) |
| List | no | ordered or unordered, `start` honoured | `--poodle-space-stack-sm`, `--poodle-space-inline-lg` (marker inset) |
| Blockquote | no | quoted passage with a leading rule | `--poodle-color-border-subtle`, `--poodle-color-text-secondary` |
| Rule | no | thematic break, rendered as `Separator` | (Separator contract) |
| Caret | no | the streaming indicator; `aria-hidden` because it is not content | `--poodle-color-accent-base` |

Inline nodes render inside paragraphs, headings and list items: `strong` → `<strong>`,
`em` → `<em>`, `del` → `<del>`, `code` → `.agent-message__code-span`,
`link` → `TextLink`, `break` → `<br>`.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `markdown` | `string` | `""` | yes | raw markdown; the component parses it |
| `role` | `TranscriptRole` | `"assistant"` | no | `user` messages render on the subdued surface |
| `isStreaming` | `boolean` | `false` | no | shows the caret and suppresses the trailing-whitespace trim |
| `linkTarget` | `string \| null` | `null` | no | `target` for rendered links; the host decides whether links leave the app |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onLinkClick` | `((href: string) => void) \| null` | `null` | no | intercepts link activation; when set the default navigation is suppressed |

### Slots / Children

None. The content is the markdown.

### Naming Rules

Follows Poodle conventions. The Rust spec keeps `is_streaming`.

### Markdown Subset

The supported subset, and what anything outside it does:

| Construct | Rendered as |
|-----------|-------------|
| paragraph | `<p>` |
| heading, levels 1–6 | `<h1>`–`<h6>` |
| fenced code, with or without a language | `Code`, block variant |
| indented code | `Code`, block variant, no language |
| ordered / unordered list, nested, `start` honoured | `<ol>` / `<ul>` |
| blockquote | `<blockquote>` |
| thematic break | `Separator` |
| `**strong**`, `*em*`, `~~del~~`, `` `code` ``, links, hard breaks | inline elements |
| tables, footnotes, task lists, autolink literals, raw HTML | **degrade to text** |

Degrading rather than dropping is deliberate. An agent explaining HTML must not
have the explanation disappear because the parser classified part of it as raw
HTML, and silently losing content is the worst available failure for a
transcript.

Two parsers back this: `marked` on the web, `pulldown-cmark` in Rust. Neither
one's output is the model — both normalise into the shared block model, pinned
by `packages/contracts/headless/vectors/markdown-blocks.json`, which both
runtimes run. The vectors are generated from the web target, which is the parity
authority.

Normalisations that exist purely to keep the two in step, and must not be
"simplified" away:

- **List items always contain blocks**, tight or loose. Tight lists emit no
  paragraph events in `pulldown-cmark`; wrapping unconditionally removes
  tight-vs-loose as a divergence.
- **An unannotated fence has no language** (`null`), not an empty one. A
  renderer can then tell "no language given" from "a language that is the empty
  string".
- **A fence info string keeps only its first word.** `marked` drops the rest.
- **A soft break is a newline inside the text**, not a node. A hard break is a
  node.
- **Contiguous text merges into one node.** The parsers split text on different
  boundaries; unmerged splits are a structural difference even when the rendered
  result is identical.

### Shared Types

Defined in `@poodle/headless` (`markdown-blocks.ts`), mirrored in
`poodle-markdown` (Rust).

```typescript
type MdInline =
  | { type: "text"; value: string }
  | { type: "code"; value: string }
  | { type: "strong"; children: MdInline[] }
  | { type: "em"; children: MdInline[] }
  | { type: "del"; children: MdInline[] }
  | { type: "link"; href: string; children: MdInline[] }
  | { type: "break" };

type MdBlock =
  | { type: "paragraph"; children: MdInline[] }
  | { type: "heading"; level: 1 | 2 | 3 | 4 | 5 | 6; children: MdInline[] }
  | { type: "code"; lang: string | null; value: string }
  | { type: "list"; ordered: boolean; start: number; items: MdBlock[][] }
  | { type: "blockquote"; children: MdBlock[] }
  | { type: "rule" };
```

### Computed Values

| Name | Formula |
|------|---------|
| `blocks` | `blocksFromMarked(marked.lexer(markdown))` (web), `parse_markdown(markdown)` (native) |
| `plainText` | `markdownPlainText(blocks)` |
| `showsCaret` | `isStreaming` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| assistant | `role="assistant"` | prose on the page surface, no container chrome |
| user | `role="user"` | prose on the subdued surface with inset padding and a radius |
| streaming | `isStreaming` | caret follows the last rendered inline node, blinking |
| empty | `markdown` is empty | nothing renders; the component contributes no box |

### Behavior Machine

Parsing and plain-text extraction live in `@poodle/headless`
`markdown-blocks.ts`, mirrored in `poodle-markdown`.

Parsing is derived from `markdown`, never cached across changes. A streaming
message reparses on every append, which is correct and cheap at message scale;
an incremental parser would have to reason about a half-open fence, and getting
that wrong renders the rest of the message as code.

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `onLinkClick` | `href` | a rendered link is activated; suppresses default navigation when set |

## 6. Accessibility

### Semantics

- Headings render as real heading elements, so the message is navigable by
  heading in a screen reader.
- Lists render as real list elements, so item counts are announced.
- Code blocks carry their language as `data-language`; the `Code` contract owns
  the rest.
- The caret is `aria-hidden` — it is a progress hint, not content.
- The message body is **not** a live region. Announcing a streaming message per
  token would read partial sentences continuously; `AgentTranscript` owns
  announcement at the log level, when the text is final.

### Keyboard

| Key | Action |
|-----|--------|
| `Tab` | moves through rendered links and code-block controls |

## 7. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| prose measure | `--poodle-agent-message-measure`, so long answers stay readable |
| block spacing | `--poodle-space-stack-md` between blocks |
| list indent | `--poodle-space-inline-lg` |

### Composition

Rendered by `AgentTranscript` for `message` blocks. Usable standalone for any
surface that shows agent prose.

## 8. Token Usage

| Property | Token |
|----------|-------|
| body text | `--poodle-color-text-primary` |
| paragraph size | `--poodle-typography-body-size` |
| line height | `--poodle-typography-body-lineHeight` |
| block gap | `--poodle-space-stack-md` |
| inline code background | `--poodle-color-background-elevated` |
| inline code radius | `--poodle-radius-sm` |
| inline code font | `--poodle-typography-code-family`, `--poodle-typography-label-size` |
| blockquote rule | `--poodle-color-border-subtle` |
| blockquote text | `--poodle-color-text-secondary` |
| user surface | `--poodle-color-background-subtle` |
| user radius | `--poodle-radius-surface` |
| caret | `--poodle-color-accent-base` |

### Size Variants

Size drives the type ramp for paragraphs, headings, inline code and the caret.
Density drives block spacing and list indent only, never line height.

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-role` | `user`/`assistant` | root |
| `data-streaming` | `true` | root, while streaming |
| `data-size` | `xs`…`xl` | root |
| `data-density` | `compact`/`default`/`comfortable` | root |
| `data-language` | fence language | code blocks |

## 9. Svelte Notes

- `blocks` is `$derived` from `markdown`; no stored parse.
- Blocks render through a recursive snippet, since lists and blockquotes nest
  arbitrarily.
- `marked` is configured with GFM on, matching the Rust side's
  `ENABLE_STRIKETHROUGH`, so `~~text~~` behaves identically on both.

## 10. GPUI Notes

- Parses with `poodle_markdown::parse_markdown`, then maps blocks to elements.
- No streaming caret: the natives are render-only, so streaming is a host
  concern there.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] every markdown vector produces the same block model in both languages
- [ ] unsupported constructs degrade to text and never vanish
- [ ] an unannotated fence reports no language rather than an empty one
- [ ] list items always contain blocks, tight or loose
- [ ] `plainText` includes code-block source
- [ ] `onLinkClick` suppresses default navigation when set
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] prose measure, block gap and list indent match per size and density
- [ ] inline code renders on the elevated surface in the mono family
- [ ] blockquote shows a leading rule in the subtle border colour
- [ ] user messages sit on the subdued surface with a surface radius
- [ ] headings follow the type ramp
- [ ] density never changes line height

### Tier 3: Implementation Freedom

- [ ] caret animation is platform-owned
- [ ] the recursion technique for nested blocks is platform-owned
- [ ] syntax highlighting inside code blocks is out of scope for all targets

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Natives show no streaming caret | shared render-only posture across all native components | accepted | host drives re-render |
| Tables degrade to text on every target | out of the v1 subset; adding them means a table block model and four renderers | accepted | promote if a consumer needs them |
| No syntax highlighting | `Code` does not highlight either; adding it is a `Code` decision, not a message one | accepted | tracked on the Code contract |

## 13. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: tables, task lists, per-message copy, math — all
  deliberately out of v1

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a plain paragraph;
inline code, emphasis and a link; every heading level; a fenced code block with
a language; one without; a tight list; a loose list; an ordered list with a start
offset; a nested list; a list item containing a code fence; a blockquote; a
thematic break; strikethrough; a streaming message with the caret; a user
message; a long answer at the prose measure; unsupported markdown degrading to
text; full size ladder; density variants.

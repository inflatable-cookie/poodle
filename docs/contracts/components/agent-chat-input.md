# AgentChatInput

Status: detailed contract
Updated: 2026-07-24

## 1. Purpose

- Component name: `AgentChatInput`
- Layer: `composites`
- Summary: the composer surface for an AI agent conversation — an auto-growing
  message editor over a control toolbar, with attachment chips, a context-budget
  ring, and a single action button that flips between submit and stop
- In scope: controlled/uncontrolled message text, image attachments shown as
  thumbnail tiles, auto-grow between a row floor
  and ceiling, Enter/Shift+Enter/modifier submit semantics, `idle` vs `busy`
  status, submit ↔ stop action, attachment chips with removal, a context-usage
  ring driven by used/limit, a host-composed toolbar region, an optional footer
  region
- Out of scope: the conversation transcript, streaming/token plumbing, model
  invocation, file upload transport, lightbox/expanded attachment previews, slash-command or
  mention autocomplete, prompt history navigation, vendor vocabulary

Poodle owns the composer's shape and interaction; the host owns what the
controls in it mean. The canonical toolbar occupant is `ModelPicker`, but the
region takes any Poodle control.

## 2. Anatomy

```text
[Root .agent-chat-input] <div>  (carries data-size/data-density/data-status/data-disabled)
  ├── [Field .agent-chat-input__field] <div>  (the bordered, rounded composer block)
  │   ├── [Question .agent-chat-input__question] AgentQuestion (conditional: status="questioning")
  │   ├── [Attachments .agent-chat-input__attachments] <ul> (conditional: attachments non-empty)
  │   │   └── [Attachment .agent-chat-input__attachment] <li> (repeated; data-variant="chip"|"thumbnail")
  │   │       ├── [Attachment Thumb .agent-chat-input__attachment-thumb] <img>  (thumbnail variant)
  │   │       ├── [Attachment Icon .agent-chat-input__attachment-icon] Icon (chip variant; conditional: attachment has `icon`)
  │   │       ├── [Attachment Label .agent-chat-input__attachment-label] <span> (chip variant)
  │   │       └── [Attachment Remove] IconButton (icon="x", size="xs", ghost; aria-label "Remove {label}")
  │   ├── [Editor .agent-chat-input__editor] <textarea>
  │   └── [Toolbar .agent-chat-input__toolbar] <div>
  │       ├── [Leading .agent-chat-input__leading] <div>  (host-composed controls; data-dividers)
  │       └── [Trailing .agent-chat-input__trailing] <div>
  │           ├── [Context .agent-chat-input__context] Meter (shape="ring"; conditional: contextLimit set)
  │           └── [Action .agent-chat-input__action] <button type="button">
  │               └── [Action Icon] Icon (`arrow-up` when idle, `square` when busy)
  └── [Footer .agent-chat-input__footer] <div> (conditional: footer content supplied)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | layout wrapper stacking the field over the footer; carries `data-size`, `data-density`, `data-status`, `data-disabled` | — |
| Field | yes | the composer block: rounded, bordered panel holding attachments, editor and toolbar, sitting two steps up the background ladder so it reads as a distinct block against the page; shows the focus ring when focus is inside | `--poodle-radius-surface`, `--poodle-color-background-panel`, `--poodle-color-border-default`, `--poodle-color-accent-focusRing` |
| Attachments | no | horizontal wrapping chip list of pending attachments | `--poodle-space-inline-sm` |
| Attachment | no | one pending attachment; carries `data-kind` and `data-variant`. `chip`: optional icon + label + remove. `thumbnail` (when `thumbnailUrl` is set): a square image tile with the remove control floated over its corner — an image says more than its filename does | `--poodle-color-background-elevated`, `--poodle-color-border-subtle`, `--poodle-radius-control` |
| Attachment Thumb | no | the tile image, `object-fit: cover`, square at the size ladder's thumbnail edge | `--poodle-radius-control` |
| Editor | yes | the message `<textarea>`; transparent, borderless, auto-growing between `minRows` and `maxRows` — the field carries the surface, the editor never draws its own | typography tokens, `--poodle-color-text-primary`, `--poodle-color-text-secondary` at reduced opacity (placeholder) |
| Toolbar | yes | the control row under the editor | `--poodle-space-inline-sm` |
| Leading | yes | host-composed control region (model picker, mode selectors, attachment triggers); optional hairline dividers between children | `--poodle-color-border-subtle` |
| Trailing | yes | right-aligned group: context ring then action button | — |
| Context | no | `Meter shape="ring"` bound to `contextUsed` / `contextLimit`, with `high` set from `contextWarnAt` | (Meter contract) |
| Action | yes | the single submit/stop button — circular, accent-filled; `data-state="submit"` or `"stop"` | `--poodle-color-accent-base`, `--poodle-color-text-inverse` |

The action button is a plain `<button>` rather than `IconButton` because its
circular accent treatment and dual-state semantics are specific to the composer;
every other interactive part reuses a Poodle primitive.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | `""` | no | message text; controlled when bound, otherwise component-owned |
| `placeholder` | `string` | `"Send a message"` | no | editor placeholder |
| `status` | `"idle" \| "busy" \| "questioning"` | `"idle"` | no | `busy` flips the action button to stop; `questioning` renders the question region and blocks ordinary sending |
| `disabled` | `boolean` | `false` | no | disables the editor, action button and attachment removal |
| `readOnly` | `boolean` | `false` | no | editor is not editable; the action button still works (submit stays possible) |
| `ariaLabel` | `string` | `"Message"` | no | accessible name for the editor |
| `submitLabel` | `string` | `"Send"` | no | accessible name for the action button while idle |
| `stopLabel` | `string` | `"Stop"` | no | accessible name for the action button while busy |
| `submitOnEnter` | `boolean` | `true` | no | when false only `Cmd/Ctrl+Enter` submits |
| `minRows` | `number` | `2` | no | editor floor in text rows |
| `maxRows` | `number` | `12` | no | editor ceiling in text rows; beyond it the editor scrolls |
| `maxLength` | `number \| null` | `null` | no | native `maxlength` on the editor |
| `allowEmptySubmit` | `boolean` | `false` | no | when true the action button stays enabled with an empty editor |
| `attachments` | `AgentChatAttachment[]` | `[]` | no | pending attachment chips |
| `contextUsed` | `number \| null` | `null` | no | consumed context budget, in the host's own unit |
| `contextLimit` | `number \| null` | `null` | no | total context budget; when null the ring is not rendered |
| `contextWarnAt` | `number` | `0.8` | no | fraction of `contextLimit` at which the ring switches to the warning tone |
| `contextLabel` | `string` | `"Context used"` | no | accessible name for the ring |
| `toolbarDividers` | `boolean` | `true` | no | hairline dividers between leading toolbar children |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role for inherited sizing |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onSubmit` | `((value: string) => void) \| null` | `null` | no | fires on a valid submit gesture |
| `onStop` | `(() => void) \| null` | `null` | no | fires when the action button is used while `busy`, and on `Escape` while busy |
| `onValueChange` | `((value: string) => void) \| null` | `null` | no | fires on every edit |
| `onRemoveAttachment` | `((id: string) => void) \| null` | `null` | no | fires from an attachment chip's remove button |

### Slots / Children

| Slot | Renderer form | Purpose |
|------|---------------|---------|
| `toolbar` | Svelte `Snippet`, React `ReactNode` | leading toolbar controls (canonically a `ModelPicker`, which should be passed `emphasis="subdued"` here so it does not compete with the editor) |
| `footer` | Svelte `Snippet`, React `ReactNode` | the secondary bar under the composer (scope/branch/status rows) |

Native targets take these as child element vectors instead
(`js_agent_chat_input(spec, theme, toolbar_children, footer_children)`).

### Naming Rules

Follows Poodle conventions: `camelCase` multi-word props, `on*` handler props,
`size`/`density`/`sizeRole` opt into the shared presentation axes.
`disabled`/`readOnly` are documented without the `is` prefix; the Rust spec keeps
`is_disabled` / `is_read_only`.

### Shared Types

Defined in `@poodle/svelte` `types.ts`, re-exported from the package root,
redefined identically in `@poodle/react`, mirrored in `poodle-specs` (snake_case).

```typescript
type AgentChatStatus = "idle" | "busy" | "questioning";

type AgentChatAttachment = {
  id: string;
  label: string;
  /** Host-defined kind, surfaced as `data-kind` for styling hooks. */
  kind?: string;
  icon?: string;
  /** Image source for a visual attachment. When set the chip is replaced by a
   * thumbnail tile; the host owns the URL (an object URL, a served path). */
  thumbnailUrl?: string;
  disabled?: boolean;
};
```

### Controlled And Uncontrolled

- Controlled: bind `value`; every edit is mirrored through `onValueChange`
- Uncontrolled: omit `value` and the component owns the text
- Submitting does **not** clear the editor — the host decides, because a failed
  send must not lose the user's text

### Computed Values

| Name | Formula |
|------|---------|
| `isBusy` | `status === "busy"` |
| `hasText` | `value.trim().length > 0` |
| `canSubmit` | `!disabled && (isBusy \|\| hasText \|\| allowEmptySubmit)` |
| `contextPercentage` | `contextLimit ? clamp(contextUsed ?? 0, 0, contextLimit) / contextLimit * 100 : null` |
| `contextHigh` | `contextLimit !== null ? contextLimit * contextWarnAt : null` — passed to Meter as `high` |
| `actionIcon` | `isBusy ? "square" : "arrow-up"` |
| `actionAriaLabel` | `isBusy ? stopLabel : submitLabel` |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle empty | `status="idle"`, no text | placeholder shown; action button in submit state, disabled (unless `allowEmptySubmit`) |
| idle composing | text present | action button enabled, accent-filled |
| busy | `status="busy"` | action button shows the stop glyph and stays enabled; `data-status="busy"` on the root |
| focused | focus inside the field | field draws the focus ring |
| disabled | `disabled=true` | field at disabled opacity; editor, action button and chip removal all disabled |
| read-only | `readOnly=true` | editor not editable; action button unaffected |
| grown | text exceeds `minRows` | editor height grows to fit, up to `maxRows`, then scrolls |
| attachments | `attachments` non-empty | chip/tile row renders above the editor |
| image attachment | attachment has `thumbnailUrl` | that entry renders as a square tile (`data-variant="thumbnail"`) with the remove control over its corner, no filename text |
| context normal | `contextLimit` set, below `contextWarnAt` | ring in the default (success) tone |
| context high | used ≥ `contextLimit * contextWarnAt` | ring switches to the warning tone (`Meter` `data-level="high"`) |
| no context | `contextLimit` null | ring not rendered |

### Component States

Open/closed does not apply. The only component-owned state is the uncontrolled
text and the measured editor height.

### Behavior Machine

Behavior classification: `styled-only (no machine)` — adapter-owned interaction.

Submit gating, auto-grow measurement and key handling are plain component logic.
There is no `@poodle/headless` machine and no conformance vectors. The pure
helpers (`canSubmit`, `contextPercentage`, `actionIcon`, `resolveSubmitIntent`)
are the cross-renderer semantic reference and live in `agent-chat-input-model.ts`
(TS) and as methods on `AgentChatInputSpec` (Rust).

#### Machinery Dependencies

Presentation context (size/density) and the composed primitives (`Icon`,
`IconButton`, `Meter`). No dismissable layer, no focus trap, no portal.

### Submit Intent Resolution

`resolveSubmitIntent(event, { submitOnEnter, isBusy })` returns
`"submit" | "newline" | "stop" | "none"`:

| Gesture | Result |
|---------|--------|
| `Cmd/Ctrl+Enter` | `submit` (always, regardless of `submitOnEnter`) |
| `Enter` with no modifier, `submitOnEnter=true` | `submit` |
| `Enter` with no modifier, `submitOnEnter=false` | `newline` |
| `Shift+Enter` | `newline` |
| `Enter` during IME composition | `newline` (never submits) |
| `Escape` while `busy` | `stop` |
| anything else | `none` |

A `submit` intent that fails `canSubmit` is dropped. While `busy`, the action
button emits `onStop`, never `onSubmit`; a `submit` intent from the keyboard
while busy is also dropped (stop is deliberate, not accidental).

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | every edit | `string` | fires for both controlled and uncontrolled use |
| `onSubmit` | valid submit gesture (button while idle, or resolved keyboard intent) | `string` (current value) | never fires while `busy` or when `canSubmit` is false |
| `onStop` | action button while `busy`, or `Escape` while `busy` | — | never fires while idle |
| `onRemoveAttachment` | a chip's remove button | `string` (attachment id) | not fired while `disabled` |

## 6. Accessibility

### Semantics

| Element | Attribute | Value |
|---------|-----------|-------|
| Root | `data-status` | `"idle"` / `"busy"` |
| Editor | `aria-label` | from `ariaLabel` |
| Editor | `aria-multiline` | `"true"` (implicit on textarea) |
| Editor | `aria-disabled` / `disabled` | native attribute when `disabled` |
| Editor | `readonly` | native attribute when `readOnly` |
| Editor | `maxlength` | from `maxLength` when set |
| Editor | `rows` | `minRows` |
| Attachments | `aria-label` | `"Attachments"` |
| Attachment Remove | `ariaLabel` | `"Remove {label}"` |
| Attachment Thumb | `alt` / `title` | the attachment `label` — the filename is not shown as text on a tile, so it reaches assistive tech through the image and the tile's tooltip |
| Context ring | `ariaLabel` | `"{contextLabel}, {round(percentage)}%"` |
| Action | `aria-label` | `submitLabel` / `stopLabel` by status |
| Action | `aria-disabled` / `disabled` | native attribute when `canSubmit` is false |
| Action | `data-state` | `"submit"` / `"stop"` |

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | submits when `submitOnEnter` and not composing; otherwise inserts a newline |
| `Shift+Enter` | newline |
| `Cmd/Ctrl+Enter` | submits regardless of `submitOnEnter` |
| `Escape` | stops while `busy`; otherwise the event is left to the host |
| `Tab` | moves from the editor into the toolbar controls and then the action button |

### Focus And Announcement

- focus entry: the editor is the natural first stop; the component never steals
  focus on mount
- the action button keeps its accessible name in sync with `status`, so a screen
  reader announces "Stop" the moment streaming begins
- the context ring carries its percentage in the accessible name; the ring
  itself is not focusable
- the composer does not announce submits — transcript feedback is host-owned
- GPUI-native accessibility mapping: no ARIA API — accessible names map to native
  exposure where available; documented as an accepted delta

## 7. Layout

### Sizing

- Root: `display: flex`, `flex-direction: column`, `width: 100%`, `min-width: 0`
- Field: `border-radius: var(--poodle-radius-surface)` stepped up one notch for
  the composer's soft-rounded look, padding from `--poodle-space-panel-*`
- Editor: `width: 100%`, `resize: none`, `min-height: calc(minRows * 1lh)`,
  `max-height: calc(maxRows * 1lh)`; the measured height is applied inline
- Toolbar: `display: flex`, `align-items: center`, `gap` from density; Trailing
  is pushed right with `margin-left: auto`
- Footer: sits below the field, inset horizontally by `1.5rem` so it reads as a
  secondary bar tucked under the composer, with only its bottom corners rounded
- Action: square box at `--poodle-size-control-height`, `border-radius: 999px`

### Composition

- parent expectations: a chat/agent view's bottom region, a side panel, a modal
- child expectations: `ModelPicker` and other Poodle controls in the toolbar
  slot; `Meter` (ring) and `IconButton` are internal
- resizing rules: the composer fills its parent's width; height is content-driven
  between the row floor and ceiling

### Question Region

While `status="questioning"` the field carries an `AgentQuestion` above the
attachments, and the composer's own parts change role:

| Part | While questioning |
|------|-------------------|
| Editor | the free-text override; placeholder becomes `questionPlaceholder` |
| Action | submits the answer rather than a message |
| `canSubmit` | `hasText \|\| questionHasSelection` — an empty editor is submittable when an option is chosen |
| `onSubmit` | fires as the *request* to answer; the host routes it to the question's `submit()`, and the question's own `onSubmit` delivers the resolved answer |

The composer keeps its editor, toolbar and submit control; `AgentQuestion`
supplies only the prompt and the options. That division is the reason the
question lives here at all — the override *is* this editor, and rendering the
question anywhere else would put a second text input on screen with different
submit semantics.

The composer cannot resolve the answer itself — it holds the editor text but
not the selection — so the two are joined by the host:

```svelte
<AgentChatInput status="questioning" onSubmit={() => question.submit()}>
  {#snippet question()}
    <AgentQuestion bind:this={question} {questions} bind:selections override={value} onSubmit={record} />
  {/snippet}
</AgentChatInput>
```

`AgentQuestion` exports `submit()` and `canSubmit()` for exactly this. Wiring
the composer's action to clear the editor instead of answering is the easy
mistake — the text vanishes and nothing is sent.

A pending question blocks the turn, not the UI: no scrim, no focus trap, and
the transcript stays scrollable so the reader can check something before
answering. What blocks is this component refusing to send anything but an
answer. See `agent-question.md` §2.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Field | `--poodle-radius-surface` | composer corner radius |
| Field | `--poodle-color-background-panel` | composer fill — two steps up the ladder (`canvas` → `surface` → `panel`); one step is too close to the page to read as a distinct block |
| Field | `--poodle-color-border-default` | composer border |
| Field | `--poodle-space-panel-x`, `--poodle-space-panel-y` | composer padding |
| Field focus | `--poodle-color-accent-focusRing`, `--poodle-border-width-focus` | focus ring |
| Editor | `--poodle-typography-body-family`, `--poodle-typography-body-size` | editor typography |
| Editor | `--poodle-color-text-primary` | text colour |
| Editor placeholder | `--poodle-color-text-secondary` at `calc(--poodle-state-opacity-muted * 0.62)` | placeholder colour — dimmer than the shared input convention, since the composer's placeholder is a standing hint rather than a value |
| Attachment | `--poodle-color-background-elevated`, `--poodle-color-border-subtle`, `--poodle-radius-control` | chip treatment — a step above the field, so chips read as raised on the panel |
| Leading dividers | `--poodle-color-border-subtle` | hairline between toolbar children |
| Action | `--poodle-color-accent-base` | filled circle |
| Action | `--poodle-color-text-inverse` | glyph colour |
| Action hover | `color-mix(accent 88%, white)` | hover fill |
| Action (stop) | `--poodle-color-status-danger` | stop-state fill |
| Footer | `--poodle-color-background-surface`, `--poodle-color-border-subtle` | secondary bar — below the field on the ladder, so it tucks under the composer rather than sitting on top of it |
| Disabled | `--poodle-state-opacity-disabled` | disabled opacity |
| Motion | `--poodle-motion-duration-interaction`, `--poodle-motion-easing-standard` | hover/height transitions |

### Size Variants

| Size | Field padding | Editor font | Action box | Thumbnail tile | Toolbar gap |
|------|---------------|-------------|------------|----------------|-------------|
| `xs` | `0.375rem 0.5rem` | `0.75rem` | `1.5rem` | `2rem` | `0.25rem` |
| `sm` | `0.5rem 0.625rem` | `0.8125rem` | `1.75rem` | `2.5rem` | `0.375rem` |
| `md` | `0.625rem 0.75rem` | `0.875rem` | `2rem` | `3rem` | `0.5rem` |
| `lg` | `0.75rem 1rem` | `0.9375rem` | `2.375rem` | `3.5rem` | `0.625rem` |
| `xl` | `0.875rem 1.125rem` | `1rem` | `2.75rem` | `4rem` | `0.75rem` |

Density changes only the toolbar/attachment gaps and the leading divider
spacing — never the field's vertical padding or the action box.

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`–`"xl"` |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` |
| `data-status` | Root | `"idle"`, `"busy"` |
| `data-disabled` | Root | `"true"` / `"false"` |
| `data-dividers` | Leading | `"true"` / `"false"` |
| `data-kind` | Attachment | host-supplied kind, when present |
| `data-variant` | Attachment | `"chip"`, `"thumbnail"` |
| `data-state` | Action | `"submit"`, `"stop"` |

## 9. Svelte Notes

- the editor is a raw `<textarea>` owned by the component (the same posture as
  `MarkdownEditor`), not a `TextInput` — the composer needs auto-grow, a
  transparent borderless field and composer-specific key handling
- auto-grow: an `$effect` on `value` resets `height: auto` then applies
  `scrollHeight`, clamped by the `maxRows` line-height ceiling
- controlled/uncontrolled via `$bindable(value)` plus a `sync()` that writes the
  value then calls `onValueChange`
- size/density resolve via `getUiPresentation` + `resolveSemanticControlSize`
- `toolbar` and `footer` are `Snippet` props; the footer part is not rendered at
  all when the snippet is absent
- key handling delegates to `resolveSubmitIntent` from `./agent-chat-input-model`
- IME safety comes from the textarea's `compositionstart` / `compositionend`
  events, tracked in a local flag

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::agent_chat_input`
- theme access via `GpuiThemeProvider`; all dimensions/colors resolved from tokens
- the render is a faithful function of the full spec state: `AgentChatInputSpec`
  carries `value`, `status`, `attachments` and the context budget, so the native
  render shows the same attachment chips, editor text (or placeholder), toolbar
  children, context ring and submit/stop button state as the web
- text editing itself is host-owned: the native editor part renders the current
  value as text, and keystroke handling lives in the host event loop
- editor auto-grow is approximated from the value's line count clamped between
  `minRows` and `maxRows`, since neither native runtime measures wrapped text at
  spec-resolution time
- no ARIA API — accessible-name intent documented as an accepted delta

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `canSubmit` gating matches (empty text blocks submit unless `allowEmptySubmit`)
- [ ] `busy` flips the action to stop, keeps it enabled, and routes to `onStop`
- [ ] `resolveSubmitIntent` matches for every row of the gesture table
- [ ] submitting never clears the editor
- [ ] attachments render as chips, or as square tiles when `thumbnailUrl` is
      set, and remove by id either way
- [ ] context ring renders only when `contextLimit` is set, with `high` derived
      from `contextWarnAt`
- [ ] disabled suppresses editing, submitting and attachment removal
- [ ] `readOnly` blocks editing but not submitting
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] field radius, border, fill and padding match per size
- [ ] the background ladder holds: page < footer (`surface`) < field (`panel`) <
      attachment chips (`elevated`)
- [ ] editor floor/ceiling in rows matches
- [ ] toolbar leading/trailing split, dividers and gaps match
- [ ] action button is a circular accent box at the size ladder's control height
- [ ] stop state uses the danger fill
- [ ] footer sits inset below the field with only bottom corners rounded
- [ ] all five sizes and three densities match the ladders
- [ ] density never changes field vertical padding or action size

### Tier 3: Implementation Freedom

- [ ] auto-grow measurement technique is platform-owned
- [ ] IME handling is platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI/Jetstream render the editor as static text and don't drive keystrokes | shared render-only posture across all native components; text input is host-event-loop work | accepted | host wires editing |
| Native auto-grow is line-count based, not text-measurement based | neither native runtime measures wrapped text during spec resolution | accepted | revisit if a measurement API lands |
| Native context ring inherits Meter's ring delta (no swept arc) | see `meter.md` §12 | accepted | tracked on the Meter contract |
| Action button is a bespoke `<button>`, not `IconButton` | circular accent treatment plus dual-state semantics are composer-specific; every other control reuses a primitive | accepted (by design) | promote to an `IconButton` variant if a second consumer needs it |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- approvers: pending review
- downstream adopters: Loophole, Underlay apps, future agent surfaces
- future follow-up: slash-command / mention affordances, prompt history, inline
  attachment previews — all deliberately out of v1

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): default composer with
a `ModelPicker` in the toolbar; empty (submit disabled); composing (submit
enabled); busy (stop state); with attachments; with a footer bar; context ring
below the warn threshold; context ring above it; no context ring; disabled;
read-only; `submitOnEnter=false`; grown editor at the `maxRows` ceiling;
`allowEmptySubmit`; full size ladder; density variants.

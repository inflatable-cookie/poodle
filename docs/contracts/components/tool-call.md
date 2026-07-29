# ToolCall

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `ToolCall`
- Layer: `composites`
- Summary: one row of agent work — what kind it was, the argument that made it
  specific, how it ended, and its output behind a disclosure
- In scope: the row anatomy, kind label and icon, one-line truncated detail,
  status indicator, output disclosure
- Out of scope: running anything, diffing, output syntax highlighting, grouping
  (`ToolCallGroup` owns that), retry

The row's job is to be skimmable at a glance and openable when it matters. Most
tool calls are never read; the ones that are, are read because something went
wrong.

## 2. Anatomy

```text
[Root .tool-call] <div>  (carries data-status/data-expanded/data-kind)
  ├── [Trigger .tool-call__trigger] <button type="button" aria-expanded>  (when output is present)
  │   ├── [Icon .tool-call__icon] Icon
  │   ├── [Label .tool-call__label] <span>
  │   ├── [Detail .tool-call__detail] <code>
  │   ├── [Disclosure .tool-call__disclosure] Icon (icon="chevron-down")
  │   └── [Status .tool-call__status] Icon | Spinner
  └── [Output .tool-call__output] Code  (conditional: expanded and output present)
```

When there is no output the trigger is a `<div>`, not a disabled button: a
control that cannot do anything should not be in the tab order at all.

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | the row; carries status and expansion | `--poodle-space-inline-sm` |
| Trigger | yes | the whole row is the hit target, not just the chevron | `--poodle-radius-control`, `--poodle-color-background-elevated` (hover) |
| Icon | yes | the kind glyph — terminal for a command, pencil for a file change | `--poodle-color-text-secondary`, `--poodle-size-icon-sm` |
| Label | yes | the kind in words: "Ran command", "File change" | `--poodle-color-text-primary`, `--poodle-typography-label-size` |
| Detail | no | the argument line, single-line and ellipsised | `--poodle-color-text-tertiary`, `--poodle-typography-code-family` |
| Disclosure | no | chevron, rotated when expanded | `--poodle-color-text-tertiary` |
| Status | yes | tick, cross, or spinner | `--poodle-color-status-success`, `--poodle-color-status-danger` |
| Output | no | the `Code` component in block form | (Code contract) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | identity for toggle callbacks |
| `label` | `string` | — | yes | the kind in words |
| `detail` | `string \| null` | `null` | no | argument line, truncated to one line |
| `status` | `ToolCallStatus` | `"success"` | no | `running`/`success`/`error` |
| `icon` | `string \| null` | `null` | no | overrides the icon derived from `label` |
| `output` | `string \| null` | `null` | no | revealed when expanded; absent means no disclosure |
| `expanded` | `boolean` | `false` | no | controlled when bound |
| `outputLanguage` | `string \| null` | `null` | no | passed to `Code` for the output block |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onToggle` | `((id: string) => void) \| null` | `null` | no | fires when the row is opened or closed |

### Computed Values

| Name | Formula |
|------|---------|
| `hasOutput` | `output !== null && output.length > 0` |
| `isInteractive` | `hasOutput` |
| `resolvedIcon` | `icon ?? iconForLabel(label)` |
| `statusIcon` | `running → Spinner`, `success → "check"`, `error → "x"` |

`iconForLabel` maps the known kinds — "Ran command" → `terminal`, "File change" →
`file-pen`, "Searched" → `search` — and falls back to `dot` for anything else.
The map lives in the shared spec so every target agrees, and `icon` exists so a
host with its own vocabulary is never stuck with the fallback.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | `status="success"` | dimmed row, tick at the trailing edge |
| running | `status="running"` | spinner replaces the tick; detail still shown |
| error | `status="error"` | cross in the danger colour; the label takes the danger colour too |
| hover | pointer over an interactive row | elevated background across the whole row |
| expanded | `expanded` and output present | chevron rotated, output block below |
| non-interactive | no output | no chevron, no hover, not in the tab order |

Only the label takes the danger colour on error, not the detail: the detail is
already the dimmest thing in the row, and colouring it red as well makes a
failed row read as a solid block of alarm rather than a line you can scan.

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `onToggle` | `id` | the row is opened or closed |

## 6. Accessibility

### Semantics

- The row is a `<div>`, not an `<li>`. `ToolCallGroup` wraps each row in its
  own list item, because a component whose root is an `<li>` can never be valid
  standalone and this one is usable outside a group.
- With output, the trigger is a `<button>` with `aria-expanded` and
  `aria-controls` pointing at the output block.
- Status is conveyed in the accessible name, not by colour alone: the name is
  `"{label}: {detail}, {status}"`, with `success` omitted as the unremarkable
  case.
- The detail is visually truncated but not truncated in the accessible name — a
  screen reader gets the whole command, which is the thing a truncated row is
  hiding.

### Keyboard

| Key | Action |
|-----|--------|
| `Enter` / `Space` | toggles the output when interactive |
| `Tab` | skips rows with no output entirely |

## 7. Layout

### Sizing

| Aspect | Rule |
|--------|------|
| row height | `--poodle-tool-call-row-height` floor, growing with content |
| gap | `--poodle-space-inline-sm` between icon, label, detail |
| detail | `flex: 1`, `min-width: 0`, single line with ellipsis |

`min-width: 0` on the detail is load-bearing: without it the flex item refuses
to shrink below its content width and a long command pushes the status
indicator out of the row.

## 8. Token Usage

| Property | Token |
|----------|-------|
| label colour | `--poodle-color-text-primary` |
| detail colour | `--poodle-color-text-tertiary` |
| detail font | `--poodle-typography-code-family`, `--poodle-typography-label-size` |
| icon colour | `--poodle-color-text-secondary` |
| success | `--poodle-color-status-success` |
| danger | `--poodle-color-status-danger` |
| hover fill | `--poodle-color-background-elevated` |
| row radius | `--poodle-radius-control` |
| row gap | `--poodle-space-inline-sm` |
| row inset | `--poodle-space-inline-sm` |

### Size Variants

Size sets the row height floor, icon size and type scale. Density sets the row
inset and the gaps between parts. Density never changes row height.

### Data Attributes

| Attribute | Values | On |
|-----------|--------|-----|
| `data-status` | `running`/`success`/`error` | root |
| `data-expanded` | `true`/`false` | root |
| `data-interactive` | `true`/`false` | root |
| `data-size` / `data-density` | the ladders | root |

## 9. Svelte Notes

- The trigger element is chosen by `hasOutput`, not disabled — a `<div>` when
  there is nothing to open.
- Output renders lazily: the `Code` block is only created when first expanded,
  so a transcript of a thousand rows does not build a thousand code blocks.

## 10. GPUI Notes

- Static presentation; expansion is host-driven through the spec.
- Spinner is a static glyph, matching the native posture elsewhere.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] no output means no chevron, no hover and no tab stop
- [ ] the accessible name carries status, and omits it for success
- [ ] the accessible name carries the full detail, untruncated
- [ ] `iconForLabel` resolves the same for every known kind
- [ ] error colours the label but not the detail
- [ ] no vendor vocabulary anywhere in the component

### Tier 2: Visual Parity

- [ ] detail truncates to one line with an ellipsis at every width
- [ ] the status indicator never gets pushed out by a long detail
- [ ] hover fills the whole row, not just the chevron
- [ ] row height floor, gaps and insets match per size and density
- [ ] density never changes row height

### Tier 3: Implementation Freedom

- [ ] chevron rotation and hover transitions are platform-owned
- [ ] lazy output construction is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Natives render a static glyph for `running` | no animation loop in the render-only native posture | accepted | none |
| Output has no syntax highlighting | inherited from `Code` | accepted | tracked on the Code contract |

## 13. Approval And Adoption Notes

- contract status: `drafted`
- approvers: pending review
- downstream adopters: Figmatic, Loophole, future agent surfaces
- future follow-up: per-row retry, output copy, streamed output

## 14. Specimen Definitions

Required specimen coverage (Svelte preview authoritative): a successful command;
a running command; a failed command; a file change; a row with no detail; a row
with a very long detail; a row with no output (non-interactive); an expanded row
showing output; an unknown kind falling back to the default icon; a custom
`icon`; full size ladder; density variants.

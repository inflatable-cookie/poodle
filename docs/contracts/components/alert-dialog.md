# AlertDialog

Status: detailed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `AlertDialog`
- Layer: `foundation`
- Summary: a focused confirmation overlay for destructive or irreversible actions
  that composes Dialog with alertdialog semantics and optional async callbacks
- In scope: danger and warning tones, confirm/cancel action pair, working state
  that prevents premature dismiss, alertdialog ARIA role, Promise-based
  confirmation callbacks
- Out of scope: multi-step wizards, informational dialogs, inline confirmations,
  non-modal alerts

## 2. Anatomy

```text
[Root]  <Dialog role="alertdialog" width="sm">
  ├── [Title]  via Dialog title prop
  ├── [Description]  via Dialog description prop
  ├── [Item Detail .alert-dialog__item-detail]  <p> (optional, when itemLabel && itemValue)
  ├── [Body]  (`children` snippet, optional)
  └── [Actions snippet]
      ├── [Cancel Button]  <Button variant="ghost">
      └── [Confirm Button]  <Button variant="primary" tone={confirmTone}>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root (Dialog) | yes | composed Dialog with role="alertdialog", width="sm" | delegates to Dialog |
| Item Detail | no | highlighted label:value row ahead of body (when itemLabel && itemValue) | text-secondary, text-primary (strong), line-height |
| Cancel Button | yes | ghost Button for cancel action | delegates to Button |
| Confirm Button | yes | primary Button with tone from confirmTone | delegates to Button |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null \| undefined` | `undefined` | no | dialog visibility; when supplied, the host owns updates through `onOpenChange` |
| `title` | `string` | — | yes | visible title text passed to Dialog |
| `description` | `string \| null` | `null` | no | description text passed to Dialog |
| `itemLabel` | `string \| null` | `null` | no | optional highlighted detail-item label rendered ahead of the body content |
| `itemValue` | `string \| null` | `null` | no | optional highlighted detail-item value rendered ahead of the body content |
| `tone` | `"danger" \| "warning"` | `"danger"` | no | controls confirm button tone (variant is always `"primary"`) |
| `confirmLabel` | `string` | `"Confirm"` | no | label for confirm button |
| `cancelLabel` | `string` | `"Cancel"` | no | label for cancel button |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |
| `workingLabel` | `string` | `"Working…"` | no | label shown while confirm work is in flight |
| `onConfirm` | `(() => void \| Promise<void>) \| null` | `null` | no | callback invoked by the built-in confirm button; awaited when it returns a Promise |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `onCancel` | `(() => void) \| null` | `null` | no | callback invoked by built-in cancel and dismissal paths |

### Snippets

| Snippet | Purpose |
|------|---------|
| `children` | optional body content between description and actions |

### Controlled And Uncontrolled

- `open` is host-owned when supplied; the component requests changes through
  `onOpenChange`
- Internal `working` state is always internally managed
- `onConfirm` / `onCancel` / `onOpenChange` are the public interaction hooks

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | `open=false` or default | dialog not rendered |
| open | `open=true` or triggered | dialog visible with cancel and confirm buttons |
| working | confirm activated | dismiss on escape and backdrop disabled, buttons remain visible |
| danger tone | `tone="danger"` (default) | confirm button uses variant="primary" with tone="danger" |
| warning tone | `tone="warning"` | confirm button uses variant="primary" with tone="default" (warning maps to default) |

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onConfirm` | confirm button is clicked | `void \| Promise<void>` | awaited when it returns a Promise |
| `onCancel` | cancel button clicked or dialog dismissed | `void` | suppressed while working |
| `onOpenChange` | dialog open state changes | `boolean` | passthrough from Dialog |

## 6. Accessibility

### Semantics

- Role: `alertdialog` via Dialog `role` prop
- `aria-modal`: `"true"` (via Dialog)
- `aria-label`: from ariaLabel prop when provided
- `aria-labelledby`: auto-linked to title (via Dialog)
- `aria-describedby`: auto-linked to description (via Dialog)
- Focus trap: managed by Dialog

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates focused button |
| `Space` | activates focused button |
| `Tab` | cycles focus within dialog (focus trap) |
| `Shift+Tab` | cycles focus backward within dialog |
| `Escape` | dismisses dialog (disabled while working) |

### Focus And Announcement

- focus entry: confirm button receives initial focus
- focus exit: focus restores to previously focused element (via Dialog)
- working state: escape and backdrop dismiss suppressed
- failed async confirm: dialog stays open so caller can show errors or retry

## 7. Layout

### Sizing

- Delegates entirely to Dialog sizing
- Actions row: flex layout with gap, cancel left, confirm right

### Composition

- parent expectations: triggered by user action in any view context
- child expectations: optional body content via `children` snippet
- resizing: inherits Dialog responsive behavior

## 8. Token Usage — Exact Values

AlertDialog has almost no unique CSS of its own — visual presentation is
delegated to the composed Dialog and Button components. The one exception is the
optional item-detail row (below).

### Item Detail `.alert-dialog__item-detail`

| Property | Value |
|----------|-------|
| `margin` | `0 0 0.75rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `line-height` | `1.5` |
| `strong` color | `var(--poodle-color-text-primary)` |

### Confirm Button mapping

| Tone | Button variant | Button tone |
|------|---------------|-------------|
| `"danger"` | `"primary"` | `"danger"` |
| `"warning"` | `"primary"` | `"default"` |

The confirm tone derives as `tone === "danger" ? "danger" : "default"`, so the
warning tone resolves to a primary Button with the default tone (no warning
accent on the confirm button).

### Cancel Button

| Property | Value |
|----------|-------|
| variant | `"ghost"` |

### Dialog props passed through

| Dialog Prop | Value |
|-------------|-------|
| `role` | `"alertdialog"` |
| `width` | `"sm"` (fixed) |
| `open` | from AlertDialog `open` prop |
| `title` | from AlertDialog `title` prop |
| `description` | from AlertDialog `description` prop |
| `dismissOnEscape` | `false` while working, `true` otherwise |
| `dismissOnBackdrop` | `false` while working, `true` otherwise |
| `showCloseButton` | `false` while working, `true` otherwise |

### Size adjustments

Size is delegated to the composed Dialog and Button components. The `size` and `sizeRole` props are passed through to Dialog.

## 9. Svelte Notes

- `data-size` passed through to composed Dialog
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Composes `Dialog` component directly; does not replicate Dialog internals
- `working` state is internal and suppresses both dismiss routes and button reuse
- Confirm handler awaits `onConfirm` when provided
- Successful built-in confirm closes the dialog; thrown errors keep it open
- Cancel button and external dismiss routes use `onCancel`
- body content is snippet-first via `children`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::alert_dialog`
- Composes `PoodleDialog` with `DialogKind::AlertDialog`
- Working state modeled as internal `bool` field
- Confirm and cancel are callback props
- Tone enum maps to button variant selection

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] tone prop maps to correct confirm button variant
- [ ] working state suppresses escape, backdrop, and close-button dismiss
- [ ] built-in callbacks fire correctly
- [ ] alertdialog role is set
- [ ] focus trap is active

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] cancel button is ghost variant
- [ ] confirm button matches tone-to-variant mapping
- [ ] Dialog visual presentation matches

### Tier 3: Implementation Freedom

- [ ] working state implementation details are platform-owned
- [ ] focus target on open (confirm button) is recommended but platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| No unique CSS deltas | delegates entirely to Dialog and Button | n/a | n/a |

## 13. Specimen Definitions

### Danger Tone (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Danger tone (default) | `tone="danger"` (default), `title="Delete this item?"`, `description="This action cannot be undone..."`, `confirmLabel="Delete"`, `cancelLabel="Keep it"` | Dialog with danger-variant confirm button, ghost cancel button, title and description text |

### Warning Tone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Warning tone | `tone="warning"`, `title="Reset all settings?"`, `description="Your customized settings will be restored..."`, `confirmLabel="Reset"`, `cancelLabel="Cancel"` | Dialog with primary-variant confirm button (warning tone maps to primary), ghost cancel button |

### With Body Content

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With body content | `tone="danger"` (default), `title="Remove this user?"`, `description="The following user will lose access..."`, `confirmLabel="Remove"`, `children` snippet contains user card | Dialog with body content rendered between description and action buttons, showing user name and email in a styled card |
| Async confirm callback | `tone="danger"`, `title="Archive this project?"`, `confirmLabel="Archive"`, `workingLabel="Archiving…"`, `onConfirm` returns a Promise | Dialog stays open while working, built-in buttons disable, confirm label swaps to working label, dialog closes after Promise resolves |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: destructive action confirmations, settings changes, data deletion flows
- future follow-up: richer error-handling recipe examples for rejected async confirms

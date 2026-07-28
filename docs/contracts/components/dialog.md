# Dialog

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Dialog`
- Layer: `foundation`
- Summary: a modal blocking overlay for confirmation, form, or focused task
  flows
- In scope: modal semantics, backdrop, title/description, focus trap,
  dismissal on escape and backdrop, focus restoration, body scroll lock,
  dialog and alertdialog roles, width presets, bare mode, custom header/footer
  snippets
- Out of scope: anchored contextual overlays, full-screen multi-step workflows,
  toast notifications, edge-anchored drawers

## 2. Anatomy

```text
[Wrapper .dialog]  <div data-size data-density data-width> (fixed overlay, conditional render)
  ├── [Backdrop .dialog__backdrop]  <button aria-label="Dismiss dialog backdrop">
  └── [Surface .dialog__surface]  <div role={role} aria-modal tabindex="-1" aria-labelledby=title-id>
      ├── [Close Button .dialog__close]  <IconButton> (optional, receives resolvedSize)
      │
      │   ── when bare=true ──
      ├── [Snippet: children]  (no structural wrappers)
      │
      │   ── when bare=false ──
      ├── [Header .dialog__header]  <div> (optional: when `header` snippet used, OR title/description provided)
      │   ├── [Snippet: header]  (custom header content, replaces built-in title/description)
      │   ├── [Title]  <strong>  (built-in, when no header snippet)
      │   └── [Description]  <p>  (built-in, when no header snippet)
      ├── [Body .dialog__body]  <div>
      │   └── [Snippet: children]
      ├── [Footer .dialog__footer]  <div> (optional, when footer snippet used — replaces actions)
      │   └── [Snippet: footer]
      └── [Actions .dialog__actions]  <div> (optional, when actions snippet used and no footer snippet)
          └── [Snippet: actions]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Wrapper | yes | fixed full-viewport overlay container | z-index, padding, data-size, data-density, data-width |
| Backdrop | yes | background scrim and interaction block | overlay background, cursor |
| Surface | yes | modal content container | border, radius, background, elevation, padding, max sizing, width presets |
| Close Button | no | dismiss affordance in the top-right corner; receives `resolvedSize` and uses `sizeRole="chrome"` | position, z-index |
| Header | no | title and description region, or custom header snippet content | gap, margin, typography |
| Body | yes | primary content area | min-width |
| Footer | no | custom footer region (replaces actions when present) | margin |
| Actions | no | action button row (only when footer snippet is not used) | flex layout, gap, margin |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null \| undefined` | `undefined` | no | dialog visibility; bindable, and also reported through `onOpenChange`; omit the prop for uncontrolled mode |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `title` | `string \| null` | `null` | no | visible title text; ignored when `header` snippet is used |
| `description` | `string \| null` | `null` | no | visible supporting description; ignored when `header` snippet is used |
| `role` | `"dialog" \| "alertdialog"` | `"dialog"` | no | semantic ARIA role for the surface |
| `kind` | `"dialog" \| "alertdialog" \| undefined` | `undefined` | no | **deprecated** — legacy alias for `role`; when provided, overrides `role` |
| `width` | `"sm" \| "md" \| "lg" \| "xl" \| "full"` | `"md"` | no | surface width preset |
| `bare` | `boolean` | `false` | no | when true, surface has no padding or internal structure; consumers control all layout |
| `dismissOnEscape` | `boolean` | `true` | no | whether Escape key dismisses the dialog |
| `dismissOnBackdrop` | `boolean` | `true` | no | whether backdrop click dismisses the dialog |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible title exists |
| `contentClassName` | `string` | `""` | no | additional class name added to the dialog surface |
| `contentStyle` | `string` | `""` | no | additional inline style applied to the dialog surface |
| `overlayClassName` | `string` | `""` | no | additional class name added to the backdrop button |
| `showCloseButton` | `boolean` | `false` | no | whether to render the built-in close button |
| `closeLabel` | `string` | `"Close dialog"` | no | accessible label for the close button |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |

### Width Presets

| Width | CSS Value |
|-------|-----------|
| `"sm"` | `min(24rem, 100%)` (384px) |
| `"md"` | `min(34rem, 100%)` (544px) — default |
| `"lg"` | `min(48rem, 100%)` (768px) |
| `"xl"` | `min(64rem, 100%)` (1024px) |
| `"full"` | `100%` |

### Snippets

| Snippet | Purpose |
|------|---------|
| `children` | body content (or full content when `bare=true`) |
| `header` | custom header content; replaces built-in title/description when present |
| `footer` | custom footer content; replaces the actions row when present |
| `actions` | action button row (renders `.dialog__actions` wrapper when present and no `footer` snippet) |

### Controlled And Uncontrolled

- controlled: supplying `open` makes it host-owned through `onOpenChange`
- uncontrolled: omit `open` and use `defaultOpen`

> **`bind:open` works.** The Dialog writes the new value back through the
> binding and then calls `onOpenChange`, matching every other bindable Poodle
> component.
>
> A host that wants to *refuse* a close — unsaved changes, a job in flight —
> re-asserts `open` inside `onOpenChange`. That assignment lands after the
> write-back, so the dialog stays up and no intermediate state is rendered.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | entire dialog tree unmounted from DOM |
| open | `open=true` or triggered open | wrapper, backdrop, and surface rendered |

### Component States

- Dialog is conditionally rendered with `{#if isOpen}` (full mount/unmount)
- No intermediate opening/closing animation states in current implementation
- Body scroll locked while open (`document.body.style.overflow = "hidden"`)

### Behavior Machine

Behavior classification: machine-backed (`modalTransition` in
`@poodle/headless`)

Modal overlay machine shared by Dialog, AlertDialog (which composes Dialog),
and Drawer.

- States: `closed` | `open`
- Context: `dismissOnEscape`, `dismissOnBackdrop`; open state is
  controllable (controlled mode writes `open` back through the binding, then
  emits `emitOpenChange`; a host refuses a close by re-asserting `open` in the
  handler)
- Events: `OPEN` / `CLOSE` (programmatic), `REQUEST_CLOSE` (close button or
  caller), `ESCAPE` (dismissable-layer stack), `BACKDROP_CLICK`
- Transitions: user-initiated close paths (`REQUEST_CLOSE`, guarded
  `ESCAPE`, guarded `BACKDROP_CLICK`) emit `emitRequestClose` before
  `emitOpenChange(false)`, preserving the onRequestClose -> onOpenChange
  ordering; programmatic `CLOSE` skips `emitRequestClose`
- Effects on open: `saveFocusAndEnter` (store the previously focused
  element, focus the first focusable in the surface or the surface itself),
  `lockBodyScroll`; on close: `unlockBodyScroll`, `restoreFocus`
- Focus trap: shared `trapFocusKeydown` machinery — Tab wraps last->first
  and first->last; with no focusable children focus pins to the surface
- Machinery dependencies: dismissable-layer stack (escape targets the
  innermost open layer; when `dismissOnEscape` is false the modal still
  occupies the stack, so escape is swallowed rather than reaching outer
  layers — modal semantics), focus (`getFocusableElements`,
  `trapFocusKeydown`).

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | dialog opens or closes | `boolean` | state ownership callback |
| `onRequestClose` | user attempts dismissal (escape or backdrop) | `void` | runs before `onOpenChange(false)` |

## 6. Accessibility

### Semantics

- Surface: `role` set to `role` prop value (`"dialog"` or `"alertdialog"`); legacy `kind` prop overrides when provided
- Surface: `aria-modal="true"`
- Surface: `aria-labelledby` pointing at the rendered `.dialog__title` element
  whenever a `title` is present — this is how a titled dialog gets its
  accessible name
- Surface: `aria-label` from prop only when there is no `title` to reference
  (including when a custom `header` snippet replaces the title element)
- Surface: `tabindex="-1"` for programmatic focus
- Backdrop: `<button>` with `aria-label="Dismiss dialog backdrop"`
- Close button: `IconButton` with `ariaLabel` from `closeLabel`, `sizeRole="chrome"`, `size={resolvedSize}`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | cycles focus within the surface (focus trap) |
| `Shift+Tab` | reverse-cycles focus within the surface |
| `Escape` | requests dismissal when `dismissOnEscape=true` |

### Focus And Announcement

- focus entry: on open, focus moves to first focusable element within surface;
  if none found, surface itself receives focus (via tabindex="-1")
- focus trap: Tab/Shift+Tab cycle is constrained to the surface; when no
  focusable elements exist, Tab is prevented and surface is re-focused
- focus restoration: on close, focus returns to the element that was focused
  before the dialog opened (`document.activeElement` captured at open time)
- body scroll lock: `document.body.style.overflow` saved and set to `"hidden"`
  on open, restored on close and on component unmount
- escape listener: global `keydown` listener added on mount, removed on
  unmount; checks `isOpen` and `dismissOnEscape` before firing
- GPUI-native accessibility mapping notes: GPUI must create a true modal
  accessible subtree, mark background content unavailable to assistive
  technology while blocked, and preserve deterministic focus restoration on
  close

## 7. Layout

### Sizing

- Wrapper: fixed full-viewport overlay with 2rem padding for safe area
- Surface: width controlled by `width` prop preset (default `min(34rem, 100%)`),
  height constrained to `min(80vh, 42rem)` with overflow auto
- Content flows vertically: header, body, actions/footer
- When `bare=true`, surface has no padding and no internal structure — the
  `children` snippet renders directly inside the surface

### Composition

- parent expectations: confirmation flows, settings sheets, focused tasks
- child expectations: structured header/body/footer content via props and snippets;
  or fully custom layout via `bare` mode
- resizing: surface respects viewport constraints; overflow scrolls within
  surface

## 8. Token Usage — Exact Values

### Wrapper `.dialog`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `inset` | `0` |
| `z-index` | `var(--poodle-overlay-z-dialog)` |
| `display` | `grid` |
| `place-items` | `center` |
| `padding` | `2rem` |

### Backdrop `.dialog__backdrop`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `var(--poodle-color-background-overlay)` |
| `cursor` | `default` |

### Surface `.dialog__surface`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `z-index` | `1` |
| `width` | `min(34rem, 100%)` (default, overridden by width preset) |
| `max-height` | `min(80vh, 42rem)` |
| `overflow` | `auto` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-treatment-surface-elevated-border, color-mix(in srgb, var(--poodle-color-border-default) 78%, transparent))` |
| `border-radius` | `var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface))` |
| `background` | `var(--poodle-treatment-surface-elevated-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel)))` |
| `--poodle-surface` | `var(--poodle-treatment-surface-elevated-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel)))` |
| `box-shadow` | `var(--poodle-treatment-surface-elevated-shadow, var(--poodle-elevation-dialog))` |

### Surface bare modifier `.dialog__surface--bare`

| Property | Value |
|----------|-------|
| `padding` | `0` |

### Width presets (on `.dialog[data-width]`)

| Width | CSS Rule |
|-------|----------|
| `sm` | `.dialog__surface { width: min(24rem, 100%); }` |
| `md` | (default — `min(34rem, 100%)` from base rule) |
| `lg` | `.dialog__surface { width: min(48rem, 100%); }` |
| `xl` | `.dialog__surface { width: min(64rem, 100%); }` |
| `full` | `.dialog__surface { width: 100%; }` |

### Close Button `.dialog__close`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `var(--poodle-space-inline-sm)` |
| `right` | `var(--poodle-space-inline-sm)` |
| `z-index` | `1` |

The close button renders an `IconButton` with `icon="x"`, `variant="ghost"`, `sizeRole="chrome"`, and `size={resolvedSize}`.

### Header `.dialog__header`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |
| `margin-bottom` | `var(--poodle-space-stack-md)` |

### Header title (`.dialog__header strong`)

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-heading-family)` |
| `font-size` | `1rem` |
| `line-height` | `1.2` |

### Header description (`.dialog__header p`)

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |

### Body `.dialog__body`

| Property | Value |
|----------|-------|
| `min-width` | `0` |

### Actions `.dialog__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `justify-content` | `flex-end` |
| `margin-top` | `var(--poodle-space-stack-lg)` |

### Footer `.dialog__footer`

| Property | Value |
|----------|-------|
| `margin-top` | `var(--poodle-space-stack-lg)` |

### Size adjustments

| Size | header title font-size | header description font-size |
|------|----------------------|---------------------------|
| `xs` | `0.8125rem` | `0.75rem` |
| `sm` | `0.875rem` | (default) |
| `md` | `1rem` | (default) |
| `lg` | `1.0625rem` | (default) |
| `xl` | `1.125rem` | (default) |

### Density adjustments (applied to surface padding, not body)

| Density | Surface padding | Bare surface |
|---------|----------------|-------------|
| `compact` | `0.5rem 0.75rem` | `0` (override preserved) |
| `default` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` | `0` |
| `comfortable` | `1rem 1.25rem` | `0` (override preserved) |

## 9. Svelte Notes

- `data-size` attribute on wrapper reflects the resolved size
- `data-density` attribute on wrapper reflects the resolved density (`compact`, `default`, or `comfortable`)
- `data-width` attribute on wrapper reflects the width preset (`sm`, `md`, `lg`, `xl`, `full`)
- `role` prop replaces the deprecated `kind` prop; `kind` is still accepted as a
  legacy alias and overrides `role` when provided (`effectiveRole = kind ?? role`)
- No `<dialog>` element used; modal behavior implemented with fixed overlay,
  manual focus trap, and body scroll lock
- Backdrop is a `<button>` element (not a div) for click handling
- Focus trap implemented via `trapFocus` keydown handler on the surface:
  uses `getFocusableElements()` from `./internal` to find tabbable children
- Global escape listener registered via `onMount` lifecycle, cleaned up on
  unmount
- Body overflow saved/restored on open/close and on component teardown
- `previousOpen` reactive variable tracks transitions to detect open/close
  edges
- Surface uses `bind:this` for DOM reference needed by focus trap
- Entire dialog tree conditionally rendered with `{#if isOpen}` (mount/unmount)
- Close button rendered only when `showCloseButton=true`; receives `size={resolvedSize}` and `sizeRole="chrome"`
- Header rendered when the `header` snippet is used OR when
  `title`/`description` is provided
  - When the `header` snippet is present, it replaces the built-in
    title/description
  - Built-in title/description render only when no `header` snippet is used
- Footer snippet renders `.dialog__footer` wrapper; when present, suppresses
  the actions snippet entirely
- Actions wrapper rendered only when the `actions` snippet is used and no
  `footer` snippet is present
- When `bare=true`, the `children` snippet renders directly inside the surface
  with no header, body, actions, or footer wrappers

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::dialog`
- GPUI implementation must explicitly own modal stacking, focus trapping,
  background blocking, announcement, and restoration behavior
- Dialog and alertdialog roles must be correctly exposed
- Background inertness must be enforced (not just visual overlay)
- Focus trap must handle edge cases: empty surface, single focusable element
- Body scroll lock equivalent required in GPUI context
- Width presets must be mapped to equivalent pixel constraints

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] dialog/alertdialog role and aria-modal match
- [ ] accessible name from title or ariaLabel matches
- [ ] focus trap behavior matches (Tab cycling, empty surface handling)
- [ ] focus restoration to previously focused element matches
- [ ] escape dismissal behavior matches (respects dismissOnEscape)
- [ ] backdrop dismissal behavior matches (respects dismissOnBackdrop)
- [ ] body scroll lock while open matches
- [ ] onOpenChange and onRequestClose callback payloads match
- [ ] controlled and uncontrolled modes match
- [ ] bare mode disables padding and internal structure

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] all five width presets match (sm/md/lg/xl/full)
- [ ] backdrop overlay color matches (background-overlay)
- [ ] surface border uses treatment-surface-elevated-border fallback matches
- [ ] surface background uses treatment-surface-elevated-fill fallback matches
- [ ] surface elevation shadow uses treatment-surface-elevated-shadow fallback matches
- [ ] surface border-radius uses treatment-surface-elevated-radius fallback matches
- [ ] surface padding uses space-panel-y and space-panel-x
- [ ] surface max-height constraint (min(80vh, 42rem)) matches
- [ ] header gap (0.375rem) and margin-bottom (space-stack-md) match
- [ ] title typography (heading-family, 1rem, 1.2) matches
- [ ] description color (text-secondary) and margin reset match
- [ ] actions flex layout (wrap, flex-end, space-inline-sm gap) matches
- [ ] actions margin-top (space-stack-lg) matches
- [ ] footer margin-top (space-stack-lg) matches
- [ ] wrapper z-index uses overlay-z-dialog
- [ ] wrapper padding (2rem) matches
- [ ] density variants adjust surface padding correctly
- [ ] density variants preserve bare mode (padding: 0)

### Tier 3: Implementation Freedom

- [ ] focus trap implementation internals are platform-owned
- [ ] body scroll lock mechanism is platform-owned
- [ ] mount/unmount vs show/hide is platform-owned
- [ ] escape listener registration mechanism is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact transition timing may differ slightly | runtime animation systems differ | allowed | keep modality, focus trap, and dismissal semantics strict |
| CSS color-mix vs GPUI color blending | different color systems per platform | allowed | same visual result required |
| backdrop as button vs div with click handler | semantic choice for click handling | allowed | backdrop dismissal behavior must match |
| getFocusableElements utility internals | focus detection mechanism varies | allowed | Tab cycling behavior must match |
| treatment-surface-elevated tokens with fallbacks | GPUI may resolve tokens differently | allowed | visual result must match reference |

## 13. Specimen Definitions

All preview apps must render the following specimens identically. Each dialog is triggered by a button in the specimen page.

### Basic dialog

Triggered by "Open dialog" button:

| Property | Value |
|----------|-------|
| Title | Confirm action |
| Description | Are you sure you want to proceed? This action cannot be undone. |
| Actions | Cancel (secondary), Confirm (primary) |

### Alert dialog

Triggered by "Open alert" button:

| Property | Value |
|----------|-------|
| Title | Delete item? |
| Description | This will permanently remove the item and all associated data. |
| Actions | Cancel (secondary), Delete (primary, danger tone) |

### No backdrop dismiss

Triggered by "Open persistent" button:

| Property | Value |
|----------|-------|
| Title | Persistent dialog |
| Description | This dialog can only be closed via the buttons or Escape key. |
| Actions | Got it (primary) |
| `dismissOnBackdrop` | `false` |

### Width presets

A row of trigger buttons ("SM", "MD", "LG", "XL", "Full") each opening the
same dialog content at the corresponding `width` preset.

### Bare dialog

Triggered by "Open bare" button:

| Property | Value |
|----------|-------|
| `bare` | `true` |
| Content | Custom layout with no structural padding or wrappers |

### Custom header and footer

Triggered by "Open custom" button:

| Property | Value |
|----------|-------|
| `header` snippet | Custom header with icon and styled title |
| `footer` snippet | Custom footer with status text and action buttons |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings shells, confirmations, focused task flows
- future follow-up: connect wizard and multi-step composite flows in later
  milestones; coordinate with Drawer for edge-anchored modal patterns

> **Surface elevation**: Dialog is a surface creator — see [surface-elevation.md](./surface-elevation.md).

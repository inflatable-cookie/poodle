# ToastHost

Status: detailed contract
Updated: 2026-09-02

## 1. Purpose

- Component name: `ToastHost`
- Layer: `composites`
- Summary: a small store-aware runtime host over the presentational `ToastStack`
  component; manages fixed-position viewport placement, auto-dismiss timers,
  sticky toast treatment, and store-to-stack item mapping
- In scope: store subscription, fixed-position viewport host, auto-dismiss
  timers, sticky toast treatment via tone or explicit flag, dismiss wiring,
  action callback passthrough, placement variants, responsive narrow-viewport
  treatment, variant-to-tone normalization
- Out of scope: toast creation API, a second creation helper, persistence,
  cross-tab sync, app-specific retry or cancellation, promise ownership,
  pending/settled lifecycle fields, progress anatomy, toast animation (owned
  by ToastStack)

## 2. Anatomy

```text
[Host <div>]  (fixed-position viewport wrapper, only rendered when items exist)
  └── [ToastStack]
        └── [Toast...]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Host | `<div>` | Class `poodle-toast-host`, `data-placement` attribute, `position: fixed`, `z-index: 80` |
| ToastStack | `ToastStack` | Receives normalized items, ariaLabel, size, sizeRole, density props |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `store` | `ToastHostStore` | -- | yes | Store providing readable toasts plus `dismiss(id)` |
| `autoDismissMs` | `number` | `6000` | no | Auto-dismiss delay for non-sticky toasts; `<= 0` disables new timers. `6000` is the default fixture, not universal settlement law. |
| `stickyTones` | `ToastTone[]` | `["danger"]` | no | Tones that stay sticky without `sticky: true`. `["danger"]` is the default fixture, not universal settlement law. |
| `placement` | `"bottom-end" \| "bottom-start" \| "top-end" \| "top-start"` | `"bottom-end"` | no | Viewport placement |
| `ariaLabel` | `string` | `"Notifications"` | no | Forwarded to `ToastStack` |
| `size` | `ControlSize \| null` | `null` | no | Forwarded to `ToastStack` |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | Forwarded to `ToastStack` |
| `density` | `ControlDensity \| null` | `null` | no | Forwarded to `ToastStack` |
| `onAction` | `((id: string) => void) \| null` | `null` | no | Optional callback when a toast action button fires |
| `onDismiss` | `((id: string) => void) \| null` | `null` | no | Optional callback after a toast is dismissed from the store |

### Types

```ts
type ToastHostPlacement = "bottom-end" | "bottom-start" | "top-end" | "top-start";

type ToastHostStoreItem = {
  id: string;
  title?: string;
  message: string;
  tone?: ToastTone;
  variant?: "info" | "success" | "warning" | "error" | "danger";
  actionLabel?: string | null;
  sticky?: boolean;
};

type ToastHostStore = {
  toasts: Readable<ToastHostStoreItem[]>;
  dismiss: (id: string) => void;
};

type ToastItem = {
  id: string;
  title: string;
  message?: string | null;
  tone?: ToastTone;
  actionLabel?: string | null;
};
```

### Slots

None.

### Controlled And Uncontrolled

Items are externally driven via the store. Auto-dismiss timers are managed
internally but can be configured via `autoDismissMs` and `stickyTones`.
Pending versus settled is consumer operation state plus those fields; the
host stores no lifecycle enum.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | Store has no toasts | Host `<div>` is not rendered at all |
| populated | Store has one or more toasts | Host renders at viewport placement with ToastStack |
| auto-dismissing | Non-sticky toast present and `autoDismissMs > 0` | One clock from first non-sticky appearance using the current configured delay; toast auto-dismissed after that delay |
| sticky | Toast has `sticky: true` or tone is in the current `stickyTones` | No clock; remains until explicitly dismissed |

### Internal State

- `items: ToastItem[]` -- normalized toast items derived from store subscription
- `timers: Map<string, ReturnType<typeof setTimeout>>` -- active auto-dismiss timers

### Behavior Machine

Behavior classification: machine-backed (toast machinery in
`@inflatable-cookie/poodle-core`)

Pure host machinery; the adapter owns real timers and the store
subscription.

- `resolveToastTone`: explicit `tone` wins; `variant` maps
  error->danger, warning->warning, success->success; default `info`
- `normalizeToast`: title falls back to message then "Notification";
  message is kept only alongside a real title
- `isToastSticky`: explicit `sticky: true`, or tone listed in the current
  `stickyTones`. There is no `pending | settled` field.
- `uniqueToastInputs(next)`: one live row per `id`. First occurrence keeps
  position; last occurrence wins copy, tone, action, and sticky fields.
- `reconcileToastTimers(runningIds, next, { autoDismissMs, stickyTones })`
  returns a plan `{ clear, start }` over the unique snapshot:
  - sticky rows own no clock
  - clear timers whose toasts left the store, or whose live row became sticky
  - start exactly one timer for a newly non-sticky row when `autoDismissMs > 0`,
    using that current configured delay — never a baked-in `6000`
  - copy, tone, or action churn on a running non-sticky row does not reset
    the clock
  - changing `autoDismissMs` preserves an existing non-sticky clock; changing
    `stickyTones` (or `sticky`) clears when the row becomes sticky and starts
    the current delay when a sticky row becomes non-sticky
  - non-positive `autoDismissMs` starts nothing; removal clears the clock
- Same-id replacement keeps the live row. Reuse after ToastStack exit cleanup
  is a new record.
- Machinery dependencies: none; ToastStack remains a presentational list
  (styled-only).

## 5. Callbacks

| Callback | When It Fires | Signature | Notes |
|----------|---------------|-----------|-------|
| `onDismiss` | After host dismiss wiring runs (manual or auto) | `(id: string) => void` | Store dismiss is called first, then callback runs |
| `onAction` | Toast action button pressed | `(id: string) => void` | Called when a toast action button is activated |

## 6. Accessibility

- Relies on `ToastStack` for live-region semantics (`aria-live`, `role="list"`) and per-toast dismissal
- Viewport placement must not hide the stack behind application chrome
- A row that is sticky under the current configuration remains until explicitly dismissed
- Persistent failures remain reachable in a durable host surface; a toast must
  not be the only record of an unresolved problem (spec 015)
- `ariaLabel` forwarded to ToastStack for the `<section>` landmark

## 7. Layout

### Host Positioning

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `z-index` | `80` |
| `width` | `min(28rem, calc(100vw - 2rem))` |

### Placement Variants

| Placement | Positioning |
|-----------|-------------|
| `bottom-end` | `right: 1rem; bottom: 1rem` |
| `bottom-start` | `left: 1rem; bottom: 1rem` |
| `top-end` | `right: 1rem; top: 1rem` |
| `top-start` | `left: 1rem; top: 1rem` |

### Narrow Viewport (`@media (max-width: 40rem)`)

| Property | Value |
|----------|-------|
| `width` | `calc(100vw - 1rem)` |

For bottom placements: `left: 0.5rem; right: 0.5rem; bottom: 0.5rem; width: auto`

For top placements: `left: 0.5rem; right: 0.5rem; top: 0.5rem; width: auto`

### Composition

- Parent expectations: mounted at application root level, outside normal layout flow
- Child expectations: ToastStack with normalized items

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-placement` | Host `<div>` | `"bottom-end"`, `"bottom-start"`, `"top-end"`, `"top-start"` |

### `.poodle-toast-host`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `z-index` | `80` |
| `width` | `min(28rem, calc(100vw - 2rem))` |

### `.poodle-toast-host[data-placement="bottom-end"]`

| Property | Value |
|----------|-------|
| `right` | `1rem` |
| `bottom` | `1rem` |

### `.poodle-toast-host[data-placement="bottom-start"]`

| Property | Value |
|----------|-------|
| `left` | `1rem` |
| `bottom` | `1rem` |

### `.poodle-toast-host[data-placement="top-end"]`

| Property | Value |
|----------|-------|
| `right` | `1rem` |
| `top` | `1rem` |

### `.poodle-toast-host[data-placement="top-start"]`

| Property | Value |
|----------|-------|
| `left` | `1rem` |
| `top` | `1rem` |

### Narrow Viewport Override (`@media (max-width: 40rem)`)

#### `.poodle-toast-host`

| Property | Value |
|----------|-------|
| `width` | `calc(100vw - 1rem)` |

#### Bottom placements

| Property | Value |
|----------|-------|
| `left` | `0.5rem` |
| `right` | `0.5rem` |
| `bottom` | `0.5rem` |
| `width` | `auto` |

#### Top placements

| Property | Value |
|----------|-------|
| `left` | `0.5rem` |
| `right` | `0.5rem` |
| `top` | `0.5rem` |
| `width` | `auto` |

### Light Theme Overrides

None (styling is minimal; visual treatment is owned by ToastStack).

## 9. Svelte Notes

- Host `<div>` is conditionally rendered only when `items.length > 0`
- Store subscription + timer cleanup run via two `$effect` blocks: one subscribes to `store.toasts` and unsubscribes on its teardown return; the other clears all active timers on teardown (no `onMount`/`onDestroy`)
- `resolveTone` normalizes the legacy `variant` field to `tone`, but only `error`/`warning`/`success` are branched:
  - explicit `tone` always wins
  - `variant="error"` maps to `tone="danger"`
  - `variant="warning"` maps to `tone="warning"`
  - `variant="success"` maps to `tone="success"`
  - all others (including `variant="info"` and `variant="danger"`) default to `"info"` — note `variant="danger"` is NOT normalized to danger tone; only an explicit `tone` or `variant="error"` yields danger
- `normalizeToast` maps store items to ToastStack items:
  - `title` falls back to `toast.title?.trim() || toast.message || "Notification"` — when both title and message are empty, the literal `"Notification"` is used
  - when there is no explicit trimmed `title`, `message` becomes the `title` and the detail message is set to `null`
  - `actionLabel` passed through (defaults to `null`)
- `isSticky` checks `toast.sticky === true` or tone membership in `stickyTones`
- Store snapshots are uniqued by `id` before mapping and timer reconcile
- `reconcileTimers` applies the core plan: clear departed or become-sticky
  clocks, start one current-delay clock for newly non-sticky rows, preserve a
  running non-sticky clock across copy/tone/action churn and `autoDismissMs`
  changes, and start nothing when `autoDismissMs` is not positive. Config
  changes re-run the same plan against the current snapshot.
- `handleDismiss`/`handleAction` forward into `ToastStack` and invoke the optional `onDismiss`/`onAction` callbacks
- Forwards `size`, `sizeRole`, `density`, `ariaLabel` to `ToastStack`

## 10. Native Notes

- `packages/render/src/toast_host.rs` owns the shared positioned container and
  composes the real `ToastStack` node.
- GPUI and Jetstream backends interpret the same placement, accessibility, and
  action metadata.
- Store subscription, timer scheduling, and overlay mounting remain host-owned
  runtime concerns. Native render does not own clocks.
- Same-id uniqueness and sticky/timer laws are web-host machinery. Native
  danger severity is ToastStack's `NodeRole` mapping, not a ToastHost timer API.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] store subscription and item normalization logic matches
- [ ] variant-to-tone mapping matches
- [ ] sticky determination logic matches (explicit sticky flag + current stickyTones)
- [ ] auto-dismiss timer behavior matches, including become-sticky clear and
      become-non-sticky start of the current configured delay
- [ ] same-id uniqueness keeps one live row
- [ ] `onDismiss` and `onAction` callback payloads match
- [ ] host only renders when items exist

### Tier 2: Visual Parity

- [ ] placement positioning matches all four corners
- [ ] host width constraint matches (`min(28rem, ...)`)
- [ ] narrow viewport responsive behavior matches
- [ ] z-index matches

### Tier 3: Implementation Freedom

- [ ] timer implementation details stay internal
- [ ] store subscription mechanism is platform-owned

## 12. Specimen Definitions

### Default (Bottom-End)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bottom-end placement | Store with success toast ("Saved", "Your changes were saved.") and error toast ("Publishing failed."), `placement="bottom-end"` | Toast stack anchored to bottom-right viewport corner; error toast persists under default `stickyTones`, success toast auto-dismisses |

### Same-Id Pending To Settled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Same-id settle | One store id starts sticky pending ("Publishing…") then upserts success ("Published") without a percent in toast copy | The same visual row stays; enter motion does not restart; Progress is not inside the toast |

### Top-Start Placement

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Top-start placement | Store with info toast, `placement="top-start"` | Toast stack anchored to top-left viewport corner |

### With Action

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Action toast | Store with toast including `actionLabel="Retry"`, `onAction` handler wired | Toast with action button; clicking calls the action callback |

# ToastHost

Status: seed contract
Updated: 2026-03-28

## 1. Purpose

- Component name: `ToastHost`
- Layer: `composites`
- Summary: a small store-aware runtime host over presentational `ToastStack`
- In scope: store subscription, fixed-position viewport host, auto-dismiss
  timers, sticky toast treatment, dismiss wiring, action event passthrough
- Out of scope: toast creation API, persistence, cross-tab sync, app-specific
  retry logic

## 2. Anatomy

```text
[ToastHost viewport]
  └── [ToastStack]
        └── [Toast...]
```

| Part | Required | Description |
|------|----------|-------------|
| Host | yes | fixed-position viewport wrapper |
| Stack | yes | presentational `ToastStack` fed from store items |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `store` | `ToastHostStore` | — | yes | store providing readable toasts plus `dismiss(id)` |
| `autoDismissMs` | `number` | `6000` | no | auto-dismiss delay for non-sticky toasts; `<= 0` disables timers |
| `stickyTones` | `ToastTone[]` | `["danger"]` | no | tones that should never auto-dismiss unless explicitly dismissed |
| `placement` | `"bottom-end" \| "bottom-start" \| "top-end" \| "top-start"` | `"bottom-end"` | no | viewport placement |
| `ariaLabel` | `string` | `"Notifications"` | no | forwarded to `ToastStack` |
| `size` | `ControlSize \| null` | `null` | no | forwarded to `ToastStack` |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | forwarded to `ToastStack` |
| `density` | `ControlDensity \| null` | `null` | no | forwarded to `ToastStack` |
| `onAction` | `((id: string) => void) \| null` | `null` | no | optional callback when a toast action button fires |

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
```

## 4. Events

| Event | Payload | Notes |
|-------|---------|-------|
| `dismiss` | `{ id: string }` | fires after host dismiss wiring runs |
| `action` | `{ id: string }` | fires when a toast action button is pressed |

## 5. Behavior

- subscribes directly to the provided store
- maps store items onto `ToastStack`
- treats `variant="error"` as `tone="danger"`
- if a toast has no explicit title, the message becomes the stack title and the
  detail message is omitted
- auto-dismiss timers run only for non-sticky toasts
- timers clear when a toast disappears or is manually dismissed
- fixed-position host stays outside normal layout flow

## 6. Accessibility

- relies on `ToastStack` for live-region semantics and per-toast dismissal
- viewport placement must not hide the stack behind application chrome
- sticky danger toasts remain present until explicitly dismissed

## 7. Usage

```svelte
<script lang="ts">
  import { writable } from "svelte/store";
  import { ToastHost, type ToastHostStoreItem } from "@poodle/svelte-composites";

  const toasts = writable<ToastHostStoreItem[]>([
    { id: "1", variant: "success", title: "Saved", message: "Your changes were saved." },
    { id: "2", variant: "error", message: "Publishing failed." }
  ]);

  const store = {
    toasts,
    dismiss(id: string) {
      toasts.update((items) => items.filter((item) => item.id !== id));
    }
  };
</script>

<ToastHost {store} />
```

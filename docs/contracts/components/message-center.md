# MessageCenter

Status: active contract
Updated: 2026-08-14

## 1. Purpose

- Component name: `MessageCenter`
- Layer: composite
- Summary: an always-available icon trigger and popover activity surface for durable messages (notifications, mentions, job results) and live activity rows (running jobs, in-flight operations)
- Composes: `IconButton`, `Popover`, `Button`, `Icon`, `TimeAgo`, `EmptyState`, `Progress`
- In scope: unread indication, read/unread requests, item selection, removal, mark-all-read, empty state, bounded scrolling, per-item interaction policies, determinate/indeterminate progress presentation
- Out of scope: toast delivery, message transport, persistence, ordering, pagination, automatic read-state mutation, job lifecycle transitions, cancellation, and retry policy

`MessageCenter` defaults to notification language and a bell icon. Its item
contract stays generic — hosts can also represent direct messages, mentions,
system events, job results, and live operations. Durable messages and live
rows share one surface; Poodle never joins the two streams. Hosts own
ordering, persistence, and the replacement of a terminal live row with its
retained result notification.

## 2. Anatomy

```text
MessageCenter
├── Popover
│   ├── Trigger
│   │   ├── IconButton
│   │   └── Unread count indicator (when unread count > 0)
│   └── Surface
│       ├── Header
│       │   ├── Title + unread summary
│       │   └── Mark all read (optional)
│       ├── Item list
│       │   └── Item row
│       │       ├── Tone icon or unread dot
│       │       ├── Title, body, meta, time
│       │       ├── Progress (optional, when item carries progress)
│       │       └── Read toggle + remove actions (per-item policy)
│       └── EmptyState
```

## 3. Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `items` | `MessageCenterItem[]` | `[]` | Caller-owned item list: durable messages and live rows. |
| `open` | `boolean \| null` | `null` | Controlled open state; Svelte supports binding. |
| `defaultOpen` | `boolean` | `false` | Initial uncontrolled state. |
| `title` | `string` | `"Notifications"` | Surface heading and default accessible label. |
| `ariaLabel` | `string \| null` | `null` | Overrides the surface label. |
| `triggerLabel` | `string \| null` | `null` | Overrides the trigger label; otherwise includes unread count. |
| `triggerIcon` | `IconProp` | `"bell"` | Trigger glyph. |
| `placement` | `OverlayPlacement` | `"bottom-end"` | Popover placement hint. |
| `emptyTitle` | `string` | `"No messages"` | Empty-state heading. |
| `emptyMessage` | `string` | `"New messages will appear here."` | Empty-state copy. |
| `size` | `ControlSize \| null` | `null` | Explicit semantic size. |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | Inherited-size role. |
| `density` | `ControlDensity \| null` | `null` | Explicit density. |
| `onOpenChange` | `(open: boolean) => void` | `null` | Open-state request. |
| `onItemSelect` | `(id: string) => void` | `null` | Makes selectable rows actionable. |
| `onReadChange` | `(id: string, read: boolean) => void` | `null` | Requests one item's next read state. |
| `onRemove` | `(id: string) => void` | `null` | Requests removal. |
| `onMarkAllRead` | `() => void` | `null` | Adds the header action when unread items exist. |

### Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onOpenChange` | The popover opens or closes | `boolean` | The next open state |
| `onItemSelect` | A message row is activated | `string` | The item's id. Rows are only interactive when this is supplied **and** the item is selectable |
| `onReadChange` | The read toggle on a row is pressed | `(id: string, read: boolean)` | The id and the **next** read state, not the current one |
| `onRemove` | A row's remove control is pressed | `string` | The item's id. The control renders only when supplied **and** the item is removable |
| `onMarkAllRead` | The Mark-all-read control is pressed | — | The control renders only when supplied and the unread count is above zero |

## 4. Item contract

```ts
type MessageCenterItemProgress = {
  value: number | null;
  max?: number;
  indeterminate?: boolean;
};

type MessageCenterItem = {
  id: string;
  title: string;
  message?: string | null;
  meta?: string | null;
  timestamp?: Date | string | number | null;
  read: boolean;
  tone?: "neutral" | "info" | "success" | "warning" | "danger" | "pending";
  icon?: IconProp | null;
  progress?: MessageCenterItemProgress | null;
  selectable?: boolean;
  removable?: boolean;
  readControl?: boolean;
};
```

- `title` can represent a notification subject, sender, event label, or job name.
- `message` is supporting body copy, clamped to two lines in the archive.
- `meta` can represent a channel, source, workstation, category, or live phase.
- `tone` colors the leading indicator; it does not change announcement behavior.
- Supplying `icon` replaces the default read/unread dot.
- `progress` renders a compact `Progress` bar under the copy. It is a pure
  projection of host-owned values: the component never animates, estimates,
  or mutates it. `max` defaults to `100`. When `indeterminate` is true the
  bar animates without a value; otherwise `value` drives the fill.
- `selectable`, `removable`, and `readControl` default to `true`. Setting one
  to `false` removes that interaction for that item alone, even when the
  matching global callback is supplied. Live rows opt out with
  `selectable: false`, `removable: false`, and `readControl: false`.
- Live rows are expected to arrive with `read: true` so they do not inflate
  the unread count. The component never infers liveness from progress.

## 5. Behavior

- Unread count is derived from `items`; the component never keeps a second unread store.
- The trigger indicator is visual only. The same count is included in the trigger's accessible name.
- Read, remove, select, and mark-all controls emit requests. They do not mutate `items`.
- Selecting a row does not implicitly mark it read. Hosts decide whether viewing and reading are equivalent.
- A non-selectable row is not a button and emits no `onItemSelect`, even when the callback exists.
- Progress, phase, and tone updates render in place from the latest `items`; the component holds no per-row authority.
- The archive is not a live region. Immediate delivery belongs to host-controlled toasts or announcements.
- The item list scrolls inside a bounded surface; the trigger remains usable without activating a dedicated panel.

Behavior classification: composite state plus delegated primitives. `Popover`
owns dismissal and focus behavior; `MessageCenter` owns controlled/uncontrolled
open wiring and derived unread presentation.

## 6. Accessibility

- Trigger label includes the unread count unless the host supplies `triggerLabel`.
- The popover surface is a labelled dialog and receives initial focus.
- Rows are native buttons only when `onItemSelect` exists and the item is selectable.
- Progress bars expose `role="progressbar"` semantics per the `Progress` contract; determinate bars carry `aria-valuemin`/`aria-valuemax`/`aria-valuenow`, indeterminate bars omit them. Progress is labelled with the item title.
- Read and remove controls have item-specific accessible names.
- No interactive element is nested inside another interactive element.
- Escape and outside interaction close through `Popover`; focus returns to the trigger.

## 7. Framework parity

Svelte, React, GPUI, and Jetstream share the same item shape, defaults,
callbacks, read-state semantics, sizing, density, and token usage. The Rust
targets use `MessageCenterSpec` plus the shared `poodle-render` node tree;
open/read/remove/select/mark-all state remains host-owned. This component does
not introduce an Underlay-facing dependency.

## 8. Token Usage

| Property | Token |
|----------|-------|
| interactive row fill | `--poodle-recipe-message-center-item-hover-fill` |
| interactive row radius | `--poodle-radius-control` — the row is full-width inside the rounded surface; the inset focus ring and the hover fill round with it |
| focus ring | `--poodle-border-width-focus`, `--poodle-color-accent-focusRing`, `-0.125rem` inset offset |
| progress track + indicator | per the `Progress` contract §8; the row renders `Progress` at size `xs` |

## 9. Usage

```svelte
<script lang="ts">
  import { MessageCenter, type MessageCenterItem } from "@inflatable-cookie/poodle-svelte";

  let items: MessageCenterItem[] = [
    {
      id: "job-42",
      title: "Mix preview",
      message: "Rendering…",
      meta: "Render queue · 60%",
      read: true,
      tone: "pending",
      progress: { value: 60 },
      selectable: false,
      removable: false,
      readControl: false,
    },
    {
      id: "job-17",
      title: "Uploading stems",
      meta: "Cloud sync",
      read: true,
      tone: "info",
      progress: { value: null, indeterminate: true },
      selectable: false,
      removable: false,
      readControl: false,
    },
    {
      id: "build-42",
      title: "Render complete",
      message: "Mix preview 42 is ready for review.",
      timestamp: Date.now(),
      read: false,
      tone: "success",
    },
  ];
</script>

<MessageCenter
  {items}
  onReadChange={(id, read) => items = items.map((item) => item.id === id ? { ...item, read } : item)}
  onRemove={(id) => items = items.filter((item) => item.id !== id)}
  onMarkAllRead={() => items = items.map((item) => ({ ...item, read: true }))}
/>
```

When a live job reaches a terminal state, the host removes its live row and
supplies the retained result notification; Poodle performs neither the
removal nor the replacement.

# MessageCenter

Status: active contract
Updated: 2026-08-11

## 1. Purpose

- Component name: `MessageCenter`
- Layer: composite
- Summary: an always-available icon trigger and popover archive for notifications, operator messages, mentions, and similar durable message streams
- Composes: `IconButton`, `Popover`, `Button`, `Icon`, `TimeAgo`, `EmptyState`
- In scope: unread indication, read/unread requests, item selection, removal, mark-all-read, empty state, bounded scrolling
- Out of scope: toast delivery, message transport, persistence, ordering, pagination, and automatic read-state mutation

`MessageCenter` defaults to notification language and a bell icon. Its item
contract stays message-shaped so hosts can also represent direct messages,
mentions, system events, and job results.

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
│       ├── Message list
│       │   └── Message row
│       │       ├── Tone icon or unread dot
│       │       ├── Title, body, meta, time
│       │       └── Read toggle + remove actions
│       └── EmptyState
```

## 3. Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `items` | `MessageCenterItem[]` | `[]` | Caller-owned message archive. |
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
| `onItemSelect` | `(id: string) => void` | `null` | Makes message bodies actionable. |
| `onReadChange` | `(id: string, read: boolean) => void` | `null` | Requests one item's next read state. |
| `onRemove` | `(id: string) => void` | `null` | Requests removal. |
| `onMarkAllRead` | `() => void` | `null` | Adds the header action when unread items exist. |

### Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onOpenChange` | The popover opens or closes | `boolean` | The next open state |
| `onItemSelect` | A message row is activated | `string` | The item's id. Rows are only interactive when this is supplied |
| `onReadChange` | The read toggle on a row is pressed | `(id: string, read: boolean)` | The id and the **next** read state, not the current one |
| `onRemove` | A row's remove control is pressed | `string` | The item's id. The control renders only when supplied |
| `onMarkAllRead` | The Mark-all-read control is pressed | — | The control renders only when supplied and the unread count is above zero |

## 4. Item contract

```ts
type MessageCenterItem = {
  id: string;
  title: string;
  message?: string | null;
  meta?: string | null;
  timestamp?: Date | string | number | null;
  read: boolean;
  tone?: "neutral" | "info" | "success" | "warning" | "danger" | "pending";
  icon?: IconProp | null;
};
```

- `title` can represent a notification subject, sender, or event label.
- `message` is supporting body copy, clamped to two lines in the archive.
- `meta` can represent a channel, source, workstation, or category.
- `tone` colors the leading indicator; it does not change announcement behavior.
- Supplying `icon` replaces the default read/unread dot.

## 5. Behavior

- Unread count is derived from `items`; the component never keeps a second unread store.
- The trigger indicator is visual only. The same count is included in the trigger's accessible name.
- Read, remove, select, and mark-all controls emit requests. They do not mutate `items`.
- Selecting a row does not implicitly mark it read. Hosts decide whether viewing and reading are equivalent.
- The archive is not a live region. Immediate delivery belongs to host-controlled toasts or announcements.
- The message list scrolls inside a bounded surface; the trigger remains usable without activating a dedicated panel.

Behavior classification: composite state plus delegated primitives. `Popover`
owns dismissal and focus behavior; `MessageCenter` owns controlled/uncontrolled
open wiring and derived unread presentation.

## 6. Accessibility

- Trigger label includes the unread count unless the host supplies `triggerLabel`.
- The popover surface is a labelled dialog and receives initial focus.
- Rows are native buttons only when `onItemSelect` exists.
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

## 9. Usage

```svelte
<script lang="ts">
  import { MessageCenter, type MessageCenterItem } from "@inflatable-cookie/poodle-svelte";

  let items: MessageCenterItem[] = [
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

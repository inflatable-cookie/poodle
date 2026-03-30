# LogList

Status: active contract
Updated: 2026-03-28

## 1. Purpose

- Component name: `LogList`
- Layer: `composites`
- Summary: a shared operational log surface that supports both streaming runtime logs and audit/activity lists
- In scope:
  - stream-viewer mode with level chips, text search, capped entry retention, and auto-scroll
  - audit-list mode with callback-driven filters, loading/error/empty states, refresh/export controls, pagination, actor/resource linking, and custom entry detail rendering
- Out of scope:
  - log ingestion or persistence
  - server-side query orchestration
  - CSV generation
  - bulk actions or table-style column controls

## 2. Variants

### Stream mode

Use for runtime or console-like logs:

- entries shaped as `{ timestamp, level, message }`
- level chip filtering
- text filtering
- scroll-to-latest behavior

### Audit mode

Use for admin/activity histories:

- entries shaped as `{ occurredAt, actor, action, resourceType, resourceId }`
- callback-driven filter toolbar
- loading, error, and empty states
- optional refresh and export controls
- optional pagination controls
- actor and resource link builders
- optional custom entry details snippet

`variant="auto"` detects the mode from the supplied entry shape.

## 3. Props

### Shared

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `entries` | `LogEntry[]` | `[]` | Stream or audit entries. |
| `variant` | `"auto" \| "stream" \| "audit"` | `"auto"` | Auto detects from the entry shape when not specified. |
| `ariaLabel` | `string` | `"Log output"` | Accessible label for the root region. |
| `size` | `ControlSize \| null` | `null` | Explicit semantic size override. |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | Semantic size role used when inheriting presentation scale. |
| `density` | `ControlDensity \| null` | `null` | Explicit density override. |

### Stream mode

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `maxEntries` | `number` | `500` | Maximum number of stream entries rendered. |
| `autoScroll` | `boolean` | `true` | Auto-scrolls to the latest entry unless the user has scrolled up. |
| `filterLevel` | `LogLevel \| null` | `null` | Active level filter. |
| `filterText` | `string` | `""` | Client-side text filter for stream entries. |

### Audit mode

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `loading` | `boolean` | `false` | Shows loading state when there are no current entries. |
| `error` | `string \| null` | `null` | Error message displayed in audit mode. |
| `emptyMessage` | `string` | `"No log entries found"` | Empty-state text for audit mode. |
| `filters` | `LogFilter[]` | `[]` | Filter definitions for select/date controls. |
| `filterValues` | `Record<string, string>` | `{}` | Current filter values keyed by field. |
| `page` | `number` | `1` | Current page for audit pagination. |
| `pageSize` | `number` | `50` | Page size used for pagination copy. |
| `total` | `number \| undefined` | `undefined` | Total row count; enables pagination when greater than `pageSize`. |
| `onFilterChange` | `(field: string, value: string) => void` | `undefined` | Called when a filter value changes. |
| `onClearFilters` | `() => void` | `undefined` | Clears active filters. |
| `onPageChange` | `(page: number) => void` | `undefined` | Pagination callback. |
| `onRefresh` | `() => void` | `undefined` | Optional refresh action. |
| `onExport` | `() => void` | `undefined` | Optional export action. |
| `actionIcon` | `Snippet<[LogActionType]> \| undefined` | `undefined` | Custom action icon content per audit entry. |
| `entryDetails` | `Snippet<[AuditLogEntry]> \| undefined` | `undefined` | Custom detail rendering below the main audit row. |
| `getActionType` | `(action: string) => LogActionType` | `undefined` | Overrides default action classification. |
| `formatAction` | `(action: string) => string` | `undefined` | Overrides default action label formatting. |
| `formatResourceType` | `(resourceType: string) => string` | `undefined` | Overrides default resource label formatting. |
| `getActorHref` | `(actor: LogActor) => string` | `undefined` | Builds actor links. |
| `getResourceHref` | `(resourceType: string, resourceId: string, action: string) => string \| null` | `undefined` | Builds resource links. |

## 4. Types

```ts
type LogLevel = "info" | "warn" | "error";

type StreamLogEntry = {
  id?: string;
  timestamp: Date | string | number;
  level: LogLevel;
  message: string;
};

type LogActor = {
  id: string;
  email?: string;
  name?: string;
};

type AuditLogEntry = {
  id: string;
  occurredAt: string;
  actor?: LogActor | null;
  action: string;
  resourceType: string;
  resourceId: string;
  resourceLabel?: string;
  details?: Record<string, unknown>;
};

type LogFilter = {
  field: string;
  label: string;
  type: "select" | "date";
  options?: { value: string; label: string }[];
  placeholder?: string;
};

type LogActionType =
  | "create"
  | "update"
  | "delete"
  | "restore"
  | "upload"
  | "login"
  | "logout"
  | "security"
  | "other";

type LogEntry = StreamLogEntry | AuditLogEntry;
```

## 5. Slots

| Slot | Notes |
|------|-------|
| `actionIcon` | Audit mode only. Receives `(actionType)` and replaces the default action marker. |
| `entryDetails` | Audit mode only. Receives `(entry)` and renders below the main row content. |

## 6. Accessibility

- Stream mode uses `role="log"` and supports live-updating output semantics.
- Audit mode uses a labelled region with native form controls and buttons.
- Pagination, refresh, and export actions must remain keyboard accessible.
- Actor and resource links are optional; when absent, plain text is rendered instead.

## 7. Usage

### Audit list

```svelte
<script lang="ts">
  import { LogList, type LogEntry, type LogFilter } from "@poodle/svelte-composites";

  const filters: LogFilter[] = [
    {
      field: "action",
      label: "Action",
      type: "select",
      options: [
        { value: "create", label: "Create" },
        { value: "update", label: "Update" },
        { value: "delete", label: "Delete" },
      ],
    },
  ];

  let filterValues = {};
  const entries: LogEntry[] = [
    {
      id: "1",
      occurredAt: new Date().toISOString(),
      actor: { id: "u-1", name: "Alice" },
      action: "create",
      resourceType: "project",
      resourceId: "p-1",
      resourceLabel: "Launch Plan",
    },
  ];
</script>

<LogList
  {entries}
  {filters}
  {filterValues}
  onFilterChange={(field, value) => {
    filterValues = { ...filterValues, [field]: value };
  }}
  getActorHref={(actor) => `/users/${actor.id}`}
  getResourceHref={(resourceType, resourceId) => `/${resourceType}/${resourceId}`}
/>
```

### Stream viewer

```svelte
<script lang="ts">
  import { LogList } from "@poodle/svelte-composites";

  const entries = [
    { id: "1", level: "info", message: "Server started", timestamp: Date.now() },
    { id: "2", level: "warn", message: "Slow query detected", timestamp: Date.now() },
  ];
</script>

<LogList {entries} variant="stream" autoScroll />
```

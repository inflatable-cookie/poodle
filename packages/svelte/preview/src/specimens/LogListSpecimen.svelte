<script lang="ts">
  import { LogList } from "@inflatable-cookie/poodle-svelte";
  import type { LogEntry, LogFilter } from "@inflatable-cookie/poodle-svelte";
  import { Button } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const now = Date.now();

  let entries: LogEntry[] = $state([
    { id: "1", timestamp: new Date(now - 60000), level: "info", message: "Application started" },
    { id: "2", timestamp: new Date(now - 55000), level: "info", message: "Connecting to database..." },
    { id: "3", timestamp: new Date(now - 54000), level: "info", message: "Database connected (pool: 5)" },
    { id: "4", timestamp: new Date(now - 45000), level: "warn", message: "Slow query detected: SELECT * FROM users (2.3s)" },
    { id: "5", timestamp: new Date(now - 30000), level: "info", message: "HTTP server listening on :3000" },
    { id: "6", timestamp: new Date(now - 20000), level: "error", message: "Failed to fetch /api/analytics: ECONNREFUSED" },
    { id: "7", timestamp: new Date(now - 15000), level: "info", message: "Retrying analytics endpoint..." },
    { id: "8", timestamp: new Date(now - 14000), level: "info", message: "Analytics endpoint recovered" },
    { id: "9", timestamp: new Date(now - 5000), level: "warn", message: "Memory usage at 78% (threshold: 80%)" },
    { id: "10", timestamp: new Date(now), level: "info", message: "Health check passed" },
  ]);

  let counter = 10;
  const auditFilters: LogFilter[] = [
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
    {
      field: "resource_type",
      label: "Resource",
      type: "select",
      options: [
        { value: "user", label: "User" },
        { value: "media", label: "Media" },
        { value: "session", label: "Session" },
      ],
    },
  ];
  const auditEntries: LogEntry[] = [
    {
      id: "audit-1",
      occurredAt: new Date(now - 3600000).toISOString(),
      actor: { id: "u-1", name: "Alice Johnson", email: "alice@example.com" },
      action: "create",
      resourceType: "project",
      resourceId: "project-1",
      resourceLabel: "Spring Launch",
    },
    {
      id: "audit-2",
      occurredAt: new Date(now - 1800000).toISOString(),
      actor: { id: "u-2", name: "Marcus Lee", email: "marcus@example.com" },
      action: "update",
      resourceType: "media",
      resourceId: "media-42",
      resourceLabel: "Campaign Header",
    },
    {
      id: "audit-3",
      occurredAt: new Date(now - 300000).toISOString(),
      action: "delete",
      resourceType: "user",
      resourceId: "user-77",
      resourceLabel: "Legacy operator",
    },
  ];
  let auditFilterValues: Record<string, string> = $state({});

  function addEntry() {
    counter += 1;
    const levels: Array<"info" | "warn" | "error"> = ["info", "info", "info", "warn", "error"];
    const level = levels[Math.floor(Math.random() * levels.length)];
    const msgs: Record<string, string[]> = {
      info: ["Request processed in 42ms", "Cache hit for /api/users", "Scheduled task completed"],
      warn: ["High latency detected (1.5s)", "Disk usage at 85%", "Rate limit approaching"],
      error: ["Connection timeout after 30s", "Invalid token received", "Out of memory in worker 3"],
    };
    const message = msgs[level][Math.floor(Math.random() * msgs[level].length)];
    entries = [...entries, { id: String(counter), timestamp: new Date(), level, message }];
  }

  function handleAuditFilterChange(field: string, value: string) {
    auditFilterValues = {
      ...auditFilterValues,
      [field]: value,
    };
  }

  function clearAuditFilters() {
    auditFilterValues = {};
  }
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Log output with filtering" bare>
    <LogList {entries} ariaLabel="Application logs" />
    <div class="poodle-specimen__actions">
      <Button variant="secondary" onClick={addEntry}>Add log entry</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Audit activity list" bare>
    <LogList
      entries={auditEntries}
      ariaLabel="Audit activity"
      filters={auditFilters}
      filterValues={auditFilterValues}
      onFilterChange={handleAuditFilterChange}
      onClearFilters={clearAuditFilters}
      getActorHref={(actor) => `/users/${actor.id}`}
      getResourceHref={(resourceType, resourceId, action) =>
        action === "delete" ? null : `/${resourceType}/${resourceId}`
      }
    />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__actions {
    display: flex;
    gap: 0.5rem;
  }
</style>

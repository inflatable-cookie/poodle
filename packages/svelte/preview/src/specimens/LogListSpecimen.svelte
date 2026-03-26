<script lang="ts">
  import { LogList } from "@poodle/svelte-composites";
  import type { LogEntry } from "@poodle/svelte-composites";
  import { Eyebrow, Button, UiPresentationProvider } from "@poodle/svelte-primitives";

  const now = Date.now();

  let entries: LogEntry[] = [
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
  ];

  let counter = 10;

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
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Log output with filtering</Eyebrow>
    <LogList {entries} ariaLabel="Application logs" />
    <div class="specimen__actions">
      <Button variant="secondary" on:click={addEntry}>Add log entry</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <LogList {entries} ariaLabel="Compact application logs" />
        <LogList {entries} ariaLabel="Prominent application logs" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__actions {
    display: flex;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>

import { useState } from "react";
import { LogList, type AuditLogEntry, type StreamLogEntry } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const streamEntries: StreamLogEntry[] = [
  { id: "1", timestamp: Date.now() - 3000, level: "info", message: "Server started" },
  { id: "2", timestamp: Date.now() - 2000, level: "warn", message: "Cache miss on warmup" },
  { id: "3", timestamp: Date.now() - 1000, level: "error", message: "Connection refused" },
  { id: "4", timestamp: Date.now(), level: "info", message: "Retrying connection" },
];

const auditEntries: AuditLogEntry[] = [
  {
    id: "a1",
    occurredAt: new Date(Date.now() - 3600_000).toISOString(),
    actor: { id: "u1", name: "Ada" },
    action: "create",
    resourceType: "workspace",
    resourceId: "w1",
    resourceLabel: "Marketing",
  },
  {
    id: "a2",
    occurredAt: new Date(Date.now() - 7200_000).toISOString(),
    actor: null,
    action: "delete",
    resourceType: "api_key",
    resourceId: "k1",
  },
];

registerSpecimen({
  slug: "log-list",
  title: "LogList",
  render: () => (
    <SpecimenSection title="LogList">
      <LogList entries={streamEntries} ariaLabel="Stream logs" />
      <LogList entries={auditEntries} ariaLabel="Audit logs" total={40} pageSize={20} page={1} onPageChange={() => {}} />
    </SpecimenSection>
  ),
});

import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import LogList from "../src/LogList.svelte";
import type { AuditLogEntry, LogFilter, StreamLogEntry } from "../src/types";

const streamEntries: StreamLogEntry[] = [
  { id: "1", timestamp: 1720000000000, level: "info", message: "Server started" },
  { id: "2", timestamp: 1720000001000, level: "info", message: "Health check ok" },
  { id: "3", timestamp: 1720000002000, level: "warn", message: "Slow query detected" },
  { id: "4", timestamp: 1720000003000, level: "error", message: "Connection failed" },
];

const auditEntries: AuditLogEntry[] = [
  {
    id: "a1",
    occurredAt: "2026-08-01T10:00:00Z",
    actor: { id: "u-1", name: "Alice" },
    action: "project.created",
    resourceType: "project",
    resourceId: "p-1",
    resourceLabel: "Launch Plan",
  },
];

describe("LogList (svelte)", () => {
  it("renders stream mode as a log region with level counts", () => {
    const { container } = render(LogList, { props: { entries: streamEntries } });
    const root = container.querySelector(".poodle-log-list--stream") as HTMLElement;
    expect(root.getAttribute("role")).toBe("log");
    expect(root.textContent).toContain("Server started");
    const counts = [...container.querySelectorAll(".poodle-log-list__count")].map(
      (el) => el.textContent,
    );
    expect(counts).toContain("4");
    expect(counts).toContain("2");
    expect(counts).toContain("1");
  });

  it("filters stream entries by level when a level chip is activated", async () => {
    const { container } = render(LogList, { props: { entries: streamEntries } });
    const warnChip = [...container.querySelectorAll("button")].find((button) =>
      button.textContent?.startsWith("Warn"),
    ) as HTMLButtonElement;
    await fireEvent.click(warnChip);
    const messages = [...container.querySelectorAll(".poodle-log-list__msg")].map(
      (el) => el.textContent,
    );
    expect(messages).toEqual(["Slow query detected"]);
  });

  it("filters stream entries by text", async () => {
    const { container } = render(LogList, { props: { entries: streamEntries } });
    const search = container.querySelector(
      'input[aria-label="Filter log messages"]',
    ) as HTMLInputElement;
    await fireEvent.input(search, { target: { value: "slow" } });
    const messages = [...container.querySelectorAll(".poodle-log-list__msg")].map(
      (el) => el.textContent,
    );
    expect(messages).toEqual(["Slow query detected"]);
  });

  it("caps stream entries at maxEntries", () => {
    const many = Array.from({ length: 7 }, (_, index) => ({
      id: String(index),
      timestamp: 1720000000000 + index,
      level: "info" as const,
      message: `entry ${index}`,
    }));
    const { container } = render(LogList, { props: { entries: many, maxEntries: 3 } });
    expect(container.querySelectorAll(".poodle-log-list__entry").length).toBe(3);
    expect(container.querySelector(".poodle-log-list__msg")?.textContent).toBe("entry 4");
  });

  it("auto-detects audit mode from the entry shape", () => {
    const { container } = render(LogList, { props: { entries: auditEntries } });
    expect(container.querySelector(".poodle-log-list--audit")).not.toBeNull();
    expect(container.textContent).toContain("Alice");
    expect(container.textContent).toContain("project.created");
  });

  it("links actors when getActorHref is provided", () => {
    const { container } = render(LogList, {
      props: { entries: auditEntries, getActorHref: (actor) => `/users/${actor.id}` },
    });
    const actorLink = container.querySelector(".poodle-log-list__audit-link") as HTMLAnchorElement;
    expect(actorLink.getAttribute("href")).toBe("/users/u-1");
  });

  it("links resources when getResourceHref returns a href", () => {
    const { container } = render(LogList, {
      props: {
        entries: auditEntries,
        getResourceHref: (resourceType, resourceId) => `/${resourceType}/${resourceId}`,
      },
    });
    const resourceLink = container.querySelector(
      ".poodle-log-list__audit-resource",
    ) as HTMLAnchorElement;
    expect(resourceLink.getAttribute("href")).toBe("/project/p-1");
  });

  it("renders audit loading, error, and empty states", () => {
    const loading = render(LogList, {
      props: { entries: [], variant: "audit", loading: true },
    });
    expect(loading.container.textContent).toContain("Loading log entries...");

    const error = render(LogList, {
      props: { entries: [], variant: "audit", error: "Audit failed" },
    });
    expect(error.container.querySelector('[role="alert"]')?.textContent).toContain("Audit failed");

    const empty = render(LogList, {
      props: { entries: [], variant: "audit", emptyMessage: "Nothing logged" },
    });
    expect(empty.container.textContent).toContain("Nothing logged");
  });

  it("renders pagination when total exceeds page size and reports page changes", async () => {
    const onPageChange = vi.fn();
    const { container } = render(LogList, {
      props: {
        entries: auditEntries,
        variant: "audit",
        total: 120,
        pageSize: 50,
        page: 1,
        onPageChange,
      },
    });
    expect(container.querySelector(".poodle-log-list__pagination")).not.toBeNull();
    const next = container.querySelector(
      '[aria-label="Next page"]',
    ) as HTMLButtonElement;
    await fireEvent.click(next);
    expect(onPageChange).toHaveBeenCalledWith(2);
  });

  it("shows the clear-filters control only with active filters and a callback", async () => {
    const onClearFilters = vi.fn();
    const filters: LogFilter[] = [
      { field: "action", label: "Action", type: "select", options: [{ value: "create", label: "Create" }] },
    ];

    const active = render(LogList, {
      props: {
        entries: auditEntries,
        variant: "audit",
        filters,
        filterValues: { action: "create" },
        onClearFilters,
      },
    });
    const clear = [...active.container.querySelectorAll("button")].find((button) =>
      button.textContent?.includes("Clear"),
    ) as HTMLButtonElement;
    expect(clear).not.toBeNull();
    await fireEvent.click(clear);
    expect(onClearFilters).toHaveBeenCalledTimes(1);

    const inactive = render(LogList, {
      props: { entries: auditEntries, variant: "audit", filters, onClearFilters },
    });
    expect(
      [...inactive.container.querySelectorAll("button")].some((button) =>
        button.textContent?.includes("Clear"),
      ),
    ).toBe(false);
  });
});

import { render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { OverlaySurfaceGeometryChange } from "@inflatable-cookie/poodle-core";

import { Menu, Popover } from "../src";

describe("React built-in overlay geometry", () => {
  beforeEach(() => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(
      function (this: HTMLElement) {
        if (this.matches('[role="menu"], .poodle-popover__surface')) {
          return new DOMRect(100, 120, 180, 90);
        }
        return new DOMRect(20, 30, 80, 24);
      },
    );
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("reports Popover and Menu surfaces and removes both on teardown", async () => {
    const popover: OverlaySurfaceGeometryChange[] = [];
    const menu: OverlaySurfaceGeometryChange[] = [];
    const view = render(
      <>
        <Popover
          defaultOpen
          ariaLabel="Geometry popover"
          trigger={<span>Popover trigger</span>}
          onSurfaceGeometryChange={(change) => popover.push(change)}
        >
          Popover content
        </Popover>
        <Menu
          defaultOpen
          ariaLabel="Geometry menu"
          trigger={<span>Menu trigger</span>}
          items={[{ value: "action", label: "Action" }]}
          onSurfaceGeometryChange={(change) => menu.push(change)}
        />
      </>,
    );

    await waitFor(() => {
      expect(popover.some((change) => change.type === "upsert")).toBe(true);
      expect(menu.some((change) => change.type === "upsert")).toBe(true);
    });

    const popoverIds = new Set(
      popover
        .filter((change) => change.type === "upsert")
        .map((change) => change.surface.surfaceId),
    );
    const menuIds = new Set(
      menu
        .filter((change) => change.type === "upsert")
        .map((change) => change.surface.surfaceId),
    );

    view.unmount();

    expect(
      new Set(
        popover
          .filter((change) => change.type === "remove")
          .map((change) => change.surfaceId),
      ),
    ).toEqual(popoverIds);
    expect(
      new Set(
        menu
          .filter((change) => change.type === "remove")
          .map((change) => change.surfaceId),
      ),
    ).toEqual(menuIds);
  });
});

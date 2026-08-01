import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { OverlaySurfaceGeometryChange } from "@poodle/headless";

import Harness from "./OverlayGeometryHarness.svelte";

describe("Svelte built-in overlay geometry", () => {
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

  it("reports Popover and recursive Menu surfaces, then removes every id", async () => {
    const popover: OverlaySurfaceGeometryChange[] = [];
    const menu: OverlaySurfaceGeometryChange[] = [];
    const view = render(Harness, {
      props: {
        onPopoverGeometry: (change) => popover.push(change),
        onMenuGeometry: (change) => menu.push(change),
      },
    });

    await waitFor(() => {
      expect(popover.some((change) => change.type === "upsert")).toBe(true);
      expect(menu.some((change) => change.type === "upsert")).toBe(true);
    });

    expect(
      new Set(
        menu
          .filter((change) => change.type === "upsert")
          .map((change) => change.surface.surfaceId),
      ).size,
    ).toBe(1);
    expect(menu.some((change) => change.type === "remove")).toBe(false);

    const parent = view.getByRole("menuitem", { name: "Parent" });
    await fireEvent.pointerEnter(parent);

    await waitFor(() => {
      const ids = new Set(
        menu
          .filter((change) => change.type === "upsert")
          .map((change) => change.surface.surfaceId),
      );
      expect(ids.size).toBe(2);
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

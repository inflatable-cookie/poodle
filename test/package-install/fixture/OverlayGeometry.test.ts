import { render, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import type { OverlaySurfaceGeometryChange } from "@poodle/svelte";

import Harness from "./OverlayGeometryHarness.svelte";

describe("packed overlay geometry", () => {
  it("reports through public types without exposing the surface element", async () => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(
      function (this: HTMLElement) {
        return this.matches(".poodle-popover__surface")
          ? new DOMRect(100, 120, 180, 90)
          : new DOMRect(20, 30, 80, 24);
      },
    );
    const changes: OverlaySurfaceGeometryChange[] = [];
    const view = render(Harness, {
      props: {
        onSurfaceGeometryChange: (change) => changes.push(change),
      },
    });

    await waitFor(() => {
      expect(changes.at(-1)).toMatchObject({
        type: "upsert",
        surface: {
          rect: { left: 100, top: 120, width: 180, height: 90 },
          visible: true,
        },
      });
    });

    view.unmount();
    expect(changes.at(-1)?.type).toBe("remove");
  });
});

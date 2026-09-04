import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { DockRegion } from "../src";

// Mirror of the Svelte interactions.test.ts showTabs case (g16.100). The
// Svelte<->React parity gate diffs prop names, not rendered anatomy, so a
// ported prop without a matching test would pass silently.
describe("DockRegion showTabs (react)", () => {
  it("renders the tab strip by default", () => {
    const { queryByRole } = render(
      <DockRegion
        items={[{ value: "inspector", label: "Inspector" }]}
        value="inspector"
      />,
    );
    expect(queryByRole("tab")).toBeTruthy();
  });

  it("showTabs=false omits the tab strip but keeps the collapse toggle and body", () => {
    const { container, queryByRole, getByRole, getByText } = render(
      <DockRegion
        edge="left"
        showTabs={false}
        collapsible
        items={[{ value: "inspector", label: "Inspector" }]}
        value="inspector"
      >
        {() => "Inspector body"}
      </DockRegion>,
    );

    expect(queryByRole("tab")).toBeNull();
    expect(container.querySelector(".poodle-dock-region__strip")).toBeNull();
    expect(
      container.querySelector('.poodle-dock-region[data-show-tabs="false"]'),
    ).toBeTruthy();
    expect(getByRole("button", { name: "Collapse left dock" })).toBeTruthy();
    expect(getByText("Inspector body")).toBeTruthy();
  });

  it("showTabs=false keeps the toggle in the collapsed icon-strip", () => {
    const { container, queryByRole, getByRole } = render(
      <DockRegion
        edge="left"
        showTabs={false}
        collapsible
        collapsed
        items={[{ value: "inspector", label: "Inspector" }]}
        value="inspector"
      />,
    );

    expect(queryByRole("tab")).toBeNull();
    expect(
      container.querySelector('.poodle-dock-region__strip[data-orientation="vertical"]'),
    ).toBeTruthy();
    expect(getByRole("button", { name: "Expand left dock" })).toBeTruthy();
  });
});

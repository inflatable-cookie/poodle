import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

vi.mock("../src/specimens/registry", () => ({
  specimenMap: {},
}));

import ComponentsSection from "../src/sections/ComponentsSection.svelte";

describe("Svelte catalogue navigation", () => {
  it("renders three sections and collapsed families on the landing route", () => {
    render(ComponentsSection, { props: { search: "" } });
    expect(screen.getByRole("heading", { name: "Foundations" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Composition" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Systems" })).toBeTruthy();
    expect(
      document.querySelector(
        '.poodle-catalogue-sidebar [data-catalogue-family="actions-selection"][data-open]',
      ),
    ).toBeNull();
    expect(
      document.querySelector(
        '.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"] .poodle-catalogue-family__count',
      )?.textContent,
    ).toBe("11");
    expect(
      screen.getAllByRole("link", { name: /Button/ }).length,
    ).toBeGreaterThan(0);
  });

  it("discloses the active family on a direct component route", async () => {
    render(ComponentsSection, {
      props: { activeComponent: "agent-plan", search: "" },
    });
    expect(
      document.querySelector(
        '.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"][data-open="true"]',
      ),
    ).toBeTruthy();
    const active = document.querySelector(
      '.poodle-catalogue-sidebar a[href="#components/agent-plan"]',
    );
    expect(active?.getAttribute("aria-current")).toBe("page");
    await fireEvent.click(
      screen.getByRole("button", { name: /Agent & tools/ }),
    );
    expect(
      document.querySelector(
        '.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"][data-open]',
      ),
    ).toBeNull();
    await fireEvent.click(
      screen.getByRole("button", { name: /Agent & tools/ }),
    );
    expect(
      document.querySelector(
        '.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"][data-open="true"]',
      ),
    ).toBeTruthy();
  });

  it("replaces the hierarchy with Family · Kind search results", () => {
    render(ComponentsSection, { props: { search: "model connection" } });
    expect(
      document.querySelector(
        ".poodle-catalogue-sidebar [data-catalogue-search]",
      ),
    ).toBeTruthy();
    expect(
      document.querySelector(
        ".poodle-catalogue-sidebar [data-catalogue-section]",
      ),
    ).toBeNull();
    const result = document.querySelector(
      '[data-catalogue-result="model-connection-picker"]',
    );
    expect(result?.textContent).toContain("Model connections · Composite");
    expect(result?.getAttribute("href")).toBe(
      "#components/model-connection-picker",
    );
  });
});

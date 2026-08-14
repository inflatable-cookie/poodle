import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

vi.mock("../src/gallery/specimen-map", () => ({
  specimenMap: {},
}));

import { ComponentsSection } from "../src/gallery/ComponentsSection";

describe("React catalogue navigation", () => {
  it("renders three sections and collapsed families on the landing route", () => {
    render(<ComponentsSection search="" />);
    expect(screen.getByRole("heading", { name: "Foundations" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Composition" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Systems" })).toBeTruthy();
    expect(
      document.querySelector('.poodle-catalogue-sidebar [data-catalogue-family="actions-selection"][data-open]'),
    ).toBeNull();
    expect(
      document.querySelector('.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"] .poodle-catalogue-family__count')
        ?.textContent,
    ).toBe("9");
    expect(screen.getAllByRole("link", { name: /Button/ }).length).toBeGreaterThan(0);
  });

  it("discloses the active family on a direct component route", () => {
    render(<ComponentsSection activeComponent="agent-transcript" search="" />);
    expect(
      document.querySelector('.poodle-catalogue-sidebar [data-catalogue-family="agent-tools"][data-open="true"]'),
    ).toBeTruthy();
    const active = document.querySelector('.poodle-catalogue-sidebar a[href="#components/agent-transcript"]');
    expect(active?.getAttribute("aria-current")).toBe("page");
  });

  it("replaces the hierarchy with Family · Kind search results", () => {
    render(<ComponentsSection search="model connection" />);
    expect(document.querySelector(".poodle-catalogue-sidebar [data-catalogue-search]")).toBeTruthy();
    expect(document.querySelector(".poodle-catalogue-sidebar [data-catalogue-section]")).toBeNull();
    const result = document.querySelector('[data-catalogue-result="model-connection-picker"]');
    expect(result?.textContent).toContain("Model connections · Composite");
    expect(result?.getAttribute("href")).toBe("#components/model-connection-picker");
  });
});

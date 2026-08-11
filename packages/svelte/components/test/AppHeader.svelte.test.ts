import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import AppHeader from "../src/AppHeader.svelte";
import AppHeaderElementHarness from "./AppHeaderElementHarness.svelte";

// Element access (g13.014): `bind:element` yields the raw `<header>` DOM
// element (never a handle object), so a host can attach behaviour to the
// root. The binding must not change the rendered output.
describe("AppHeader (svelte)", () => {
  it("bind:element resolves to the rendered header element", () => {
    render(AppHeaderElementHarness);
    expect(screen.getByTestId("bound-element").textContent).toBe("HEADER poodle-app-header");
  });

  it("renders the header with unchanged anatomy", () => {
    const { container } = render(AppHeader, { props: { title: "Finch", dragRegion: true } });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header).toBeTruthy();
    expect(header.getAttribute("data-drag-region")).toBe("true");
    expect(header.getAttribute("aria-label")).toBe("Finch");
    expect(header.querySelector("strong")?.textContent).toBe("Finch");
  });

  it("renders data-drag-region=false when dragRegion is false (default)", () => {
    const { container } = render(AppHeader, { props: { title: "Finch" } });
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header.getAttribute("data-drag-region")).toBe("false");
  });
});

import { createRef } from "react";
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AppHeader } from "../src/AppHeader";

// Element access (g13.014): React forwards `ref` to the raw `<header>` DOM
// element — the same thing the Svelte `bind:element` prop exposes, never a
// handle object. The forwarding must not change the rendered output.
describe("AppHeader (react)", () => {
  it("forwards ref to the rendered header element", () => {
    const ref = createRef<HTMLElement>();
    const { container } = render(<AppHeader title="Finch" dragRegion ref={ref} />);
    const header = container.querySelector("header.poodle-app-header")!;
    expect(ref.current).toBe(header);
  });

  it("renders the header with unchanged anatomy", () => {
    const { container } = render(<AppHeader title="Finch" dragRegion />);
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header).toBeTruthy();
    expect(header.getAttribute("data-drag-region")).toBe("true");
    expect(header.getAttribute("aria-label")).toBe("Finch");
    expect(header.querySelector("strong")?.textContent).toBe("Finch");
  });

  it("renders data-drag-region=false when dragRegion is false (default)", () => {
    const { container } = render(<AppHeader title="Finch" />);
    const header = container.querySelector("header.poodle-app-header")!;
    expect(header.getAttribute("data-drag-region")).toBe("false");
  });
});

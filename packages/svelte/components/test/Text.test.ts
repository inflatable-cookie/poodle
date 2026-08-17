import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import Text from "../src/Text.svelte";
import { asSnippet } from "./snippet";

describe("Text (svelte)", () => {
  it("renders the requested semantic element", () => {
    const p = render(Text, { props: { children: asSnippet(() => "Body") } });
    expect(p.container.querySelector(".poodle-text")?.tagName).toBe("P");

    const span = render(Text, { props: { as: "span", children: asSnippet(() => "Inline") } });
    expect(span.container.querySelector(".poodle-text")?.tagName).toBe("SPAN");

    const div = render(Text, { props: { as: "div", children: asSnippet(() => "Block") } });
    expect(div.container.querySelector(".poodle-text")?.tagName).toBe("DIV");
  });

  it("projects tone, size, weight, and leading data attributes", () => {
    const { container } = render(Text, {
      props: { tone: "danger", size: "xs", weight: "semibold", leading: "relaxed", children: asSnippet(() => "Warn") },
    });
    const root = container.querySelector(".poodle-text") as HTMLElement;
    expect(root.dataset.tone).toBe("danger");
    expect(root.dataset.size).toBe("xs");
    expect(root.dataset.weight).toBe("semibold");
    expect(root.dataset.leading).toBe("relaxed");
  });

  it("projects compact spacing and line clamp values", () => {
    const { container } = render(Text, {
      props: { spacing: "compact", clamp: 2, children: asSnippet(() => "Long copy") },
    });
    const root = container.querySelector(".poodle-text") as HTMLElement;
    expect(root.dataset.spacing).toBe("compact");
    expect(root.dataset.clamp).toBe("2");
  });

  it("does not add ARIA roles", () => {
    const { container } = render(Text, { props: { children: asSnippet(() => "Body") } });
    expect(container.querySelector(".poodle-text")?.getAttribute("role")).toBeNull();
  });
});
import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { Text } from "../src/Text";

describe("Text (react)", () => {
  it("renders the requested semantic element", () => {
    const p = render(<Text>Body</Text>);
    expect(p.container.querySelector(".poodle-text")?.tagName).toBe("P");

    const span = render(<Text as="span">Inline</Text>);
    expect(span.container.querySelector(".poodle-text")?.tagName).toBe("SPAN");

    const div = render(<Text as="div">Block</Text>);
    expect(div.container.querySelector(".poodle-text")?.tagName).toBe("DIV");
  });

  it("projects tone, size, weight, and leading data attributes", () => {
    const { container } = render(
      <Text tone="danger" size="xs" weight="semibold" leading="relaxed">
        Warn
      </Text>,
    );
    const root = container.querySelector(".poodle-text") as HTMLElement;
    expect(root.dataset.tone).toBe("danger");
    expect(root.dataset.size).toBe("xs");
    expect(root.dataset.weight).toBe("semibold");
    expect(root.dataset.leading).toBe("relaxed");
  });

  it("projects compact spacing and line clamp values", () => {
    const { container } = render(
      <Text spacing="compact" clamp={2}>
        Long copy
      </Text>,
    );
    const root = container.querySelector(".poodle-text") as HTMLElement;
    expect(root.dataset.spacing).toBe("compact");
    expect(root.dataset.clamp).toBe("2");
  });

  it("does not add ARIA roles", () => {
    const { container } = render(<Text>Body</Text>);
    expect(container.querySelector(".poodle-text")?.getAttribute("role")).toBeNull();
  });
});
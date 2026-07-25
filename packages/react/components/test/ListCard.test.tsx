import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { ListCard } from "../src/ListCard";

// Mirrors packages/svelte/components/test/ListCard.test.ts: the <a> and <div>
// roots must resolve data-size from the same sizeRole.
describe("ListCard (react)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  it("emits the same data-size from the div and anchor roots", () => {
    const div = rootOf(render(<ListCard title="Card" />).container);
    const anchor = rootOf(render(<ListCard title="Card" href="#" />).container);

    expect(anchor.tagName).toBe("A");
    expect(div.tagName).toBe("DIV");
    expect(anchor.dataset.size).toBe(div.dataset.size);
  });

  it("honours an explicit size on both roots", () => {
    const div = rootOf(render(<ListCard title="Card" size="lg" />).container);
    const anchor = rootOf(
      render(<ListCard title="Card" href="#" size="lg" />).container,
    );

    expect(div.dataset.size).toBe("lg");
    expect(anchor.dataset.size).toBe("lg");
  });
});

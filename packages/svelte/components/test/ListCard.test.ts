import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import ListCard from "../src/ListCard.svelte";

// ListCard has a dual root: <a> when href is set (and not selectable), <div>
// otherwise. Both roots must resolve data-size from the same sizeRole — the
// anchor branch previously emitted the "chrome" role, so identical props
// rendered at different sizes depending on href.
describe("ListCard (svelte)", () => {
  const rootOf = (container: HTMLElement) =>
    container.querySelector(".poodle-list-card") as HTMLElement;

  it("emits the same data-size from the div and anchor roots", () => {
    const div = rootOf(render(ListCard, { props: { title: "Card" } }).container);
    const anchor = rootOf(
      render(ListCard, { props: { title: "Card", href: "#" } }).container,
    );

    expect(anchor.tagName).toBe("A");
    expect(div.tagName).toBe("DIV");
    expect(anchor.dataset.size).toBe(div.dataset.size);
  });

  it("honours an explicit size on both roots", () => {
    const div = rootOf(
      render(ListCard, { props: { title: "Card", size: "lg" } }).container,
    );
    const anchor = rootOf(
      render(ListCard, { props: { title: "Card", href: "#", size: "lg" } })
        .container,
    );

    expect(div.dataset.size).toBe("lg");
    expect(anchor.dataset.size).toBe("lg");
  });
});

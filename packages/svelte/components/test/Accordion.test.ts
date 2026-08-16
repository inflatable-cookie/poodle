import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Accordion from "../src/Accordion.svelte";

// happy-dom lacks the Web Animations API, which the panel's `transition:slide`
// calls through `element.animate`. Same polyfill precedent as
// DrawerDismissOutside.svelte.test.ts.
if (!("animate" in Element.prototype)) {
  (Element.prototype as unknown as { animate: () => unknown }).animate = () => {
    const animation = {
      onfinish: null as (() => void) | null,
      cancel: () => {},
      playState: "finished",
      currentTime: 0,
      effect: null,
      finished: Promise.resolve(),
    };
    queueMicrotask(() => animation.onfinish?.());
    return animation;
  };
}

const items = [
  { value: "one", label: "One" },
  { value: "two", label: "Two" },
];

describe("Accordion (svelte)", () => {
  it("toggles a single item open and closed in the default single mode", async () => {
    const { container, getAllByRole } = render(Accordion, { props: { items } });
    const triggers = getAllByRole("button");

    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-accordion__panel")).toBeNull();

    await fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
    expect(container.querySelector(".poodle-accordion__panel")).not.toBeNull();

    await fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
  });

  it("keeps only one item open in single mode and reports changes", async () => {
    const onValueChange = vi.fn();
    const { getAllByRole } = render(Accordion, {
      props: { items, onValueChange },
    });
    const triggers = getAllByRole("button");

    await fireEvent.click(triggers[0]);
    await fireEvent.click(triggers[1]);

    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
    expect(triggers[1].getAttribute("aria-expanded")).toBe("true");
    expect(onValueChange).toHaveBeenLastCalledWith("two");
  });

  it("allows several open items in multiple mode", async () => {
    const { getAllByRole } = render(Accordion, {
      props: { items, selectionMode: "multiple" },
    });
    const triggers = getAllByRole("button");

    await fireEvent.click(triggers[0]);
    await fireEvent.click(triggers[1]);

    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
    expect(triggers[1].getAttribute("aria-expanded")).toBe("true");
  });

  it("refuses deactivation when collapsible is false", async () => {
    const { getAllByRole } = render(Accordion, {
      props: { items, collapsible: false, defaultValue: "one" },
    });
    const triggers = getAllByRole("button");
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");

    await fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
  });

  it("disables an item trigger and skips it in the open set", async () => {
    const { getAllByRole } = render(Accordion, {
      props: {
        items: [items[0], { ...items[1], disabled: true }],
        defaultValue: "one",
      },
    });
    const triggers = getAllByRole("button");
    expect((triggers[1] as HTMLButtonElement).disabled).toBe(true);

    await fireEvent.click(triggers[1]);
    expect(triggers[1].getAttribute("aria-expanded")).toBe("false");
  });
});

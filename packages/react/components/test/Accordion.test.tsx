import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { Accordion } from "../src/Accordion";

const items = [
  { value: "one", label: "One" },
  { value: "two", label: "Two" },
];

describe("Accordion (react)", () => {
  it("toggles a single item open and closed in the default single mode", () => {
    const { container, getAllByRole } = render(<Accordion items={items} />);
    const triggers = getAllByRole("button");

    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
    expect(container.querySelector(".poodle-accordion__panel")).toBeNull();

    fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
    expect(container.querySelector(".poodle-accordion__panel")).not.toBeNull();

    fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
  });

  it("keeps only one item open in single mode and reports changes", () => {
    const onValueChange = vi.fn();
    const { getAllByRole } = render(<Accordion items={items} onValueChange={onValueChange} />);
    const triggers = getAllByRole("button");

    fireEvent.click(triggers[0]);
    fireEvent.click(triggers[1]);

    expect(triggers[0].getAttribute("aria-expanded")).toBe("false");
    expect(triggers[1].getAttribute("aria-expanded")).toBe("true");
    expect(onValueChange).toHaveBeenLastCalledWith("two");
  });

  it("allows several open items in multiple mode", () => {
    const { getAllByRole } = render(<Accordion items={items} selectionMode="multiple" />);
    const triggers = getAllByRole("button");

    fireEvent.click(triggers[0]);
    fireEvent.click(triggers[1]);

    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
    expect(triggers[1].getAttribute("aria-expanded")).toBe("true");
  });

  it("refuses deactivation when collapsible is false", () => {
    const { getAllByRole } = render(
      <Accordion items={items} collapsible={false} defaultValue="one" />,
    );
    const triggers = getAllByRole("button");
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");

    fireEvent.click(triggers[0]);
    expect(triggers[0].getAttribute("aria-expanded")).toBe("true");
  });

  it("disables an item trigger and skips it in the open set", () => {
    const { getAllByRole } = render(
      <Accordion items={[items[0], { ...items[1], disabled: true }]} defaultValue="one" />,
    );
    const triggers = getAllByRole("button");
    expect(triggers[1].disabled).toBe(true);

    fireEvent.click(triggers[1]);
    expect(triggers[1].getAttribute("aria-expanded")).toBe("false");
  });
});

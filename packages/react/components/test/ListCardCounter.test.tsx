import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { ListCardCounter } from "../src/ListCardCounter";

describe("ListCardCounter (react)", () => {
  it("renders the count and the icon in a span by default", () => {
    const { container } = render(<ListCardCounter icon="mail" count={12} />);
    const root = container.querySelector(".poodle-list-card-counter") as HTMLElement;
    expect(root.tagName).toBe("SPAN");
    expect(root.textContent).toContain("12");
    expect(root.querySelector(".poodle-icon")).not.toBeNull();
  });

  it("renders an anchor when href is set", () => {
    const { container } = render(<ListCardCounter icon="mail" count={12} href="/threads/1" />);
    const root = container.querySelector(".poodle-list-card-counter") as HTMLElement;
    expect(root.tagName).toBe("A");
    expect(root.getAttribute("href")).toBe("/threads/1");
  });

  it("emits onClick for a linked counter only", () => {
    const onClick = vi.fn();
    const { container } = render(
      <ListCardCounter icon="mail" count={12} href="/threads/1" onClick={onClick} />,
    );
    const anchor = container.querySelector(".poodle-list-card-counter") as HTMLElement;

    fireEvent.click(anchor);

    expect(onClick).toHaveBeenCalledTimes(1);

    // An unlinked counter is a statistic, not a control: the contract scopes
    // `onClick` to linked counters (`list-card-counter.md` §3/§5, §10a).
    const plain = render(<ListCardCounter icon="mail" count={12} onClick={onClick} />);
    fireEvent.click(plain.container.querySelector(".poodle-list-card-counter") as HTMLElement);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("wraps the body in a tooltip when tooltip is set", () => {
    const { container } = render(
      <ListCardCounter icon="mail" count={12} tooltip="Comments" />,
    );
    expect(container.querySelector(".poodle-tooltip")).not.toBeNull();
  });
});

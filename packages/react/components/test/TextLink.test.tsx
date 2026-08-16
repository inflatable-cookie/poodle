import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { TextLink } from "../src/TextLink";

describe("TextLink (react)", () => {
  it("renders an anchor when href is provided and a button without one", () => {
    const link = render(<TextLink href="/docs">Docs</TextLink>);
    const anchor = link.container.querySelector(".poodle-text-link") as HTMLElement;
    expect(anchor.tagName).toBe("A");
    expect(anchor.getAttribute("href")).toBe("/docs");

    const button = render(<TextLink>Docs</TextLink>);
    expect((button.container.querySelector(".poodle-text-link") as HTMLElement).tagName).toBe(
      "BUTTON",
    );
  });

  it("renders a disabled button even when href is present, so no dead navigation target exists", () => {
    const onClick = vi.fn();
    const { container } = render(
      <TextLink href="/docs" disabled onClick={onClick}>
        Docs
      </TextLink>,
    );
    const root = container.querySelector(".poodle-text-link") as HTMLButtonElement;
    expect(root.tagName).toBe("BUTTON");
    expect(root.disabled).toBe(true);
    expect(root.hasAttribute("href")).toBe(false);

    fireEvent.click(root);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("forwards tone, aria-label, target and rel to the anchor", () => {
    const { container } = render(
      <TextLink
        href="/docs"
        target="_blank"
        rel="noopener"
        tone="secondary"
        ariaLabel="Poodle docs"
      >
        Docs
      </TextLink>,
    );
    const anchor = container.querySelector(".poodle-text-link") as HTMLAnchorElement;
    expect(anchor.getAttribute("target")).toBe("_blank");
    expect(anchor.getAttribute("rel")).toBe("noopener");
    expect(anchor.getAttribute("aria-label")).toBe("Poodle docs");
    expect(anchor.dataset.tone).toBe("secondary");
  });

  it("fires onClick on enabled activation", () => {
    const onClick = vi.fn();
    const { container } = render(<TextLink href="/docs" onClick={onClick}>Docs</TextLink>);
    const anchor = container.querySelector(".poodle-text-link") as HTMLAnchorElement;

    fireEvent.click(anchor);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("appends className to the root class list", () => {
    const { container } = render(<TextLink href="/x" className="extra-class">X</TextLink>);
    const root = container.querySelector(".poodle-text-link") as HTMLElement;
    expect(root.classList.contains("extra-class")).toBe(true);
  });
});
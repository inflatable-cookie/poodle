import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import TextLink from "../src/TextLink.svelte";
import { asSnippet } from "./snippet";

describe("TextLink (svelte)", () => {
  it("renders an anchor when href is provided and a button without one", () => {
    const link = render(TextLink, {
      props: { href: "/docs", children: asSnippet(() => "Docs") },
    });
    const anchor = link.container.querySelector(".poodle-text-link") as HTMLElement;
    expect(anchor.tagName).toBe("A");
    expect(anchor.getAttribute("href")).toBe("/docs");

    const button = render(TextLink, { props: { children: asSnippet(() => "Docs") } });
    expect((button.container.querySelector(".poodle-text-link") as HTMLElement).tagName).toBe(
      "BUTTON",
    );
  });

  it("renders a disabled button even when href is present, so no dead navigation target exists", async () => {
    const onClick = vi.fn();
    const { container } = render(TextLink, {
      props: {
        href: "/docs",
        disabled: true,
        onClick,
        children: asSnippet(() => "Docs"),
      },
    });
    const root = container.querySelector(".poodle-text-link") as HTMLButtonElement;
    expect(root.tagName).toBe("BUTTON");
    expect(root.disabled).toBe(true);
    expect(root.hasAttribute("href")).toBe(false);

    await fireEvent.click(root);
    expect(onClick).not.toHaveBeenCalled();
  });

  it("forwards tone, aria-label, target and rel to the anchor", () => {
    const { container } = render(TextLink, {
      props: {
        href: "/docs",
        target: "_blank",
        rel: "noopener",
        tone: "secondary",
        ariaLabel: "Poodle docs",
        children: asSnippet(() => "Docs"),
      },
    });
    const anchor = container.querySelector(".poodle-text-link") as HTMLAnchorElement;
    expect(anchor.getAttribute("target")).toBe("_blank");
    expect(anchor.getAttribute("rel")).toBe("noopener");
    expect(anchor.getAttribute("aria-label")).toBe("Poodle docs");
    expect(anchor.dataset.tone).toBe("secondary");
  });

  it("fires onClick on enabled activation", async () => {
    const onClick = vi.fn();
    const { container } = render(TextLink, {
      props: {
        href: "/docs",
        onClick,
        children: asSnippet(() => "Docs"),
      },
    });
    const anchor = container.querySelector(".poodle-text-link") as HTMLAnchorElement;

    await fireEvent.click(anchor);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("appends className to the root class list", () => {
    const { container } = render(TextLink, {
      props: { href: "/x", className: "extra-class", children: asSnippet(() => "X") },
    });
    const root = container.querySelector(".poodle-text-link") as HTMLElement;
    expect(root.classList.contains("extra-class")).toBe(true);
  });
});
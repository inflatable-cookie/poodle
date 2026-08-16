import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AgentMessage } from "../src/AgentMessage";

describe("AgentMessage (react)", () => {
  it("renders markdown blocks with the shared block model", () => {
    const { container } = render(
      <AgentMessage
        markdown="# Plan

A paragraph with **bold** and `code`.

```ts
const x = 1;
```"
      />,
    );
    const root = container.querySelector(".poodle-agent-message") as HTMLElement;
    expect(root.dataset.role).toBe("assistant");
    expect(root.querySelector("h1.poodle-agent-message__heading")?.textContent).toBe("Plan");
    expect(root.querySelector(".poodle-agent-message__paragraph")?.textContent).toContain("bold");
    expect(root.querySelector(".poodle-agent-message__paragraph strong")).not.toBeNull();
    expect(root.querySelector(".poodle-agent-message__code-span")?.textContent).toBe("code");
    expect(root.querySelector(".poodle-code")).not.toBeNull();
  });

  it("renders lists, blockquotes, and separators as real elements", () => {
    // Expression form, not a JSX string attribute: `"\n"` in an attribute
    // literal stays a backslash and an `n`, collapsing the whole fixture to
    // one line.
    const { container } = render(<AgentMessage markdown={"- one\n- two\n\n> quoted\n\n---"} />);
    const root = container.querySelector(".poodle-agent-message") as HTMLElement;
    expect(root.querySelector("ul.poodle-agent-message__list")).not.toBeNull();
    expect(root.querySelectorAll(".poodle-agent-message__list-item").length).toBe(2);
    expect(root.querySelector("blockquote.poodle-agent-message__quote")?.textContent).toContain(
      "quoted",
    );
    expect(root.querySelector(".poodle-separator")).not.toBeNull();
  });

  it("shows the streaming caret and projects streaming state, hidden from assistive tech", () => {
    const { container } = render(<AgentMessage markdown="hello" isStreaming />);
    const root = container.querySelector(".poodle-agent-message") as HTMLElement;
    expect(root.dataset.streaming).toBe("true");
    const caret = root.querySelector(".poodle-agent-message__caret") as HTMLElement;
    expect(caret).not.toBeNull();
    expect(caret.getAttribute("aria-hidden")).toBe("true");
  });

  it("renders nothing at all for an empty message", () => {
    const { container } = render(<AgentMessage markdown="" />);
    expect(container.querySelector(".poodle-agent-message")).toBeNull();
  });

  it("renders user messages with the user role", () => {
    const { container } = render(<AgentMessage markdown="hi" role="user" />);
    expect(container.querySelector(".poodle-agent-message")?.getAttribute("data-role")).toBe("user");
  });

  it("intercepts link activation through onLinkClick and suppresses navigation", () => {
    const onLinkClick = vi.fn();
    const { container } = render(
      <AgentMessage markdown="[docs](https://example.com)" onLinkClick={onLinkClick} />,
    );
    const link = container.querySelector(".poodle-text-link") as HTMLAnchorElement;
    expect(link.getAttribute("href")).toBe("https://example.com");

    const event = new MouseEvent("click", { bubbles: true, cancelable: true });
    fireEvent(link, event);
    expect(event.defaultPrevented).toBe(true);
    expect(onLinkClick).toHaveBeenCalledWith("https://example.com");
  });

  it("forwards link target to rendered links", () => {
    const { container } = render(
      <AgentMessage markdown="[docs](https://example.com)" linkTarget="_blank" />,
    );
    expect(container.querySelector(".poodle-text-link")?.getAttribute("target")).toBe("_blank");
  });
});
import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import Code from "../src/Code.svelte";

describe("Code (svelte)", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  const clipboardStub = () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal("navigator", { ...navigator, clipboard: { writeText } });
    return writeText;
  };

  it("splits block source into numbered lines and highlights the requested ones", () => {
    const { container } = render(Code, {
      props: { source: "a\nb\nc", showLineNumbers: true, highlightLines: [2] },
    });
    const lines = container.querySelectorAll(".poodle-code__line");
    expect(lines.length).toBe(3);
    expect(container.querySelectorAll(".poodle-code__line-number").length).toBe(3);
    expect(container.querySelector(".poodle-code__line-number")?.textContent).toBe("1");
    expect(lines[1].classList.contains("poodle-code__line--highlighted")).toBe(true);
    expect(lines[0].classList.contains("poodle-code__line--highlighted")).toBe(false);
  });

  it("renders the inline mode as a <code> fragment without a block toolbar", () => {
    const { container } = render(Code, {
      props: { source: "npm install", inline: true, language: "bash" },
    });
    expect(container.querySelector("code.poodle-code--inline")?.textContent).toBe("npm install");
    expect(container.querySelector(".poodle-code__toolbar")).toBeNull();
    expect(container.querySelector(".poodle-code__pre")).toBeNull();
  });

  it("shows the language label in the block toolbar", () => {
    const { container } = render(Code, { props: { source: "x", language: "typescript" } });
    expect(container.querySelector(".poodle-code__language")?.textContent).toBe("typescript");
  });

  it("writes the source to the clipboard and swaps the copy label to feedback", async () => {
    const writeText = clipboardStub();
    const { container } = render(Code, { props: { source: "echo hi" } });
    const button = container.querySelector(".poodle-code__copy") as HTMLButtonElement;
    expect(button.getAttribute("aria-label")).toBe("Copy to clipboard");

    await fireEvent.click(button);

    expect(writeText).toHaveBeenCalledWith("echo hi");
    expect(button.getAttribute("aria-label")).toBe("Copied");
  });
});

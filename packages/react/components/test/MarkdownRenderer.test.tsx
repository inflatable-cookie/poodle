import { render } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { renderToString } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { MarkdownEditor } from "../src/MarkdownEditor";
import { MarkdownRenderer } from "../src/MarkdownRenderer";
import { renderMarkdownHtml } from "../src/markdown-content";

const markdownEditorCss = readFileSync(
  new URL("../../../core/src/styles/markdown-editor.css", `file://${import.meta.dirname}/`),
  "utf8",
);

function injectStyles(): void {
  const style = document.createElement("style");
  style.textContent = markdownEditorCss;
  document.head.appendChild(style);
}

const TRUSTED_FIXTURE = `<div class="trusted-fixture" data-note="kept"><script>window.__trusted = true;</script><p onclick="window.__clicked = true">Trusted body</p></div>`;

describe("MarkdownRenderer (react)", () => {
  it("renders read-only document content with no editor mechanics", () => {
    const { container } = render(
      <MarkdownRenderer value={"# Title\n\nBody with **bold** and [link](https://example.com)."} />,
    );
    expect(container.querySelector(".poodle-md-renderer")).not.toBeNull();
    const content = container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(content.classList.contains("poodle-md-prose")).toBe(true);
    expect(content.querySelector("h1")?.textContent).toBe("Title");
    expect(content.querySelector("a")?.getAttribute("href")).toBe("https://example.com");
    expect(container.querySelector("textarea")).toBeNull();
    expect(container.querySelector("button")).toBeNull();
    expect(container.querySelector("[contenteditable]")).toBeNull();
    expect(container.querySelector(".poodle-md-editor__toolbar")).toBeNull();
  });

  it("sanitizes built-in output by default", () => {
    const { container } = render(
      <MarkdownRenderer
        value={`<script>window.__pwned = true;</script>\n\n<img src="/x" onerror="window.__pwned = true">\n\n[bad](javascript:alert(1))`}
      />,
    );
    const content = container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(content.querySelector("script")).toBeNull();
    expect(content.querySelector("img")?.getAttribute("onerror")).toBeNull();
    expect(content.querySelector("a")?.getAttribute("href")).toBeNull();
    expect(content.innerHTML).not.toContain("javascript:");
  });

  it("sanitizes custom renderHtml output so supplying it cannot bypass policy", () => {
    const { container } = render(
      <MarkdownRenderer
        value="ignored"
        renderHtml={() => `<iframe src="https://evil.example"></iframe><p onclick="x">kept text</p>`}
      />,
    );
    const content = container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(content.querySelector("iframe")).toBeNull();
    expect(content.querySelector("p")?.getAttribute("onclick")).toBeNull();
    expect(content.textContent).toContain("kept text");
  });

  it("bypasses sanitization only for the explicit trusted policy", () => {
    const safe = render(<MarkdownRenderer value={TRUSTED_FIXTURE} />);
    const safeContent = safe.container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(safeContent.querySelector("script")).toBeNull();
    expect(safeContent.querySelector("[data-note]")).toBeNull();

    const trusted = render(<MarkdownRenderer value={TRUSTED_FIXTURE} htmlPolicy="trusted" />);
    const trustedContent = trusted.container.querySelector(
      ".poodle-md-renderer__content",
    ) as HTMLElement;
    expect(trustedContent.innerHTML).toBe(TRUSTED_FIXTURE);
  });

  it("keeps an empty source neutral with no editor placeholder copy", () => {
    const { container } = render(<MarkdownRenderer value="" />);
    const content = container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(content.innerHTML).toBe("");
    expect(container.textContent).not.toContain("Nothing to preview");
    expect(container.querySelector(".poodle-md-editor__preview-empty")).toBeNull();
  });

  it("leaves ordinary document semantics unless a label is supplied", () => {
    const plain = render(<MarkdownRenderer value="text" />);
    const plainRoot = plain.container.querySelector(".poodle-md-renderer") as HTMLElement;
    expect(plainRoot.getAttribute("role")).toBeNull();
    expect(plainRoot.getAttribute("aria-label")).toBeNull();

    const labelled = render(<MarkdownRenderer value="text" ariaLabel="Release notes" />);
    const labelledRoot = labelled.container.querySelector(".poodle-md-renderer") as HTMLElement;
    expect(labelledRoot.getAttribute("role")).toBe("region");
    expect(labelledRoot.getAttribute("aria-label")).toBe("Release notes");
  });

  it("matches the editor preview output for the same source", () => {
    const value = "# Shared\n\n- one\n- two\n\n> quote\n\n`code` and [link](/docs).";
    const editor = render(<MarkdownEditor mode="preview" value={value} />);
    const renderer = render(<MarkdownRenderer value={value} />);
    const preview = editor.container.querySelector(".poodle-md-editor__preview") as HTMLElement;
    const content = renderer.container.querySelector(".poodle-md-renderer__content") as HTMLElement;
    expect(content.innerHTML.replace(/<!---->/g, "")).toBe(
      preview.innerHTML.replace(/<!---->/g, ""),
    );
    expect(content.classList.contains("poodle-md-prose")).toBe(true);
    expect(preview.classList.contains("poodle-md-prose")).toBe(true);
  });

  it("uses the same prose typography as the editor preview across density", () => {
    injectStyles();
    for (const density of ["compact", "default", "comfortable"] as const) {
      const editor = render(<MarkdownEditor mode="preview" value="## H2" density={density} />);
      const renderer = render(<MarkdownRenderer value="## H2" density={density} />);
      const preview = editor.container.querySelector(".poodle-md-editor__preview") as HTMLElement;
      const content = renderer.container.querySelector(".poodle-md-renderer__content") as HTMLElement;
      const previewStyle = getComputedStyle(preview);
      const contentStyle = getComputedStyle(content);
      expect(contentStyle.fontFamily).toBe(previewStyle.fontFamily);
      expect(contentStyle.fontSize).toBe(previewStyle.fontSize);
      expect(contentStyle.lineHeight).toBe(previewStyle.lineHeight);
      expect(
        (renderer.container.querySelector(".poodle-md-renderer") as HTMLElement).dataset.density,
      ).toBe(density);
      editor.unmount();
      renderer.unmount();
    }
  });

  it("exposes the shared private path used by both surfaces", () => {
    expect(renderMarkdownHtml("x", () => "<b>x</b><script>1</script>", "safe")).toBe("<b>x</b>");
    expect(renderMarkdownHtml("x", () => "<b>x</b><script>1</script>", "trusted")).toBe(
      "<b>x</b><script>1</script>",
    );
  });

  it("server-renders deterministic sanitized output with no editor mechanics", () => {
    const source = `<script>window.__pwned = true;</script>\n\n# Title\n\nBody with **bold**.`;
    const first = renderToString(<MarkdownRenderer value={source} />);
    const second = renderToString(<MarkdownRenderer value={source} />);
    expect(first).toBe(second);
    expect(first).toContain("poodle-md-prose");
    expect(first).toContain("<h1>Title</h1>");
    expect(first).not.toContain("<script>");
    expect(first).not.toContain("textarea");
    const safe = renderToString(<MarkdownRenderer value="<b data-x>x</b>" />);
    const trusted = renderToString(<MarkdownRenderer value="<b data-x>x</b>" htmlPolicy="trusted" />);
    expect(safe).toContain("<b>x</b>");
    expect(safe).not.toContain("data-x");
    expect(trusted).toContain("<b data-x>x</b>");
  });
});

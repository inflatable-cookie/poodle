import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import MarkdownRenderer from "../../src/MarkdownRenderer.svelte";
import { renderMarkdownHtml } from "../../src/markdown-content";

// g18.019 server-render evidence: the renderer parses and sanitizes without a
// browser, produces deterministic bytes, and shares the editor preview output.

const SAFE_SOURCE = `# Title\n\nBody with **bold**, \`code\`, and [link](https://example.com).`;

describe("MarkdownRenderer SSR", () => {
  it("renders sanitized document content without editor mechanics", () => {
    const { body } = render(MarkdownRenderer, {
      props: { value: `<script>window.__pwned = true;</script>\n\n${SAFE_SOURCE}` },
    });
    expect(body).toContain("poodle-md-renderer__content");
    expect(body).toContain("poodle-md-prose");
    expect(body).toContain("<h1>Title</h1>");
    expect(body).not.toContain("<script>");
    expect(body).not.toContain("textarea");
    expect(body).not.toContain("contenteditable");
    expect(body).not.toContain("poodle-md-editor__toolbar");
  });

  it("is deterministic across repeated renders", () => {
    const first = render(MarkdownRenderer, { props: { value: SAFE_SOURCE } }).body;
    const second = render(MarkdownRenderer, { props: { value: SAFE_SOURCE } }).body;
    expect(first).toBe(second);
  });

  it("keeps the trusted bypass explicit on the server", () => {
    const safe = render(MarkdownRenderer, { props: { value: "<b data-x>x</b>" } }).body;
    const trusted = render(MarkdownRenderer, {
      props: { value: "<b data-x>x</b>", htmlPolicy: "trusted" },
    }).body;
    expect(safe).toContain("<b>x</b>");
    expect(safe).not.toContain("data-x");
    expect(trusted).toContain("<b data-x>x</b>");
  });

  it("matches the shared private path output on the server", () => {
    const expected = renderMarkdownHtml(SAFE_SOURCE, null, "safe");
    const renderer = render(MarkdownRenderer, { props: { value: SAFE_SOURCE } }).body;
    for (const fragment of ["<h1>Title</h1>", '<a href="https://example.com">link</a>']) {
      expect(expected).toContain(fragment);
      expect(renderer).toContain(fragment);
    }
    expect(expected).not.toContain("<script");
  });
});

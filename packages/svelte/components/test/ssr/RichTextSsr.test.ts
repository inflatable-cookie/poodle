import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import Harness from "./RichTextSsrHarness.svelte";

/**
 * g18.003 server-render evidence. The TipTap engine mounts exclusively after
 * client mount: server HTML carries the roots, viewport, and content region
 * contract without constructing a ProseMirror editor, touching the document,
 * or loading document content outside a browser.
 */
describe("RichText SSR", () => {
  it("server HTML has the editor root and viewport but no engine surface", () => {
    const { body } = render(Harness, { props: { value: undefined } });
    expect(body).toContain("poodle-rich-text-editor");
    expect(body).toContain("poodle-rich-text-editor__viewport");
    expect(body).toContain("poodle-rich-text-renderer");
    expect(body).not.toContain("ProseMirror");
    expect(body).not.toContain("contenteditable");
    expect(body).not.toContain("<h1");
    expect(body).not.toContain("<strong");
  });

  it("the renderer region renders without browser globals", () => {
    const { body } = render(Harness, { props: { value: undefined } });
    expect(body).toContain("poodle-rich-text-renderer__content");
  });
});

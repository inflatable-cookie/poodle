import { render } from "svelte/server";
import { describe, expect, it } from "vitest";

import Harness from "./CodeEditorSsrHarness.svelte";

/**
 * g18.002 server-render evidence. The editor engine mounts exclusively after
 * client mount: server HTML carries the root, viewport, density, and label
 * contract without constructing an EditorView, touching the document, or
 * spawning workers at import.
 */
describe("CodeEditor SSR", () => {
  it("server HTML has the editor root and viewport but no engine surface", () => {
    const { body } = render(Harness, { props: { value: "const x = 1;" } });
    expect(body).toContain("poodle-code-editor");
    expect(body).toContain("poodle-code-editor__viewport");
    expect(body).not.toContain("cm-editor");
    expect(body).not.toContain("cm-content");
  });
});

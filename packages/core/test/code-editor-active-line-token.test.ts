import { readFileSync } from "node:fs";

import { describe, expect, test } from "bun:test";

const editorCss = readFileSync(new URL("../src/styles/code-editor.css", import.meta.url), "utf8");
const tokenCss = readFileSync(
  new URL("../src/tokens/generated/css/poodle-tokens.css", import.meta.url),
  "utf8",
);

describe("code-editor active line token", () => {
  test("active line and gutter use the selection role", () => {
    expect(editorCss).toMatch(
      /\.poodle-code-editor__viewport \.cm-activeLineGutter,\s*\n\.poodle-code-editor__viewport \.cm-activeLine \{\s*\n\s*background: var\(--poodle-style-selection-itemFill\);/,
    );
    expect(tokenCss).toMatch(
      /--poodle-style-selection-itemFill:\s*var\(--poodle-color-select-neutral\);/,
    );
  });
});

import { readFileSync } from "node:fs";

import { describe, expect, test } from "bun:test";

const editorCss = readFileSync(new URL("../src/styles/code-editor.css", import.meta.url), "utf8");
const tokenCss = readFileSync(
  new URL("../src/tokens/generated/css/poodle-tokens.css", import.meta.url),
  "utf8",
);

describe("code-editor active line token", () => {
  test("active line and gutter use the schema surface-hover token", () => {
    expect(editorCss).toMatch(
      /\.cm-activeLineGutter,\s*\n\.poodle-code-editor__viewport \.cm-activeLine \{\s*\n\s*background: var\(--poodle-color-surface-hover\);/,
    );
    expect(tokenCss).toMatch(/--poodle-color-surface-hover:\s*rgba\(148, 163, 184, 0\.12\);/);
  });
});

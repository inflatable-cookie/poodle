import { describe, expect, test } from "bun:test";

import {
  applyCodeEditorEdits,
  CODE_EDITOR_LANGUAGES,
  CODE_EDITOR_MAX_BYTES,
  codeEditorByteLength,
  isCodeEditorLanguage,
  isCodeEditorValueAdmissible,
  toCodeEditorChange,
  validateCodeEditorDiagnostics,
} from "../src/code-editor.ts";

describe("code-editor change translation", () => {
  test("single insertion binds offsets into the previous value", () => {
    const change = toCodeEditorChange("hello", [{ range: { from: 5, to: 5 }, insert: " world" }]);
    expect(change.value).toBe("hello world");
    expect(change.edits).toEqual([{ range: { from: 5, to: 5 }, insert: " world" }]);
  });

  test("multi-range edit reports offsets after an earlier replacement", () => {
    const previous = "const a = 1;\nconst b = 2;";
    const change = toCodeEditorChange(previous, [
      { range: { from: 23, to: 24 }, insert: "3" },
      { range: { from: 6, to: 7 }, insert: "z" },
    ]);
    expect(change.edits.map((edit) => edit.range.from)).toEqual([6, 23]);
    expect(applyCodeEditorEdits(previous, change.edits)).toBe(change.value);
  });

  test("astral characters, trailing spaces, and tabs survive exactly", () => {
    const previous = "a\u{1F600}  \t";
    const change = toCodeEditorChange(previous, [{ range: { from: 0, to: 1 }, insert: "b" }]);
    expect(change.value).toBe("b\u{1F600}  \t");
    expect(applyCodeEditorEdits(previous, change.edits)).toBe(change.value);
  });

  test("LF content round-trips byte-exact", () => {
    const previous = "a\nb\n";
    const change = toCodeEditorChange(previous, [{ range: { from: 2, to: 2 }, insert: "c" }]);
    expect(change.value).toBe("a\ncb\n");
    expect(codeEditorByteLength(change.value)).toBe(5);
  });

  test("overlapping edits throw instead of emitting a lying payload", () => {
    expect(() =>
      toCodeEditorChange("hello", [
        { range: { from: 0, to: 3 }, insert: "x" },
        { range: { from: 2, to: 5 }, insert: "y" },
      ]),
    ).toThrow();
  });

  test("out-of-bounds ranges throw", () => {
    expect(() => toCodeEditorChange("hi", [{ range: { from: 0, to: 9 }, insert: "x" }])).toThrow();
    expect(() => toCodeEditorChange("hi", [{ range: { from: -1, to: 1 }, insert: "x" }])).toThrow();
  });
});

describe("code-editor language admission", () => {
  test("the admitted set is exactly the eleven contract languages", () => {
    const expected = [
      "plain-text",
      "markdown",
      "json",
      "yaml",
      "toml",
      "javascript",
      "typescript",
      "html",
      "css",
      "rust",
      "shell",
    ];
    expect([...CODE_EDITOR_LANGUAGES].sort()).toEqual(expected.sort());
  });

  test("svelte and unknown strings fail closed", () => {
    expect(isCodeEditorLanguage("svelte")).toBe(false);
    expect(isCodeEditorLanguage("python")).toBe(false);
    expect(isCodeEditorLanguage("TypeScript")).toBe(false);
    expect(isCodeEditorLanguage("typescript")).toBe(true);
  });
});

describe("code-editor diagnostics", () => {
  test("valid ranges pass through untouched", () => {
    const diagnostics = [
      { id: "a", severity: "error" as const, message: "bad", range: { from: 0, to: 3 } },
      { id: "b", severity: "warning" as const, message: "meh", range: { from: 4, to: 4 } },
    ];
    const result = validateCodeEditorDiagnostics(diagnostics, 5);
    expect(result.valid).toEqual(diagnostics);
    expect(result.refused).toEqual([]);
  });

  test("invalid, out-of-bounds, and duplicate diagnostics are refused, never clamped", () => {
    const result = validateCodeEditorDiagnostics(
      [
        { id: "ok", severity: "info" as const, message: "fine", range: { from: 0, to: 1 } },
        { id: "far", severity: "error" as const, message: "far", range: { from: 4, to: 9 } },
        { id: "backwards", severity: "error" as const, message: "back", range: { from: 2, to: 1 } },
        { id: "ok", severity: "error" as const, message: "dup", range: { from: 0, to: 1 } },
      ],
      5,
    );
    expect(result.valid.map((diagnostic) => diagnostic.id)).toEqual(["ok"]);
    expect(result.refused).toEqual(["far", "backwards", "ok"]);
  });
});

describe("code-editor document envelope", () => {
  test("2 MiB inclusive is admissible; one byte more is not", () => {
    expect(CODE_EDITOR_MAX_BYTES).toBe(2 * 1024 * 1024);
    expect(isCodeEditorValueAdmissible("x".repeat(2 * 1024 * 1024))).toBe(true);
    expect(isCodeEditorValueAdmissible(`x${"y".repeat(2 * 1024 * 1024)}`)).toBe(false);
  });

  test("astral characters count as UTF-8 bytes", () => {
    expect(codeEditorByteLength("\u{1F600}")).toBe(4);
  });
});

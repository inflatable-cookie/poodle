import { describe, expect, test } from "bun:test";

import {
  applyCodeEditorEdits,
  CODE_EDITOR_MAX_BYTES,
  CODE_EDITOR_PLAIN_TEXT,
  codeEditorByteLength,
  createCodeEditorLanguageRegistry,
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

describe("code-editor language registry", () => {
  const loader = () => Promise.resolve({ extension: true });

  test("record and entry inputs build equivalent registries", () => {
    const fromRecord = createCodeEditorLanguageRegistry({ python: loader, kdl: loader });
    const fromEntries = createCodeEditorLanguageRegistry([
      ["python", loader],
      ["kdl", loader],
    ] as const);
    expect(fromRecord.has("python")).toBe(true);
    expect(fromEntries.has("python")).toBe(true);
    expect(fromRecord.has("haskell")).toBe(false);
  });

  test("admits arbitrary consumer-defined ids, including ids absent from Poodle source", () => {
    const registry = createCodeEditorLanguageRegistry({ "brand/lang+2026": loader });
    expect(registry.has("brand/lang+2026")).toBe(true);
    expect(registry.load("brand/lang+2026")).resolves.toEqual({ extension: true });
  });

  test("construction fails closed on empty ids, non-function loaders, and duplicates", () => {
    expect(() => createCodeEditorLanguageRegistry({ "": loader })).toThrow(/non-empty strings/);
    expect(() => createCodeEditorLanguageRegistry([["kdl", "not a function"]] as never)).toThrow(
      /lazy loader function/,
    );
    expect(() =>
      createCodeEditorLanguageRegistry([
        ["kdl", loader],
        ["kdl", loader],
      ] as const),
    ).toThrow(/duplicate language/);
  });

  test("plain-text is built in and cannot be registered", () => {
    expect(CODE_EDITOR_PLAIN_TEXT).toBe("plain-text");
    expect(() => createCodeEditorLanguageRegistry({ "plain-text": loader })).toThrow(
      /built in/,
    );
  });

  test("unknown ids reject and never fall back to plain text", async () => {
    const registry = createCodeEditorLanguageRegistry({ python: loader });
    await expect(registry.load("cobol")).rejects.toThrow(/unsupported language/);
  });

  test("each admitted id loads exactly once per registry, including across switches", async () => {
    let calls = 0;
    const registry = createCodeEditorLanguageRegistry({
      slow: () => {
        calls += 1;
        return new Promise((resolve) => setTimeout(() => resolve({ extension: true }), 0));
      },
    });
    const [first, second] = await Promise.all([registry.load("slow"), registry.load("slow")]);
    expect(calls).toBe(1);
    expect(first).toBe(second);
    await registry.load("slow");
    expect(calls).toBe(1);
  });

  test("a rejected load stays memoized as the failure it is", async () => {
    let calls = 0;
    const registry = createCodeEditorLanguageRegistry({
      broken: () => {
        calls += 1;
        return Promise.reject(new Error("grammar exploded"));
      },
    });
    await expect(registry.load("broken")).rejects.toThrow("grammar exploded");
    await expect(registry.load("broken")).rejects.toThrow("grammar exploded");
    expect(calls).toBe(1);
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

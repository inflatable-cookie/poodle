import { describe, expect, test } from "bun:test";

import {
  countRichTextNodes,
  isRichTextCommand,
  isRichTextDocumentAdmissible,
  isRichTextFeature,
  RICH_TEXT_COMMANDS,
  RICH_TEXT_COMMAND_LABELS,
  RICH_TEXT_FEATURES,
  RICH_TEXT_FEATURE_COMMANDS,
  RICH_TEXT_MAX_BYTES,
  RICH_TEXT_MAX_NODES,
  RICH_TEXT_STANDARD_FEATURES,
  RICH_TEXT_TOGGLE_COMMANDS,
  resolveRichTextToolbar,
  richTextAdmittedCommands,
  richTextSerializedByteLength,
  validateRichTextFeatures,
  validateRichTextToolbar,
  type ProseMirrorDocumentJSON,
} from "../src/rich-text.ts";

const DOC: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [{ type: "paragraph", content: [{ type: "text", text: "hello" }] }],
};

describe("rich-text feature registry", () => {
  test("the feature set is closed and exact", () => {
    expect(RICH_TEXT_FEATURES).toHaveLength(9);
    expect(RICH_TEXT_STANDARD_FEATURES).toHaveLength(8);
    expect(RICH_TEXT_STANDARD_FEATURES.includes("images")).toBe(false);
    for (const feature of RICH_TEXT_FEATURES) expect(isRichTextFeature(feature)).toBe(true);
    expect(isRichTextFeature("embeds")).toBe(false);
    expect(isRichTextFeature("mentions")).toBe(false);
    expect(isRichTextFeature("")).toBe(false);
  });

  test("unknown and duplicate features fail closed without silent dedupe", () => {
    const result = validateRichTextFeatures([
      "formatting",
      "formatting",
      "tables",
      "mentions",
    ] as never[]);
    expect(result.features).toEqual(["formatting", "tables"]);
    expect(result.refused).toEqual(["formatting", "mentions"]);
  });

  test("a fully admitted list survives unchanged", () => {
    const result = validateRichTextFeatures(RICH_TEXT_FEATURES);
    expect(result.features).toEqual([...RICH_TEXT_FEATURES]);
    expect(result.refused).toEqual([]);
  });
});

describe("rich-text command registry", () => {
  test("the command set is closed and every command has a label", () => {
    expect(RICH_TEXT_COMMANDS).toHaveLength(20);
    for (const command of RICH_TEXT_COMMANDS) {
      expect(isRichTextCommand(command)).toBe(true);
      expect(RICH_TEXT_COMMAND_LABELS[command]).toBeTruthy();
    }
    expect(isRichTextCommand("insert-embed")).toBe(false);
  });

  test("feature-to-command ownership covers every non-history command", () => {
    const owned = new Set<string>();
    for (const feature of RICH_TEXT_FEATURES) {
      for (const command of RICH_TEXT_FEATURE_COMMANDS[feature]) owned.add(command);
    }
    owned.add("undo");
    owned.add("redo");
    expect([...owned].sort()).toEqual([...RICH_TEXT_COMMANDS].sort());
  });

  test("admitted commands follow the enabled features", () => {
    expect(richTextAdmittedCommands(RICH_TEXT_STANDARD_FEATURES).includes("insert-image")).toBe(
      false,
    );
    expect(richTextAdmittedCommands(["images", "tables"]).includes("bold")).toBe(false);
    const admitted = richTextAdmittedCommands(["images"]);
    expect(admitted).toEqual(
      expect.arrayContaining(["undo", "redo", "insert-image"]),
    );
  });

  test("the automatic toolbar derives from features and requires requestImage for images", () => {
    const withoutImages = resolveRichTextToolbar("auto", RICH_TEXT_STANDARD_FEATURES, null);
    expect(withoutImages.refused).toEqual([]);
    expect(withoutImages.commands).toEqual(
      expect.arrayContaining(["undo", "bold", "heading-2", "insert-table", "delete-table"]),
    );
    expect(withoutImages.commands).not.toContain("insert-image");

    const withImages = resolveRichTextToolbar("auto", ["images"], () => Promise.resolve(null));
    expect(withImages.commands).toEqual(["undo", "redo", "insert-image"]);

    // Images enabled but no host requestImage: the command never appears.
    const imagesWithoutChoice = resolveRichTextToolbar("auto", ["images"], null);
    expect(imagesWithoutChoice.commands).toEqual(["undo", "redo"]);
  });

  test("explicit toolbars refuse unknown and unowned commands", () => {
    const result = validateRichTextToolbar(
      ["bold", "insert-image", "heading-9", "bold"] as never[],
      ["formatting"],
    );
    expect(result.commands).toEqual(["bold"]);
    expect(result.refused).toEqual(["insert-image", "heading-9"]);
  });
});

describe("rich-text toggle commands", () => {
  test("toggle commands are pressed-state controls", () => {
    expect(RICH_TEXT_TOGGLE_COMMANDS).toHaveLength(12);
    expect(RICH_TEXT_TOGGLE_COMMANDS).not.toContain("undo");
    expect(RICH_TEXT_TOGGLE_COMMANDS).not.toContain("insert-table");
  });
});

describe("rich-text document envelope", () => {
  test("2 MiB of UTF-8 JSON and 10,000 nodes are the exact bounds", () => {
    expect(RICH_TEXT_MAX_BYTES).toBe(2 * 1024 * 1024);
    expect(RICH_TEXT_MAX_NODES).toBe(10_000);
    expect(isRichTextDocumentAdmissible(DOC)).toBe(true);
    expect(richTextSerializedByteLength(DOC)).toBeGreaterThan(0);
  });

  test("node counting covers text, marks, and nested content", () => {
    const rich: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [
        {
          type: "paragraph",
          content: [
            { type: "text", text: "a", marks: [{ type: "bold" }, { type: "italic" }] },
            { type: "hardBreak" },
            { type: "text", text: "b" },
          ],
        },
      ],
    };
    // doc + paragraph + 2 text + 2 marks + hardBreak
    expect(countRichTextNodes(rich)).toBe(7);
  });

  test("oversized documents are refused", () => {
    const many: ProseMirrorDocumentJSON = {
      type: "doc",
      content: Array.from({ length: RICH_TEXT_MAX_NODES }, () => ({
        type: "paragraph",
        content: [{ type: "text", text: "x" }],
      })),
    };
    expect(isRichTextDocumentAdmissible(many)).toBe(false);
    const big: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [{ type: "paragraph", content: [{ type: "text", text: "y".repeat(RICH_TEXT_MAX_BYTES) }] }],
    };
    expect(isRichTextDocumentAdmissible(big)).toBe(false);
  });
});

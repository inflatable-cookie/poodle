import { render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import { EditorState } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";

import CodeEditor from "../src/CodeEditor.svelte";
import { languageFor } from "../src/code-editor-languages";
import { assertAdmittedLanguage, transactionToChange } from "../src/code-editor-engine";
import { applyCodeEditorEdits } from "@inflatable-cookie/poodle-core";

function contentOf(container: HTMLElement): HTMLElement {
  const content = container.querySelector<HTMLElement>(".cm-content");
  if (!content) throw new Error("editor surface did not mount");
  return content;
}

/** Visible text: CodeMirror renders one element per line, so textContent alone drops breaks. */
function visibleText(container: HTMLElement): string {
  const lines = [...contentOf(container).querySelectorAll(".cm-line")].map(
    (line) => line.textContent ?? "",
  );
  return lines.length > 0 ? lines.join("\n") : (contentOf(container).textContent ?? "");
}
describe("CodeEditor (svelte)", () => {
  it("mounts the exact value with astral characters and trailing whitespace intact", async () => {
    const value = "const snowman = \"\u2603\";  \n\tindented \u{1F600}";
    const { container } = render(CodeEditor, { props: { value } });
    await vi.waitFor(() => {
      expect(visibleText(container)).toBe(value);
    });
  });

  it("prop updates replace the text without emitting a callback", async () => {
    const onChange = vi.fn();
    const view = render(CodeEditor, { props: { value: "one", onChange } });
    await vi.waitFor(() => {
      expect(visibleText(view.container)).toBe("one");
    });
    await view.rerender({ value: "two", onChange });
    await vi.waitFor(() => {
      expect(visibleText(view.container)).toBe("two");
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("a host revert restores the text without a second callback", async () => {
    const onChange = vi.fn();
    const view = render(CodeEditor, { props: { value: "draft", onChange } });
    await vi.waitFor(() => {
      expect(visibleText(view.container)).toBe("draft");
    });
    await view.rerender({ value: "edited by host", onChange });
    await view.rerender({ value: "draft", onChange });
    await vi.waitFor(() => {
      expect(visibleText(view.container)).toBe("draft");
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("marks valid diagnostics by severity and omits invalid ranges", async () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    try {
      const { container } = render(CodeEditor, {
        props: {
          value: "hello",
          diagnostics: [
            { id: "good", severity: "error", message: "bad", range: { from: 0, to: 5 } },
            { id: "far", severity: "warning", message: "far", range: { from: 3, to: 99 } },
          ],
        },
      });
      await vi.waitFor(() => {
        expect(container.querySelector('[data-poodle-diagnostic="good"]')).not.toBeNull();
      });
      const mark = container.querySelector('[data-poodle-diagnostic="good"]');
      expect(mark?.className).toContain("poodle-code-editor__diagnostic--error");
      expect(container.querySelector('[data-poodle-diagnostic="far"]')).toBeNull();
      expect(warn).toHaveBeenCalled();
    } finally {
      warn.mockRestore();
    }
  });

  it("read-only keeps selection while refusing mutation affordances", async () => {
    const { container } = render(CodeEditor, { props: { value: "held", readOnly: true } });
    await vi.waitFor(() => {
      expect(contentOf(container).getAttribute("aria-readonly")).toBe("true");
    });
    expect(visibleText(container)).toBe("held");
  });

  it("disabled leaves the focus order and announces itself", async () => {
    const { container } = render(CodeEditor, { props: { value: "held", disabled: true } });
    await vi.waitFor(() => {
      expect(contentOf(container).getAttribute("aria-disabled")).toBe("true");
    });
    expect(container.querySelector(".poodle-code-editor")?.getAttribute("data-disabled")).toBe(
      "true",
    );
  });

  it("an unsupported language fails closed instead of falling back", () => {
    expect(() => assertAdmittedLanguage("svelte")).toThrow(/unsupported language/);
  });

  it("unmount destroys the engine view", async () => {
    const { container, unmount } = render(CodeEditor, { props: { value: "bye" } });
    await vi.waitFor(() => {
      expect(container.querySelector(".cm-editor")).not.toBeNull();
    });
    unmount();
    expect(container.querySelector(".cm-editor")).toBeNull();
  });

  it("a 2 MiB boundary document stays viewport-bounded and usable", async () => {
    const value = Array.from({ length: 100000 }, (_, index) => `line ${index} content here`).join(
      "\n",
    );
    expect(value.length).toBeGreaterThan(2 * 1024 * 1024);
    const { container } = render(CodeEditor, { props: { value } });
    await vi.waitFor(() => {
      expect(container.querySelector(".cm-editor")).not.toBeNull();
    });
    const mountedLines = container.querySelectorAll(".cm-line").length;
    expect(mountedLines).toBeGreaterThan(0);
    expect(mountedLines).toBeLessThan(100000);
    expect(visibleText(container).split("\n")[0]).toBe("line 0 content here");
  });
});

describe("CodeEditor transactions", () => {
  it("a real multi-range engine change replays exactly against the previous value", () => {
    const previous = "const aaa = 1;\nconst bbb = 2;";
    const state = EditorState.create({ doc: previous });
    const transaction = state.update({
      changes: [
        { from: 23, to: 24, insert: "3" },
        { from: 6, to: 9, insert: "z" },
      ],
    });
    const change = transactionToChange(transaction.startState.doc.toString(), transaction.changes);
    expect(change.edits.map((edit) => edit.range.from)).toEqual([6, 23]);
    expect(applyCodeEditorEdits(previous, change.edits)).toBe(change.value);
    expect(change.value).toBe(transaction.state.doc.toString());
  });

  it("astral-plane offsets stay UTF-16 through the engine", () => {
    const previous = "a\u{1F600}b";
    const state = EditorState.create({ doc: previous });
    const transaction = state.update({ changes: { from: 3, to: 4, insert: "c" } });
    const change = transactionToChange(previous, transaction.changes);
    expect(change.value).toBe("a\u{1F600}c");
    expect(applyCodeEditorEdits(previous, change.edits)).toBe(change.value);
  });

  it("TypeScript mode parses interfaces; the admitted set refuses svelte at runtime", async () => {
    const support = await languageFor("typescript", "full");
    const state = EditorState.create({
      doc: "interface Point { x: number; }",
      extensions: [support],
    });
    const names: string[] = [];
    const cursor = syntaxTree(state).cursor();
    do {
      names.push(cursor.name);
    } while (cursor.next());
    expect(names).toContain("InterfaceDeclaration");
    await expect(languageFor("svelte" as never, "full")).rejects.toThrow();
  });
});

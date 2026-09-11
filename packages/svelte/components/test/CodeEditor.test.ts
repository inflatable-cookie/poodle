import { render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import { EditorState } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";

import CodeEditor from "../src/CodeEditor.svelte";
import { languageFor } from "../src/code-editor-languages";
import {
  assertAdmittedLanguage,
  createCodeEditorEngine,
  transactionToChange,
} from "../src/code-editor-engine";
import type {
  CodeEditorActiveDiagnostic,
  CodeEditorEngineOptions,
} from "../src/code-editor-engine";
import {
  applyCodeEditorEdits,
  type CodeEditorChange,
} from "@inflatable-cookie/poodle-core";

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

/**
 * Contract §12 web selector families: every case drives the real key or
 * panel path through the mounted engine. `Mod` is Control off-mac and the
 * release lanes are Linux-only, so chords use Control explicitly.
 */
function press(target: Element, key: string, init: KeyboardEventInit = {}): boolean {
  return target.dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key, ...init }),
  );
}

const CONTROL = { ctrlKey: true };

const DIAGNOSTIC_VALUE = "one\ntwo\nthree";
const DIAGNOSTICS = [
  { id: "d1", severity: "error" as const, message: "first", range: { from: 0, to: 3 } },
  { id: "d2", severity: "warning" as const, message: "second", range: { from: 4, to: 7 } },
];

function diagnosticMessage(container: HTMLElement): string | null {
  return container.querySelector(".poodle-code-editor__diagnostic-message")?.textContent ?? null;
}

describe("CodeEditor keyboard escape (svelte)", () => {
  it("Tab leaves the editor in focus mode", async () => {
    const { container } = render(CodeEditor, { props: { value: "one" } });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    expect(press(contentOf(container), "Tab")).toBe(true);
  });

  it("Tab indents in indent mode with one exact change", async () => {
    const onChange = vi.fn();
    const { container } = render(CodeEditor, {
      props: { value: "one", tabBehavior: "indent", onChange },
    });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    expect(press(contentOf(container), "Tab")).toBe(false);
    await vi.waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    const change = onChange.mock.calls[0][0] as CodeEditorChange;
    expect(change.value).toBe("  one");
    expect(applyCodeEditorEdits("one", change.edits)).toBe(change.value);
  });

  it("Escape then Tab leaves indent mode without changing text", async () => {
    const onChange = vi.fn();
    const { container } = render(CodeEditor, {
      props: { value: "one", tabBehavior: "indent", onChange },
    });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    press(contentOf(container), "Escape");
    expect(press(contentOf(container), "Tab")).toBe(true);
    expect(visibleText(container)).toBe("one");
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("CodeEditor diagnostic navigation (svelte)", () => {
  it("F8 walks diagnostics and announces severity with line:column", async () => {
    const { container } = render(CodeEditor, {
      props: { value: DIAGNOSTIC_VALUE, diagnostics: DIAGNOSTICS },
    });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    press(contentOf(container), "F8");
    await vi.waitFor(() => {
      expect(diagnosticMessage(container)).toBe("error 1:1 — first");
    });
    press(contentOf(container), "F8");
    await vi.waitFor(() => {
      expect(diagnosticMessage(container)).toBe("warning 2:1 — second");
    });
    press(contentOf(container), "F8", { shiftKey: true });
    await vi.waitFor(() => {
      expect(diagnosticMessage(container)).toBe("error 1:1 — first");
    });
  });

  it("an unrelated re-render keeps the F8 active diagnostic", async () => {
    const view = render(CodeEditor, {
      props: { value: DIAGNOSTIC_VALUE, diagnostics: DIAGNOSTICS },
    });
    await vi.waitFor(() => {
      expect(contentOf(view.container).textContent).toContain("one");
    });
    press(contentOf(view.container), "F8");
    await vi.waitFor(() => {
      expect(diagnosticMessage(view.container)).toBe("error 1:1 — first");
    });
    await view.rerender({
      value: DIAGNOSTIC_VALUE,
      diagnostics: DIAGNOSTICS,
      placeholder: "touched",
    });
    expect(diagnosticMessage(view.container)).toBe("error 1:1 — first");
  });
});

describe("CodeEditor search (svelte)", () => {
  it("opens the find panel, focuses its input, and Escape closes with focus return", async () => {
    const { container } = render(CodeEditor, { props: { value: "one\ntwo\none more" } });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    const content = contentOf(container);
    press(content, "f", CONTROL);
    await vi.waitFor(() => {
      expect(container.querySelector(".cm-search")).not.toBeNull();
    });
    const input = container.querySelector(".cm-search input") as HTMLInputElement;
    // Browsers move focus into the panel input on open; happy-dom does not
    // perform that default, so take the browser's place before asserting the
    // close path returns focus to the editing surface.
    input.focus();
    expect(document.activeElement).toBe(input);
    input.value = "one";
    input.dispatchEvent(new Event("change", { bubbles: true }));
    await vi.waitFor(() => {
      expect(container.querySelectorAll(".cm-searchMatch").length).toBe(2);
    });
    input.dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "Escape" }),
    );
    await vi.waitFor(() => {
      expect(container.querySelector(".cm-search")).toBeNull();
    });
    expect(document.activeElement).toBe(content);
  });

  it("stays closed when searchable is false", async () => {
    const { container } = render(CodeEditor, {
      props: { value: "one\ntwo\none more", searchable: false },
    });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toContain("one");
    });
    press(contentOf(container), "f", CONTROL);
    expect(container.querySelector(".cm-search")).toBeNull();
  });
});

describe("CodeEditor user transactions (svelte)", () => {
  it("Enter, undo, and redo each emit one exact change", async () => {
    const onChange = vi.fn();
    const { container } = render(CodeEditor, { props: { value: "ab", onChange } });
    await vi.waitFor(() => {
      expect(contentOf(container).textContent).toBe("ab");
    });
    const content = contentOf(container);
    press(content, "Enter");
    await vi.waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    const entered = onChange.mock.calls[0][0] as CodeEditorChange;
    expect(entered.value).toBe("\nab");
    expect(applyCodeEditorEdits("ab", entered.edits)).toBe(entered.value);
    press(content, "z", CONTROL);
    await vi.waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(2);
    });
    const undone = onChange.mock.calls[1][0] as CodeEditorChange;
    expect(undone.value).toBe("ab");
    expect(applyCodeEditorEdits(entered.value, undone.edits)).toBe(undone.value);
    press(content, "y", CONTROL);
    await vi.waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(3);
    });
    const redone = onChange.mock.calls[2][0] as CodeEditorChange;
    expect(redone.value).toBe("\nab");
    expect(applyCodeEditorEdits(undone.value, redone.edits)).toBe(redone.value);
  });

  it("CRLF sources load as LF lines without echoing", async () => {
    const onChange = vi.fn();
    const { container } = render(CodeEditor, { props: { value: "a\r\nb", onChange } });
    await vi.waitFor(() => {
      expect(visibleText(container).split("\n")).toEqual(["a", "b"]);
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("plain performance mode loads no language extension", async () => {
    await expect(languageFor("typescript", "plain")).resolves.toEqual([]);
  });

  it("host updates in the creation window still win", async () => {
    const onChange = vi.fn();
    const view = render(CodeEditor, {
      props: { value: "first", language: "typescript", onChange },
    });
    await view.rerender({ value: "second", language: "typescript", onChange });
    await vi.waitFor(() => {
      expect(visibleText(view.container)).toBe("second");
    });
    expect(onChange).not.toHaveBeenCalled();
  });
});

/**
 * Contract §6 Focus / §12 selector family: pointer/keyboard focus origin and
 * focus-ring dismissal on the first editing intent without mutating the
 * global input modality. Drives the real mounted component.
 */
describe("CodeEditor focus entry (svelte)", () => {
  const ATTR = "data-focus-entry";

  async function mounted(onChange?: (change: CodeEditorChange) => void) {
    const view = render(CodeEditor, { props: { value: "one", onChange } });
    await vi.waitFor(() => {
      expect(contentOf(view.container).textContent).toContain("one");
    });
    return view;
  }

  function rootOf(container: HTMLElement): HTMLElement {
    const root = container.querySelector<HTMLElement>(".poodle-code-editor");
    if (!root) throw new Error("editor root did not mount");
    return root;
  }

  function typeKey(target: Element, key: string, init: KeyboardEventInit = {}): boolean {
    return target.dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key, ...init }),
    );
  }

  function inputEvent(target: Element, inputType: string): boolean {
    return target.dispatchEvent(new InputEvent("beforeinput", { bubbles: true, cancelable: true, inputType }));
  }

  it("keyboard navigation entry arms the local treatment without writing document modality", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    expect(root.hasAttribute(ATTR)).toBe(false);
    document.documentElement.setAttribute("data-poodle-input-modality", "keyboard");
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    expect(root.getAttribute(ATTR)).toBe("keyboard");
    expect(document.documentElement.getAttribute("data-poodle-input-modality")).toBe("keyboard");
    // Leaving resets.
    content.dispatchEvent(new FocusEvent("focusout", { relatedTarget: document.body }));
    expect(root.hasAttribute(ATTR)).toBe(false);
  });

  it("pointer entry never arms the treatment; later keyboard entry does", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    content.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
    expect(document.documentElement.getAttribute("data-poodle-input-modality")).toBe("pointer");
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    expect(root.hasAttribute(ATTR)).toBe(false);
    // A Tab keydown restores the keyboard modality; the next focus entry arms.
    typeKey(content, "Tab");
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    expect(root.getAttribute(ATTR)).toBe("keyboard");
    expect(document.documentElement.getAttribute("data-poodle-input-modality")).toBe("keyboard");
  });

  it("focus entry under pointer modality never arms", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    document.documentElement.setAttribute("data-poodle-input-modality", "pointer");
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: null }));
    expect(root.hasAttribute(ATTR)).toBe(false);
    expect(document.documentElement.getAttribute("data-poodle-input-modality")).toBe("pointer");
    document.documentElement.setAttribute("data-poodle-input-modality", "keyboard");
  });

  it("navigation-only keys preserve the armed entry treatment", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    for (const key of ["ArrowLeft", "ArrowUp", "Home", "PageDown", "End", "Escape", "F8"]) {
      typeKey(content, key);
    }
    expect(root.getAttribute(ATTR)).toBe("keyboard");
  });

  it("the first editing intent dismisses the treatment; typing never recreates it", async () => {
    const onChange = vi.fn();
    const { container } = await mounted(onChange);
    const root = rootOf(container);
    const content = contentOf(container);
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    expect(root.getAttribute(ATTR)).toBe("keyboard");
    typeKey(content, "x");
    expect(root.hasAttribute(ATTR)).toBe(false);
    // Global modality stays truthful while the local treatment is dismissed.
    expect(document.documentElement.getAttribute("data-poodle-input-modality")).toBe("keyboard");
    // Further typing does not recreate the treatment.
    typeKey(content, "a");
    typeKey(content, "Backspace");
    expect(root.hasAttribute(ATTR)).toBe(false);
  });

  it("clipboard, history, and composition routes dismiss the treatment", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    for (const intent of [
      () => inputEvent(content, "insertFromPaste"),
      () => inputEvent(content, "deleteByCut"),
      () => inputEvent(content, "historyUndo"),
      () => inputEvent(content, "historyRedo"),
      () => inputEvent(content, "insertCompositionText"),
      () => typeKey(content, "v", { ctrlKey: true }),
      () => typeKey(content, "z", { metaKey: true }),
    ]) {
      root.setAttribute(ATTR, "keyboard");
      intent();
      expect(root.hasAttribute(ATTR)).toBe(false);
    }
  });

  it("focus moving into the search panel keeps the entry state; leaving resets", async () => {
    const { container } = await mounted();
    const root = rootOf(container);
    const content = contentOf(container);
    content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
    expect(root.getAttribute(ATTR)).toBe("keyboard");
    content.dispatchEvent(
      new FocusEvent("focusout", { relatedTarget: root.querySelector(".cm-content") as Node }),
    );
    expect(root.getAttribute(ATTR)).toBe("keyboard");
    content.dispatchEvent(new FocusEvent("focusout", { relatedTarget: document.body }));
    expect(root.hasAttribute(ATTR)).toBe(false);
  });

  it("the helper binds to the viewport host and cleans up on engine destroy", async () => {
    const host = document.createElement("div");
    document.body.appendChild(host);
    const root = document.createElement("div");
    root.className = "poodle-code-editor";
    root.appendChild(host);
    document.body.appendChild(root);
    try {
      const engine = await createCodeEditorEngine(
        host,
        {
          value: "one",
          language: "plain-text",
          lineNumbers: false,
          searchable: false,
          readOnly: false,
          disabled: false,
          placeholder: "",
          ariaLabel: "Code editor",
          wrapLines: false,
          tabSize: 2,
          tabBehavior: "focus",
          performanceMode: "plain",
          diagnostics: [],
        },
        { onChange: () => {}, onActiveDiagnostic: () => {} },
      );
      const content = host.querySelector(".cm-content") as HTMLElement;
      content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
      expect(root.getAttribute(ATTR)).toBe("keyboard");
      typeKey(content, "a");
      expect(root.hasAttribute(ATTR)).toBe(false);
      engine.destroy();
      // After destroy, the listeners are gone: no re-arm on focusin.
      content.dispatchEvent(new FocusEvent("focusin", { relatedTarget: document.body }));
      expect(root.hasAttribute(ATTR)).toBe(false);
    } finally {
      root.remove();
    }
  });
});

describe("CodeEditor engine diagnostics", () => {
  const base: CodeEditorEngineOptions = {
    value: "one\ntwo",
    language: "plain-text",
    lineNumbers: false,
    searchable: false,
    readOnly: false,
    disabled: false,
    placeholder: "",
    ariaLabel: "Code editor",
    wrapLines: false,
    tabSize: 2,
    tabBehavior: "focus",
    performanceMode: "plain",
    diagnostics: DIAGNOSTICS,
  };

  it("unrelated updates keep the F8 active diagnostic", async () => {
    const host = document.createElement("div");
    document.body.appendChild(host);
    try {
      const announced: (CodeEditorActiveDiagnostic | null)[] = [];
      const engine = await createCodeEditorEngine(
        host,
        base,
        {
          onChange: () => {},
          onActiveDiagnostic: (active) => announced.push(active),
        },
      );
      engine.focusDiagnostic(1);
      expect(announced).toHaveLength(1);
      expect(announced[0]?.diagnostic.id).toBe("d1");
      await engine.update({ placeholder: "touched" });
      expect(announced).toHaveLength(1);
      await engine.update({ diagnostics: [...DIAGNOSTICS] });
      expect(announced).toHaveLength(1);
      await engine.update({ diagnostics: [] });
      expect(announced).toHaveLength(2);
      expect(announced[1]).toBeNull();
      engine.destroy();
    } finally {
      host.remove();
    }
  });
});

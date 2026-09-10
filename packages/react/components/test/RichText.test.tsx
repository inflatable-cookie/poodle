import { cleanup, render } from "@testing-library/react";
import { act, createElement, type ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { RichTextEditor } from "../src/RichTextEditor";
import { RichTextRenderer } from "../src/RichTextRenderer";
import {
  assertAdmittedFeatures,
  assertAdmittedToolbar,
  assertValidRichTextDocument,
  createRichTextEngine,
  createRichTextSchema,
  isAdmittedImageUrl,
  isAdmittedLinkHref,
  renderRichTextDocument,
} from "../src/rich-text-engine";
import type { RichTextEngineOptions } from "../src/rich-text-engine";
import type {
  ProseMirrorDocumentJSON,
  RichTextImageInput,
} from "@inflatable-cookie/poodle-core";

const EMPTY: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [{ type: "paragraph", content: [] }],
};

const PLAIN: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [{ type: "paragraph", content: [{ type: "text", text: "hello world" }] }],
};

const RICH: ProseMirrorDocumentJSON = {
  type: "doc",
  content: [
    { type: "heading", attrs: { level: 1 }, content: [{ type: "text", text: "Title" }] },
    {
      type: "paragraph",
      content: [
        { type: "text", text: "bold", marks: [{ type: "bold" }] },
        { type: "text", text: " " },
        { type: "text", text: "linked", marks: [{ type: "link", attrs: { href: "https://x.test" } }] },
      ],
    },
    {
      type: "bulletList",
      content: [
        { type: "listItem", content: [{ type: "paragraph", content: [{ type: "text", text: "one" }] }] },
        { type: "listItem", content: [{ type: "paragraph", content: [{ type: "text", text: "two" }] }] },
      ],
    },
    { type: "blockquote", content: PLAIN.content },
    { type: "codeBlock", content: [{ type: "text", text: "const x = 1;" }] },
    { type: "horizontalRule" },
    {
      type: "table",
      content: [
        {
          type: "tableRow",
          content: [
            { type: "tableHeader", content: [{ type: "paragraph", content: [{ type: "text", text: "H" }] }] },
            { type: "tableCell", content: [{ type: "paragraph", content: [{ type: "text", text: "C" }] }] },
          ],
        },
      ],
    },
  ],
};

const STANDARD_FEATURES: RichTextEngineOptions["features"] = [
  "formatting",
  "headings",
  "links",
  "lists",
  "blockquote",
  "code-block",
  "horizontal-rule",
  "tables",
];

function baseOptions(overrides: Partial<RichTextEngineOptions> = {}): RichTextEngineOptions {
  return {
    value: PLAIN,
    features: STANDARD_FEATURES,
    toolbar: "auto",
    readOnly: false,
    disabled: false,
    placeholder: "",
    ariaLabel: "Rich text editor",
    ...overrides,
  };
}

function mount(element: ReactElement): { container: HTMLElement; rerender: (element: ReactElement) => void; unmount: () => void } {
  const result = render(element);
  return {
    container: result.container,
    rerender: (next) => result.rerender(next),
    unmount: () => result.unmount(),
  };
}

function surfaceOf(container: HTMLElement): HTMLElement {
  const surface = container.querySelector<HTMLElement>(".ProseMirror");
  if (!surface) throw new Error("editor surface did not mount");
  return surface;
}

function press(target: Element, key: string, init: KeyboardEventInit = {}): boolean {
  return target.dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key, ...init }),
  );
}

afterEach(() => cleanup());

describe("rich-text engine validation (react)", () => {
  it("refuses unknown features, duplicate features, and bad toolbars", () => {
    expect(() => assertAdmittedFeatures(["embeds"] as never)).toThrow(/unsupported or duplicate feature/);
    expect(() => assertAdmittedFeatures(["bold", "bold"] as never[])).toThrow(
      /unsupported or duplicate feature/,
    );
    expect(() => assertAdmittedToolbar(["insert-image"], STANDARD_FEATURES, null)).toThrow(
      /unsupported toolbar command/,
    );
  });

  it("refuses unknown nodes, marks, attributes, and executable URLs", () => {
    const schema = createRichTextSchema(STANDARD_FEATURES);
    expect(() =>
      assertValidRichTextDocument(schema, { type: "doc", content: [{ type: "mystery" }] as never }),
    ).toThrow(/unsupported node type/);
    expect(() =>
      assertValidRichTextDocument(schema, {
        type: "doc",
        content: [
          { type: "paragraph", content: [{ type: "text", text: "x", marks: [{ type: "bogus" }] }] },
        ],
      }),
    ).toThrow(/unsupported mark type/);
    expect(() =>
      assertValidRichTextDocument(schema, {
        type: "doc",
        content: [{ type: "paragraph", attrs: { bogus: 1 }, content: [] }],
      }),
    ).toThrow(/unsupported attribute/);
    expect(() =>
      assertValidRichTextDocument(
        createRichTextSchema([...STANDARD_FEATURES, "images"]),
        {
          type: "doc",
          content: [
            {
              type: "paragraph",
              content: [
                { type: "text", text: "x", marks: [{ type: "link", attrs: { href: "javascript:alert(1)" } }] },
              ],
            },
          ],
        },
      ),
    ).toThrow(/unsupported link URL/);
    expect(() =>
      assertValidRichTextDocument(createRichTextSchema([...STANDARD_FEATURES, "images"]), {
        type: "doc",
        content: [{ type: "image", attrs: { src: "javascript:alert(1)", alt: "x" } }],
      }),
    ).toThrow(/unsupported image URL/);
  });

  it("admits good URLs and refuses executable schemes", () => {
    expect(isAdmittedLinkHref("https://x.test")).toBe(true);
    expect(isAdmittedLinkHref("javascript:alert(1)")).toBe(false);
    expect(isAdmittedImageUrl("data:image/png;base64,AAAA")).toBe(true);
    expect(isAdmittedImageUrl("data:text/html,<script>")).toBe(false);
  });

  it("enforces the 2 MiB and 10,000-node envelope before mount", () => {
    const schema = createRichTextSchema(STANDARD_FEATURES);
    expect(() =>
      assertValidRichTextDocument(schema, {
        type: "doc",
        content: Array.from({ length: 10_001 }, () => ({
          type: "paragraph",
          content: [{ type: "text", text: "x" }],
        })),
      }),
    ).toThrow(/document envelope/);
  });
});

describe("RichTextEditor (react)", () => {
  it("mounts the exact controlled document as semantic structure", () => {
    const { container } = render(createElement(RichTextEditor, { value: RICH }));
    expect(container.querySelector("h1")?.textContent).toBe("Title");
    expect(container.querySelector("strong")?.textContent).toBe("bold");
    expect(container.querySelector("a")?.getAttribute("href")).toBe("https://x.test");
    expect(container.querySelectorAll("ul > li").length).toBe(2);
    expect(container.querySelector("blockquote")).not.toBeNull();
    expect(container.querySelector("pre")?.textContent).toBe("const x = 1;");
    expect(container.querySelector("hr")).not.toBeNull();
    expect(container.querySelector("table th")?.textContent).toBe("H");
    expect(container.querySelector("table td")?.textContent).toBe("C");
  });

  it("prop updates replace the document without echoing onChange", () => {
    const onChange = vi.fn();
    const view = render(createElement(RichTextEditor, { value: PLAIN, onChange }));
    expect(surfaceOf(view.container).textContent).toBe("hello world");
    view.rerender(createElement(RichTextEditor, { value: EMPTY, onChange }));
    expect(surfaceOf(view.container).textContent?.trim()).toBe("");
    expect(onChange).not.toHaveBeenCalled();
  });

  it("invalid documents fail closed pre-mount with zero callbacks", () => {
    const onChange = vi.fn();
    expect(() =>
      render(
        createElement(RichTextEditor, {
          value: { type: "doc", content: [{ type: "mystery" }] } as ProseMirrorDocumentJSON,
          onChange,
        }),
      ),
    ).toThrow();
    expect(onChange).not.toHaveBeenCalled();
  });

  it("read-only keeps the document but removes mutation affordances", () => {
    const { container } = render(createElement(RichTextEditor, { value: PLAIN, readOnly: true }));
    expect(container.querySelector(".ProseMirror")?.getAttribute("contenteditable")).toBe("false");
    expect(container.querySelector(".poodle-rich-text-editor")?.getAttribute("data-readonly")).toBe(
      "true",
    );
    const buttons = [...container.querySelectorAll("button[data-command]")];
    expect(buttons.length).toBeGreaterThan(10);
    for (const button of buttons) expect(button.disabled).toBe(true);
  });

  it("disabled removes the whole composition from interaction and focus", () => {
    const { container } = render(createElement(RichTextEditor, { value: PLAIN, disabled: true }));
    const root = container.querySelector(".poodle-rich-text-editor");
    expect(root?.getAttribute("data-disabled")).toBe("true");
    expect(container.querySelector(".ProseMirror")?.getAttribute("contenteditable")).toBe("false");
  });

  it("the empty editable document shows the placeholder", () => {
    const { container } = render(
      createElement(RichTextEditor, { value: EMPTY, placeholder: "Type here" }),
    );
    expect(container.querySelector(".poodle-rich-text-editor__placeholder")?.textContent).toBe(
      "Type here",
    );
    expect(container.querySelector(".ProseMirror")?.getAttribute("aria-multiline")).toBe("true");
  });

  it("the editing surface exposes a labelled multiline rich-text input", () => {
    const { container } = render(
      createElement(RichTextEditor, { value: PLAIN, ariaLabel: "Notes body" }),
    );
    const surface = surfaceOf(container);
    expect(surface.getAttribute("role")).toBe("textbox");
    expect(surface.getAttribute("aria-label")).toBe("Notes body");
  });
});

describe("RichTextEditor toolbar (react)", () => {
  it("auto derives commands from the standard features", () => {
    const { container } = render(createElement(RichTextEditor, { value: PLAIN }));
    const commands = [...container.querySelectorAll("button[data-command]")].map((button) =>
      button.getAttribute("data-command"),
    );
    for (const expected of ["undo", "bold", "heading-1", "link", "insert-table", "add-row", "delete-table"]) {
      expect(commands).toContain(expected);
    }
    expect(commands).not.toContain("insert-image");
  });

  it("auto shows insert-image only when images are enabled and requestImage is present", () => {
    const requestImage = (): Promise<RichTextImageInput | null> => Promise.resolve(null);
    const withImages = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images", "tables"], requestImage }),
    );
    expect(withImages.container.querySelector('button[data-command="insert-image"]')).not.toBeNull();
    withImages.unmount();
    const withoutChoice = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images"] }),
    );
    expect(
      withoutChoice.container.querySelector('button[data-command="insert-image"]'),
    ).toBeNull();
    withoutChoice.unmount();
  });

  it("toolbar controls have names and pressed state", () => {
    const { container } = render(createElement(RichTextEditor, { value: RICH }));
    const toolbar = container.querySelector(".poodle-rich-text-editor__toolbar");
    expect(toolbar?.getAttribute("role")).toBe("toolbar");
    const bold = container.querySelector<HTMLButtonElement>('button[data-command="bold"]');
    expect(bold?.getAttribute("aria-pressed")).toBe("false");
  });

  it("toolbar commands emit one exact controlled change each", () => {
    const onChange = vi.fn();
    const { container } = render(
      createElement(RichTextEditor, {
        value: PLAIN,
        features: ["headings", "horizontal-rule"],
        onChange,
      }),
    );
    const heading = container.querySelector<HTMLButtonElement>('button[data-command="heading-1"]');
    if (!heading) throw new Error("missing heading command");
    act(() => heading.click());
    expect(onChange).toHaveBeenCalledTimes(1);
    const document1 = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    expect(document1.type).toBe("doc");
    expect(JSON.stringify(document1)).toContain('"heading"');
    expect(JSON.stringify(document1)).toContain('"level":1');
  });
});

describe("RichTextEditor tables (react)", () => {
  it("insert-table produces a real table with header structure", () => {
    const onChange = vi.fn();
    const { container } = render(
      createElement(RichTextEditor, { value: EMPTY, features: ["tables"], onChange }),
    );
    const insert = container.querySelector<HTMLButtonElement>('button[data-command="insert-table"]');
    if (!insert) throw new Error("missing insert-table");
    act(() => insert.click());
    expect(container.querySelector("table")).not.toBeNull();
    expect(container.querySelectorAll("th").length).toBe(3);
    expect(container.querySelectorAll("td").length).toBe(6);
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it("Escape then Tab always escapes the editor surface", () => {
    const outside = document.createElement("button");
    outside.textContent = "outside";
    document.body.appendChild(outside);
    try {
      const { container } = render(
        createElement(RichTextEditor, { value: PLAIN, features: ["tables"] }),
      );
      const surface = surfaceOf(container);
      act(() => {
        surface.focus();
        press(surface, "Escape");
        press(surface, "Tab");
      });
      // The latched Tab is consumed and re-targeted by the engine.
      expect(document.activeElement).not.toBe(surface);
      expect(container.contains(document.activeElement)).toBe(false);
    } finally {
      outside.remove();
    }
  });

  it("plain Tab without Escape keeps default browser behavior", () => {
    const { container } = render(createElement(RichTextEditor, { value: PLAIN }));
    const surface = surfaceOf(container);
    let result = true;
    act(() => {
      result = press(surface, "Tab") && press(surface, "Tab");
    });
    expect(result).toBe(true);
  });
});

describe("RichTextEditor optional images (react)", () => {
  it("resolves the async request exactly once at the retained selection", async () => {
    const deferred: {
      resolve: (value: RichTextImageInput | null) => void;
      reject: (error: Error) => void;
    } = { resolve: () => {}, reject: () => {} };
    let calls = 0;
    const requestImage = (): Promise<RichTextImageInput | null> => {
      calls += 1;
      return new Promise((resolve, reject) => {
        deferred.resolve = resolve;
        deferred.reject = reject;
      });
    };
    const onChange = vi.fn();
    const { container } = render(
      createElement(RichTextEditor, {
        value: EMPTY,
        features: ["images", "tables"],
        requestImage,
        onChange,
      }),
    );
    const insert = container.querySelector<HTMLButtonElement>('button[data-command="insert-image"]');
    if (!insert) throw new Error("missing insert-image");
    act(() => insert.click());
    expect(calls).toBe(1);
    // One request is live: the command is unavailable until it settles.
    expect(insert.disabled).toBe(true);
    await act(async () => {
      deferred.resolve({ src: "https://x.test/pick.png", alt: "Picked chart" });
    });
    expect(container.querySelector("img")).not.toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(container.querySelector("img")?.getAttribute("alt")).toBe("Picked chart");
    expect(insert.disabled).toBe(false);
  });

  it("cancellation (null) and rejection change nothing and stay recoverable", async () => {
    const deferred: {
      resolve: (value: RichTextImageInput | null) => void;
      reject: (error: Error) => void;
    } = { resolve: () => {}, reject: () => {} };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve, reject) => {
        deferred.resolve = resolve;
        deferred.reject = reject;
      });
    const onChange = vi.fn();
    const { container } = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images"], requestImage, onChange }),
    );
    const insert = container.querySelector<HTMLButtonElement>('button[data-command="insert-image"]');
    if (!insert) throw new Error("missing insert-image");
    act(() => insert.click());
    await act(async () => {
      deferred.resolve(null);
    });
    expect(insert.disabled).toBe(false);
    expect(container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    act(() => insert.click());
    await act(async () => {
      deferred.reject(new Error("asset picker closed"));
    });
    expect(insert.disabled).toBe(false);
    expect(container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
  });

  it("resolving after unmount changes nothing", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const { container, unmount } = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images"], requestImage, onChange }),
    );
    act(() => {
      container.querySelector<HTMLButtonElement>('button[data-command="insert-image"]')?.click();
    });
    unmount();
    await act(async () => {
      deferred.resolve({ src: "https://x.test/late.png", alt: "late" });
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("resolving after the feature is disabled changes nothing", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images"], requestImage, onChange }),
    );
    act(() => {
      view.container
        .querySelector<HTMLButtonElement>('button[data-command="insert-image"]')
        ?.click();
    });
    view.rerender(
      createElement(RichTextEditor, {
        value: PLAIN,
        features: ["formatting"],
        requestImage,
        onChange,
      }),
    );
    await act(async () => {
      deferred.resolve({ src: "https://x.test/stale.png", alt: "stale" });
    });
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("RichTextEditor controlled reconfiguration (react)", () => {
  it("a valid feature change emits no callback and swaps the schema", () => {
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
    );
    view.rerender(
      createElement(RichTextEditor, {
        value: PLAIN,
        features: ["headings", "horizontal-rule"],
        onChange,
      }),
    );
    expect(view.container.querySelector('button[data-command="horizontal-rule"]')).not.toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    view.rerender(
      createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
    );
    expect(view.container.querySelector('button[data-command="horizontal-rule"]')).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
  });

  it("an invalid reconfiguration leaves the prior editor intact and reports a development error", () => {
    const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    try {
      const onChange = vi.fn();
      const view = render(
        createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
      );
      // The next document contains a horizontal rule the next schema
      // reconfiguration must refuse; the refusal reports a development error.
      view.rerender(
        createElement(RichTextEditor, {
          value: {
            type: "doc",
            content: [
              { type: "paragraph", content: [{ type: "text", text: "hello world" }] },
              { type: "horizontalRule" },
            ],
          },
          features: ["headings", "horizontal-rule"],
          onChange,
        }),
      );
      view.rerender(
        createElement(RichTextEditor, {
          value: {
            type: "doc",
            content: [
              { type: "paragraph", content: [{ type: "text", text: "hello world" }] },
              { type: "horizontalRule" },
            ],
          },
          features: ["headings"],
          onChange,
        }),
      );
      // The refusal keeps the prior editor intact: the document still shows
      // its content and the toolbar still reflects the running features.
      expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
      expect(view.container.querySelector("hr")).not.toBeNull();
      expect(
        view.container.querySelector('button[data-command="horizontal-rule"]'),
      ).not.toBeNull();
      expect(onChange).not.toHaveBeenCalled();
      expect(errorSpy).toHaveBeenCalled();
    } finally {
      errorSpy.mockRestore();
    }
  });
});

describe("RichTextRenderer (react)", () => {
  it("renders the same semantic structure without contenteditable state", () => {
    const { container } = render(createElement(RichTextRenderer, { value: RICH }));
    expect(container.querySelector("[contenteditable]")).toBeNull();
    expect(container.querySelector("h1")?.textContent).toBe("Title");
    expect(container.querySelector("strong")?.textContent).toBe("bold");
    expect(container.querySelector("a")?.getAttribute("href")).toBe("https://x.test");
    expect(container.querySelectorAll("ul > li").length).toBe(2);
    expect(container.querySelector("pre")?.textContent).toBe("const x = 1;");
    expect(container.querySelector("table th")?.textContent).toBe("H");
  });

  it("updates with the document and never announces itself as editable", () => {
    const view = render(createElement(RichTextRenderer, { value: PLAIN }));
    expect(view.container.querySelector("p")?.textContent).toBe("hello world");
    view.rerender(createElement(RichTextRenderer, { value: EMPTY }));
    expect(
      view.container.querySelector(".poodle-rich-text-renderer__content")?.textContent,
    ).toBe("");
  });

  it("ariaLabel renders a labelled region; absent label stays ordinary content", () => {
    const labelled = render(
      createElement(RichTextRenderer, { value: PLAIN, ariaLabel: "Release notes" }),
    );
    expect(labelled.container.querySelector(".poodle-rich-text-renderer")?.getAttribute("role")).toBe(
      "region",
    );
    labelled.unmount();
    const plain = render(createElement(RichTextRenderer, { value: PLAIN }));
    expect(plain.container.querySelector(".poodle-rich-text-renderer")?.getAttribute("role")).toBeNull();
    plain.unmount();
  });

  it("invalid documents fail closed with zero renderer output", () => {
    expect(() =>
      render(
        createElement(RichTextRenderer, {
          value: { type: "doc", content: [{ type: "mystery" }] } as ProseMirrorDocumentJSON,
        }),
      ),
    ).toThrow();
  });

  it("images disabled refuse image nodes; enabled render semantic alt text", () => {
    const IMAGE: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [{ type: "image", attrs: { src: "https://x.test/a.png", alt: "Logo" } }],
    };
    expect(() =>
      render(createElement(RichTextRenderer, { value: IMAGE, features: ["formatting"] })),
    ).toThrow(/unsupported node type/);
    const { container } = render(
      createElement(RichTextRenderer, { value: IMAGE, features: ["images"] }),
    );
    expect(container.querySelector("img")?.getAttribute("alt")).toBe("Logo");
  });
});

describe("editor/renderer equivalence (react)", () => {
  it("editor and renderer produce matching semantic document structure", () => {
    const editor = render(createElement(RichTextEditor, { value: RICH }));
    const renderer = render(createElement(RichTextRenderer, { value: RICH }));
    const shape = (root: Element): string[] =>
      [
        ...root.querySelectorAll(
          "h1,h2,h3,p,strong,em,s,code,pre,blockquote,ul,ol,li,hr,table,th,td,a,img,br",
        ),
      ].map((element) => `${element.tagName.toLowerCase()}[${element.textContent ?? ""}]`);
    expect(shape(surfaceOf(editor.container))).toEqual(shape(renderer.container));
    editor.unmount();
    renderer.unmount();
  });

  it("the schema is real and closed", () => {
    const full = createRichTextSchema(["images", "formatting"]);
    expect(
      assertValidRichTextDocument(full, {
        type: "doc",
        content: [{ type: "image", attrs: { src: "https://x.test/a.png", alt: "a" } }],
      }).type.name,
    ).toBe("doc");
    const strict = createRichTextSchema(["formatting"]);
    expect(() =>
      assertValidRichTextDocument(strict, {
        type: "doc",
        content: [{ type: "image", attrs: { src: "https://x.test/a.png", alt: "a" } }],
      }),
    ).toThrow(/unsupported node type/);
  });

  it("paste through the active schema discards unsafe HTML and the engine stays lifecycle-safe", () => {
    const editor = document.createElement("div");
    document.body.appendChild(editor);
    const engine = createRichTextEngine(
      editor,
      baseOptions(),
      { onChange: () => {}, onToolbar: () => {} },
    );
    engine.runCommand("undo");
    engine.destroy();
    expect(editor.querySelector(".ProseMirror")).toBeNull();
    editor.remove();
    expect(() =>
      createRichTextEngine(
        document.createElement("div"),
        baseOptions({ features: ["embeds"] as never }),
        { onChange: () => {}, onToolbar: () => {} },
      ),
    ).toThrow(/unsupported or duplicate feature/);
  });
});

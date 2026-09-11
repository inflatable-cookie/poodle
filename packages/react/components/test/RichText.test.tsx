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
import {
  RICH_TEXT_COMMAND_GROUP_LABELS,
  RICH_TEXT_COMMAND_PRESENTATION,
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
    const buttons = [...container.querySelectorAll<HTMLButtonElement>("[data-command] button")];
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
    const commands = [...container.querySelectorAll("[data-command]")].map((button) =>
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
    expect(withImages.container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    withImages.unmount();
    const withoutChoice = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["images"] }),
    );
    expect(
      withoutChoice.container.querySelector('[data-command="insert-image"] button'),
    ).toBeNull();
    withoutChoice.unmount();
  });

  it("toolbar controls have names and pressed state", () => {
    const { container } = render(createElement(RichTextEditor, { value: RICH }));
    const toolbar = container.querySelector(".poodle-rich-text-editor__toolbar");
    expect(toolbar?.getAttribute("role")).toBe("toolbar");
    const bold = container.querySelector<HTMLButtonElement>('[data-command="bold"] button');
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
    const heading = container.querySelector<HTMLButtonElement>('[data-command="heading-1"] button');
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
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-table"] button');
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
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button');
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
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button');
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
      container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button')?.click();
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
        .querySelector<HTMLButtonElement>('[data-command="insert-image"] button')
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
    expect(view.container.querySelector('[data-command="horizontal-rule"] button')).not.toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    view.rerender(
      createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
    );
    expect(view.container.querySelector('[data-command="horizontal-rule"] button')).toBeNull();
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
        view.container.querySelector('[data-command="horizontal-rule"] button'),
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

describe("heading feature gating (react)", () => {
  it("headings disabled: the schema refuses heading nodes", () => {
    const HEADING_DOC: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [{ type: "heading", attrs: { level: 1 }, content: [{ type: "text", text: "T" }] }],
    };
    expect(() =>
      assertValidRichTextDocument(createRichTextSchema(["formatting"]), HEADING_DOC),
    ).toThrow(/unsupported node type/);
    expect(() =>
      assertValidRichTextDocument(createRichTextSchema(["headings"]), HEADING_DOC),
    ).not.toThrow();
  });

  it("headings disabled: heading commands are inert and unavailable", () => {
    const host = document.createElement("div");
    const onChange = vi.fn();
    const engine = createRichTextEngine(
      host,
      baseOptions({ features: ["formatting"] }),
      { onChange, onToolbar: () => {} },
    );
    expect(engine.commandState("heading-1")).toEqual({ available: false, active: false });
    engine.runCommand("heading-1");
    engine.destroy();
    expect(onChange).not.toHaveBeenCalled();
  });

  it("headings disabled: the component refuses a heading document pre-mount", () => {
    const HEADING_DOC: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [{ type: "heading", attrs: { level: 1 }, content: [{ type: "text", text: "T" }] }],
    };
    const onChange = vi.fn();
    expect(() =>
      render(createElement(RichTextEditor, { value: HEADING_DOC, features: ["formatting"], onChange })),
    ).toThrow(/unsupported node type/);
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("host revert of a user edit (react)", () => {
  it("restores the prior document without a second callback (engine)", () => {
    const host = document.createElement("div");
    const onChange = vi.fn();
    const engine = createRichTextEngine(
      host,
      baseOptions({ features: ["headings"] }),
      { onChange, onToolbar: () => {} },
    );
    engine.runCommand("heading-1");
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(host.querySelector("h1")).not.toBeNull();
    engine.update({ value: PLAIN });
    expect(host.querySelector("h1")).toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
    engine.destroy();
  });

  it("restores the prior document without a second callback (component)", () => {
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
    );
    const heading = view.container.querySelector<HTMLButtonElement>(
      '[data-command="heading-1"] button',
    );
    if (!heading) throw new Error("missing heading command");
    act(() => heading.click());
    expect(view.container.querySelector("h1")).not.toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
    // The host restores the pre-edit value (a genuinely new value object,
    // as a controlled host state update would send).
    act(() => {
      view.rerender(
        createElement(RichTextEditor, { value: { ...PLAIN }, features: ["headings"], onChange }),
      );
    });
    expect(view.container.querySelector("h1")).toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it("a host echo of the engine's own payload is a no-op (table normalization)", () => {
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, { value: EMPTY, features: ["tables"], onChange }),
    );
    const insert = view.container.querySelector<HTMLButtonElement>(
      '[data-command="insert-table"] button',
    );
    if (!insert) throw new Error("missing insert-table");
    act(() => insert.click());
    expect(view.container.querySelector("table")).not.toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
    const emitted = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    act(() => {
      view.rerender(
        createElement(RichTextEditor, { value: emitted, features: ["tables"], onChange }),
      );
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(view.container.querySelector("table")).not.toBeNull();
  });
});

describe("live reconfiguration keeps the validator fresh (react)", () => {
  it("a later value-only update mounts a newly enabled feature", () => {
    const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    try {
      const onChange = vi.fn();
      const view = render(
        createElement(RichTextEditor, { value: PLAIN, features: ["headings"], onChange }),
      );
      act(() => {
        view.rerender(
          createElement(RichTextEditor, {
            value: PLAIN,
            features: ["headings", "horizontal-rule"],
            onChange,
          }),
        );
      });
      act(() => {
        view.rerender(
          createElement(RichTextEditor, {
            value: {
              type: "doc",
              content: [
                { type: "paragraph", content: [{ type: "text", text: "a" }] },
                { type: "horizontalRule" },
              ],
            },
            features: ["headings", "horizontal-rule"],
            onChange,
          }),
        );
      });
      expect(view.container.querySelector("hr")).not.toBeNull();
      expect(onChange).not.toHaveBeenCalled();
      expect(errorSpy).not.toHaveBeenCalled();
    } finally {
      errorSpy.mockRestore();
    }
  });
});

describe("requestImage URL admission (react)", () => {
  it("refuses executable URLs and non-string alt without inserting", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, {
        value: PLAIN,
        features: ["images"],
        requestImage,
        onChange,
      }),
    );
    const insert = view.container.querySelector<HTMLButtonElement>(
      '[data-command="insert-image"] button',
    );
    if (!insert) throw new Error("missing insert-image");
    act(() => insert.click());
    expect(insert.disabled).toBe(true);
    await act(async () => {
      deferred.resolve({ src: "javascript:alert(1)", alt: "x" });
    });
    expect(insert.disabled).toBe(false);
    expect(view.container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    // Focus is recoverable: the command is available for a new request.
    act(() => insert.click());
    expect(insert.disabled).toBe(true);
    await act(async () => {
      deferred.resolve({ src: "https://ok.test/a.png", alt: undefined as never });
    });
    expect(insert.disabled).toBe(false);
    expect(view.container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("image onChange round-trip (react)", () => {
  it("emits only admitted attributes that pass the same validator", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const view = render(
      createElement(RichTextEditor, {
        value: PLAIN,
        features: ["images"],
        requestImage,
        onChange,
      }),
    );
    const insert = view.container.querySelector<HTMLButtonElement>(
      '[data-command="insert-image"] button',
    );
    if (!insert) throw new Error("missing insert-image");
    act(() => insert.click());
    await act(async () => {
      deferred.resolve({ src: "https://x.test/a.png", alt: "chart" });
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    const emitted = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    const imageNode = emitted.content?.find((child) => child.type === "image");
    if (!imageNode) throw new Error("no image in the emitted document");
    for (const key of Object.keys(imageNode.attrs ?? {})) {
      expect(["src", "alt", "title"]).toContain(key);
    }
    assertValidRichTextDocument(createRichTextSchema(["images"]), emitted);
    // The host echo of its own onChange payload is a no-op.
    act(() => {
      view.rerender(
        createElement(RichTextEditor, { value: emitted, features: ["images"], requestImage, onChange }),
      );
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(view.container.querySelector("img")).not.toBeNull();
  });
});

describe("paste through the active schema (react)", () => {
  it("emits exactly one controlled document and discards unsafe HTML", () => {
    const onChange = vi.fn();
    const { container } = render(
      createElement(RichTextEditor, {
        value: EMPTY,
        features: [...STANDARD_FEATURES, "images"],
        onChange,
      }),
    );
    const surface = surfaceOf(container);
    const clipboard = new DataTransfer();
    clipboard.setData(
      "text/html",
      '<p onmouseover="x()">pasted <script>alert(1)</script><strong>bold</strong></p>' +
        '<img src="javascript:alert(1)"><img src="https://ok.test/a.png">' +
        '<a href="javascript:alert(2)">bad link</a>',
    );
    let accepted = true;
    act(() => {
      accepted = surface.dispatchEvent(
        new ClipboardEvent("paste", { clipboardData: clipboard, bubbles: true, cancelable: true }),
      );
    });
    expect(accepted).toBe(false);
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(container.querySelector("script")).toBeNull();
    expect(container.querySelector("[onmouseover]")).toBeNull();
    expect(container.querySelectorAll("img")).toHaveLength(1);
    expect(container.querySelector("img")?.getAttribute("src")).toBe("https://ok.test/a.png");
    expect(container.querySelector("a[href]")).toBeNull();
    assertValidRichTextDocument(
      createRichTextSchema([...STANDARD_FEATURES, "images"]),
      onChange.mock.calls[0][0],
    );
  });
});

describe("RichTextEditor toolbar presentation (react)", () => {
  const commandOf = (wrapper: Element): string => wrapper.getAttribute("data-command") ?? "";

  function renderEditor(props: Record<string, unknown> = {}) {
    return render(createElement(RichTextEditor, { value: PLAIN, ...props }));
  }

  function renderAutoToolbar() {
    const view = renderEditor();
    expect(view.container.querySelector('[data-command="bold"] button')).not.toBeNull();
    return view;
  }

  it("renders every admitted command as a real Poodle icon button with the shared name", () => {
    const { container } = renderAutoToolbar();
    const wrappers = [...container.querySelectorAll("[data-command]")];
    expect(wrappers.length).toBeGreaterThan(10);
    for (const wrapper of wrappers) {
      const command = commandOf(wrapper);
      const button = wrapper.querySelector("button.poodle-icon-button");
      expect(button, command).not.toBeNull();
      expect(button?.getAttribute("aria-label")).toBe(
        RICH_TEXT_COMMAND_PRESENTATION[command as keyof typeof RICH_TEXT_COMMAND_PRESENTATION].label,
      );
      // Control chrome belongs to Poodle primitives; no bespoke toolbar class.
      expect(wrapper.querySelector(".poodle-rich-text-editor__toolbar-button")).toBeNull();
    }
  });

  it("groups consecutive same-group commands into intact labelled clusters", () => {
    const { container } = renderAutoToolbar();
    const groups = [...container.querySelectorAll(".poodle-rich-text-editor__group")];
    expect(groups.length).toBeGreaterThan(1);
    const clusterGroups: string[] = [];
    for (const group of groups) {
      expect(group.getAttribute("role")).toBe("group");
      clusterGroups.push(group.getAttribute("aria-label") ?? "");
      for (const wrapper of group.querySelectorAll("[data-command]")) {
        const presentation =
          RICH_TEXT_COMMAND_PRESENTATION[commandOf(wrapper) as keyof typeof RICH_TEXT_COMMAND_PRESENTATION];
        expect(RICH_TEXT_COMMAND_GROUP_LABELS[presentation.group]).toBe(
          group.getAttribute("aria-label"),
        );
      }
    }
    // Every cluster label is a known group name and neighbours never repeat.
    expect(new Set(clusterGroups).size).toBe(clusterGroups.length);
    const featureOrder = [...container.querySelectorAll("[data-command]")].map(commandOf);
    expect(featureOrder[0]).toBe("undo");
    expect(featureOrder.indexOf("insert-table")).toBeGreaterThan(featureOrder.indexOf("link"));
  });

  it("heading controls carry typographic glyphs; icon controls carry SVG icons", () => {
    const { container } = renderAutoToolbar();
    const h1 = container.querySelector('[data-command="heading-1"] button');
    expect(h1?.textContent).toContain("H1");
    expect(h1?.querySelector("svg")).toBeNull();
    const bold = container.querySelector('[data-command="bold"] button');
    expect(bold?.querySelector("svg")).not.toBeNull();
  });

  it("the destructive table command renders distinguishably without changing semantics", () => {
    const { container } = renderAutoToolbar();
    const remove = container.querySelector('[data-command="delete-table"] button');
    expect(remove?.getAttribute("data-tone")).toBe("danger");
    expect(remove?.getAttribute("aria-label")).toBe("Delete table");
  });

  it("pressed state is truthful: only toggle commands expose aria-pressed", () => {
    const { container } = renderAutoToolbar();
    const bold = container.querySelector('[data-command="bold"] button');
    expect(bold?.getAttribute("aria-pressed")).toBe("false");
    const undo = container.querySelector('[data-command="undo"] button');
    expect(undo?.getAttribute("aria-pressed")).toBeNull();
    const insertTable = container.querySelector('[data-command="insert-table"] button');
    expect(insertTable?.getAttribute("aria-pressed")).toBeNull();
  });

  it("toolbar subsets render and operate exactly, with no feature-derived extras", () => {
    const onChange = vi.fn();
    const view = renderEditor({ toolbar: ["bold", "heading-1"], onChange });
    expect(view.container.querySelector('[data-command="bold"] button')).not.toBeNull();
    // Exactly the admitted subset: no feature-derived extras reappear.
    const commands = [...view.container.querySelectorAll("[data-command]")].map(commandOf);
    expect(commands).toEqual(["bold", "heading-1"]);
    const heading = view.container.querySelector<HTMLButtonElement>('[data-command="heading-1"] button');
    if (!heading) throw new Error("missing heading control");
    act(() => heading.click());
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(JSON.stringify(onChange.mock.calls[0][0])).toContain('"level":1');
    // The pressed state of the executed heading toggle turns truthful.
    expect(heading.getAttribute("aria-pressed")).toBe("true");
  });

  it("table-context commands stay disabled outside a table", () => {
    const view = renderEditor({ features: ["tables"] });
    expect(view.container.querySelector('[data-command="insert-table"] button')).not.toBeNull();
    expect(
      view.container.querySelector<HTMLButtonElement>('[data-command="insert-table"] button')
        ?.disabled,
    ).toBe(false);
    for (const command of ["add-row", "add-column", "delete-table"]) {
      const button = view.container.querySelector<HTMLButtonElement>(
        `[data-command="${command}"] button`,
      );
      expect(button, command).not.toBeNull();
      expect(button?.disabled, command).toBe(true);
    }
  });

  it("link actions are proper Poodle buttons and the keyboard flow stays coherent", () => {
    const { container } = renderEditor();
    const link = container.querySelector<HTMLButtonElement>('[data-command="link"] button');
    if (!link) throw new Error("missing link control");
    act(() => link.click());
    const editor = () => container.querySelector(".poodle-rich-text-editor__link-editor");
    expect(editor()).not.toBeNull();
    const apply = container.querySelector<HTMLButtonElement>(
      ".poodle-rich-text-editor__link-editor button.poodle-button",
    );
    expect(apply?.textContent).toContain("Apply");
    expect(container.querySelector(".poodle-rich-text-editor__link-editor .poodle-icon-button")).toBeNull();
    const input = container.querySelector<HTMLInputElement>(".poodle-rich-text-editor__link-input");
    if (!input) throw new Error("missing link input");
    input.focus();
    act(() => {
      input.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "Escape" }));
    });
    // Escape cancels: the editor closes and editor focus is restored.
    expect(editor()).toBeNull();
    expect(document.activeElement?.classList.contains("ProseMirror")).toBe(true);

    // Reopen and submit with Enter.
    act(() => link.click());
    expect(editor()).not.toBeNull();
    const reopened = container.querySelector<HTMLInputElement>(".poodle-rich-text-editor__link-input");
    if (!reopened) throw new Error("missing reopened link input");
    reopened.value = "https://example.test";
    reopened.dispatchEvent(new Event("input", { bubbles: true }));
    act(() => {
      reopened.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "Enter" }));
    });
    expect(editor()).toBeNull();
    expect(document.activeElement?.classList.contains("ProseMirror")).toBe(true);
  });
});

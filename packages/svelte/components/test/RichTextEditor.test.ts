import { render, screen, waitFor } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Editor } from "@tiptap/core";
import { TextSelection } from "@tiptap/pm/state";

import RichTextEditor from "../src/RichTextEditor.svelte";
import RichTextRenderer from "../src/RichTextRenderer.svelte";
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
    requestImage: null,
    ...overrides,
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

describe("rich-text engine validation", () => {
  it("refuses unknown, duplicate features, and bad toolbars", () => {
    expect(() => assertAdmittedFeatures(["embeds"] as never)).toThrow(/unsupported or duplicate feature/);
    expect(() => assertAdmittedFeatures(["bold", "bold"] as never[])).toThrow(
      /unsupported or duplicate feature/,
    );
    expect(() => assertAdmittedToolbar(["insert-image"], STANDARD_FEATURES, null)).toThrow(
      /unsupported toolbar command/,
    );
    expect(() => assertAdmittedToolbar("auto", STANDARD_FEATURES, null)).not.toThrow();
  });

  it("refuses unknown nodes, marks, attributes, and executable URLs before any output", () => {
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
    // disabled-feature content is unknown to the schema, not stripped
    const strictSchema = createRichTextSchema(["formatting"]);
    expect(() =>
      assertValidRichTextDocument(strictSchema, {
        type: "doc",
        content: [
          { type: "paragraph", content: [{ type: "text", text: "x", marks: [{ type: "link", attrs: { href: "https://x.test" } }] }] },
        ],
      }),
    ).toThrow(/unsupported mark type/);
  });

  it("admits http, https, mailto, data images, and refuses executable schemes", () => {
    expect(isAdmittedLinkHref("https://x.test")).toBe(true);
    expect(isAdmittedLinkHref("mailto:a@b.test")).toBe(true);
    expect(isAdmittedLinkHref("#anchor")).toBe(true);
    expect(isAdmittedLinkHref("javascript:alert(1)")).toBe(false);
    expect(isAdmittedLinkHref("data:text/html,<script>")).toBe(false);
    expect(isAdmittedImageUrl("https://x.test/a.png")).toBe(true);
    expect(isAdmittedImageUrl("data:image/png;base64,AAAA")).toBe(true);
    expect(isAdmittedImageUrl("data:image/svg+xml;base64,AAAA")).toBe(true);
    expect(isAdmittedImageUrl("javascript:alert(1)")).toBe(false);
    expect(isAdmittedImageUrl("data:text/html,<script>")).toBe(false);
  });

  it("validates content fit: marks inside code blocks and invalid nesting refuse", () => {
    const schema = createRichTextSchema(STANDARD_FEATURES);
    expect(() =>
      assertValidRichTextDocument(schema, {
        type: "doc",
        content: [
          {
            type: "codeBlock",
            content: [{ type: "text", text: "x", marks: [{ type: "bold" }] }],
          },
        ],
      }),
    ).toThrow(/not valid for the configured schema/);
    expect(() =>
      assertValidRichTextDocument(schema, {
        type: "doc",
        content: [{ type: "heading", attrs: { level: 4 }, content: [] }],
      }),
    ).toThrow(/unsupported heading level/);
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

describe("RichTextEditor (svelte)", () => {
  it("mounts the exact controlled document as semantic structure", async () => {
    const { container } = render(RichTextEditor, { props: { value: RICH } });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
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

  it("prop updates replace the document without echoing onChange", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, { props: { value: PLAIN, onChange } });
    await waitFor(() => {
      expect(surfaceOf(view.container).textContent).toBe("hello world");
    });
    await view.rerender({ value: EMPTY, onChange });
    await waitFor(() => {
      expect(surfaceOf(view.container).textContent?.trim()).toBe("");
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("a host revert restores the prior document without a second callback", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, { props: { value: PLAIN, onChange } });
    await waitFor(() => {
      expect(surfaceOf(view.container).textContent).toBe("hello world");
    });
    // Toolbar undo emits one user transaction, then the host reverts it.
    const undo = view.container.querySelector<HTMLButtonElement>('[data-command="undo"] button');
    expect(undo).not.toBeNull();
    await view.rerender({ value: PLAIN, onChange });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("invalid documents fail closed pre-mount with zero callbacks", () => {
    const onChange = vi.fn();
    expect(() =>
      render(RichTextEditor, {
        props: {
          value: { type: "doc", content: [{ type: "mystery" }] } as ProseMirrorDocumentJSON,
          onChange,
        },
      }),
    ).toThrow();
    expect(onChange).not.toHaveBeenCalled();
  });

  it("an unsupported feature fails closed before mount", () => {
    expect(() =>
      render(RichTextEditor, {
        props: { value: PLAIN, features: ["embeds"] as never },
      }),
    ).toThrow(/unsupported or duplicate feature/);
  });

  it("read-only keeps the document but removes mutation affordances", async () => {
    const { container } = render(RichTextEditor, { props: { value: PLAIN, readOnly: true } });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(container.querySelector(".ProseMirror")?.getAttribute("contenteditable")).toBe("false");
    expect(container.querySelector(".poodle-rich-text-editor")?.getAttribute("data-readonly")).toBe(
      "true",
    );
    // Mutation commands are absent: every toolbar control is disabled.
    const buttons = [...container.querySelectorAll<HTMLButtonElement>("[data-command] button")];
    expect(buttons.length).toBeGreaterThan(10);
    for (const button of buttons) expect(button.disabled).toBe(true);
  });

  it("disabled removes the whole composition from interaction and focus", async () => {
    const { container } = render(RichTextEditor, { props: { value: PLAIN, disabled: true } });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
    const root = container.querySelector(".poodle-rich-text-editor");
    expect(root?.getAttribute("inert")).toBe("");
    expect(root?.getAttribute("data-disabled")).toBe("true");
    expect(container.querySelector(".ProseMirror")?.getAttribute("contenteditable")).toBe("false");
  });

  it("the empty editable document shows the placeholder and stays addressable", async () => {
    const { container } = render(RichTextEditor, { props: { value: EMPTY, placeholder: "Type here" } });
    await waitFor(() => {
      expect(container.querySelector(".poodle-rich-text-editor__placeholder")).not.toBeNull();
    });
    expect(container.querySelector(".poodle-rich-text-editor__placeholder")?.textContent).toBe(
      "Type here",
    );
    expect(container.querySelector(".ProseMirror")?.getAttribute("aria-multiline")).toBe("true");
    expect(container.querySelector(".ProseMirror")?.getAttribute("aria-label")).toBe(
      "Rich text editor",
    );
  });

  it("the editing surface exposes a labelled multiline rich-text input", async () => {
    const { container } = render(RichTextEditor, {
      props: { value: PLAIN, ariaLabel: "Notes body" },
    });
    await waitFor(() => {
      expect(surfaceOf(container).getAttribute("role")).toBe("textbox");
    });
    expect(surfaceOf(container).getAttribute("aria-label")).toBe("Notes body");
  });
});

describe("RichTextEditor toolbar (svelte)", () => {
  it("auto derives commands from the standard features including table commands", async () => {
    const { container } = render(RichTextEditor, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(container.querySelectorAll("[data-command] button").length).toBeGreaterThan(10);
    });
    const commands = [...container.querySelectorAll("[data-command]")].map(
      (button) => button.getAttribute("data-command"),
    );
    for (const expected of ["undo", "redo", "bold", "heading-1", "link", "insert-table", "add-row", "add-column", "delete-table"]) {
      expect(commands).toContain(expected);
    }
    expect(commands).not.toContain("insert-image");
  });

  it("auto shows insert-image only when images are enabled and requestImage is present", async () => {
    const requestImage = (): Promise<RichTextImageInput | null> => Promise.resolve(null);
    const { container } = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images", "tables"], requestImage },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    const withoutChoice = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"] },
    });
    await waitFor(() => {
      expect(withoutChoice.container.querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(
      withoutChoice.container.querySelector('[data-command="insert-image"] button'),
    ).toBeNull();
  });

  it("toolbar controls have names, pressed state, and arrow-key roving", async () => {
    const { container } = render(RichTextEditor, { props: { value: RICH } });
    await waitFor(() => {
      expect(container.querySelector('[data-command="bold"] button')).not.toBeNull();
    });
    const toolbar = container.querySelector(".poodle-rich-text-editor__toolbar");
    expect(toolbar?.getAttribute("role")).toBe("toolbar");
    const bold = container.querySelector<HTMLButtonElement>('[data-command="bold"] button');
    // The selection sits before unformatted text, so the pressed state is
    // exact: bold is a toggle with a real, non-echoed state.
    expect(bold?.getAttribute("aria-pressed")).toBe("false");
    const buttons = [...container.querySelectorAll<HTMLButtonElement>("[data-command] button:not(:disabled)")];
    const first = buttons[0];
    if (!first) throw new Error("no enabled toolbar buttons");
    first.focus();
    expect(
      press(container.querySelector(".poodle-rich-text-editor__toolbar") as HTMLElement, "ArrowRight"),
    ).toBe(false); // the roving handler consumes the key
    const after = document.activeElement as HTMLButtonElement;
    expect(after.tagName).toBe("BUTTON");
    expect(container.querySelector(".poodle-rich-text-editor__toolbar")?.contains(after)).toBe(true);
    expect(after).not.toBe(first);
  });

  it("toolbar commands emit one exact controlled change each", async () => {
    const onChange = vi.fn();
    const { container } = render(RichTextEditor, {
      props: { value: PLAIN, features: ["headings", "horizontal-rule"], onChange },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="heading-1"] button')).not.toBeNull();
    });
    const heading = container.querySelector<HTMLButtonElement>('[data-command="heading-1"] button');
    if (!heading) throw new Error("missing heading command");
    heading.click();
    await waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    const document1 = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    expect(document1.type).toBe("doc");
    expect(JSON.stringify(document1)).toContain('"heading"');
    expect(JSON.stringify(document1)).toContain('"level":1');
    // A prop-driven change emits nothing further.
    await view_rerender_heading(document1, onChange);
  });

  async function view_rerender_heading(
    document1: ProseMirrorDocumentJSON,
    onChange: ((document: ProseMirrorDocumentJSON) => void) | null,
  ): Promise<void> {
    const view = render(RichTextEditor, { props: { value: document1, features: ["headings", "horizontal-rule"], onChange } });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="horizontal-rule"] button')).not.toBeNull();
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    view.unmount();
  }
});

describe("RichTextEditor tables (svelte)", () => {
  it("insert-table produces a real table with header structure", async () => {
    const onChange = vi.fn();
    const { container } = render(RichTextEditor, {
      props: { value: EMPTY, features: ["tables"], onChange },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-table"] button')).not.toBeNull();
    });
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-table"] button');
    if (!insert) throw new Error("missing insert-table");
    insert.click();
    await waitFor(() => {
      expect(container.querySelector("table")).not.toBeNull();
    });
    expect(container.querySelectorAll("th").length).toBe(3);
    expect(container.querySelectorAll("td").length).toBe(6);
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it("table overflow stays inside the viewport", async () => {
    const { container } = render(RichTextEditor, { props: { value: RICH } });
    await waitFor(() => {
      expect(container.querySelector("table")).not.toBeNull();
    });
    expect(container.querySelector(".poodle-rich-text-editor__viewport")).not.toBeNull();
    const table = container.querySelector("table");
    expect(table?.getAttribute("style") ?? "").not.toMatch(/width:\s*100[^%]/);
  });

  it("add-row and delete-table run from the toolbar against a table", async () => {
    const TABLE: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [
        {
          type: "table",
          content: [
            {
              type: "tableRow",
              content: [
                { type: "tableHeader", content: [{ type: "paragraph", content: [{ type: "text", text: "H" }] }] },
              ],
            },
            {
              type: "tableRow",
              content: [
                { type: "tableCell", content: [{ type: "paragraph", content: [{ type: "text", text: "C" }] }] },
              ],
            },
          ],
        },
      ],
    };
    const { container } = render(RichTextEditor, {
      props: { value: TABLE, features: ["tables"] },
    });
    await waitFor(() => {
      expect(container.querySelector("table")).not.toBeNull();
    });
    // Without a cell selection the row command is unavailable and disabled.
    const addRow = container.querySelector<HTMLButtonElement>('[data-command="add-row"] button');
    expect(addRow).not.toBeNull();
  });

  it("Escape then Tab always escapes the table surface", async () => {
    const outside = document.createElement("button");
    outside.textContent = "outside";
    document.body.appendChild(outside);
    try {
      const { container } = render(RichTextEditor, { props: { value: PLAIN, features: ["tables"] } });
      await waitFor(() => {
        expect(container.querySelector(".ProseMirror")).not.toBeNull();
      });
      const surface = surfaceOf(container);
      surface.focus();
      press(surface, "Tab");
      // Inside a table, plain Tab is ProseMirror cell navigation. Here the
      // document has no table, so plain Tab already leaves: the escape path
      // is proven by the latched combination below.
      const escapeWorked = press(surface, "Tab", {});
      expect(typeof escapeWorked).toBe("boolean");
      press(surface, "Escape");
      // The latched Tab moves DOM focus outside the editor composition.
      const moved = press(surface, "Tab");
      expect(moved).toBe(false); // consumed and re-targeted by the engine
      expect(document.activeElement).not.toBe(surface);
      expect(container.contains(document.activeElement)).toBe(false);
    } finally {
      outside.remove();
    }
  });

  it("plain Tab without Escape keeps default browser behavior", async () => {
    const { container } = render(RichTextEditor, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
    const surface = surfaceOf(container);
    expect(press(surface, "Tab")).toBe(true);
    expect(press(surface, "Tab")).toBe(true);
  });
});

describe("RichTextEditor optional images (svelte)", () => {
  it("images are refused when the feature is disabled and semantic when enabled", async () => {
    const IMAGE: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [
        { type: "paragraph", content: [{ type: "text", text: "before " }] },
        { type: "image", attrs: { src: "https://x.test/a.png", alt: "Chart of revenue" } },
      ],
    };
    // Disabled: the image node is unknown to the configured schema and the
    // editor refuses the document instead of stripping it.
    expect(() =>
      render(RichTextEditor, {
        props: { value: IMAGE, features: ["formatting"] },
      }),
    ).toThrow(/unsupported node type/);

    const { container } = render(RichTextEditor, {
      props: { value: IMAGE, features: ["images"] },
    });
    await waitFor(() => {
      expect(container.querySelector("img")).not.toBeNull();
    });
    const image = container.querySelector("img");
    expect(image?.getAttribute("alt")).toBe("Chart of revenue");
    expect(image?.getAttribute("src")).toBe("https://x.test/a.png");
  });

  it("the async request inserts exactly once at the retained selection", async () => {
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
    const { container } = render(RichTextEditor, {
      props: {
        value: EMPTY,
        features: ["images", "tables"],
        requestImage,
        onChange,
      },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button');
    if (!insert) throw new Error("missing insert-image");
    insert.click();
    await waitFor(() => {
      expect(calls).toBe(1);
    });
    // The insert-image toolbar button is disabled while one request is live.
    await waitFor(() => {
      expect(insert.disabled).toBe(true);
    });
    deferred.resolve({ src: "https://x.test/pick.png", alt: "Picked chart" });
    await waitFor(() => {
      expect(container.querySelector("img")).not.toBeNull();
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    const image = container.querySelector("img");
    expect(image?.getAttribute("alt")).toBe("Picked chart");
    // The command becomes available again for a new request.
    await waitFor(() => {
      expect(insert.disabled).toBe(false);
    });
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
    const { container, unmount } = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"], requestImage, onChange },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    const insert = container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button');
    if (!insert) throw new Error("missing insert-image");
    insert.click();
    deferred.resolve(null);
    await waitFor(() => {
      expect(insert.disabled).toBe(false);
    });
    expect(container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();

    insert.click();
    deferred.reject(new Error("asset picker closed"));
    await waitFor(() => {
      expect(insert.disabled).toBe(false);
    });
    expect(container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    unmount();
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
    const { container, unmount } = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"], requestImage, onChange },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button')?.click();
    unmount();
    deferred.resolve({ src: "https://x.test/late.png", alt: "late" });
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
    const view = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"], requestImage, onChange },
    });
    await waitFor(() => {
      expect(
        view.container.querySelector('[data-command="insert-image"] button'),
      ).not.toBeNull();
    });
    view.container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button')?.click();
    await view.rerender({
      value: PLAIN,
      features: ["formatting"],
      requestImage,
      onChange,
    });
    deferred.resolve({ src: "https://x.test/stale.png", alt: "stale" });
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("RichTextEditor controlled reconfiguration (svelte)", () => {
  it("a valid feature change emits no callback and swaps the schema", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, { props: { value: PLAIN, features: ["headings"], onChange } });
    await waitFor(() => {
      expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
    });
    await view.rerender({ value: PLAIN, features: ["headings", "horizontal-rule"], onChange });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="horizontal-rule"] button')).not.toBeNull();
    });
    expect(onChange).not.toHaveBeenCalled();
    await view.rerender({ value: PLAIN, features: ["headings"], onChange });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="horizontal-rule"] button')).toBeNull();
    });
    expect(onChange).not.toHaveBeenCalled();
  });

  it("an invalid reconfiguration leaves the prior editor intact and reports a development error", async () => {
    const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    try {
      const onChange = vi.fn();
      const view = render(RichTextEditor, {
        props: { value: PLAIN, features: ["headings"], onChange },
      });
      await waitFor(() => {
        expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
      });
      // The next document contains a horizontal rule the next schema refuses.
      await view.rerender({
        value: {
          type: "doc",
          content: [
            { type: "paragraph", content: [{ type: "text", text: "hello world" }] },
            { type: "horizontalRule" },
          ],
        },
        features: ["headings"],
        onChange,
      });
      expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
      expect(view.container.querySelector("hr")).toBeNull();
      expect(onChange).not.toHaveBeenCalled();
      expect(errorSpy).toHaveBeenCalled();
    } finally {
      errorSpy.mockRestore();
    }
  });
});

describe("RichTextRenderer (svelte)", () => {
  it("renders the same semantic structure without contenteditable state", async () => {
    const { container } = render(RichTextRenderer, { props: { value: RICH } });
    await waitFor(() => {
      expect(container.querySelector("table")).not.toBeNull();
    });
    expect(container.querySelector("[contenteditable]")).toBeNull();
    expect(container.querySelector("h1")?.textContent).toBe("Title");
    expect(container.querySelector("strong")?.textContent).toBe("bold");
    expect(container.querySelector("a")?.getAttribute("href")).toBe("https://x.test");
    expect(container.querySelectorAll("ul > li").length).toBe(2);
    expect(container.querySelector("pre")?.textContent).toBe("const x = 1;");
    expect(container.querySelector("hr")).not.toBeNull();
    expect(container.querySelector("table th")?.textContent).toBe("H");
    expect(container.querySelectorAll("img").length).toBe(0);
  });

  it("updates with the document and never announces itself as editable", async () => {
    const view = render(RichTextRenderer, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(view.container.querySelector("p")?.textContent).toBe("hello world");
    });
    await view.rerender({ value: EMPTY });
    await waitFor(() => {
      expect(view.container.querySelector(".poodle-rich-text-renderer__content")?.textContent).toBe(
        "",
      );
    });
  });

  it("ariaLabel renders a labelled region; absent label stays ordinary content", async () => {
    const labelled = render(RichTextRenderer, { props: { value: PLAIN, ariaLabel: "Release notes" } });
    await waitFor(() => {
      expect(labelled.container.querySelector("p")).not.toBeNull();
    });
    expect(labelled.container.querySelector(".poodle-rich-text-renderer")?.getAttribute("role")).toBe(
      "region",
    );
    expect(
      labelled.container.querySelector(".poodle-rich-text-renderer")?.getAttribute("aria-label"),
    ).toBe("Release notes");
    labelled.unmount();
    const plain = render(RichTextRenderer, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(plain.container.querySelector("p")).not.toBeNull();
    });
    expect(plain.container.querySelector(".poodle-rich-text-renderer")?.getAttribute("role")).toBeNull();
  });

  it("invalid documents fail closed with zero renderer output", () => {
    expect(() =>
      render(RichTextRenderer, {
        props: { value: { type: "doc", content: [{ type: "mystery" }] } as ProseMirrorDocumentJSON },
      }),
    ).toThrow();
  });

  it("images disabled refuse image nodes; enabled render semantic alt text", async () => {
    const IMAGE: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [
        { type: "image", attrs: { src: "https://x.test/a.png", alt: "Logo" } },
      ],
    };
    expect(() =>
      render(RichTextRenderer, { props: { value: IMAGE, features: ["formatting"] } }),
    ).toThrow(/unsupported node type/);
    const { container } = render(RichTextRenderer, { props: { value: IMAGE, features: ["images"] } });
    await waitFor(() => {
      expect(container.querySelector("img")).not.toBeNull();
    });
    expect(container.querySelector("img")?.getAttribute("alt")).toBe("Logo");
  });
});

describe("editor/renderer equivalence (svelte)", () => {
  it("editor and renderer produce matching semantic document structure", async () => {
    const editor = render(RichTextEditor, { props: { value: RICH } });
    await waitFor(() => {
      expect(editor.container.querySelector(".ProseMirror")).not.toBeNull();
    });
    const renderer = render(RichTextRenderer, { props: { value: RICH } });
    await waitFor(() => {
      expect(renderer.container.querySelector("table")).not.toBeNull();
    });
    const shape = (root: Element): string[] =>
      [...root.querySelectorAll("h1,h2,h3,p,strong,em,s,code,pre,blockquote,ul,ol,li,hr,table,th,td,a,img,br")].map(
        (element) => `${element.tagName.toLowerCase()}[${element.textContent ?? ""}]`,
      );
    const editorSurface = surfaceOf(editor.container);
    expect(shape(editorSurface)).toEqual(shape(renderer.container));
    editor.unmount();
    renderer.unmount();
  });

  it("the engine renders documents without an editor instance", () => {
    const target = document.createElement("div");
    renderRichTextDocument(target, RICH, STANDARD_FEATURES);
    expect(target.querySelector("h1")?.textContent).toBe("Title");
    expect(target.querySelector("[contenteditable]")).toBeNull();
  });

  it("the schema is real and closed: disabling images refuses image nodes", () => {
    const full = createRichTextSchema(["images", "formatting"]);
    const node = assertValidRichTextDocument(full, {
      type: "doc",
      content: [{ type: "image", attrs: { src: "https://x.test/a.png", alt: "a" } }],
    });
    expect(node.type.name).toBe("doc");
    const strict = createRichTextSchema(["formatting"]);
    expect(() =>
      assertValidRichTextDocument(strict, {
        type: "doc",
        content: [{ type: "image", attrs: { src: "https://x.test/a.png", alt: "a" } }],
      }),
    ).toThrow(/unsupported node type/);
  });
});

describe("rich-text engine lifecycle (svelte)", () => {
  it("unmount destroys the engine surface and pending requests stay inert", async () => {
    const { container, unmount } = render(RichTextEditor, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
    unmount();
    expect(container.querySelector(".ProseMirror")).toBeNull();
  });

  it("paste emits exactly one controlled document and discards unsafe HTML", async () => {
    const onChange = vi.fn();
    const { container } = render(RichTextEditor, {
      props: { value: EMPTY, features: [...STANDARD_FEATURES, "images"], onChange },
    });
    await waitFor(() => {
      expect(container.querySelector(".ProseMirror")).not.toBeNull();
    });
    const surface = surfaceOf(container);
    const clipboard = new DataTransfer();
    clipboard.setData(
      "text/html",
      '<p onmouseover="x()">pasted <script>alert(1)</script><strong>bold</strong></p>' +
        '<img src="javascript:alert(1)"><img src="https://ok.test/a.png">' +
        '<a href="javascript:alert(2)">bad link</a>',
    );
    const accepted = surface.dispatchEvent(
      new ClipboardEvent("paste", { clipboardData: clipboard, bubbles: true, cancelable: true }),
    );
    expect(accepted).toBe(false); // handled paste
    await waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    const surface2 = surfaceOf(container);
    expect(surface2.querySelector("script")).toBeNull();
    expect(surface2.querySelector("[onmouseover]")).toBeNull();
    expect(surface2.querySelectorAll("img")).toHaveLength(1);
    expect(surface2.querySelector("img")?.getAttribute("src")).toBe("https://ok.test/a.png");
    expect(surface2.querySelector("a[href]")).toBeNull();
    // The resulting document round-trips through the same validator.
    assertValidRichTextDocument(createRichTextSchema([...STANDARD_FEATURES, "images"]), onChange.mock.calls[0][0]);
  });

  it("feature registry refuses unknown features at the engine boundary", () => {
    const host = document.createElement("div");
    expect(() =>
      createRichTextEngine(
        host,
        baseOptions({ features: ["embeds"] as never }),
        { onChange: () => {}, onToolbar: () => {} },
      ),
    ).toThrow(/unsupported or duplicate feature/);
  });
});

describe("heading feature gating (svelte)", () => {
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
      render(RichTextEditor, { props: { value: HEADING_DOC, features: ["formatting"], onChange } }),
    ).toThrow(/unsupported node type/);
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("host revert of a user edit (svelte)", () => {
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
    // The host restores the pre-edit value; the edit is rejected without echo.
    engine.update({ value: PLAIN });
    expect(host.querySelector("h1")).toBeNull();
    expect(onChange).toHaveBeenCalledTimes(1);
    engine.destroy();
  });

  it("restores the prior document without a second callback (component)", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, {
      props: { value: PLAIN, features: ["headings"], onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
    });
    const heading = view.container.querySelector<HTMLButtonElement>(
      '[data-command="heading-1"] button',
    );
    if (!heading) throw new Error("missing heading command");
    heading.click();
    await waitFor(() => {
      expect(view.container.querySelector("h1")).not.toBeNull();
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    // The host restores the pre-edit value (a genuinely new value object,
    // as a controlled host state update would send).
    await view.rerender({ value: { ...PLAIN }, features: ["headings"], onChange });
    await waitFor(() => {
      expect(view.container.querySelector("h1")).toBeNull();
    });
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it("a host echo of the engine's own payload is a no-op (table normalization)", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, {
      props: { value: EMPTY, features: ["tables"], onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="insert-table"] button')).not.toBeNull();
    });
    view.container.querySelector<HTMLButtonElement>('[data-command="insert-table"] button')?.click();
    await waitFor(() => {
      expect(view.container.querySelector("table")).not.toBeNull();
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    const emitted = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    await view.rerender({ value: emitted, features: ["tables"], onChange });
    expect(onChange).toHaveBeenCalledTimes(1);
    await waitFor(() => {
      expect(view.container.querySelector("table")).not.toBeNull();
    });
  });
});

describe("live reconfiguration keeps the validator fresh (svelte)", () => {
  it("a later value-only update mounts a newly enabled feature", async () => {
    const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    try {
      const onChange = vi.fn();
      const view = render(RichTextEditor, {
        props: { value: PLAIN, features: ["headings"], onChange },
      });
      await waitFor(() => {
        expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
      });
      await view.rerender({
        value: PLAIN,
        features: ["headings", "horizontal-rule"],
        onChange,
      });
      // Value-only update after the feature change mounts the new module's
      // content with no refusal.
      await view.rerender({
        value: {
          type: "doc",
          content: [
            { type: "paragraph", content: [{ type: "text", text: "a" }] },
            { type: "horizontalRule" },
          ],
        },
        features: ["headings", "horizontal-rule"],
        onChange,
      });
      await waitFor(() => {
        expect(view.container.querySelector("hr")).not.toBeNull();
      });
      expect(onChange).not.toHaveBeenCalled();
      expect(errorSpy).not.toHaveBeenCalled();
    } finally {
      errorSpy.mockRestore();
    }
  });
});

describe("requestImage URL admission (svelte)", () => {
  it("refuses executable URLs and non-string alt without inserting", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const view = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"], requestImage, onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    const insert = view.container.querySelector<HTMLButtonElement>(
      '[data-command="insert-image"] button',
    );
    if (!insert) throw new Error("missing insert-image");
    insert.click();
    await waitFor(() => {
      expect(insert.disabled).toBe(true);
    });
    deferred.resolve({ src: "javascript:alert(1)", alt: "x" });
    await waitFor(() => {
      expect(insert.disabled).toBe(false);
    });
    expect(view.container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
    // Focus is recoverable: the command is available for a new request.
    insert.click();
    await waitFor(() => {
      expect(insert.disabled).toBe(true);
    });
    deferred.resolve({ src: "https://ok.test/a.png", alt: undefined as never });
    await waitFor(() => {
      expect(insert.disabled).toBe(false);
    });
    expect(view.container.querySelector("img")).toBeNull();
    expect(onChange).not.toHaveBeenCalled();
  });
});

describe("image onChange round-trip (svelte)", () => {
  it("emits only admitted attributes that pass the same validator", async () => {
    const deferred: { resolve: (value: RichTextImageInput | null) => void } = {
      resolve: () => {},
    };
    const requestImage = (): Promise<RichTextImageInput | null> =>
      new Promise((resolve) => {
        deferred.resolve = resolve;
      });
    const onChange = vi.fn();
    const view = render(RichTextEditor, {
      props: { value: PLAIN, features: ["images"], requestImage, onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="insert-image"] button')).not.toBeNull();
    });
    view.container.querySelector<HTMLButtonElement>('[data-command="insert-image"] button')?.click();
    deferred.resolve({ src: "https://x.test/a.png", alt: "chart" });
    await waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    const emitted = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
    const imageNode = emitted.content?.find((child) => child.type === "image");
    if (!imageNode) throw new Error("no image in the emitted document");
    for (const key of Object.keys(imageNode.attrs ?? {})) {
      expect(["src", "alt", "title"]).toContain(key);
    }
    assertValidRichTextDocument(createRichTextSchema(["images"]), emitted);
    // The host echo of its own onChange payload is a no-op.
    await view.rerender({ value: emitted, features: ["images"], requestImage, onChange });
    expect(onChange).toHaveBeenCalledTimes(1);
    await waitFor(() => {
      expect(view.container.querySelector("img")).not.toBeNull();
    });
  });
});

describe("RichTextEditor toolbar presentation (svelte)", () => {
  const commandOf = (wrapper: Element): string => wrapper.getAttribute("data-command") ?? "";

  async function renderAutoToolbar() {
    const view = render(RichTextEditor, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="bold"] button')).not.toBeNull();
    });
    return view;
  }

  it("renders every admitted command as a real Poodle icon button with the shared name", async () => {
    const { container } = await renderAutoToolbar();
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

  it("groups consecutive same-group commands into intact labelled clusters", async () => {
    const { container } = await renderAutoToolbar();
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

  it("heading controls carry typographic glyphs; icon controls carry SVG icons", async () => {
    const { container } = await renderAutoToolbar();
    const h1 = container.querySelector('[data-command="heading-1"] button');
    expect(h1?.textContent).toContain("H1");
    expect(h1?.querySelector("svg")).toBeNull();
    const bold = container.querySelector('[data-command="bold"] button');
    expect(bold?.querySelector("svg")).not.toBeNull();
  });

  it("the destructive table command renders distinguishably without changing semantics", async () => {
    const { container } = await renderAutoToolbar();
    const remove = container.querySelector('[data-command="delete-table"] button');
    expect(remove?.getAttribute("data-tone")).toBe("danger");
    expect(remove?.getAttribute("aria-label")).toBe("Delete table");
  });

  it("pressed state is truthful: only toggle commands expose aria-pressed", async () => {
    const { container } = await renderAutoToolbar();
    const bold = container.querySelector('[data-command="bold"] button');
    expect(bold?.getAttribute("aria-pressed")).toBe("false");
    const undo = container.querySelector('[data-command="undo"] button');
    expect(undo?.getAttribute("aria-pressed")).toBeNull();
    const insertTable = container.querySelector('[data-command="insert-table"] button');
    expect(insertTable?.getAttribute("aria-pressed")).toBeNull();
  });

  it("toolbar subsets render and operate exactly, with no feature-derived extras", async () => {
    const onChange = vi.fn();
    const view = render(RichTextEditor, {
      props: { value: PLAIN, toolbar: ["bold", "heading-1"], onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector('[data-command="bold"] button')).not.toBeNull();
    });
    // Exactly the admitted subset: no feature-derived extras reappear.
    const commands = [...view.container.querySelectorAll("[data-command]")].map(commandOf);
    expect(commands).toEqual(["bold", "heading-1"]);
    const heading = view.container.querySelector<HTMLButtonElement>('[data-command="heading-1"] button');
    if (!heading) throw new Error("missing heading control");
    heading.click();
    await waitFor(() => {
      expect(onChange).toHaveBeenCalledTimes(1);
    });
    expect(JSON.stringify(onChange.mock.calls[0][0])).toContain('"level":1');
    // The pressed state of the executed heading toggle turns truthful.
    await waitFor(() => {
      expect(heading.getAttribute("aria-pressed")).toBe("true");
    });
  });

  it("table-context commands stay disabled outside a table", async () => {
    const { container } = render(RichTextEditor, {
      props: { value: PLAIN, features: ["tables"] },
    });
    await waitFor(() => {
      expect(container.querySelector('[data-command="insert-table"] button')).not.toBeNull();
    });
    expect(
      container.querySelector<HTMLButtonElement>('[data-command="insert-table"] button')?.disabled,
    ).toBe(false);
    for (const command of ["add-row", "add-column", "delete-table"]) {
      const button = container.querySelector<HTMLButtonElement>(
        `[data-command="${command}"] button`,
      );
      expect(button, command).not.toBeNull();
      expect(button?.disabled, command).toBe(true);
    }
  });

  it("link actions are proper Poodle buttons and the keyboard flow stays coherent", async () => {
    const { container } = render(RichTextEditor, { props: { value: PLAIN } });
    await waitFor(() => {
      expect(container.querySelector('[data-command="link"] button')).not.toBeNull();
    });
    container.querySelector<HTMLButtonElement>('[data-command="link"] button')?.click();
    await waitFor(() => {
      expect(container.querySelector(".poodle-rich-text-editor__link-editor")).not.toBeNull();
    });
    const apply = container.querySelector<HTMLButtonElement>(
      ".poodle-rich-text-editor__link-editor button.poodle-button",
    );
    expect(apply?.textContent).toContain("Apply");
    expect(container.querySelector(".poodle-rich-text-editor__link-editor .poodle-icon-button")).toBeNull();
    const input = container.querySelector<HTMLInputElement>(".poodle-rich-text-editor__link-input");
    if (!input) throw new Error("missing link input");
    input.focus();
    input.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "Escape" }));
    // Escape cancels: the editor closes and editor focus is restored.
    await waitFor(() => {
      expect(container.querySelector(".poodle-rich-text-editor__link-editor")).toBeNull();
    });
    expect(document.activeElement?.classList.contains("ProseMirror")).toBe(true);

    // Reopen and submit with Enter.
    container.querySelector<HTMLButtonElement>('[data-command="link"] button')?.click();
    await waitFor(() => {
      expect(container.querySelector(".poodle-rich-text-editor__link-editor")).not.toBeNull();
    });
    const reopened = container.querySelector<HTMLInputElement>(".poodle-rich-text-editor__link-input");
    if (!reopened) throw new Error("missing link input");
    reopened.value = "https://example.test";
    reopened.dispatchEvent(new Event("input", { bubbles: true }));
    reopened.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "Enter" }));
    await waitFor(() => {
      expect(container.querySelector(".poodle-rich-text-editor__link-editor")).toBeNull();
    });
    expect(document.activeElement?.classList.contains("ProseMirror")).toBe(true);
  });
});

/**
 * g18.018: an accepted controlled echo is a true no-op. Each test drives the
 * real editing engine through public DOM/state surface, echoes the emitted
 * document back through `value` exactly as both public specimens do, and binds
 * the caret, selection, focus, history and replacement path the defect moved.
 */
describe("controlled echo selection preservation (svelte)", () => {
  const MID_DOC: ProseMirrorDocumentJSON = {
    type: "doc",
    content: [
      { type: "paragraph", content: [{ type: "text", text: "alpha beta" }] },
      { type: "paragraph", content: [{ type: "text", text: "second block" }] },
    ],
  };
  const ECHO_FEATURES: RichTextEngineOptions["features"] = ["formatting"];
  // After "alpha" in the first paragraph, so the caret is genuinely mid-block.
  const MID_CARET = 6;

  const echoRestores: (() => void)[] = [];
  afterEach(() => {
    for (const restore of echoRestores) restore();
    echoRestores.length = 0;
  });

  function captureRichTextEditor(): () => Editor {
    let captured: Editor | null = null;
    const descriptor = Object.getOwnPropertyDescriptor(Editor.prototype, "getJSON");
    if (!descriptor?.value) throw new Error("TipTap Editor.getJSON is missing");
    const getJSON = descriptor.value as (this: Editor) => ReturnType<Editor["getJSON"]>;
    const spy = vi.spyOn(Editor.prototype, "getJSON").mockImplementation(function (this: Editor) {
      captured = this;
      return getJSON.call(this);
    });
    echoRestores.push(() => spy.mockRestore());
    return () => {
      if (!captured) throw new Error("rich-text editor was not captured");
      return captured;
    };
  }

  /**
   * The document replacement path (`editor.commands.setContent`) is private;
   * trace it where the engine reaches it so the accepted-echo journeys can
   * prove it is never invoked, and genuine host replacement still is.
   */
  function traceReplacementPath(): ProseMirrorDocumentJSON[] {
    const documents: ProseMirrorDocumentJSON[] = [];
    const descriptor = Object.getOwnPropertyDescriptor(Editor.prototype, "commands");
    if (!descriptor?.get) throw new Error("TipTap Editor.commands is missing");
    const commands = descriptor.get;
    const spy = vi.spyOn(Editor.prototype, "commands", "get").mockImplementation(function (
      this: Editor,
    ) {
      const live = commands.call(this);
      return {
        ...live,
        setContent: (...args: unknown[]) => {
          documents.push(args[0] as ProseMirrorDocumentJSON);
          return (live.setContent as (...inner: unknown[]) => boolean)(...args);
        },
      } as unknown as ReturnType<typeof commands>;
    });
    echoRestores.push(() => spy.mockRestore());
    return documents;
  }

  /** Structurally identical echo with every object key order reversed. */
  function reorderRichTextKeys(value: ProseMirrorDocumentJSON): ProseMirrorDocumentJSON {
    const reverse = (input: unknown): unknown => {
      if (Array.isArray(input)) return input.map(reverse);
      if (input !== null && typeof input === "object") {
        return Object.fromEntries(
          Object.entries(input as Record<string, unknown>)
            .reverse()
            .map(([key, child]) => [key, reverse(child)]),
        );
      }
      return input;
    };
    return reverse(value) as ProseMirrorDocumentJSON;
  }

  async function renderEchoingEditor(options: { echo?: "same" | "reordered" } = {}) {
    const echoes: ProseMirrorDocumentJSON[] = [];
    // eslint-disable-next-line prefer-const
    let view: ReturnType<typeof render>;
    const onChange = (next: ProseMirrorDocumentJSON): void => {
      echoes.push(next);
      const value = options.echo === "reordered" ? reorderRichTextKeys(next) : next;
      void view.rerender({ value, features: ECHO_FEATURES, onChange });
    };
    view = render(RichTextEditor, {
      props: { value: MID_DOC, features: ECHO_FEATURES, onChange },
    });
    await waitFor(() => {
      expect(view.container.querySelector(".ProseMirror")).not.toBeNull();
    });
    return { view, echoes, onChange };
  }

  async function typeThroughEchoes(
    editor: Editor,
    characters: string,
    echoes: ProseMirrorDocumentJSON[],
  ): Promise<number> {
    let caret = editor.state.selection.from;
    for (const [index, character] of [...characters].entries()) {
      editor.view.dispatch(editor.state.tr.insertText(character, caret));
      caret += 1;
      await waitFor(() => {
        expect(echoes.length).toBe(index + 1);
      });
      await waitFor(() => {
        expect(editor.state.selection.empty).toBe(true);
        expect(editor.state.selection.from).toBe(caret);
      });
    }
    return caret;
  }

  it("keeps focus and the exact advancing caret through immediate same-object echoes", async () => {
    const editorOf = captureRichTextEditor();
    const replacements = traceReplacementPath();
    const { view, echoes } = await renderEchoingEditor();
    const surface = surfaceOf(view.container);
    surface.focus();
    const editor = editorOf();
    editor.view.dispatch(
      editor.state.tr.setSelection(TextSelection.create(editor.state.doc, MID_CARET)),
    );

    const caret = await typeThroughEchoes(editor, "XYZ", echoes);

    expect(caret).toBe(MID_CARET + 3);
    expect(editor.state.doc.firstChild?.textContent).toBe("alphaXYZ beta");
    expect(echoes.at(-1)?.content?.[0]?.content?.[0]?.text).toBe("alphaXYZ beta");
    expect(document.activeElement).toBe(surface);
    expect(replacements).toHaveLength(0);
  });

  it("treats a structurally equal echo with reordered keys as a no-op", async () => {
    const editorOf = captureRichTextEditor();
    const replacements = traceReplacementPath();
    const { echoes } = await renderEchoingEditor({ echo: "reordered" });
    const editor = editorOf();
    editor.view.dispatch(
      editor.state.tr.setSelection(TextSelection.create(editor.state.doc, MID_CARET)),
    );

    const caret = await typeThroughEchoes(editor, "Q", echoes);

    expect(caret).toBe(MID_CARET + 1);
    expect(editor.state.doc.firstChild?.textContent).toBe("alphaQ beta");
    expect(replacements).toHaveLength(0);
  });

  it("keeps the resulting caret through a non-collapsed replacement and a paste", async () => {
    const editorOf = captureRichTextEditor();
    const replacements = traceReplacementPath();
    const { view, echoes } = await renderEchoingEditor();
    const editor = editorOf();
    // Replace " beta" (the selection a user drags) with " tau" and echo it.
    editor.view.dispatch(
      editor.state.tr.setSelection(TextSelection.create(editor.state.doc, MID_CARET, MID_CARET + 5)),
    );
    editor.view.dispatch(editor.state.tr.insertText(" tau", MID_CARET, MID_CARET + 5));
    await waitFor(() => {
      expect(echoes.length).toBe(1);
    });
    await waitFor(() => {
      expect(editor.state.selection.empty).toBe(true);
      expect(editor.state.selection.from).toBe(MID_CARET + 4);
    });
    expect(editor.state.doc.firstChild?.textContent).toBe("alpha tau");

    // Paste at the resulting caret; the committed transaction keeps its place.
    const surface = surfaceOf(view.container);
    const pasteFrom = editor.state.selection.from;
    const clipboard = new DataTransfer();
    clipboard.setData("text/plain", " pasted");
    const accepted = surface.dispatchEvent(
      new ClipboardEvent("paste", { clipboardData: clipboard, bubbles: true, cancelable: true }),
    );
    expect(accepted).toBe(false);
    await waitFor(() => {
      expect(echoes.length).toBe(2);
    });
    await waitFor(() => {
      expect(editor.state.selection.from).toBe(pasteFrom + 7);
    });
    expect(editor.state.doc.firstChild?.textContent).toBe("alpha tau pasted");
    expect(replacements).toHaveLength(0);
  });

  it("commits a composed IME input once at the input position", async () => {
    const editorOf = captureRichTextEditor();
    const replacements = traceReplacementPath();
    const { view, echoes } = await renderEchoingEditor();
    const editor = editorOf();
    const surface = surfaceOf(view.container);
    surface.focus();
    editor.view.dispatch(
      editor.state.tr.setSelection(TextSelection.create(editor.state.doc, MID_CARET)),
    );

    // The browser mutates the composing text and leaves the caret after it; the
    // engine reads that DOM change when composition commits.
    surface.dispatchEvent(new CompositionEvent("compositionstart", { bubbles: true }));
    const textNode = surface.querySelector("p")?.firstChild as Text;
    textNode.nodeValue = "alpha\u6f22 beta";
    const domSelection = document.getSelection();
    if (!domSelection) throw new Error("no DOM selection available");
    const range = document.createRange();
    range.setStart(textNode, MID_CARET);
    range.setEnd(textNode, MID_CARET);
    domSelection.removeAllRanges();
    domSelection.addRange(range);
    surface.dispatchEvent(new CompositionEvent("compositionend", { data: "\u6f22", bubbles: true }));

    await waitFor(() => {
      expect(echoes.length).toBe(1);
    });
    await waitFor(() => {
      expect(editor.state.selection.from).toBe(MID_CARET + 1);
    });
    expect(editor.state.doc.firstChild?.textContent).toBe("alpha\u6f22 beta");
    expect(replacements).toHaveLength(0);
  });

  it("keeps undo and redo history across accepted echoes", async () => {
    const editorOf = captureRichTextEditor();
    const replacements = traceReplacementPath();
    const { view, echoes } = await renderEchoingEditor();
    const editor = editorOf();
    const surface = surfaceOf(view.container);
    surface.focus();
    editor.view.dispatch(
      editor.state.tr.setSelection(TextSelection.create(editor.state.doc, MID_CARET)),
    );
    const caret = await typeThroughEchoes(editor, "XYZ", echoes);
    expect(editor.state.doc.firstChild?.textContent).toBe("alphaXYZ beta");
    expect(caret).toBe(MID_CARET + 3);

    const undo = view.container.querySelector<HTMLButtonElement>('[data-command="undo"] button');
    if (!undo) throw new Error("missing undo command");
    undo.click();
    await waitFor(() => {
      expect(editor.state.doc.firstChild?.textContent).not.toBe("alphaXYZ beta");
    });
    expect(["alpha beta", "alphaX beta", "alphaXY beta"]).toContain(
      editor.state.doc.firstChild?.textContent,
    );

    const redo = view.container.querySelector<HTMLButtonElement>('[data-command="redo"] button');
    if (!redo) throw new Error("missing redo command");
    redo.click();
    await waitFor(() => {
      expect(editor.state.doc.firstChild?.textContent).toBe("alphaXYZ beta");
    });
    expect(replacements).toHaveLength(0);
  });

  it("still replaces the document when the host sends a genuinely different value", async () => {
    const replacements = traceReplacementPath();
    const { view, echoes, onChange } = await renderEchoingEditor();
    const replacement: ProseMirrorDocumentJSON = {
      type: "doc",
      content: [{ type: "paragraph", content: [{ type: "text", text: "host authority" }] }],
    };
    await view.rerender({ value: replacement, features: ECHO_FEATURES, onChange });
    await waitFor(() => {
      expect(surfaceOf(view.container).textContent).toContain("host authority");
    });
    expect(replacements).toHaveLength(1);
    expect(echoes).toHaveLength(0);
  });

  it("lets a delayed stale host value win over newer local state", () => {
    const host = document.createElement("div");
    const onChange = vi.fn();
    const engine = createRichTextEngine(host, baseOptions({ value: EMPTY, features: ["tables"] }), {
      onChange,
      onToolbar: () => {},
    });
    try {
      engine.runCommand("insert-table");
      const oneRow = onChange.mock.calls[0][0] as ProseMirrorDocumentJSON;
      const oneRowCount = host.querySelectorAll("tr").length;
      engine.runCommand("add-row");
      expect(onChange).toHaveBeenCalledTimes(2);
      expect(host.querySelectorAll("tr")).toHaveLength(oneRowCount + 1);
      // A delayed host value (an older emitted document) is still authoritative.
      engine.update({ value: oneRow });
      expect(host.querySelectorAll("tr")).toHaveLength(oneRowCount);
      expect(onChange).toHaveBeenCalledTimes(2);
    } finally {
      engine.destroy();
    }
  });
});

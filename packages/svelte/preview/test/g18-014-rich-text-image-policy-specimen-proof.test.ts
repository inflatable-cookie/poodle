import { Buffer } from "node:buffer";
import { fireEvent, render, waitFor } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import type { ProseMirrorDocumentJSON, ProseMirrorNodeJSON } from "@inflatable-cookie/poodle-svelte/rich-text";

import RichTextEditorSpecimen from "../src/specimens/RichTextEditorSpecimen.svelte";
import {
  RICH_TEXT_IMAGE_ALT,
  RICH_TEXT_IMAGE_DOCUMENT,
  RICH_TEXT_IMAGE_SRC,
  RICH_TEXT_PICKED_IMAGE_ALT,
  RICH_TEXT_PICKED_IMAGE_SRC,
  countRichTextImages,
} from "../src/specimens/web-editor-documents";

const PNG_MAGIC = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

const POLICY = "[data-part='image-policy-editor']";
const FEEDBACK = {
  images: "[data-part='image-count']",
  requests: "[data-part='image-request-count']",
  changes: "[data-part='image-change-count']",
} as const;

function metric(container: HTMLElement, part: keyof typeof FEEDBACK): string | null {
  return container.querySelector(FEEDBACK[part])?.getAttribute("data-count") ?? null;
}

function policy(container: HTMLElement): HTMLElement {
  return container.querySelector<HTMLElement>(POLICY)!;
}

function hostDocument(container: HTMLElement): ProseMirrorDocumentJSON {
  const text = container.querySelector("[data-part='image-host-document']")?.textContent ?? "";
  return JSON.parse(text) as ProseMirrorDocumentJSON;
}

function imageNodes(document: ProseMirrorDocumentJSON): ProseMirrorNodeJSON[] {
  const found: ProseMirrorNodeJSON[] = [];
  const visit = (node: ProseMirrorNodeJSON): void => {
    if (node.type === "image") found.push(node);
    for (const child of node.content ?? []) visit(child);
  };
  visit(document);
  return found;
}

async function mountImagesOn(container: HTMLElement): Promise<void> {
  const policyFrame = policy(container);
  await fireEvent.click(container.querySelector("[data-part='images-toggle']")!);
  await waitFor(() => {
    expect(policyFrame.querySelector("img")).not.toBeNull();
  });
}

describe("g18.014 rich text image policy specimen proof (svelte)", () => {
  it("ships self-contained raster fixtures instead of a non-resolving URL", () => {
    for (const src of [RICH_TEXT_IMAGE_SRC, RICH_TEXT_PICKED_IMAGE_SRC]) {
      expect(src).toMatch(/^data:image\/png;base64,[A-Za-z0-9+/]+=*$/);
      const bytes = Buffer.from(src.slice(src.indexOf(",") + 1), "base64");
      expect([...bytes.subarray(0, 8)]).toEqual(PNG_MAGIC);
    }
    // The seeded fixture and the host-picked fixture must not be the same
    // asset: an inserted image has to look different from the seeded one.
    expect(RICH_TEXT_PICKED_IMAGE_SRC).not.toBe(RICH_TEXT_IMAGE_SRC);
    expect(RICH_TEXT_PICKED_IMAGE_ALT).not.toBe(RICH_TEXT_IMAGE_ALT);
    // Deterministic and offline: no DNS, no external service, no mutable
    // remote content anywhere in the seeded image document.
    expect(JSON.stringify(RICH_TEXT_IMAGE_DOCUMENT)).not.toMatch(/https?:\/\//);
    expect(countRichTextImages(RICH_TEXT_IMAGE_DOCUMENT)).toBe(1);
    expect(imageNodes(RICH_TEXT_IMAGE_DOCUMENT).map((node) => node.attrs)).toEqual([
      { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT },
    ]);
  });

  it("shows no image content and no insertion command while images are off", async () => {
    const { container } = render(RichTextEditorSpecimen);
    const toggle = container.querySelector("[data-part='images-toggle']")!;
    await waitFor(() => {
      expect(policy(container).querySelector(".ProseMirror")).not.toBeNull();
    });
    expect(toggle.getAttribute("aria-pressed")).toBe("false");
    expect(toggle.textContent?.trim()).toBe("Images off");
    expect(policy(container).querySelector("img")).toBeNull();
    expect(policy(container).querySelector("[data-command='insert-image'] button")).toBeNull();
    // Ordinary standard content still mounts: images-off refuses nothing.
    expect(policy(container).querySelector("h1")?.textContent).toBe("Release notes");
    // The image-enabled host document is retained, not stripped or echoed.
    expect(metric(container, "images")).toBe("1");
    expect(metric(container, "requests")).toBe("0");
    expect(metric(container, "changes")).toBe("0");
  });

  it("loads the seeded fixture and admits the insert command when images are on", async () => {
    const { container } = render(RichTextEditorSpecimen);
    await waitFor(() => {
      expect(container.querySelector(POLICY)?.querySelector(".ProseMirror")).not.toBeNull();
    });
    await mountImagesOn(container);
    const image = policy(container).querySelector("img")!;
    expect(image.getAttribute("src")).toBe(RICH_TEXT_IMAGE_SRC);
    expect(image.getAttribute("alt")).toBe(RICH_TEXT_IMAGE_ALT);
    expect(
      policy(container).querySelector("[data-command='insert-image'] button"),
    ).not.toBeNull();
    expect(metric(container, "images")).toBe("1");
    expect(metric(container, "requests")).toBe("0");
  });

  it("inserts exactly one host-picked image and reports the host round trip", async () => {
    const { container } = render(RichTextEditorSpecimen);
    await waitFor(() => {
      expect(container.querySelector(POLICY)?.querySelector(".ProseMirror")).not.toBeNull();
    });
    await mountImagesOn(container);
    await fireEvent.click(
      policy(container).querySelectorAll<HTMLButtonElement>("[data-command='insert-image'] button")[0]!,
    );
    await waitFor(() => {
      expect(metric(container, "changes")).toBe("1");
    });
    // One click, one request, one document change, one additional image.
    expect(metric(container, "requests")).toBe("1");
    expect(metric(container, "images")).toBe("2");
    // The click inserts at the editor's live selection, so the exact position
    // is pinned by the browser probe (test/rich-text-image-policy). Here:
    // exactly one additional image carrying the callback's exact attributes.
    const rendered = policy(container).querySelectorAll("img");
    expect([...rendered].map((node) => node.getAttribute("alt")).sort()).toEqual(
      [RICH_TEXT_IMAGE_ALT, RICH_TEXT_PICKED_IMAGE_ALT].sort(),
    );
    const document = hostDocument(container);
    expect(countRichTextImages(document)).toBe(2);
    const nodes = imageNodes(document);
    expect(nodes.length).toBe(2);
    expect(nodes.map((node) => ({ src: node.attrs?.src, alt: node.attrs?.alt }))).toEqual(
      expect.arrayContaining([
        { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT },
        { src: RICH_TEXT_PICKED_IMAGE_SRC, alt: RICH_TEXT_PICKED_IMAGE_ALT },
      ]),
    );
  });

  it("retains the image document across the toggle without callback echo", async () => {
    const { container } = render(RichTextEditorSpecimen);
    await waitFor(() => {
      expect(container.querySelector(POLICY)?.querySelector(".ProseMirror")).not.toBeNull();
    });
    await mountImagesOn(container);
    await fireEvent.click(
      policy(container).querySelectorAll<HTMLButtonElement>("[data-command='insert-image'] button")[0]!,
    );
    await waitFor(() => {
      expect(metric(container, "images")).toBe("2");
    });
    const documentAfterInsert = hostDocument(container);
    const mountedBefore = policy(container).querySelector(".ProseMirror");
    const toggle = container.querySelector("[data-part='images-toggle']")!;

    await fireEvent.click(toggle);
    await waitFor(() => {
      expect(policy(container).querySelector(".ProseMirror")).not.toBe(mountedBefore);
    });
    expect(policy(container).querySelector("img")).toBeNull();
    expect(policy(container).querySelector("[data-command='insert-image'] button")).toBeNull();
    expect(metric(container, "images")).toBe("2");
    expect(metric(container, "requests")).toBe("1");
    expect(metric(container, "changes")).toBe("1");

    await mountImagesOn(container);
    expect(policy(container).querySelectorAll("img").length).toBe(2);
    expect(metric(container, "requests")).toBe("1");
    expect(metric(container, "changes")).toBe("1");
    expect(hostDocument(container)).toEqual(documentAfterInsert);
  });
});

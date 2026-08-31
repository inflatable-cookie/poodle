/**
 * Mounted Svelte external-file specimen (g16.027).
 *
 * The curated states, in the framework a consumer actually uses: a zone being
 * offered files, one refusing them with a reason, the accepted names after a
 * commit, and an export moving through its own visible lifecycle. The
 * exhaustive transport, validation, and cleanup matrix is in
 * `test/headless-dom/inbound-files-and-drag-out.test.ts`; these are the claims
 * that only a mounted surface can make.
 */

import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import ExternalFileHarness from "./ExternalFileHarness.svelte";
import {
  createExportHost,
  createInboundHost,
  inboundBatch,
  inboundFile,
  HOST_PATH,
} from "./external-files";

const ZONE = { left: 10, top: 80, width: 80, height: 20, right: 90, bottom: 100, x: 10, y: 80, toJSON: () => ({}) };
const CLIP = { left: 10, top: 10, width: 80, height: 20, right: 90, bottom: 30, x: 10, y: 10, toJSON: () => ({}) };

function measurable(element: HTMLElement, box: typeof ZONE): HTMLElement {
  element.getBoundingClientRect = () => box as DOMRect;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  return element;
}

function layout(container: HTMLElement): { zone: HTMLElement; clip: HTMLElement } {
  return {
    zone: measurable(container.querySelector('[data-testid="library"]') as HTMLElement, ZONE),
    clip: measurable(container.querySelector('[data-testid="clip"]') as HTMLElement, CLIP),
  };
}

function text(container: HTMLElement, id: string): string {
  return container.querySelector(`[data-testid="${id}"]`)?.textContent?.trim() ?? "";
}

async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

describe("external files (svelte)", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("shows an offered batch, then the names it accepted", async () => {
    const host = createInboundHost();
    const { container } = render(ExternalFileHarness, {
      props: { inboundFileBridge: host.bridge },
    });
    layout(container);
    expect(text(container, "library-state")).toBe("Drop audio files here");

    host.send({ type: "entered", batch: inboundBatch([inboundFile("take-01.wav")]), x: 40, y: 90 });
    await settle();
    expect(text(container, "library-state")).toBe("Drop 1 here");

    host.send({ type: "dropped", batch: inboundBatch([inboundFile("take-01.wav")]), x: 40, y: 90 });
    await settle();

    expect(text(container, "library-files")).toBe("take-01.wav");
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "committed" }]);
  });

  it("shows a refusal with its reason and takes nothing", async () => {
    const host = createInboundHost();
    const { container } = render(ExternalFileHarness, {
      props: { inboundFileBridge: host.bridge, constraints: { maxFiles: 1 } },
    });
    layout(container);

    host.send({
      type: "entered",
      batch: inboundBatch([inboundFile("a.wav"), inboundFile("b.wav")]),
      x: 40,
      y: 90,
    });
    await settle();

    expect(text(container, "library-state")).toBe("Cannot take these files: too-many");
    expect(text(container, "library-files")).toBe("");
  });

  /**
   * The export's own lifecycle is visible without any of it being real to the
   * surface: the host holds the path, the specimen shows a name.
   */
  it("shows the export preparing, dragging, and ending without ever seeing a path", async () => {
    const host = createExportHost();
    const { container } = render(ExternalFileHarness, { props: { exportBridge: host.bridge } });
    const { clip } = layout(container);

    await fireEvent.pointerDown(clip, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await settle();
    expect(text(container, "export-state")).toBe("armed");
    expect(text(container, "export-name")).toBe("take-01.wav");
    expect(clip.getAttribute("data-poodle-drag-export")).toBe("armed");

    await fireEvent(clip, new DragEvent("dragstart", { bubbles: true, cancelable: true }));
    await settle();
    expect(text(container, "export-state")).toBe("dragging");

    host.report({ status: "ended" });
    await settle();

    expect(text(container, "export-state")).toBe("ended");
    expect(host.cancels).toEqual([]);
    expect(text(container, "announcement")).toBe("Finished exporting Intro clip");
    expect(container.innerHTML).not.toContain(HOST_PATH);
    expect(container.innerHTML).not.toContain("export-1");
  });

  it("leaves a source with no export bridge alone", () => {
    const { container } = render(ExternalFileHarness, { props: {} });
    const { clip } = layout(container);

    expect(clip.hasAttribute("data-poodle-drag-export")).toBe(false);
    expect(text(container, "export-state")).toBe("idle");
  });
});

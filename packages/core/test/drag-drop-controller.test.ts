import { afterEach, describe, expect, test } from "bun:test";
import { Window } from "happy-dom";

import type { CrossWindowDragReceipt, CrossWindowDragSourceBridge } from "../src/cross-window-drag.ts";
import { createDragDropController } from "../src/dom/drag-drop-controller.ts";
import type { DragExportBridge, PreparedFileExport } from "../src/external-file-drag.ts";

let activeWindow: Window | null = null;
const originalNode = globalThis.Node;
const originalElement = globalThis.Element;
const originalHTMLElement = globalThis.HTMLElement;
const originalSVGElement = globalThis.SVGElement;
const originalPointerEvent = globalThis.PointerEvent;

afterEach(() => {
  activeWindow?.close();
  activeWindow = null;
  globalThis.Node = originalNode;
  globalThis.Element = originalElement;
  globalThis.HTMLElement = originalHTMLElement;
  globalThis.SVGElement = originalSVGElement;
  globalThis.PointerEvent = originalPointerEvent;
});

function setupDom(): { window: Window; document: Document; root: HTMLElement; source: HTMLElement } {
  const window = new Window();
  activeWindow = window;
  globalThis.Node = window.Node as unknown as typeof Node;
  globalThis.Element = window.Element as unknown as typeof Element;
  globalThis.HTMLElement = window.HTMLElement as unknown as typeof HTMLElement;
  globalThis.SVGElement = window.SVGElement as unknown as typeof SVGElement;
  globalThis.PointerEvent = window.PointerEvent as unknown as typeof PointerEvent;

  const document = window.document as unknown as Document;
  const root = document.createElement("div");
  const source = document.createElement("div");
  root.appendChild(source);
  document.body.appendChild(root);
  return { window, document, root, source };
}

function flushMicrotasks(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

describe("createDragDropController async cancel safety (happy-dom)", () => {
  test("catches async rejection from cross-window bridge.cancel on session cancel and superseded prepare", async () => {
    const { window, root, source } = setupDom();
    const unhandled: unknown[] = [];
    const onUnhandled = (reason: unknown) => {
      unhandled.push(reason);
    };
    process.on("unhandledRejection", onUnhandled);

    try {
      let cancelCalls = 0;
      let resolvePrepare: ((receipt: CrossWindowDragReceipt | null) => void) | null = null;
      const bridge: CrossWindowDragSourceBridge = {
        capabilities: { pointer: true, touch: false, keyboardTargetPicker: false },
        prepare() {
          return new Promise((resolve) => {
            resolvePrepare = resolve;
          });
        },
        start() {
          return () => {};
        },
        cancel() {
          cancelCalls += 1;
          return Promise.reject(new Error("host cancel rejected"));
        },
      };

      const controller = createDragDropController({ createSessionId: () => "s-cw" });
      const disconnect = controller.connect(root);
      controller.registerSource(source, {
        sourceId: "src-cw",
        subject: { kind: "track", id: "t1" },
        allowedOperations: ["move"],
        label: "Track 1",
        crossWindowSourceBridge: bridge,
      });

      // Path 1: prepare resolves after cancel (superseded receipt cancel).
      source.dispatchEvent(
        new window.PointerEvent("pointerdown", {
          pointerId: 1,
          pointerType: "mouse",
          isPrimary: true,
          button: 0,
          clientX: 10,
          clientY: 10,
          bubbles: true,
        }) as unknown as Event,
      );
      controller.cancel();
      resolvePrepare!({ protocolVersion: 1, token: "tok-superseded" });
      await flushMicrotasks();

      // Path 2: prepare resolves while active, then controller.cancel() releases the receipt.
      source.dispatchEvent(
        new window.PointerEvent("pointerdown", {
          pointerId: 2,
          pointerType: "mouse",
          isPrimary: true,
          button: 0,
          clientX: 10,
          clientY: 10,
          bubbles: true,
        }) as unknown as Event,
      );
      resolvePrepare!({ protocolVersion: 1, token: "tok-active" });
      await flushMicrotasks();
      controller.cancel();
      await flushMicrotasks();

      expect(cancelCalls).toBe(2);
      expect(unhandled).toEqual([]);
      disconnect();
      controller.destroy();
    } finally {
      process.off("unhandledRejection", onUnhandled);
    }
  });

  test("catches async rejection from file-export bridge.cancel on session cancel", async () => {
    const { window, root, source } = setupDom();
    const unhandled: unknown[] = [];
    const onUnhandled = (reason: unknown) => {
      unhandled.push(reason);
    };
    process.on("unhandledRejection", onUnhandled);

    try {
      let cancelCalls = 0;
      const prepared: PreparedFileExport = {
        receiptId: "exp-1",
        displayName: "track.wav",
        form: "existing-file",
      };
      const bridge: DragExportBridge = {
        capabilities: { files: true, multipleFiles: false, promisedFiles: false, customDataTypes: [] },
        prepare() {
          return Promise.resolve(prepared);
        },
        start() {
          return () => {};
        },
        cancel() {
          cancelCalls += 1;
          return Promise.reject(new Error("export cancel rejected"));
        },
      };

      const controller = createDragDropController({ createSessionId: () => "s-exp" });
      const disconnect = controller.connect(root);
      controller.registerSource(source, {
        sourceId: "src-exp",
        subject: { kind: "track", id: "t1" },
        allowedOperations: ["copy"],
        label: "Track 1",
        fileExportBridge: bridge,
      });

      source.dispatchEvent(
        new window.PointerEvent("pointerdown", {
          pointerId: 1,
          pointerType: "mouse",
          isPrimary: true,
          button: 0,
          clientX: 10,
          clientY: 10,
          bubbles: true,
        }) as unknown as Event,
      );
      await flushMicrotasks();
      controller.cancel();
      await flushMicrotasks();

      expect(cancelCalls).toBe(1);
      expect(unhandled).toEqual([]);
      disconnect();
      controller.destroy();
    } finally {
      process.off("unhandledRejection", onUnhandled);
    }
  });
});

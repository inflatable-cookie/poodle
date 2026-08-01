import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  observeOverlaySurfaceGeometry,
  type OverlaySurfaceGeometryChange,
} from "../../packages/core/src";

describe("observeOverlaySurfaceGeometry", () => {
  let frames: FrameRequestCallback[];
  let rect: DOMRect;
  let resize: ResizeObserverCallback | null;

  beforeEach(() => {
    frames = [];
    rect = new DOMRect(20, 30, 120, 80);
    resize = null;
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      frames.push(callback);
      return frames.length;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    vi.stubGlobal(
      "ResizeObserver",
      class {
        constructor(callback: ResizeObserverCallback) {
          resize = callback;
        }
        observe() {}
        disconnect() {}
      },
    );
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  function flushFrame(): void {
    const pending = frames.splice(0);
    for (const callback of pending) {
      callback(0);
    }
  }

  function mountSurface(): HTMLElement {
    const node = document.createElement("div");
    vi.spyOn(node, "getBoundingClientRect").mockImplementation(() => rect);
    document.body.appendChild(node);
    return node;
  }

  it("reports copied viewport geometry, movement, visibility, and one teardown", () => {
    const changes: OverlaySurfaceGeometryChange[] = [];
    const node = mountSurface();
    const observer = observeOverlaySurfaceGeometry(node, "surface-1", {
      onChange: (change) => changes.push(change),
      placement: "bottom-start",
    });

    flushFrame();
    expect(changes).toEqual([
      {
        type: "upsert",
        surface: {
          surfaceId: "surface-1",
          rect: {
            x: 20,
            y: 30,
            width: 120,
            height: 80,
            top: 30,
            right: 140,
            bottom: 110,
            left: 20,
          },
          placement: "bottom-start",
          visible: true,
        },
      },
    ]);

    rect = new DOMRect(50, 60, 120, 80);
    window.dispatchEvent(new Event("scroll"));
    flushFrame();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { rect: { left: 50, top: 60 } },
    });

    rect = new DOMRect(50, 60, 150, 100);
    resize?.([], {} as ResizeObserver);
    flushFrame();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { rect: { width: 150, height: 100 } },
    });

    rect = new DOMRect(70, 80, 150, 100);
    window.dispatchEvent(new Event("resize"));
    flushFrame();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { rect: { left: 70, top: 80 } },
    });

    node.dataset.anchorHidden = "true";
    observer.report();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { visible: false },
    });

    delete node.dataset.anchorHidden;
    rect = new DOMRect(50, 60, 0, 80);
    observer.report();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { visible: false },
    });

    observer.destroy();
    observer.destroy();
    expect(changes.filter((change) => change.type === "remove")).toEqual([
      { type: "remove", surfaceId: "surface-1" },
    ]);
  });

  it("does no reporting work until a callback is supplied", () => {
    const changes: OverlaySurfaceGeometryChange[] = [];
    const observer = observeOverlaySurfaceGeometry(mountSurface(), "surface-2", {
      onChange: null,
      placement: null,
    });

    window.dispatchEvent(new Event("scroll"));
    flushFrame();
    expect(changes).toEqual([]);

    observer.update({
      onChange: (change) => changes.push(change),
      placement: "top",
    });
    flushFrame();
    expect(changes.at(-1)).toMatchObject({
      type: "upsert",
      surface: { placement: "top" },
    });

    observer.destroy();
  });
});

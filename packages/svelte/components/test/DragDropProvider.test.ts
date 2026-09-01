/**
 * Mounted Svelte custom-surface drag substrate (g16.022).
 */

import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import DragDropCustomSurface from "./DragDropCustomSurface.svelte";

const SOURCE = { left: 10, top: 10, width: 80, height: 20, right: 90, bottom: 30, x: 10, y: 10, toJSON: () => ({}) };
const TARGET = { left: 10, top: 80, width: 80, height: 20, right: 90, bottom: 100, x: 10, y: 80, toJSON: () => ({}) };

function measurable(element: HTMLElement, box: typeof SOURCE): HTMLElement {
  element.getBoundingClientRect = () => box as DOMRect;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  return element;
}

function layout(container: HTMLElement): { source: HTMLElement; list: HTMLElement } {
  const source = measurable(container.querySelector('[data-testid="source-a"]') as HTMLElement, SOURCE);
  const list = measurable(container.querySelector('[data-testid="scope-a"] [data-testid="drop-list"]') as HTMLElement, TARGET);
  return { source, list };
}

describe("DragDropProvider (svelte)", () => {
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

  it("commits a pointer drag onto the list", async () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const { container } = render(DragDropCustomSurface, { props: { onDropA } });
    const { source } = layout(container);

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    await fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });

    expect(onDropA).toHaveBeenCalledTimes(1);
    expect(onDropA).toHaveBeenCalledWith({
      targetId: "scope-a-list",
      position: "inside",
      operation: "move",
    });
  });

  it("publishes consecutive preview coordinates to consumers and custom renderers", async () => {
    const { container } = render(DragDropCustomSurface);
    const { source } = layout(container);
    const consumer = () => container.querySelector('[data-testid="preview-x"]')?.textContent ?? "";

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 40, clientY: 20 });
    const first = Number(consumer());
    const firstRenderer = Number(container.querySelector('[data-testid="custom-preview-x"]')?.textContent);
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 70, clientY: 20 });
    const second = Number(consumer());
    const secondRenderer = Number(container.querySelector('[data-testid="custom-preview-x"]')?.textContent);
    await fireEvent.pointerUp(source, { pointerId: 1, clientX: 70, clientY: 20 });

    expect(first).toBeGreaterThan(0);
    expect(second).toBeGreaterThan(first);
    expect(firstRenderer).toBe(first);
    expect(secondRenderer).toBe(second);
    expect(second - first).toBe(30);
  });

  it("commits a touch-like pointer after travelling past mouse distance", async () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const { container } = render(DragDropCustomSurface, { props: { onDropA } });
    const { source } = layout(container);

    await fireEvent.pointerDown(source, {
      button: 0,
      pointerId: 1,
      pointerType: "pen",
      clientX: 20,
      clientY: 20,
    });
    await fireEvent.pointerMove(source, { pointerId: 1, pointerType: "pen", clientX: 30, clientY: 90 });
    await fireEvent.pointerUp(source, { pointerId: 1, pointerType: "pen", clientX: 30, clientY: 90 });

    expect(onDropA).toHaveBeenCalledTimes(1);
  });

  it("picks up from the keyboard and drops", async () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const { container } = render(DragDropCustomSurface, { props: { onDropA } });
    const { source } = layout(container);
    source.focus();

    await fireEvent.keyDown(source, { key: " " });
    await fireEvent.keyDown(source, { key: "ArrowDown" });
    await fireEvent.keyDown(source, { key: "Enter" });

    expect(onDropA).toHaveBeenCalledTimes(1);
  });

  it("rejects an ineligible target", async () => {
    const onDropA = vi.fn();
    const { container, getByTestId } = render(DragDropCustomSurface, {
      props: { rejectA: true, onDropA },
    });
    const { source } = layout(container);

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });

    expect(getByTestId("scope-a").querySelector('[data-poodle-drop-target="rejected"]')).not.toBeNull();
    await fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(onDropA).not.toHaveBeenCalled();
  });

  it("cancels on Escape and restores the source attribute", async () => {
    const { container } = render(DragDropCustomSurface, { props: {} });
    const { source } = layout(container);

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(source.getAttribute("data-poodle-drag-source")).toBe("dragging");
    await fireEvent.keyDown(source, { key: "Escape" });
    expect(source.hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("cleans up on unmount", async () => {
    const { container, unmount } = render(DragDropCustomSurface, { props: {} });
    const { source } = layout(container);

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    unmount();
    expect(source.hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("keeps two provider scopes independent", async () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const onDropB = vi.fn(() => ({ status: "committed" as const }));
    const { container } = render(DragDropCustomSurface, { props: { onDropA, onDropB } });
    const { source } = layout(container);
    const sourceB = measurable(container.querySelector('[data-testid="source-b"]') as HTMLElement, SOURCE);
    const listB = measurable(
      container.querySelector('[data-testid="scope-b"] [data-testid="drop-list"]') as HTMLElement,
      TARGET,
    );

    await fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    await fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(source.getAttribute("data-poodle-drag-source")).toBe("dragging");
    expect(sourceB.hasAttribute("data-poodle-drag-source")).toBe(false);

    await fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(onDropA).toHaveBeenCalledTimes(1);
    expect(onDropB).not.toHaveBeenCalled();
    expect(listB.hasAttribute("data-poodle-drop-target")).toBe(false);
  });
});

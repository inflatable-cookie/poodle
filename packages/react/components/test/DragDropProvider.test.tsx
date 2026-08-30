/**
 * Mounted React custom-surface drag substrate (g16.022).
 */

import { fireEvent, render } from "@testing-library/react";
import { StrictMode, useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { createDragDropController } from "@inflatable-cookie/poodle-core";

import { DragDropProvider, useDragSource, useDropTarget } from "../src/drag-drop";
import { DragDropCustomSurface } from "./DragDropCustomSurface";

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

describe("DragDropProvider (react)", () => {
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

  it("commits a pointer drag onto the list", () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const view = render(<DragDropCustomSurface onDropA={onDropA} />);
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });

    expect(onDropA).toHaveBeenCalledTimes(1);
    expect(onDropA.mock.calls[0]?.[0]).toEqual({
      targetId: "scope-a-list",
      position: "inside",
      operation: "move",
    });
  });

  it("commits a pen-shaped pointer drag", () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const view = render(<DragDropCustomSurface onDropA={onDropA} />);
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, pointerType: "pen", clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, pointerType: "pen", clientX: 30, clientY: 90 });
    fireEvent.pointerUp(source, { pointerId: 1, pointerType: "pen", clientX: 30, clientY: 90 });

    expect(onDropA).toHaveBeenCalledTimes(1);
  });

  it("picks up from the keyboard and drops", () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const view = render(<DragDropCustomSurface onDropA={onDropA} />);
    const { source } = layout(view.container);
    source.focus();

    fireEvent.keyDown(source, { key: " " });
    fireEvent.keyDown(source, { key: "ArrowDown" });
    fireEvent.keyDown(source, { key: "Enter" });

    expect(onDropA).toHaveBeenCalledTimes(1);
  });

  it("rejects an ineligible target", () => {
    const onDropA = vi.fn();
    const view = render(<DragDropCustomSurface rejectA onDropA={onDropA} />);
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });

    expect(view.container.querySelector('[data-testid="scope-a"] [data-poodle-drop-target="rejected"]')).not.toBeNull();
    fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(onDropA).not.toHaveBeenCalled();
  });

  it("cancels on Escape", () => {
    const view = render(<DragDropCustomSurface />);
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(source.getAttribute("data-poodle-drag-source")).toBe("dragging");
    fireEvent.keyDown(source, { key: "Escape" });
    expect(source.hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("cleans up on unmount", () => {
    const view = render(<DragDropCustomSurface />);
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    view.unmount();
    expect(source.hasAttribute("data-poodle-drag-source")).toBe(false);
  });

  it("keeps two provider scopes independent", () => {
    const onDropA = vi.fn(() => ({ status: "committed" as const }));
    const onDropB = vi.fn(() => ({ status: "committed" as const }));
    const view = render(<DragDropCustomSurface onDropA={onDropA} onDropB={onDropB} />);
    const { source } = layout(view.container);
    const sourceB = measurable(view.container.querySelector('[data-testid="source-b"]') as HTMLElement, SOURCE);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(source.getAttribute("data-poodle-drag-source")).toBe("dragging");
    expect(sourceB.hasAttribute("data-poodle-drag-source")).toBe(false);

    fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(onDropA).toHaveBeenCalledTimes(1);
    expect(onDropB).not.toHaveBeenCalled();
  });

  it("composes a consumer ref on the source", () => {
    const seen: HTMLElement[] = [];

    function Source() {
      const { getSourceProps } = useDragSource({
        sourceId: "a",
        subject: { kind: "item", id: "a" },
        allowedOperations: ["move"],
        label: "Alpha",
      });
      return (
        <button
          type="button"
          {...getSourceProps({
            ref: (node) => {
              if (node) seen.push(node);
            },
          })}
        >
          Alpha
        </button>
      );
    }

    function Target() {
      const { getTargetProps } = useDropTarget({
        targetId: "list",
        acceptedKinds: ["item"],
        label: "List",
        resolvePosition: () => "inside",
        canDrop: (intent) => ({ accepted: true, intent }),
        onDrop: () => ({ status: "committed" }),
      });
      return <div {...getTargetProps()} />;
    }

    const view = render(
      <DragDropProvider>
        <Source />
        <Target />
      </DragDropProvider>,
    );
    expect(seen[0]).toBe(view.getByRole("button", { name: "Alpha" }));
  });

  it("survives StrictMode remount without destroying the live controller", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const view = render(
      <StrictMode>
        <DragDropCustomSurface onDropA={onDrop} />
      </StrictMode>,
    );
    const { source } = layout(view.container);

    fireEvent.pointerDown(source, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(source, { pointerId: 1, clientX: 30, clientY: 90 });
    fireEvent.pointerUp(source, { pointerId: 1, clientX: 30, clientY: 90 });

    expect(onDrop).toHaveBeenCalledTimes(1);
  });

  it("disconnects an injected controller on unmount without destroying it", () => {
    const controller = createDragDropController();
    const destroy = vi.spyOn(controller, "destroy");

    function Source() {
      const { getSourceProps } = useDragSource({
        sourceId: "a",
        subject: { kind: "item", id: "a" },
        allowedOperations: ["move"],
        label: "Alpha",
      });
      return <button type="button" {...getSourceProps()}>Alpha</button>;
    }

    function Target() {
      const { getTargetProps } = useDropTarget({
        targetId: "list",
        acceptedKinds: ["item"],
        label: "List",
        resolvePosition: () => "inside",
        canDrop: (intent) => ({ accepted: true, intent }),
        onDrop: () => ({ status: "committed" }),
      });
      return <div {...getTargetProps()} />;
    }

    const view = render(
      <DragDropProvider controller={controller}>
        <Source />
        <Target />
      </DragDropProvider>,
    );
    view.unmount();
    expect(destroy).not.toHaveBeenCalled();
    const host = document.createElement("div");
    document.body.append(host);
    const disconnect = controller.connect(host);
    disconnect();
    controller.destroy();
    host.remove();
  });

  it("re-registers when the host node is replaced", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));

    function Swap() {
      const [asDiv, setAsDiv] = useState(false);
      const { getSourceProps } = useDragSource({
        sourceId: "a",
        subject: { kind: "item", id: "a" },
        allowedOperations: ["move"],
        label: "Alpha",
      });
      const { getTargetProps } = useDropTarget({
        targetId: "list",
        acceptedKinds: ["item"],
        label: "List",
        resolvePosition: () => "inside",
        canDrop: (intent) => ({ accepted: true, intent }),
        onDrop,
      });
      const Tag = asDiv ? "div" : "button";
      return (
        <>
          <Tag data-testid="src" {...getSourceProps()}>Alpha</Tag>
          <button type="button" data-testid="swap" onClick={() => setAsDiv(true)}>
            swap
          </button>
          <div data-testid="dst" {...getTargetProps()} />
        </>
      );
    }

    const view = render(
      <DragDropProvider>
        <Swap />
      </DragDropProvider>,
    );
    const first = measurable(view.getByTestId("src"), SOURCE);
    expect(first.getAttribute("draggable")).toBe("false");
    fireEvent.click(view.getByTestId("swap"));
    expect(first.hasAttribute("draggable")).toBe(false);

    const second = measurable(view.getByTestId("src"), SOURCE);
    const target = measurable(view.getByTestId("dst"), TARGET);
    expect(second.tagName).toBe("DIV");
    expect(second.getAttribute("draggable")).toBe("false");

    fireEvent.pointerDown(second, { button: 0, pointerId: 1, clientX: 20, clientY: 20 });
    fireEvent.pointerMove(second, { pointerId: 1, clientX: 30, clientY: 90 });
    fireEvent.pointerUp(second, { pointerId: 1, clientX: 30, clientY: 90 });
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(target).toBeTruthy();
  });
});

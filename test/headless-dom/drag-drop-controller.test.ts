/**
 * Framework-free drag-drop controller lifetime (g16.022).
 *
 * Kernel transitions are pinned by the shared `dragDrop` vectors. These tests
 * prove the DOM adapter: registration identity, pointer/touch/keyboard sensors,
 * async drop identity, teardown, and two isolated controllers.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  createDragDropController,
  type DragSourceRegistration,
  type DropTargetRegistration,
  type KeyboardDropTargetRegistration,
} from "../../packages/core/src";

const SOURCE_BOX = { x: 10, y: 10, width: 80, height: 20 };
const TARGET_BOX = { x: 10, y: 80, width: 80, height: 20 };
const NESTED_BOX = { x: 20, y: 90, width: 40, height: 10 };

function asRect(box: { x: number; y: number; width: number; height: number }): DOMRect {
  return {
    x: box.x,
    y: box.y,
    width: box.width,
    height: box.height,
    top: box.y,
    left: box.x,
    right: box.x + box.width,
    bottom: box.y + box.height,
    toJSON() {
      return this;
    },
  } as DOMRect;
}

function layout<T extends Element>(
  element: T,
  box: { x: number; y: number; width: number; height: number },
): T {
  (element as HTMLElement).getBoundingClientRect = () => asRect(box);
  (element as HTMLElement).setPointerCapture = vi.fn();
  (element as HTMLElement).releasePointerCapture = vi.fn();
  return element;
}

function pointer(
  type: "pointerdown" | "pointermove" | "pointerup" | "pointercancel" | "lostpointercapture",
  init: PointerEventInit,
): PointerEvent {
  return new PointerEvent(type, {
    bubbles: true,
    cancelable: true,
    pointerId: 1,
    pointerType: "mouse",
    button: 0,
    buttons: type === "pointerdown" || type === "pointermove" ? 1 : 0,
    isPrimary: true,
    clientX: 0,
    clientY: 0,
    ...init,
  });
}

function sourceReg(overrides: Partial<DragSourceRegistration> = {}): DragSourceRegistration {
  return {
    sourceId: "src",
    subject: { kind: "item", id: "a" },
    allowedOperations: ["move"],
    label: "Alpha",
    ...overrides,
  };
}

function targetReg(overrides: Partial<DropTargetRegistration> = {}): DropTargetRegistration {
  return {
    targetId: "dst",
    acceptedKinds: ["item"],
    label: "List",
    resolvePosition: () => "inside",
    canDrop: (intent) => ({ accepted: true, intent }),
    onDrop: () => ({ status: "committed" }),
    ...overrides,
  };
}

function keyboardTargetReg(
  overrides: Partial<KeyboardDropTargetRegistration> = {},
): KeyboardDropTargetRegistration {
  return {
    targetId: "dst",
    acceptedKinds: ["item"],
    label: "List",
    order: 0,
    resolvePosition: (input) =>
      input.direction === "previous" || input.direction === "first" ? "before" : "after",
    canDrop: (intent, subject) =>
      subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
    onDrop: () => ({ status: "committed" }),
    ...overrides,
  };
}

describe("createDragDropController", () => {
  let root: HTMLElement;
  let sourceEl: HTMLElement;
  let targetEl: HTMLElement;

  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    root = document.createElement("div");
    sourceEl = layout(document.createElement("button"), SOURCE_BOX);
    sourceEl.textContent = "Alpha";
    targetEl = layout(document.createElement("div"), TARGET_BOX);
    targetEl.textContent = "List";
    root.append(sourceEl, targetEl);
    document.body.append(root);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  it("connects once and treats a second connect as an error", () => {
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    expect(() => controller.connect(root)).toThrow(/already connected/);
    disconnect();
    const again = controller.connect(root);
    again();
    controller.destroy();
  });

  it("rejects duplicate live source and target ids", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    const other = layout(document.createElement("button"), { x: 200, y: 10, width: 80, height: 20 });
    root.append(other);
    expect(() => controller.registerSource(other, sourceReg())).toThrow(/Duplicate drag source id/);

    controller.registerTarget(targetEl, targetReg());
    const otherTarget = layout(document.createElement("div"), { x: 200, y: 80, width: 80, height: 20 });
    root.append(otherTarget);
    expect(() => controller.registerTarget(otherTarget, targetReg())).toThrow(/Duplicate drop target id/);
    controller.destroy();
  });

  it("unregisters idempotently and isolates two controllers", () => {
    const first = createDragDropController();
    const second = createDragDropController();
    first.connect(root);
    const rootB = document.createElement("div");
    const sourceB = layout(document.createElement("button"), SOURCE_BOX);
    sourceB.textContent = "Beta";
    const targetB = layout(document.createElement("div"), TARGET_BOX);
    rootB.append(sourceB, targetB);
    document.body.append(rootB);
    second.connect(rootB);

    const handle = first.registerSource(sourceEl, sourceReg());
    first.registerTarget(targetEl, targetReg());
    second.registerSource(sourceB, sourceReg({ sourceId: "src", label: "Beta" }));
    second.registerTarget(targetB, targetReg());

    handle.unregister();
    handle.unregister();

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 40 }));
    expect(first.getSnapshot().phase).toBe("idle");

    sourceB.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceB.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 90 }));
    expect(second.getSnapshot().phase).toBe("dragging");
    expect(first.getSnapshot().phase).toBe("idle");

    first.destroy();
    second.destroy();
  });

  it("activates mouse drag after distance and commits on an accepted target", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onStart = vi.fn();
    const onEnd = vi.fn();
    const controller = createDragDropController({ createSessionId: () => "s1" });
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragStart: onStart, onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg({ onDrop }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("idle");

    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 24, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(onStart).toHaveBeenCalledTimes(1);
    expect(sourceEl.getAttribute("data-poodle-drag-source")).toBe("dragging");

    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    expect(targetEl.getAttribute("data-poodle-drop-target")).toBe("accepted");

    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(onEnd).toHaveBeenCalledWith({
      status: "committed",
      intent: { targetId: "dst", position: "inside", operation: "move" },
    });
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(sourceEl.hasAttribute("data-poodle-drag-source")).toBe(false);
    expect(targetEl.hasAttribute("data-poodle-drop-target")).toBe(false);
    controller.destroy();
  });

  it("treats a mouse press that never travels as a tap", () => {
    const onStart = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragStart: onStart }));
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointerup", { clientX: 21, clientY: 20 }));
    expect(onStart).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().announcement).toBeNull();
    controller.destroy();
  });

  it("lets a native scroll before the hold cancel the candidate", () => {
    vi.useFakeTimers();
    const onStart = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({
        onDragStart: onStart,
        activation: { touch: { holdMs: 250, tolerance: 8 } },
      }),
    );
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    root.dispatchEvent(new Event("scroll", { bubbles: true }));
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("lets touch scrolling win when movement exceeds tolerance before the hold", () => {
    vi.useFakeTimers();
    const onStart = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({
        onDragStart: onStart,
        activation: { touch: { holdMs: 250, tolerance: 8 } },
      }),
    );
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { pointerType: "touch", clientX: 20, clientY: 40 }));
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(sourceEl.setPointerCapture).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("activates touch after the hold while movement stays inside tolerance", () => {
    vi.useFakeTimers();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({ activation: { touch: { holdMs: 250, tolerance: 8 } } }),
    );
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { pointerType: "touch", clientX: 22, clientY: 21 }));
    expect(controller.getSnapshot().phase).toBe("idle");
    vi.advanceTimersByTime(250);
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(sourceEl.setPointerCapture).toHaveBeenCalledWith(1);
    controller.destroy();
  });

  it("does not pick up from Space or Enter unless the source declares keyboardOrder", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg());
    sourceEl.focus();

    const space = new KeyboardEvent("keydown", { key: " ", bubbles: true, cancelable: true });
    sourceEl.dispatchEvent(space);
    expect(space.defaultPrevented).toBe(false);
    expect(controller.getSnapshot().phase).toBe("idle");

    const enter = new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true });
    sourceEl.dispatchEvent(enter);
    expect(enter.defaultPrevented).toBe(false);
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("picks up from the keyboard, moves intent, and drops", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ onDrop }));
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().inputKind).toBe("keyboard");

    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetPosture).toBe("accepted");

    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(document.activeElement).toBe(sourceEl);
    controller.destroy();
  });

  it("moves keyboard intent from the source rather than wrapping to the first target", () => {
    const rowA = layout(document.createElement("div"), { x: 10, y: 10, width: 80, height: 20 });
    const rowB = layout(document.createElement("div"), { x: 10, y: 40, width: 80, height: 20 });
    const rowC = layout(document.createElement("div"), { x: 10, y: 70, width: 80, height: 20 });
    rowA.tabIndex = 0;
    root.replaceChildren(rowA, rowB, rowC);
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const rowTarget = (id: string): DropTargetRegistration =>
      targetReg({
        targetId: id,
        label: id,
        resolvePosition: (input) => (input.y < input.rect.top + input.rect.height / 2 ? "before" : "after"),
        canDrop: (intent, subject) =>
          subject.id === intent.targetId ? { accepted: false, reason: "self" } : { accepted: true, intent },
        onDrop,
      });

    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      rowB,
      sourceReg({ sourceId: "b", subject: { kind: "item", id: "b" }, label: "Beta", keyboardOrder: 0 }),
    );
    controller.registerTarget(rowA, rowTarget("a"));
    controller.registerTarget(rowB, rowTarget("b"));
    controller.registerTarget(rowC, rowTarget("c"));
    rowB.focus();

    rowB.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("c");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("c");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onDrop).toHaveBeenCalledWith(
      { targetId: "c", position: "after", operation: "move" },
      { subject: { kind: "item", id: "b" }, inboundFiles: null },
    );
    controller.destroy();
  });

  it("does not start a pointer drag from a data-poodle-no-drag descendant", () => {
    const row = layout(document.createElement("div"), SOURCE_BOX);
    const twisty = document.createElement("span");
    twisty.setAttribute("data-poodle-no-drag", "");
    row.append(twisty);
    root.replaceChildren(row, targetEl);
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(row, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    twisty.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    twisty.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("does not start a pointer drag from an interactive descendant of a whole-row source", () => {
    const row = layout(document.createElement("div"), SOURCE_BOX);
    const button = document.createElement("button");
    button.textContent = "Remove";
    row.append(button);
    root.replaceChildren(row, targetEl);
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(row, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    button.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    button.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("does not start a pointer drag from a bare contenteditable descendant", () => {
    const row = layout(document.createElement("div"), SOURCE_BOX);
    const editor = document.createElement("div");
    editor.setAttribute("contenteditable", "");
    editor.textContent = "Edit me";
    row.append(editor);
    root.replaceChildren(row, targetEl);
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(row, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    editor.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    editor.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("uses logical keyboard order and distinct previous/next positions", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const rowA = layout(document.createElement("div"), { x: 10, y: 10, width: 80, height: 20 });
    const rowB = layout(document.createElement("div"), { x: 10, y: 40, width: 80, height: 20 });
    rowA.tabIndex = 0;
    rowB.tabIndex = 0;
    root.replaceChildren(rowA, rowB);
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      rowB,
      sourceReg({ sourceId: "b", subject: { kind: "item", id: "b" }, label: "Beta", keyboardOrder: 1 }),
    );
    controller.registerTarget(rowA, targetReg({ targetId: "a", label: "Alpha" }));
    controller.registerTarget(rowB, targetReg({ targetId: "b", label: "Beta" }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "a", label: "Alpha", order: 0, onDrop }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "b", label: "Beta", order: 1, onDrop }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "c", label: "Gamma", order: 2, onDrop }));
    rowB.focus();

    rowB.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("c");
    expect(controller.getSnapshot().session?.intent?.position).toBe("after");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onDrop).toHaveBeenCalledWith(
      { targetId: "c", position: "after", operation: "move" },
      { subject: { kind: "item", id: "b" }, inboundFiles: null },
    );

    rowB.focus();
    rowB.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowUp", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("a");
    expect(controller.getSnapshot().session?.intent?.position).toBe("before");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onDrop).toHaveBeenLastCalledWith(
      { targetId: "a", position: "before", operation: "move" },
      { subject: { kind: "item", id: "b" }, inboundFiles: null },
    );
    controller.destroy();
  });

  it("cancels logical keyboard intent when that registration is removed", () => {
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    const logical = controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "hidden", order: 1 }));
    sourceEl.focus();
    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("hidden");
    logical.unregister();
    expect(onEnd.mock.calls[0]?.[0]).toEqual({ status: "cancelled", reason: "target-lost" });
    controller.destroy();
  });

  it("rejects duplicate logical target ids while allowing a shared DOM id", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerTarget(targetEl, targetReg({ targetId: "shared" }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "shared", order: 0 }));
    expect(() => controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "shared", order: 1 }))).toThrow(
      /Duplicate keyboard drop target id/,
    );
    controller.destroy();
  });

  it("uses spatial keyboard when logical targets do not match the source kind", () => {
    const onDomDrop = vi.fn(() => ({ status: "committed" as const }));
    const onLogicalDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ onDrop: onDomDrop }));
    controller.registerKeyboardTarget(
      keyboardTargetReg({
        targetId: "clip",
        acceptedKinds: ["clip"],
        label: "Clip",
        order: 0,
        onDrop: onLogicalDrop,
      }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("dst");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(onDomDrop).toHaveBeenCalledTimes(1);
    expect(onLogicalDrop).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("announces the DOM target during pointer drag when a logical target shares the id", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "Date"] });
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ targetId: "shared", label: "DOM List" }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "shared", label: "Keyboard List", order: 0 }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().announcement).toBe("Alpha, inside DOM List");
    expect(controller.getSnapshot().announcement).not.toContain("Keyboard List");
    controller.destroy();
  });

  it("announces the logical target during keyboard drag when a DOM target shares the id", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ targetId: "shared", label: "DOM List" }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "shared", label: "Keyboard List", order: 1 }));
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("shared");
    expect(controller.getSnapshot().announcement).toBe("Alpha, after Keyboard List");
    expect(controller.getSnapshot().announcement).not.toContain("DOM List");
    controller.destroy();
  });

  it("keeps a spatial keyboard drop on the DOM registry after a matching logical target is added", () => {
    const onDomDrop = vi.fn(() => ({ status: "committed" as const }));
    const onLogicalDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ targetId: "shared", onDrop: onDomDrop }));
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("shared");
    controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "shared", label: "Keyboard List", order: 0, onDrop: onLogicalDrop }),
    );
    root.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(onDomDrop).toHaveBeenCalledTimes(1);
    expect(onLogicalDrop).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("keeps an async logical drop on the logical registry after acceptedKinds change", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onDomDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg({ targetId: "shared", onDrop: onDomDrop }));
    const logical = controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "shared", order: 1, onDrop: () => pending }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("dropping");

    logical.update(keyboardTargetReg({ targetId: "shared", acceptedKinds: ["clip"], order: 1, onDrop: () => pending }));
    finish({ status: "committed" });
    await pending;
    await Promise.resolve();
    expect(onDomDrop).not.toHaveBeenCalled();
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({
      status: "committed",
      intent: { targetId: "shared", position: "after", operation: "move" },
    });
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("clears a rejected logical snapshot when the next resolver returns null", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerKeyboardTarget(
      keyboardTargetReg({
        targetId: "blocked",
        order: 1,
        canDrop: () => ({ accepted: false, reason: "occupied" }),
      }),
    );
    controller.registerKeyboardTarget(
      keyboardTargetReg({
        targetId: "gap",
        order: 2,
        resolvePosition: () => null,
      }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("blocked");
    expect(controller.getSnapshot().targetPosture).toBe("rejected");
    expect(controller.getSnapshot().rejectedReason).toBe("occupied");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBeNull();
    expect(controller.getSnapshot().targetPosture).toBeNull();
    expect(controller.getSnapshot().rejectedReason).toBeUndefined();
    controller.destroy();
  });

  it("clears logical intent when the next resolver returns null", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "keep", order: 1, onDrop }));
    controller.registerKeyboardTarget(
      keyboardTargetReg({
        targetId: "gap",
        order: 2,
        resolvePosition: () => null,
        onDrop,
      }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("keep");
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().session?.intent).toBeNull();
    expect(controller.getSnapshot().targetId).toBeNull();
    root.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(onDrop).not.toHaveBeenCalled();
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "cancelled", reason: "explicit" });
    controller.destroy();
  });

  it("does not commit a logical target disabled before drop", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    const logical = controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "hidden", order: 1, onDrop }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    expect(controller.getSnapshot().targetId).toBe("hidden");
    logical.update(keyboardTargetReg({ targetId: "hidden", order: 1, onDrop, disabled: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(onDrop).not.toHaveBeenCalled();
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "rejected", reason: "target-unavailable" });
    controller.destroy();
  });

  it("rejects an async logical drop when the target is disabled or unregistered before commit", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    const handle = controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "hidden", order: 1, onDrop: () => pending }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("dropping");

    handle.update(keyboardTargetReg({ targetId: "hidden", order: 1, onDrop: () => pending, disabled: true }));
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "rejected", reason: "target-unavailable" });
    expect(controller.getSnapshot().phase).toBe("idle");

    finish({ status: "committed" });
    await pending;
    await Promise.resolve();
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();

    let finishAgain: (value: { status: "committed" }) => void = () => {};
    const pendingAgain = new Promise<{ status: "committed" }>((resolve) => {
      finishAgain = resolve;
    });
    const onEndAgain = vi.fn();
    const again = createDragDropController();
    again.connect(root);
    again.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEndAgain }));
    const live = again.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "hidden", order: 1, onDrop: () => pendingAgain }),
    );
    sourceEl.focus();

    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowDown", bubbles: true }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    live.unregister();
    expect(onEndAgain.mock.calls.at(-1)?.[0]).toEqual({
      status: "rejected",
      reason: "target-unavailable",
    });

    finishAgain({ status: "committed" });
    await pendingAgain;
    await Promise.resolve();
    expect(again.getSnapshot().phase).toBe("idle");
    again.destroy();
  });

  it("requestKeyboardDrop commits through the ordinary keyboard lifecycle", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onStart = vi.fn();
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({ keyboardOrder: 0, onDragStart: onStart, onDragEnd: onEnd }),
    );
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "dst", order: 1, onDrop, label: "List" }));
    sourceEl.focus();

    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "after" })).toBe(true);
    expect(onStart).toHaveBeenCalledTimes(1);
    expect(onDrop).toHaveBeenCalledWith(
      { targetId: "dst", position: "after", operation: "move" },
      { subject: { kind: "item", id: "a" }, inboundFiles: null },
    );
    expect(onEnd).toHaveBeenCalledWith({
      status: "committed",
      intent: { targetId: "dst", position: "after", operation: "move" },
    });
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().announcement).toBe("Dropped Alpha on List");
    expect(document.activeElement).toBe(sourceEl);
    controller.destroy();
  });

  it("requestKeyboardDrop prefers a live logical target over a DOM target with the same id", () => {
    const logicalDrop = vi.fn(() => ({ status: "committed" as const }));
    const domDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0 }));
    controller.registerTarget(targetEl, targetReg({ targetId: "shared", onDrop: domDrop }));
    controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "shared", order: 1, onDrop: logicalDrop, label: "Logical" }),
    );

    expect(
      controller.requestKeyboardDrop({ sourceId: "src", targetId: "shared", position: "before" }),
    ).toBe(true);
    expect(logicalDrop).toHaveBeenCalledTimes(1);
    expect(domDrop).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("requestKeyboardDrop returns false without a session when the registration is missing, disabled, mismatched, or busy", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    controller.registerKeyboardTarget(keyboardTargetReg({ targetId: "dst", order: 1, onDrop }));

    expect(controller.requestKeyboardDrop({ sourceId: "missing", targetId: "dst", position: "after" })).toBe(false);
    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "missing", position: "after" })).toBe(false);

    const disabled = controller.registerSource(
      layout(document.createElement("button"), SOURCE_BOX),
      sourceReg({ sourceId: "off", subject: { kind: "item", id: "off" }, label: "Off", disabled: true }),
    );
    expect(controller.requestKeyboardDrop({ sourceId: "off", targetId: "dst", position: "after" })).toBe(false);
    disabled.unregister();

    controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "other", acceptedKinds: ["file"], order: 2, onDrop }),
    );
    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "other", position: "after" })).toBe(false);

    sourceEl.focus();
    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "after" })).toBe(false);
    controller.cancel();
    expect(onDrop).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("requestKeyboardDrop revalidates canDrop and rejects without invoking onDrop", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    controller.registerKeyboardTarget(
      keyboardTargetReg({
        targetId: "dst",
        order: 1,
        onDrop,
        canDrop: () => ({ accepted: false, reason: "blocked" }),
      }),
    );

    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "after" })).toBe(true);
    expect(onDrop).not.toHaveBeenCalled();
    expect(onEnd).toHaveBeenCalledWith({ status: "rejected", reason: "blocked" });
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("requestKeyboardDrop rejects an async drop when the logical target is disabled or removed before commit", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ keyboardOrder: 0, onDragEnd: onEnd }));
    const handle = controller.registerKeyboardTarget(
      keyboardTargetReg({ targetId: "dst", order: 1, onDrop: () => pending }),
    );

    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "after" })).toBe(true);
    expect(controller.getSnapshot().phase).toBe("dropping");
    handle.update(keyboardTargetReg({ targetId: "dst", order: 1, onDrop: () => pending, disabled: true }));
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "rejected", reason: "target-unavailable" });
    expect(controller.getSnapshot().phase).toBe("idle");

    finish({ status: "committed" });
    await pending;
    await Promise.resolve();
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("requestKeyboardDrop commits a distant DOM-only target with the authored position", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg({ onDrop }));

    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "after" })).toBe(true);
    expect(onDrop).toHaveBeenCalledWith(
      { targetId: "dst", position: "after", operation: "move" },
      { subject: { kind: "item", id: "a" }, inboundFiles: null },
    );
    expect(onEnd).toHaveBeenCalledWith({
      status: "committed",
      intent: { targetId: "dst", position: "after", operation: "move" },
    });
    controller.destroy();
  });

  it("requestKeyboardDrop returns false for a disabled DOM target", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg({ onDrop, disabled: true }));
    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "inside" })).toBe(false);

    const live = layout(document.createElement("div"), TARGET_BOX);
    live.setAttribute("aria-disabled", "true");
    root.append(live);
    controller.registerTarget(live, targetReg({ targetId: "aria", onDrop }));
    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "aria", position: "inside" })).toBe(false);
    expect(onDrop).not.toHaveBeenCalled();
    controller.destroy();
  });

  it("requestKeyboardDrop rejects an async DOM drop when the target is disabled or unregistered before commit", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    const handle = controller.registerTarget(targetEl, targetReg({ onDrop: () => pending }));

    expect(controller.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "inside" })).toBe(true);
    expect(controller.getSnapshot().phase).toBe("dropping");
    handle.update(targetReg({ onDrop: () => pending, disabled: true }));
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "rejected", reason: "target-unavailable" });
    expect(controller.getSnapshot().phase).toBe("idle");

    finish({ status: "committed" });
    await pending;
    await Promise.resolve();
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();

    let finishAgain: (value: { status: "committed" }) => void = () => {};
    const pendingAgain = new Promise<{ status: "committed" }>((resolve) => {
      finishAgain = resolve;
    });
    const onEndAgain = vi.fn();
    const again = createDragDropController();
    again.connect(root);
    again.registerSource(sourceEl, sourceReg({ onDragEnd: onEndAgain }));
    const live = again.registerTarget(targetEl, targetReg({ onDrop: () => pendingAgain }));
    expect(again.requestKeyboardDrop({ sourceId: "src", targetId: "dst", position: "inside" })).toBe(true);
    live.unregister();
    expect(onEndAgain.mock.calls.at(-1)?.[0]).toEqual({
      status: "rejected",
      reason: "target-unavailable",
    });
    finishAgain({ status: "committed" });
    await pendingAgain;
    await Promise.resolve();
    expect(again.getSnapshot().phase).toBe("idle");
    again.destroy();
  });

  it("cancels an active session on Escape", () => {
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 40 }));
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));

    expect(onEnd).toHaveBeenCalledWith({ status: "cancelled", reason: "escape" });
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("cancels when the dragging source unregisters", () => {
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    const handle = controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    handle.unregister();

    expect(onEnd.mock.calls[0]?.[0]).toEqual({ status: "cancelled", reason: "source-lost" });
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("cancels when the current target unregisters during a drag", () => {
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    const handle = controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().targetId).toBe("dst");
    handle.unregister();

    expect(onEnd.mock.calls[0]?.[0]).toEqual({ status: "cancelled", reason: "target-lost" });
    controller.destroy();
  });

  it("ignores a late async drop after the session was cancelled", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    controller.registerTarget(targetEl, targetReg({ onDrop: () => pending }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dropping");

    controller.cancel();
    expect(controller.getSnapshot().phase).toBe("idle");
    const cancelled = onEnd.mock.calls.at(-1)?.[0];

    finish({ status: "committed" });
    await pending;
    await Promise.resolve();

    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual(cancelled);
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("returns an immutable snapshot", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    const snap = controller.getSnapshot();
    expect(() => {
      (snap as { phase: string }).phase = "dragging";
    }).toThrow();
    controller.destroy();
  });

  it("marks a containing ineligible target as rejected", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(
      targetEl,
      targetReg({
        canDrop: () => ({ accepted: false, reason: "occupied" }),
      }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().targetPosture).toBe("rejected");
    expect(controller.getSnapshot().rejectedReason).toBe("occupied");
    expect(targetEl.getAttribute("data-poodle-drop-target")).toBe("rejected");
    controller.destroy();
  });

  it("prefers the deeper nested target through the kernel resolver", () => {
    const nested = layout(document.createElement("div"), NESTED_BOX);
    targetEl.append(nested);
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg({ targetId: "outer", label: "Outer" }));
    controller.registerTarget(
      nested,
      targetReg({ targetId: "inner", label: "Inner", onDrop }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 95 }));
    expect(controller.getSnapshot().session?.intent?.targetId).toBe("inner");
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 95 }));
    expect(onDrop).toHaveBeenCalledTimes(1);
    controller.destroy();
  });

  it("disconnects an injected controller without destroying it", () => {
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    disconnect();
    expect(() => controller.registerTarget(targetEl, targetReg())).not.toThrow();
    const reconnect = controller.connect(root);
    reconnect();
    controller.destroy();
    expect(() => controller.registerSource(sourceEl, sourceReg({ sourceId: "other" }))).toThrow(/destroyed/);
  });

  it("destroy is idempotent and leaves no source attributes", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 20, clientY: 40 }));
    controller.destroy();
    controller.destroy();
    expect(sourceEl.hasAttribute("data-poodle-drag-source")).toBe(false);
    expect(sourceEl.getAttribute("draggable")).toBeNull();
  });

  it("a new gesture remasures after a committed drop moved the elements", () => {
    const controller = createDragDropController();
    controller.connect(root);
    const left = layout(document.createElement("button"), { x: 0, y: 0, width: 100, height: 30 });
    const right = layout(document.createElement("button"), { x: 100, y: 0, width: 100, height: 30 });
    left.textContent = "A";
    right.textContent = "B";
    root.append(left, right);

    const rejectSelf = (id: string): DropTargetRegistration["canDrop"] => (intent, subject) =>
      subject.id === id ? { accepted: false, reason: "same" } : { accepted: true, intent };

    controller.registerSource(left, sourceReg({ sourceId: "src-a", subject: { kind: "item", id: "a" } }));
    controller.registerSource(right, sourceReg({ sourceId: "src-b", subject: { kind: "item", id: "b" } }));
    controller.registerTarget(
      left,
      targetReg({
        targetId: "tgt-a",
        canDrop: rejectSelf("a"),
        resolvePosition: () => "before",
      }),
    );
    controller.registerTarget(
      right,
      targetReg({
        targetId: "tgt-b",
        canDrop: rejectSelf("b"),
        resolvePosition: () => "after",
      }),
    );

    left.dispatchEvent(pointer("pointerdown", { clientX: 50, clientY: 15 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 150, clientY: 15 }));
    expect(controller.getSnapshot().targetId).toBe("tgt-b");
    document.dispatchEvent(pointer("pointerup", { clientX: 150, clientY: 15 }));

    // The drop reordered the row: A now sits where B was.
    layout(left, { x: 100, y: 0, width: 100, height: 30 });
    layout(right, { x: 0, y: 0, width: 100, height: 30 });

    left.dispatchEvent(pointer("pointerdown", { clientX: 150, clientY: 15 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 160, clientY: 15 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().targetId).not.toBe("tgt-b");
    expect(right.getAttribute("data-poodle-drop-target")).toBeNull();
    controller.destroy();
  });

  it("preview position follows the pointer while the target stays the same", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().preview).toEqual(
      expect.objectContaining({ x: 42, y: 32, label: "Alpha" }),
    );

    document.dispatchEvent(pointer("pointermove", { clientX: 70, clientY: 24 }));
    expect(controller.getSnapshot().preview).toEqual(
      expect.objectContaining({ x: 82, y: 36, label: "Alpha" }),
    );
    controller.destroy();
  });

  it("re-hit-tests after invalidateLayout", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 50 }));
    expect(controller.getSnapshot().targetPosture).not.toBe("accepted");

    layout(targetEl, { x: 10, y: 40, width: 80, height: 20 });
    controller.invalidateLayout();
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    controller.destroy();
  });

  it("abandons a pre-activation hold on disconnect, cancel, unregister, disable, visibility, and Escape", () => {
    vi.useFakeTimers();
    const onStart = vi.fn();
    const touch = { activation: { touch: { holdMs: 250, tolerance: 8 } }, onDragStart: onStart };

    function press() {
      const controller = createDragDropController();
      const disconnect = controller.connect(root);
      const handle = controller.registerSource(sourceEl, sourceReg(touch));
      controller.registerTarget(targetEl, targetReg());
      sourceEl.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
      expect(controller.getSnapshot().phase).toBe("idle");
      return { controller, handle, disconnect };
    }

    const disconnected = press();
    disconnected.disconnect();
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    disconnected.controller.destroy();

    const cancelled = press();
    cancelled.controller.cancel();
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    cancelled.controller.destroy();

    const unregistered = press();
    unregistered.handle.unregister();
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    unregistered.controller.destroy();

    const disabled = press();
    disabled.handle.update(sourceReg({ ...touch, disabled: true }));
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    disabled.controller.destroy();

    const hidden = press();
    Object.defineProperty(document, "visibilityState", { configurable: true, get: () => "hidden" });
    document.dispatchEvent(new Event("visibilitychange"));
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    hidden.controller.destroy();
    Object.defineProperty(document, "visibilityState", { configurable: true, get: () => "visible" });

    const escaped = press();
    root.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    vi.advanceTimersByTime(300);
    expect(onStart).not.toHaveBeenCalled();
    expect(escaped.controller.getSnapshot().announcement).toBeNull();
    escaped.controller.destroy();
  });

  it("re-hit-tests the pointer-up coordinates, not a queued move", () => {
    const queued: FrameRequestCallback[] = [];
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      queued.push(callback);
      return queued.length;
    });
    vi.stubGlobal("cancelAnimationFrame", () => {
      queued.length = 0;
    });

    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg({ onDrop }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    queued.splice(0).forEach((callback) => callback(0));
    expect(controller.getSnapshot().phase).toBe("dragging");

    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 50 }));
    expect(queued.length).toBeGreaterThanOrEqual(1);
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();
  });

  it("rejects an async drop when the target is disabled or unregistered before commit", async () => {
    let finish: (value: { status: "committed" }) => void = () => {};
    const pending = new Promise<{ status: "committed" }>((resolve) => {
      finish = resolve;
    });
    const onEnd = vi.fn();
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    const handle = controller.registerTarget(targetEl, targetReg({ onDrop: () => pending }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dropping");

    handle.update(targetReg({ onDrop: () => pending, disabled: true }));
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({ status: "rejected", reason: "target-unavailable" });
    expect(controller.getSnapshot().phase).toBe("idle");

    finish({ status: "committed" });
    await pending;
    await Promise.resolve();
    expect(controller.getSnapshot().phase).toBe("idle");
    controller.destroy();

    let finishAgain: (value: { status: "committed" }) => void = () => {};
    const pendingAgain = new Promise<{ status: "committed" }>((resolve) => {
      finishAgain = resolve;
    });
    const onEndAgain = vi.fn();
    const again = createDragDropController();
    again.connect(root);
    again.registerSource(sourceEl, sourceReg({ onDragEnd: onEndAgain }));
    const live = again.registerTarget(targetEl, targetReg({ onDrop: () => pendingAgain }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    live.unregister();
    expect(onEndAgain.mock.calls.at(-1)?.[0]).toEqual({
      status: "rejected",
      reason: "target-unavailable",
    });

    finishAgain({ status: "committed" });
    await pendingAgain;
    await Promise.resolve();
    expect(again.getSnapshot().phase).toBe("idle");
    again.destroy();
  });

  it("publishes throttled intent and rejection announcements into the snapshot", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "Date"] });
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(
      targetEl,
      targetReg({ canDrop: () => ({ accepted: false, reason: "occupied" }) }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(controller.getSnapshot().announcement).toBe("Picked up Alpha");

    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().targetPosture).toBe("rejected");
    expect(controller.getSnapshot().announcement).toBe("Picked up Alpha");

    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().announcement).toBe("Drop rejected: occupied");
    controller.destroy();
  });

  /**
   * g16.028. A source that narrates its own sessions silences the controller
   * for the *whole* session, and only for its own.
   *
   * The guarantee is a latch, not a lookup: it is read once when the session
   * begins. A lookup would answer differently after the source re-registers
   * mid-drag or unregisters at a terminal, which is exactly when a late
   * announcement lands.
   */
  it("silences every announcement of a self-narrating session, and only that session", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "Date"] });
    const controller = createDragDropController();
    controller.connect(root);
    const handle = controller.registerSource(sourceEl, sourceReg({ ownsAnnouncements: true }));
    controller.registerTarget(targetEl, targetReg());

    // Pickup, hover intent, and the throttled flush all stay quiet.
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(controller.getSnapshot().announcement).toBeNull();

    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().announcement).toBeNull();

    // A rebuild that re-registers the same source *without* the flag does not
    // hand the live session back to the controller. This is the case a live
    // registration lookup gets wrong: it would narrate the rest of a session
    // the component is already narrating, mid-drag.
    handle.update(sourceReg({ ownsAnnouncements: false }));
    document.dispatchEvent(pointer("pointermove", { clientX: 32, clientY: 92 }));
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().announcement).toBeNull();

    document.dispatchEvent(pointer("pointerup", { clientX: 32, clientY: 92 }));
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().announcement).toBeNull();
    handle.unregister();

    // The next ordinary session is narrated again: the latch is per session,
    // not a mode the controller falls into.
    controller.registerSource(sourceEl, sourceReg());
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(controller.getSnapshot().announcement).toBe("Picked up Alpha");
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().announcement).toBe("Dropped Alpha on List");
    controller.destroy();
  });

  /** A cancelled self-narrated session is silent too — cancel is a terminal. */
  it("silences a cancelled self-narrating session", () => {
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ ownsAnnouncements: true }));
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().announcement).toBeNull();
    controller.destroy();
  });

  it("restores authored source attributes and ignores post-destroy handles", () => {
    sourceEl.setAttribute("tabindex", "2");
    sourceEl.setAttribute("aria-label", "Mine");
    sourceEl.setAttribute("aria-description", "Hint");
    sourceEl.setAttribute("draggable", "true");
    document.body.style.setProperty("user-select", "text");
    root.style.setProperty("user-select", "auto");

    const controller = createDragDropController();
    controller.connect(root);
    const handle = controller.registerSource(sourceEl, sourceReg({ instructions: "Drag me" }));
    controller.registerTarget(targetEl, targetReg());
    expect(sourceEl.getAttribute("draggable")).toBe("false");
    expect(sourceEl.getAttribute("aria-description")).toBe("Drag me");

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(root.style.getPropertyValue("user-select")).toBe("none");
    expect(document.body.style.getPropertyValue("user-select")).toBe("text");

    handle.unregister();
    expect(sourceEl.getAttribute("tabindex")).toBe("2");
    expect(sourceEl.getAttribute("aria-label")).toBe("Mine");
    expect(sourceEl.getAttribute("aria-description")).toBe("Hint");
    expect(sourceEl.getAttribute("draggable")).toBe("true");
    expect(root.style.getPropertyValue("user-select")).toBe("auto");
    expect(document.body.style.getPropertyValue("user-select")).toBe("text");

    const again = controller.registerSource(sourceEl, sourceReg());
    controller.destroy();
    expect(sourceEl.getAttribute("tabindex")).toBe("2");
    again.update(sourceReg({ label: "Mutated" }));
    again.unregister();
    expect(sourceEl.getAttribute("aria-label")).toBe("Mine");
    expect(sourceEl.getAttribute("draggable")).toBe("true");
    document.body.style.removeProperty("user-select");
    root.style.removeProperty("user-select");
  });

  it("treats source and target ids as immutable on a live handle", () => {
    const onEnd = vi.fn();
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    const sourceHandle = controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    const targetHandle = controller.registerTarget(targetEl, targetReg({ onDrop }));

    expect(() => sourceHandle.update(sourceReg({ sourceId: "renamed", onDragEnd: onEnd }))).toThrow(
      /immutable/,
    );
    expect(() => targetHandle.update(targetReg({ targetId: "renamed", onDrop }))).toThrow(/immutable/);

    vi.useFakeTimers();
    sourceEl.dispatchEvent(pointer("pointerdown", { pointerType: "touch", clientX: 20, clientY: 20 }));
    sourceHandle.unregister();
    const afterCandidate = controller.registerSource(sourceEl, sourceReg({ sourceId: "renamed", onDragEnd: onEnd }));
    vi.advanceTimersByTime(300);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(onEnd).not.toHaveBeenCalled();
    afterCandidate.unregister();
    vi.useRealTimers();
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });

    const active = controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd }));
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().preview?.label).toBe("Alpha");
    active.unregister();
    expect(onEnd).toHaveBeenCalledWith({ status: "cancelled", reason: "source-lost" });
    expect(controller.getSnapshot().phase).toBe("idle");

    const keyboard = controller.registerSource(
      sourceEl,
      sourceReg({ onDragEnd: onEnd, label: "Keys", keyboardOrder: 0 }),
    );
    sourceEl.focus();
    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    keyboard.unregister();
    expect(onEnd).toHaveBeenCalledWith({ status: "cancelled", reason: "source-lost" });

    const next = controller.registerSource(sourceEl, sourceReg({ sourceId: "renamed", label: "Beta", onDragEnd: onEnd }));
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 90 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 30, clientY: 90 }));
    expect(onDrop).toHaveBeenCalledTimes(1);
    expect(onEnd.mock.calls.at(-1)?.[0]).toEqual({
      status: "committed",
      intent: { targetId: "dst", position: "inside", operation: "move" },
    });
    next.unregister();
    controller.destroy();
  });

  it("keeps overlapping controllers from clobbering user-select", () => {
    document.body.style.setProperty("user-select", "text");
    const rootB = document.createElement("div");
    const sourceB = layout(document.createElement("button"), SOURCE_BOX);
    sourceB.textContent = "Beta";
    const targetB = layout(document.createElement("div"), TARGET_BOX);
    rootB.append(sourceB, targetB);
    document.body.append(rootB);

    const first = createDragDropController();
    const second = createDragDropController();
    first.connect(root);
    second.connect(rootB);
    first.registerSource(sourceEl, sourceReg());
    first.registerTarget(targetEl, targetReg());
    second.registerSource(sourceB, sourceReg({ sourceId: "src-b", label: "Beta" }));
    second.registerTarget(targetB, targetReg({ targetId: "dst-b" }));

    sourceEl.dispatchEvent(pointer("pointerdown", { pointerId: 1, clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointermove", { pointerId: 1, clientX: 30, clientY: 20 }));
    expect(root.style.getPropertyValue("user-select")).toBe("none");
    expect(rootB.style.getPropertyValue("user-select")).toBe("");
    expect(document.body.style.getPropertyValue("user-select")).toBe("text");

    sourceB.dispatchEvent(pointer("pointerdown", { pointerId: 2, clientX: 20, clientY: 20 }));
    sourceB.dispatchEvent(pointer("pointermove", { pointerId: 2, clientX: 30, clientY: 20 }));
    expect(root.style.getPropertyValue("user-select")).toBe("none");
    expect(rootB.style.getPropertyValue("user-select")).toBe("none");

    document.dispatchEvent(pointer("pointerup", { pointerId: 1, clientX: 30, clientY: 20 }));
    expect(first.getSnapshot().phase).toBe("idle");
    expect(second.getSnapshot().phase).toBe("dragging");
    expect(root.style.getPropertyValue("user-select")).toBe("");
    expect(rootB.style.getPropertyValue("user-select")).toBe("none");
    expect(document.body.style.getPropertyValue("user-select")).toBe("text");

    sourceB.dispatchEvent(pointer("pointerup", { pointerId: 2, clientX: 30, clientY: 20 }));
    expect(second.getSnapshot().phase).toBe("idle");
    expect(rootB.style.getPropertyValue("user-select")).toBe("");
    expect(document.body.style.getPropertyValue("user-select")).toBe("text");

    first.destroy();
    second.destroy();
    document.body.style.removeProperty("user-select");
  });

  it("captures an SVG source after activation", () => {
    const svg = layout(
      document.createElementNS("http://www.w3.org/2000/svg", "rect"),
      SOURCE_BOX,
    );
    root.replaceChildren(svg, targetEl);
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(svg, sourceReg({ label: "Shape" }));
    controller.registerTarget(targetEl, targetReg());

    svg.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    svg.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect((svg as SVGRectElement).setPointerCapture).toHaveBeenCalledWith(1);
    controller.destroy();
  });

  it("re-hit-tests when ResizeObserver reports a target size change", () => {
    let notify: ResizeObserverCallback | null = null;
    class MockObserver {
      constructor(callback: ResizeObserverCallback) {
        notify = callback;
      }
      observe(): void {}
      unobserve(): void {}
      disconnect(): void {}
    }
    vi.stubGlobal("ResizeObserver", MockObserver);

    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 30, clientY: 50 }));
    expect(controller.getSnapshot().targetPosture).not.toBe("accepted");

    layout(targetEl, { x: 10, y: 40, width: 80, height: 20 });
    notify?.([], {} as ResizeObserver);
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    controller.destroy();
  });

  it("auto-scrolls the nearest nested container and stops on cancel", () => {
    const frames: FrameRequestCallback[] = [];
    let now = 1000;
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      frames.push(callback);
      return frames.length;
    });
    vi.stubGlobal("cancelAnimationFrame", () => {
      frames.length = 0;
    });

    function scroller(
      element: HTMLElement,
      box: { x: number; y: number; width: number; height: number },
      values: { scrollTop: number; scrollHeight: number; clientHeight: number },
    ): HTMLElement {
      element.style.overflow = "auto";
      layout(element, box);
      Object.defineProperty(element, "scrollTop", { configurable: true, writable: true, value: values.scrollTop });
      Object.defineProperty(element, "scrollHeight", { configurable: true, get: () => values.scrollHeight });
      Object.defineProperty(element, "clientHeight", { configurable: true, get: () => values.clientHeight });
      return element;
    }

    const outerState = { scrollTop: 40, scrollHeight: 400, clientHeight: 100 };
    const innerState = { scrollTop: 40, scrollHeight: 300, clientHeight: 80 };
    const outer = scroller(document.createElement("div"), { x: 0, y: 0, width: 200, height: 100 }, outerState);
    const inner = scroller(document.createElement("div"), { x: 10, y: 10, width: 180, height: 80 }, innerState);
    Object.defineProperty(outer, "scrollTop", {
      configurable: true,
      get: () => outerState.scrollTop,
      set: (value: number) => {
        outerState.scrollTop = value;
      },
    });
    Object.defineProperty(inner, "scrollTop", {
      configurable: true,
      get: () => innerState.scrollTop,
      set: (value: number) => {
        innerState.scrollTop = value;
      },
    });
    inner.append(targetEl);
    outer.append(inner);
    root.append(outer);
    layout(targetEl, { x: 20, y: 20, width: 80, height: 20 });

    const originalFromPoint = document.elementFromPoint.bind(document);
    document.elementFromPoint = () => inner;

    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg({ autoScroll: true }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 14 }));
    frames.splice(0).forEach((frame) => frame(now));
    expect(controller.getSnapshot().phase).toBe("dragging");

    const innerBefore = innerState.scrollTop;
    const outerBefore = outerState.scrollTop;
    now += 16;
    frames.splice(0).forEach((frame) => frame(now));
    expect(innerState.scrollTop).toBeLessThan(innerBefore);
    expect(outerState.scrollTop).toBe(outerBefore);

    innerState.scrollTop = 0;
    innerState.scrollHeight = 80;
    now += 16;
    frames.splice(0).forEach((frame) => frame(now));
    expect(outerState.scrollTop).toBeLessThan(outerBefore);

    const outerAfterInnerExhausted = outerState.scrollTop;
    controller.cancel();
    now += 16;
    frames.splice(0).forEach((frame) => frame(now));
    expect(outerState.scrollTop).toBe(outerAfterInnerExhausted);
    expect(controller.getSnapshot().phase).toBe("idle");

    document.elementFromPoint = originalFromPoint;
    controller.destroy();
  });

  it("stops the auto-scroll frame on leave and exhaustion, then restarts on re-entry", () => {
    const frames = new Map<number, FrameRequestCallback>();
    let nextId = 1;
    let now = 1000;
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      const id = nextId++;
      frames.set(id, callback);
      return id;
    });
    vi.stubGlobal("cancelAnimationFrame", (id: number) => {
      frames.delete(id);
    });
    function flush(): void {
      const pending = [...frames.values()];
      frames.clear();
      pending.forEach((frame) => frame(now));
    }
    function drain(): void {
      for (let i = 0; i < 8 && frames.size > 0; i += 1) {
        now += 16;
        flush();
      }
    }

    const innerState = { scrollTop: 80, scrollHeight: 300, clientHeight: 80 };
    const inner = document.createElement("div");
    inner.style.overflow = "auto";
    layout(inner, { x: 10, y: 10, width: 180, height: 80 });
    Object.defineProperty(inner, "scrollTop", {
      configurable: true,
      get: () => innerState.scrollTop,
      set: (value: number) => {
        innerState.scrollTop = value;
      },
    });
    Object.defineProperty(inner, "scrollHeight", { configurable: true, get: () => innerState.scrollHeight });
    Object.defineProperty(inner, "clientHeight", { configurable: true, get: () => innerState.clientHeight });
    inner.append(targetEl);
    root.append(inner);
    layout(targetEl, { x: 20, y: 20, width: 80, height: 20 });

    const originalFromPoint = document.elementFromPoint.bind(document);
    document.elementFromPoint = () => inner;

    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg({ autoScroll: true }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 50 }));
    flush();
    expect(controller.getSnapshot().phase).toBe("dragging");
    now += 16;
    flush();
    expect(frames.size).toBe(0);

    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 14 }));
    now += 16;
    flush();
    now += 16;
    flush();
    expect(innerState.scrollTop).toBeLessThan(80);
    expect(frames.size).toBeGreaterThan(0);

    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 50 }));
    drain();
    expect(frames.size).toBe(0);
    const afterLeave = innerState.scrollTop;
    now += 16;
    flush();
    expect(frames.size).toBe(0);
    expect(innerState.scrollTop).toBe(afterLeave);

    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 14 }));
    now += 16;
    flush();
    now += 16;
    flush();
    expect(innerState.scrollTop).toBeLessThan(afterLeave);
    expect(frames.size).toBeGreaterThan(0);

    innerState.scrollTop = 0;
    drain();
    expect(frames.size).toBe(0);
    const exhausted = innerState.scrollTop;
    now += 16;
    flush();
    expect(frames.size).toBe(0);
    expect(innerState.scrollTop).toBe(exhausted);

    document.elementFromPoint = originalFromPoint;
    controller.destroy();
  });
});

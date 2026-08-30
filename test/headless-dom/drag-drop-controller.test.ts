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

  it("picks up from the keyboard, moves intent, and drops", () => {
    const onDrop = vi.fn(() => ({ status: "committed" as const }));
    const controller = createDragDropController();
    controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
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
    expect(queued.length).toBe(1);
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

    const keyboard = controller.registerSource(sourceEl, sourceReg({ onDragEnd: onEnd, label: "Keys" }));
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
});

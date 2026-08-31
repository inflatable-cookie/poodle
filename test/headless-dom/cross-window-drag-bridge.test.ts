/**
 * Cross-window host bridge — the adversarial matrix (g16.026).
 *
 * The host is simulated rather than mocked away: one object plays the source
 * bridge in the sending surface and the target bridge in the receiving one,
 * holding a single transaction between them exactly as a real shell would.
 * That is what makes the hostile cases expressible — a preparation that
 * resolves after supersession, a native end that reports `move` before the
 * host refuses, a target that disappears between projection and drop, a
 * receipt that is not the one this window is projecting.
 *
 * Two controllers on two roots stand in for two windows. It is a bounded
 * substitute: it proves session ownership, revalidation, and terminal
 * accounting, and it deliberately does not claim to prove real cross-context
 * transport. That claim belongs to the headless Chromium/WebKit multi-context
 * probe, which drives two real browser contexts.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  createDragDropController,
  CROSS_WINDOW_DRAG_MIME_TYPE,
  CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
  type CrossWindowDragCapabilities,
  type CrossWindowDragProjection,
  type CrossWindowDragReceipt,
  type CrossWindowDragSourceBridge,
  type CrossWindowDragTargetBridge,
  type CrossWindowDragTargetEvent,
  type DragDropCommitResult,
  type DragSourceRegistration,
  type DragTerminalOutcome,
  type DropTargetRegistration,
} from "../../packages/core/src";

const SOURCE_BOX = { x: 10, y: 10, width: 80, height: 20 };
const TARGET_BOX = { x: 10, y: 80, width: 80, height: 20 };

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

function pointer(type: string, init: PointerEventInit): PointerEvent {
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

/** happy-dom drops `dataTransfer` from the event init, so it is attached here. */
function drag(type: string, dataTransfer: DataTransfer, init: MouseEventInit = {}): DragEvent {
  const event = new DragEvent(type, { bubbles: true, cancelable: true, ...init });
  Object.defineProperty(event, "dataTransfer", { value: dataTransfer, configurable: true });
  return event;
}

const POINTER_HOST: CrossWindowDragCapabilities = {
  pointer: true,
  touch: false,
  keyboardTargetPicker: false,
};

interface HostOptions {
  readonly capabilities?: CrossWindowDragCapabilities;
  /** Resolve preparation later so supersession and late completion are testable. */
  readonly deferPrepare?: boolean;
  readonly decline?: boolean;
  readonly commit?: DragDropCommitResult;
}

/**
 * One host, both halves of the bridge.
 *
 * It hands out a receipt, remembers what it issued, projects into the
 * receiving surface on demand, and records every command Poodle sends it. The
 * recording is the point: most of these cases are claims about what Poodle
 * did *not* do.
 */
function createHost(options: HostOptions = {}) {
  const capabilities = options.capabilities ?? POINTER_HOST;
  const commits: Array<{ receipt: CrossWindowDragReceipt; intent: unknown }> = [];
  const cancels: Array<{ token: string; reason: string }> = [];
  const starts: Array<{ token: string; transport: string }> = [];
  const stops: string[] = [];
  const prepares: string[] = [];
  const pending: Array<(receipt: CrossWindowDragReceipt | null) => void> = [];
  let listener: ((event: CrossWindowDragTargetEvent) => void) | null = null;
  let terminal: ((outcome: DragTerminalOutcome) => void) | null = null;
  let issued = 0;
  let pickResult: CrossWindowDragProjection | null = null;

  const source: CrossWindowDragSourceBridge = {
    capabilities,
    prepare(request) {
      prepares.push(request.sessionId);
      issued += 1;
      const receipt: CrossWindowDragReceipt = {
        protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
        token: `lease-${issued}`,
      };
      if (options.decline) return Promise.resolve(null);
      if (!options.deferPrepare) return Promise.resolve(receipt);
      return new Promise((resolve) => {
        pending.push(() => resolve(receipt));
      });
    },
    start(receipt, transport, onTerminal) {
      starts.push({ token: receipt.token, transport });
      terminal = onTerminal;
      return () => {
        stops.push(receipt.token);
        terminal = null;
      };
    },
    cancel(receipt, reason) {
      cancels.push({ token: receipt.token, reason });
    },
  };

  const target: CrossWindowDragTargetBridge = {
    capabilities,
    subscribe(next) {
      listener = next;
      return () => {
        listener = null;
      };
    },
    commit(request) {
      commits.push({ receipt: request.receipt, intent: request.intent });
      return Promise.resolve(options.commit ?? { status: "committed" });
    },
    ...(capabilities.keyboardTargetPicker
      ? { pickTarget: () => Promise.resolve(pickResult) }
      : {}),
  };

  return {
    source,
    target,
    commits,
    cancels,
    starts,
    stops,
    prepares,
    get lastReceipt(): CrossWindowDragReceipt {
      return { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: `lease-${issued}` };
    },
    receiptFor(index: number): CrossWindowDragReceipt {
      return { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: `lease-${index}` };
    },
    settlePrepare(index = 0) {
      pending[index]?.({ protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "" });
    },
    setPick(projection: CrossWindowDragProjection | null) {
      pickResult = projection;
    },
    project(projection: Partial<CrossWindowDragProjection> & { receipt: CrossWindowDragReceipt }) {
      listener?.({
        type: "projection",
        projection: {
          sourceId: "src",
          sourceLabel: "Alpha",
          subject: { kind: "item", id: "a" },
          operation: "move",
          inputKind: "pointer",
          targetId: "dst",
          position: "inside",
          ...projection,
        },
      });
    },
    left(receipt: CrossWindowDragReceipt) {
      listener?.({ type: "left", receipt });
    },
    cancelledFromHost(receipt: CrossWindowDragReceipt, reason: "window-lost" | "transport-lost") {
      listener?.({ type: "cancelled", receipt, reason });
    },
    reportTerminal(outcome: DragTerminalOutcome) {
      terminal?.(outcome);
    },
    get subscribed(): boolean {
      return listener !== null;
    },
  };
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

async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

function envelope(token: string): DataTransfer {
  const dataTransfer = new DataTransfer();
  dataTransfer.setData(
    CROSS_WINDOW_DRAG_MIME_TYPE,
    JSON.stringify({ protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token }),
  );
  return dataTransfer;
}

describe("cross-window drag bridge", () => {
  let sendingRoot: HTMLElement;
  let receivingRoot: HTMLElement;
  let sourceEl: HTMLElement;
  let targetEl: HTMLElement;

  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    sendingRoot = document.createElement("div");
    receivingRoot = document.createElement("div");
    sourceEl = layout(document.createElement("button"), SOURCE_BOX);
    sourceEl.textContent = "Alpha";
    targetEl = layout(document.createElement("div"), TARGET_BOX);
    targetEl.textContent = "List";
    sendingRoot.append(sourceEl);
    receivingRoot.append(targetEl);
    document.body.append(sendingRoot, receivingRoot);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  // ── Source ───────────────────────────────────────────────────────────────

  it("prepares on the pre-drag gesture and refuses the native drag until the receipt is armed", async () => {
    const host = createHost({ deferPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(sourceEl, sourceReg({ crossWindowSourceBridge: host.source }));

    // Pointer-down alone: the host has already been asked, and the session is
    // waiting on it rather than on the pointer.
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    expect(host.prepares).toHaveLength(1);
    expect(controller.getSnapshot().phase).toBe("preparing");

    // Past the activation distance, still unarmed: no native drag may start.
    document.dispatchEvent(pointer("pointermove", { clientX: 60, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("preparing");
    const refused = drag("dragstart", new DataTransfer());
    sourceEl.dispatchEvent(refused);
    expect(refused.defaultPrevented).toBe(true);
    expect(host.starts).toHaveLength(0);

    disconnect();
    controller.destroy();
  });

  it("activates on the browser's own dragstart, writes only the receipt, and starts one host subscription", async () => {
    const host = createHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(sourceEl, sourceReg({ crossWindowSourceBridge: host.source }));

    expect(sourceEl.getAttribute("draggable")).toBe("true");

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    expect(controller.getSnapshot().phase).toBe("armed");

    const dataTransfer = new DataTransfer();
    const start = drag("dragstart", dataTransfer);
    sourceEl.dispatchEvent(start);

    expect(start.defaultPrevented).toBe(false);
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(host.starts).toEqual([{ token: "lease-1", transport: "data-transfer" }]);
    expect([...dataTransfer.types]).toEqual([CROSS_WINDOW_DRAG_MIME_TYPE]);
    expect(JSON.parse(dataTransfer.getData(CROSS_WINDOW_DRAG_MIME_TYPE))).toEqual({
      protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
      token: "lease-1",
    });

    disconnect();
    controller.destroy();
  });

  it("ends rejected from the host result when a native end reports a move", async () => {
    const host = createHost();
    const outcomes: DragTerminalOutcome[] = [];
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(
      sourceEl,
      sourceReg({
        crossWindowSourceBridge: host.source,
        onDragEnd: (outcome) => outcomes.push(outcome),
      }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    const dataTransfer = new DataTransfer();
    sourceEl.dispatchEvent(drag("dragstart", dataTransfer));

    // The OS says the move happened. It is not a commit, and nothing about the
    // session may move because of it.
    dataTransfer.dropEffect = "move";
    sourceEl.dispatchEvent(drag("dragend", dataTransfer));
    expect(outcomes).toHaveLength(0);
    expect(controller.getSnapshot().phase).toBe("dragging");

    host.reportTerminal({ status: "rejected", reason: "lease expired" });
    expect(outcomes).toEqual([{ status: "rejected", reason: "lease expired" }]);
    expect(controller.getSnapshot().phase).toBe("idle");

    // The host closed its own transaction, so Poodle does not also cancel it.
    expect(host.cancels).toEqual([]);
    expect(host.stops).toEqual(["lease-1"]);

    // A repeat is inert.
    host.reportTerminal({ status: "committed", intent: { targetId: "dst", position: "inside", operation: "move" } });
    expect(outcomes).toHaveLength(1);

    disconnect();
    controller.destroy();
  });

  it("cancels a superseded preparation once and never arms its successor", async () => {
    const host = createHost({ deferPrepare: true });
    const outcomes: DragTerminalOutcome[] = [];
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(
      sourceEl,
      sourceReg({
        crossWindowSourceBridge: host.source,
        onDragEnd: (outcome) => outcomes.push(outcome),
      }),
    );

    // A is prepared, then abandoned before it ever arms; B starts fresh.
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 20, clientY: 20 }));
    expect(outcomes).toEqual([{ status: "cancelled", reason: "explicit" }]);

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    expect(host.prepares).toHaveLength(2);

    // A's host answer arrives late. It cannot arm B, and the lease it created
    // is handed straight back.
    host.settlePrepare(0);
    await settle();
    expect(controller.getSnapshot().session?.sessionId).not.toBe(host.prepares[0]);
    expect(outcomes).toHaveLength(1);

    disconnect();
    controller.destroy();
  });

  it("keeps local reorder working when the host declines the transfer", async () => {
    const host = createHost({ decline: true });
    const drops: unknown[] = [];
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    sendingRoot.append(targetEl);
    controller.registerSource(sourceEl, sourceReg({ crossWindowSourceBridge: host.source }));
    controller.registerTarget(
      targetEl,
      targetReg({ onDrop: (intent) => (drops.push(intent), { status: "committed" }) }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();

    // The cross-window attempt is dead; the gesture the user is still making
    // is not.
    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    document.dispatchEvent(pointer("pointerup", { clientX: 40, clientY: 90 }));

    expect(drops).toEqual([{ targetId: "dst", position: "inside", operation: "move" }]);
    expect(host.starts).toEqual([]);

    disconnect();
    controller.destroy();
  });

  it("never starts a cross-window touch gesture the host cannot observe", async () => {
    vi.useFakeTimers();
    const host = createHost({
      capabilities: { pointer: true, touch: false, keyboardTargetPicker: false },
    });
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(sourceEl, sourceReg({ crossWindowSourceBridge: host.source }));

    sourceEl.dispatchEvent(
      pointer("pointerdown", { clientX: 20, clientY: 20, pointerType: "touch" }),
    );
    expect(host.prepares).toHaveLength(0);

    // Internal touch is untouched: the hold still arms the ordinary local
    // session, it simply carries no host payload.
    vi.advanceTimersByTime(400);
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(host.starts).toEqual([]);

    disconnect();
    controller.destroy();
  });

  it("releases a live receipt exactly once when the window is lost", async () => {
    const host = createHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(sendingRoot);
    controller.registerSource(sourceEl, sourceReg({ crossWindowSourceBridge: host.source }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));

    Object.defineProperty(document, "visibilityState", { value: "hidden", configurable: true });
    document.dispatchEvent(new Event("visibilitychange"));

    expect(host.cancels).toEqual([{ token: "lease-1", reason: "window-lost" }]);
    expect(host.stops).toEqual(["lease-1"]);
    expect(controller.getSnapshot().phase).toBe("idle");

    // Late host chatter after the terminal changes nothing.
    host.reportTerminal({ status: "committed", intent: { targetId: "dst", position: "inside", operation: "move" } });
    expect(host.cancels).toHaveLength(1);

    Object.defineProperty(document, "visibilityState", { value: "visible", configurable: true });
    disconnect();
    controller.destroy();
  });

  // ── Target ───────────────────────────────────────────────────────────────

  it("projects a host target through this window's own gates and commits once", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(targetEl, targetReg());

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.project({ receipt });

    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    expect(targetEl.getAttribute("data-poodle-drop-target")).toBe("accepted");

    const dataTransfer = envelope("lease-1");
    const over = drag("dragover", dataTransfer, { clientX: 40, clientY: 90 });
    targetEl.dispatchEvent(over);
    expect(over.defaultPrevented).toBe(true);

    targetEl.dispatchEvent(drag("drop", dataTransfer, { clientX: 40, clientY: 90 }));
    await settle();

    expect(host.commits).toHaveLength(1);
    expect(host.commits[0]?.intent).toEqual({
      targetId: "dst",
      position: "inside",
      operation: "move",
    });
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(targetEl.getAttribute("data-poodle-drop-target")).toBeNull();

    disconnect();
    controller.destroy();
  });

  it("never commits a target the host projected but this window no longer accepts", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    const handle = controller.registerTarget(targetEl, targetReg());

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.project({ receipt });
    expect(controller.getSnapshot().targetPosture).toBe("accepted");

    // The consumer disables the target between hover and drop. Hover
    // acceptance does not carry over.
    handle.update(targetReg({ disabled: true }));
    targetEl.dispatchEvent(drag("drop", envelope("lease-1"), { clientX: 40, clientY: 90 }));
    await settle();

    expect(host.commits).toEqual([]);
    expect(controller.getSnapshot().phase).toBe("idle");

    disconnect();
    controller.destroy();
  });

  it("refuses a drop envelope that is not the receipt it is projecting", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(targetEl, targetReg());

    host.project({ receipt: { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" } });
    expect(controller.getSnapshot().targetPosture).toBe("accepted");

    targetEl.dispatchEvent(drag("drop", envelope("someone-elses-lease"), { clientX: 40, clientY: 90 }));
    await settle();

    expect(host.commits).toEqual([]);
    expect(controller.getSnapshot().targetPosture).toBeNull();

    // The valid envelope still works afterwards; the refusal was about the
    // receipt, not about the window.
    host.project({ receipt: { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" } });
    targetEl.dispatchEvent(drag("drop", envelope("lease-1"), { clientX: 40, clientY: 90 }));
    await settle();
    expect(host.commits).toHaveLength(1);

    disconnect();
    controller.destroy();
  });

  it("follows host geometry with no local pointer input, and a stale position cannot commit", async () => {
    const host = createHost();
    const seen: string[] = [];
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(
      targetEl,
      targetReg({
        canDrop: (intent) =>
          intent.position === "after"
            ? { accepted: false, reason: "moved away" }
            : { accepted: true, intent },
        onDrop: (intent) => (seen.push(intent.position), { status: "committed" }),
      }),
    );

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.project({ receipt, position: "before" });
    expect(controller.getSnapshot().session?.intent?.position).toBe("before");

    // The host moved its own target. No pointer event happened in this window.
    host.project({ receipt, position: "after" });
    expect(controller.getSnapshot().targetPosture).toBe("rejected");
    expect(controller.getSnapshot().session?.intent).toBeNull();

    targetEl.dispatchEvent(drag("drop", envelope("lease-1"), { clientX: 40, clientY: 90 }));
    await settle();
    expect(host.commits).toEqual([]);
    expect(seen).toEqual([]);

    disconnect();
    controller.destroy();
  });

  it("ends the receiving session once when the host cancels it", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(targetEl, targetReg());

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.project({ receipt });
    host.cancelledFromHost(receipt, "window-lost");

    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().targetPosture).toBeNull();

    // A second cancellation for a session that is already gone is inert.
    host.cancelledFromHost(receipt, "window-lost");
    expect(controller.getSnapshot().phase).toBe("idle");

    disconnect();
    controller.destroy();
  });

  it("clears the projected intent when the gesture leaves the window and keeps the session", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(targetEl, targetReg());

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.project({ receipt });
    host.left(receipt);

    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().session?.intent).toBeNull();

    host.project({ receipt });
    expect(controller.getSnapshot().session?.intent?.targetId).toBe("dst");

    disconnect();
    controller.destroy();
  });

  it("takes the keyboard picker's answer through the same revalidation and refuses a stale one", async () => {
    const host = createHost({
      capabilities: { pointer: true, touch: false, keyboardTargetPicker: true },
    });
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    controller.registerTarget(
      targetEl,
      targetReg({
        canDrop: (intent) =>
          intent.targetId === "gone"
            ? { accepted: false, reason: "stale" }
            : { accepted: true, intent },
      }),
    );

    const receipt = { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" };
    host.setPick({
      receipt,
      sourceId: "src",
      sourceLabel: "Alpha",
      subject: { kind: "item", id: "a" },
      operation: "move",
      inputKind: "keyboard",
      targetId: "gone",
      position: "inside",
    });
    host.project({ receipt, inputKind: "keyboard", targetId: null, position: null });
    await settle();

    // The picker named a target this window does not have. Ordinary
    // revalidation refuses it; no second callback path appears.
    expect(host.commits).toEqual([]);
    expect(controller.getSnapshot().session?.intent).toBeNull();

    disconnect();
    controller.destroy();
  });

  it("refuses to install a bridge that claims a keyboard picker it does not implement", () => {
    const controller = createDragDropController({
      crossWindowTargetBridge: {
        capabilities: { pointer: false, touch: false, keyboardTargetPicker: true },
        subscribe: () => () => {},
        commit: () => Promise.resolve({ status: "committed" }),
      },
    });

    expect(() => controller.connect(receivingRoot)).toThrow(/pickTarget/);
    controller.destroy();
  });

  it("does not let a projection hijack a local gesture already in progress", async () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    receivingRoot.append(sourceEl);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, targetReg());

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 90 }));
    const local = controller.getSnapshot().session?.sessionId;
    expect(controller.getSnapshot().phase).toBe("dragging");

    host.project({ receipt: { protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION, token: "lease-1" } });

    expect(controller.getSnapshot().session?.sessionId).toBe(local);
    expect(controller.getSnapshot().sourceId).toBe("src");

    disconnect();
    controller.destroy();
  });

  it("unsubscribes from the host when the controller disconnects", () => {
    const host = createHost();
    const controller = createDragDropController({ crossWindowTargetBridge: host.target });
    const disconnect = controller.connect(receivingRoot);
    expect(host.subscribed).toBe(true);
    disconnect();
    expect(host.subscribed).toBe(false);
    controller.destroy();
  });
});

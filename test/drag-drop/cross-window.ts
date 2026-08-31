/**
 * Cross-window host bridge fixture — one page, one role.
 *
 * The host is deliberately *outside* the page. A real shell is a process that
 * owns both windows and neither page can see the other, so the simulator that
 * proves this boundary has to sit in the same place: the probe process holds
 * the transaction and drives `window.__poodleHost` in each context. Nothing
 * here talks to the other window, and nothing here can, which is exactly the
 * property under test.
 *
 * `?role=source` arms and starts a transfer. `?role=target` receives
 * projections and commits. Two isolated browser contexts run one of each.
 */

import {
  createDragDropController,
  CROSS_WINDOW_DRAG_MIME_TYPE,
  type CrossWindowDragCapabilities,
  type CrossWindowDragProjection,
  type CrossWindowDragReceipt,
  type CrossWindowDragSourceBridge,
  type CrossWindowDragTargetBridge,
  type CrossWindowDragTargetEvent,
  type DragCancelReason,
  type DragDropCommitResult,
  type DragTerminalOutcome,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/drag-drop.css";

const params = new URLSearchParams(window.location.search);
const role = params.get("role") === "target" ? "target" : "source";

const root = document.getElementById("root") as HTMLElement;
const sourceEl = document.getElementById("source") as HTMLButtonElement;
const targetEl = document.getElementById("target") as HTMLElement;
const otherEl = document.getElementById("other") as HTMLElement;
const probe = document.getElementById("probe") as HTMLElement;

const capabilities: CrossWindowDragCapabilities = {
  pointer: true,
  touch: false,
  keyboardTargetPicker: false,
};

interface HostLog {
  prepares: string[];
  starts: string[];
  stops: string[];
  cancels: string[];
  outcomes: string[];
  commits: string[];
  written: string[];
}

const log: HostLog = {
  prepares: [],
  starts: [],
  stops: [],
  cancels: [],
  outcomes: [],
  commits: [],
  written: [],
};

let settlePrepare: ((receipt: CrossWindowDragReceipt | null) => void) | null = null;
let terminalSink: ((outcome: DragTerminalOutcome) => void) | null = null;
let projectionSink: ((event: CrossWindowDragTargetEvent) => void) | null = null;
let commitAnswer: DragDropCommitResult = { status: "committed" };
/** Which target the second registration refuses, so revalidation is testable. */
let refuseTargetId: string | null = null;

const sourceBridge: CrossWindowDragSourceBridge = {
  capabilities,
  prepare(request) {
    log.prepares.push(request.sessionId);
    paint();
    return new Promise((resolve) => {
      settlePrepare = resolve;
    });
  },
  start(receipt, transport, onTerminal) {
    log.starts.push(`${receipt.token}:${transport}`);
    terminalSink = onTerminal;
    paint();
    return () => {
      log.stops.push(receipt.token);
      terminalSink = null;
      paint();
    };
  },
  cancel(receipt, reason) {
    log.cancels.push(`${receipt.token}:${reason}`);
    paint();
  },
};

const targetBridge: CrossWindowDragTargetBridge = {
  capabilities,
  subscribe(listener) {
    projectionSink = listener;
    return () => {
      projectionSink = null;
    };
  },
  commit(request) {
    log.commits.push(
      `${request.receipt.token}:${request.intent.targetId}:${request.intent.position}`,
    );
    paint();
    return Promise.resolve(commitAnswer);
  },
};

const controller =
  role === "target"
    ? createDragDropController({ crossWindowTargetBridge: targetBridge })
    : createDragDropController();

controller.connect(root);

if (role === "source") {
  controller.registerSource(sourceEl, {
    sourceId: "alpha",
    subject: { kind: "item", id: "alpha" },
    allowedOperations: ["move"],
    label: "Alpha",
    crossWindowSourceBridge: sourceBridge,
    onDragEnd: (outcome) => {
      log.outcomes.push(
        outcome.status === "committed"
          ? `committed:${outcome.intent.targetId}`
          : `${outcome.status}:${"reason" in outcome ? outcome.reason ?? "" : ""}`,
      );
      paint();
    },
  });
} else {
  for (const [element, targetId] of [
    [targetEl, "list"],
    [otherEl, "other"],
  ] as const) {
    controller.registerTarget(element, {
      targetId,
      acceptedKinds: ["item"],
      label: targetId,
      resolvePosition: () => "inside",
      canDrop: (intent) =>
        refuseTargetId === intent.targetId
          ? { accepted: false, reason: "host target moved" }
          : { accepted: true, intent },
      onDrop: (): DragDropCommitResult => {
        // A cross-window drop must never reach a local target callback: the
        // host bridge owns the commit, and this records the violation if it
        // ever does.
        log.commits.push("LOCAL-ONDROP");
        paint();
        return { status: "committed" };
      },
    });
  }
}

function paint(): void {
  const snapshot = controller.getSnapshot();
  probe.dataset.phase = snapshot.phase;
  probe.dataset.posture = snapshot.targetPosture ?? "";
  probe.dataset.target = snapshot.targetId ?? "";
  probe.dataset.position = snapshot.session?.intent?.position ?? "";
  probe.dataset.draggable = sourceEl.getAttribute("draggable") ?? "";
  // Attributes only. Writing the log into the element would change its size,
  // and the controller treats a layout change like a scroll: it abandons a
  // gesture that has not activated yet, which would make the fixture itself
  // cancel the preparation it is supposed to be testing.
}

controller.subscribe(paint);
paint();

/** Attach a `DataTransfer` the way every engine will actually expose it. */
function dispatchDrag(
  element: Element,
  type: "dragstart" | "dragover" | "drop" | "dragend",
  dataTransfer: DataTransfer,
  point?: { x: number; y: number },
): boolean {
  const event = new DragEvent(type, {
    bubbles: true,
    cancelable: true,
    composed: true,
    clientX: point?.x ?? 0,
    clientY: point?.y ?? 0,
  });
  Object.defineProperty(event, "dataTransfer", { value: dataTransfer, configurable: true });
  const proceeded = element.dispatchEvent(event);
  return proceeded;
}

const host = {
  role,
  state: () => ({ ...log, phase: controller.getSnapshot().phase }),
  probe: () => ({
    phase: probe.dataset.phase ?? "",
    posture: probe.dataset.posture ?? "",
    target: probe.dataset.target ?? "",
    position: probe.dataset.position ?? "",
    draggable: probe.dataset.draggable ?? "",
  }),

  /** Answer the pending preparation with a lease, or decline it. */
  arm(token: string | null): void {
    const settle = settlePrepare;
    settlePrepare = null;
    settle?.(token === null ? null : { protocolVersion: 1, token });
  },

  /**
   * Start the browser's own drag on the source. Returns the envelope the
   * controller wrote, so the probe can assert the wire carries nothing else.
   */
  startNativeDrag(): { prevented: boolean; types: string[]; body: string } {
    const dataTransfer = new DataTransfer();
    const proceeded = dispatchDrag(sourceEl, "dragstart", dataTransfer);
    return {
      prevented: !proceeded,
      types: [...dataTransfer.types],
      body: dataTransfer.getData(CROSS_WINDOW_DRAG_MIME_TYPE),
    };
  },

  endNativeDrag(dropEffect: string): void {
    const dataTransfer = new DataTransfer();
    try {
      (dataTransfer as { dropEffect: string }).dropEffect = dropEffect;
    } catch {
      // Read-only outside a real drag on some engines; the point is only that
      // whatever it says cannot become a commit.
    }
    dispatchDrag(sourceEl, "dragend", dataTransfer);
  },

  terminal(outcome: DragTerminalOutcome): void {
    terminalSink?.(outcome);
  },

  project(projection: Partial<CrossWindowDragProjection> & { token: string }): void {
    const { token, ...rest } = projection;
    projectionSink?.({
      type: "projection",
      projection: {
        receipt: { protocolVersion: 1, token },
        sourceId: "alpha",
        sourceLabel: "Alpha",
        subject: { kind: "item", id: "alpha" },
        operation: "move",
        inputKind: "pointer",
        targetId: "list",
        position: "inside",
        ...rest,
      },
    });
  },

  left(token: string): void {
    projectionSink?.({ type: "left", receipt: { protocolVersion: 1, token } });
  },

  cancelled(token: string, reason: DragCancelReason): void {
    projectionSink?.({
      type: "cancelled",
      receipt: { protocolVersion: 1, token },
      reason,
    });
  },

  refuse(targetId: string | null): void {
    refuseTargetId = targetId;
  },

  setCommit(answer: DragDropCommitResult): void {
    commitAnswer = answer;
  },

  /** Deliver a native drop carrying an arbitrary envelope body. */
  dropEnvelope(body: string | null, targetId = "list"): void {
    const element = targetId === "other" ? otherEl : targetEl;
    const rect = element.getBoundingClientRect();
    const point = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
    const dataTransfer = new DataTransfer();
    if (body !== null) dataTransfer.setData(CROSS_WINDOW_DRAG_MIME_TYPE, body);
    dispatchDrag(element, "dragover", dataTransfer, point);
    dispatchDrag(element, "drop", dataTransfer, point);
  },

  /** Whether `dragover` was claimed — the browser only fires `drop` if it was. */
  dragOverClaimed(body: string | null, targetId = "list"): boolean {
    const element = targetId === "other" ? otherEl : targetEl;
    const rect = element.getBoundingClientRect();
    const point = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
    const dataTransfer = new DataTransfer();
    if (body !== null) dataTransfer.setData(CROSS_WINDOW_DRAG_MIME_TYPE, body);
    return !dispatchDrag(element, "dragover", dataTransfer, point);
  },
};

(window as unknown as { __poodleHost: typeof host }).__poodleHost = host;

/**
 * Inbound files and native file drag-out — the adversarial matrix (g16.027).
 *
 * The host is simulated rather than mocked away: one object plays the export
 * bridge, holds whatever it claims to have materialized, and records every
 * command Poodle sends it. The recording is the point, because most of these
 * cases are claims about what Poodle did *not* do — did not delete, did not
 * cancel a settled receipt, did not arm a superseded one, did not let a
 * capability it was never given become an affordance.
 *
 * Two boundaries are on trial:
 *
 * - going out, that a receipt is opaque, abortable, validated against its own
 *   adapter's capabilities, and that its cleanup stays with the host;
 * - coming in, that untrusted external metadata is validated before any
 *   target is asked, again at drop, and that no path or `File` reaches a
 *   public seam.
 *
 * What is *not* here: proof that an operating system or a DAW consumed a
 * dragged file. Nothing on this side of the boundary can observe that, so it
 * stays manual downstream evidence rather than a faked callback.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  createDragDropController,
  createInboundFileDataTransferBridge,
  INBOUND_FILE_PROTOCOL_VERSION,
  INBOUND_FILE_SUBJECT_KIND,
  type DragCancelReason,
  type DragExportBridge,
  type DragExportCapabilities,
  type DragExportTerminal,
  type DragSourceRegistration,
  type DragTerminalOutcome,
  type DropTargetRegistration,
  type InboundFileBatch,
  type InboundFileEvent,
  type InboundFileHostBridge,
  type InboundFileOutcome,
  type PreparedFileExport,
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

/**
 * happy-dom drops `dataTransfer` *and* the mouse coordinates from a
 * `DragEvent` init, so both are attached here. Real coordinate behaviour is
 * proved by the Chromium/WebKit probe, which uses the engines' own events.
 */
function drag(
  type: string,
  dataTransfer: DataTransfer | null,
  init: { clientX?: number; clientY?: number } = {},
): DragEvent {
  const event = new DragEvent(type, { bubbles: true, cancelable: true });
  Object.defineProperty(event, "dataTransfer", { value: dataTransfer, configurable: true });
  Object.defineProperty(event, "clientX", { value: init.clientX ?? 0, configurable: true });
  Object.defineProperty(event, "clientY", { value: init.clientY ?? 0, configurable: true });
  return event;
}

async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await Promise.resolve();
}

const FILE_HOST: DragExportCapabilities = {
  files: true,
  multipleFiles: false,
  promisedFiles: false,
  customDataTypes: [],
};

interface ExportHostOptions {
  readonly capabilities?: DragExportCapabilities;
  /** Resolve preparation later, so supersession and late answers are testable. */
  readonly deferPrepare?: boolean;
  readonly decline?: boolean;
  readonly failPrepare?: boolean;
  /** The host reports its terminal synchronously, inside `start`. */
  readonly terminalInsideStart?: DragExportTerminal;
  /** `start` itself throws — the native drag never began. */
  readonly failStart?: boolean;
  readonly prepared?: Partial<PreparedFileExport>;
}

/**
 * A host that actually holds something.
 *
 * `artifacts` is the temporary file a real shell would have written. Nothing
 * in Poodle may remove it, so the whole retention question is answerable here:
 * after any terminal, the artifact is still the host's and still present
 * unless the host itself decided otherwise.
 */
function createExportHost(options: ExportHostOptions = {}) {
  const capabilities = options.capabilities ?? FILE_HOST;
  const prepares: string[] = [];
  const starts: string[] = [];
  const stops: string[] = [];
  const cancels: Array<{ receiptId: string; reason: string }> = [];
  const artifacts = new Set<string>();
  const pending: Array<(prepared: PreparedFileExport | null) => void> = [];
  const rejects: Array<(error: unknown) => void> = [];
  let issued = 0;
  let terminal: ((terminal: DragExportTerminal) => void) | null = null;

  const bridge: DragExportBridge = {
    capabilities,
    prepare(request) {
      prepares.push(request.sessionId);
      issued += 1;
      const receiptId = `export-${issued}`;
      const prepared: PreparedFileExport = {
        receiptId,
        displayName: "take-01.wav",
        form: "materialized-file",
        ...options.prepared,
      };
      // The host writes its temporary file here, exactly as a shell would.
      artifacts.add(prepared.receiptId);

      if (options.decline) return Promise.resolve(null);
      if (options.failPrepare) return Promise.reject(new Error("render failed"));
      if (options.deferPrepare) {
        return new Promise<PreparedFileExport | null>((resolve, reject) => {
          pending.push(resolve);
          rejects.push(reject);
        });
      }
      return Promise.resolve(prepared);
    },
    start(prepared, onTerminal) {
      starts.push(prepared.receiptId);
      if (options.failStart) throw new Error("native drag refused");
      terminal = onTerminal;
      const stop = () => {
        stops.push(prepared.receiptId);
        terminal = null;
      };
      // A host is allowed to answer whenever its work resolves, and
      // "immediately" is a legal whenever.
      if (options.terminalInsideStart) onTerminal(options.terminalInsideStart);
      return stop;
    },
    cancel(prepared, reason) {
      cancels.push({ receiptId: prepared.receiptId, reason });
      // A cancellation is *not* a delete order. A real host may or may not
      // clean up here; this one keeps its artifact so the test can prove
      // Poodle never asked for its removal.
    },
  };

  return {
    bridge,
    prepares,
    starts,
    stops,
    cancels,
    artifacts,
    settlePrepare(prepared: PreparedFileExport | null, index = 0) {
      pending[index]?.(prepared);
    },
    failPending(index = 0) {
      rejects[index]?.(new Error("render failed"));
    },
    prepared(index = 1, overrides: Partial<PreparedFileExport> = {}): PreparedFileExport {
      return {
        receiptId: `export-${index}`,
        displayName: "take-01.wav",
        form: "materialized-file",
        ...overrides,
      };
    },
    report(value: DragExportTerminal) {
      terminal?.(value);
    },
    get subscribed(): boolean {
      return terminal !== null;
    },
  };
}

/** A host-transport inbound bridge: no document, no `File`, just receipts. */
function createInboundHost(multipleFiles = true) {
  const released: Array<{ batchId: string; outcome: InboundFileOutcome }> = [];
  let listener: ((event: InboundFileEvent) => void) | null = null;

  const bridge: InboundFileHostBridge = {
    capabilities: {
      files: true,
      multipleFiles,
      transport: "host",
      customDataTypes: [],
    },
    subscribe(next) {
      listener = next;
      return () => {
        if (listener === next) listener = null;
      };
    },
    release(batchId, outcome) {
      released.push({ batchId, outcome });
    },
  };

  // Deliberately captured once: a real host keeps its own callback reference
  // and may fire it after the surface has gone, which is exactly the case the
  // "answers a batch that arrives after disconnect" claim is about.
  let published: ((event: InboundFileEvent) => void) | null = null;

  return {
    bridge,
    released,
    send(event: InboundFileEvent) {
      (listener ?? published)?.(event);
    },
    get subscribed(): boolean {
      return listener !== null;
    },
    rememberListener() {
      published = listener;
    },
  };
}

function hostBatch(overrides: Partial<InboundFileBatch> = {}): InboundFileBatch {
  return {
    protocolVersion: INBOUND_FILE_PROTOCOL_VERSION,
    batchId: "batch-1",
    transport: "host",
    files: [{ receiptId: "batch-1:0", name: "take-01.wav", mediaType: "audio/wav", size: 1_024 }],
    ...overrides,
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

function fileTargetReg(overrides: Partial<DropTargetRegistration> = {}): DropTargetRegistration {
  return {
    targetId: "dst",
    acceptedKinds: [INBOUND_FILE_SUBJECT_KIND],
    label: "Drop zone",
    resolvePosition: () => "inside",
    canDrop: (intent) => ({ accepted: true, intent }),
    onDrop: () => ({ status: "committed" }),
    ...overrides,
  };
}

describe("native file drag-out", () => {
  let root: HTMLElement;
  let sourceEl: HTMLElement;

  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    root = document.createElement("div");
    sourceEl = layout(document.createElement("button"), SOURCE_BOX);
    sourceEl.textContent = "Alpha";
    root.append(sourceEl);
    document.body.append(root);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  /**
   * The advertisement is the armed receipt, not the registration. An element
   * that said `draggable` before the host prepared anything would let the
   * browser start a drag carrying a file that does not exist yet.
   */
  it("prepares before activation and refuses the native drag until the receipt is armed", async () => {
    const host = createExportHost({ deferPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    expect(sourceEl.getAttribute("draggable")).toBe("false");
    expect(sourceEl.getAttribute("data-poodle-drag-export")).toBe("idle");

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    expect(host.prepares).toHaveLength(1);
    expect(controller.getSnapshot().phase).toBe("preparing");
    expect(controller.getSnapshot().fileExport?.state).toBe("preparing");
    expect(sourceEl.getAttribute("data-poodle-drag-export")).toBe("preparing");

    document.dispatchEvent(pointer("pointermove", { clientX: 60, clientY: 20 }));
    const refused = drag("dragstart", new DataTransfer());
    sourceEl.dispatchEvent(refused);
    expect(host.starts).toHaveLength(0);

    host.settlePrepare(host.prepared(1, { form: "existing-file" }));
    await settle();
    expect(controller.getSnapshot().fileExport).toEqual({
      state: "armed",
      form: "existing-file",
      fileCount: 1,
      displayName: "take-01.wav",
      reason: null,
    });
    expect(sourceEl.getAttribute("draggable")).toBe("true");

    disconnect();
    controller.destroy();
  });

  /**
   * The host runs the operating system's drag, so the browser's own must not:
   * the documented shell pattern is to prevent the web drag and start the
   * native one. Two live drags for one gesture would also give the page a
   * `dragend` that looks like an outcome.
   */
  it("hands the native drag to the host and writes nothing into the DataTransfer", async () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    expect(controller.getSnapshot().phase).toBe("armed");

    const dataTransfer = new DataTransfer();
    const start = drag("dragstart", dataTransfer);
    sourceEl.dispatchEvent(start);

    expect(start.defaultPrevented).toBe(true);
    expect([...dataTransfer.types]).toEqual([]);
    expect(host.starts).toEqual(["export-1"]);
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().fileExport?.state).toBe("dragging");

    disconnect();
    controller.destroy();
  });

  /**
   * The whole retention question in one case. The host said the gesture ended;
   * it did not say a destination took the file, and Poodle does not guess.
   * Its artifact survives, no cancel command is issued against a settled
   * receipt, and the terminal is announced as an ending rather than as the
   * cancellation the kernel correctly recorded.
   */
  it("ends without deleting, cancelling, or claiming a destination consumed anything", async () => {
    const host = createExportHost();
    const outcomes: DragTerminalOutcome[] = [];
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({ fileExportBridge: host.bridge, onDragEnd: (outcome) => outcomes.push(outcome) }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));
    expect(host.starts).toEqual(["export-1"]);

    host.report({ status: "ended" });
    host.report({ status: "ended" });

    expect(host.cancels).toEqual([]);
    expect(host.stops).toEqual(["export-1"]);
    expect(host.artifacts.has("export-1")).toBe(true);
    expect(outcomes).toEqual([{ status: "cancelled", reason: "explicit" }]);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().fileExport?.state).toBe("ended");
    expect(sourceEl.getAttribute("data-poodle-drag-export")).toBe("ended");
    expect(controller.getSnapshot().announcement).toBe("Finished exporting Alpha");

    disconnect();
    controller.destroy();
  });

  /**
   * The host answers inside `start`. By the time `start` returns, the session
   * is already over — so the subscription it just handed back has to be closed
   * here rather than stored on a transaction nobody will release again.
   */
  it("closes a subscription whose host terminated inside start", async () => {
    const host = createExportHost({ terminalInsideStart: { status: "ended" } });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));

    expect(host.starts).toEqual(["export-1"]);
    expect(host.stops).toEqual(["export-1"]);
    expect(host.cancels).toEqual([]);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().fileExport?.state).toBe("ended");
    expect(host.artifacts.has("export-1")).toBe(true);

    disconnect();
    controller.destroy();
  });

  /**
   * A host that cannot start the native drag has *failed*, and that is what
   * the surface must keep showing — the release that follows must not
   * overwrite it with the cancellation the session technically reached.
   */
  it("keeps a start exception visibly failed and still returns the receipt", async () => {
    const host = createExportHost({ failStart: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));

    expect(host.starts).toEqual(["export-1"]);
    expect(host.stops).toEqual([]);
    expect(controller.getSnapshot().fileExport?.state).toBe("failed");
    expect(controller.getSnapshot().fileExport?.reason).toBe("native drag refused");
    expect(host.cancels).toEqual([
      { receiptId: "export-1", reason: "transport-lost" },
    ]);
    expect(host.artifacts.has("export-1")).toBe(true);
    expect(controller.getSnapshot().phase).toBe("idle");

    disconnect();
    controller.destroy();
  });

  /**
   * Every export state a surface can reach has its own Poodle-owned wording.
   * The kernel's terminal is the same cancellation in three of these cases;
   * what the person doing it experienced is not.
   */
  it("announces each export terminal in its own words", async () => {
    async function announcementFor(host: ReturnType<typeof createExportHost>, drive: (controller: ReturnType<typeof createDragDropController>) => void | Promise<void>): Promise<string | null> {
      const controller = createDragDropController();
      const disconnect = controller.connect(root);
      controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));
      sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
      await settle();
      await drive(controller);
      await settle();
      const announcement = controller.getSnapshot().announcement;
      disconnect();
      controller.destroy();
      return announcement;
    }

    const ended = createExportHost();
    expect(
      await announcementFor(ended, () => {
        sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));
        ended.report({ status: "ended" });
      }),
    ).toBe("Finished exporting Alpha");

    const cancelled = createExportHost();
    expect(
      await announcementFor(cancelled, () => {
        sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));
        cancelled.report({ status: "cancelled", reason: "window-lost" });
      }),
    ).toBe("Cancelled exporting Alpha");

    const failed = createExportHost();
    expect(
      await announcementFor(failed, () => {
        sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));
        failed.report({ status: "failed", reason: "disk full" });
      }),
    ).toBe("Export failed for Alpha: disk full");

    expect(await announcementFor(createExportHost({ decline: true }), () => {})).toBe(
      "Alpha cannot be exported",
    );
  });

  /** A late native end is not a second lifecycle, and never a result. */
  it("treats a native dragend as nothing at all", async () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));

    const end = drag("dragend", new DataTransfer());
    Object.defineProperty(end, "dataTransfer", {
      value: Object.assign(new DataTransfer(), { dropEffect: "copy" }),
      configurable: true,
    });
    sourceEl.dispatchEvent(end);

    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(host.cancels).toEqual([]);

    disconnect();
    controller.destroy();
  });

  /** Escape is Poodle's, and it reaches the host exactly once. */
  it("cancels the live receipt once and keeps the host's artifact", async () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));

    expect(host.cancels).toEqual([{ receiptId: "export-1", reason: "escape" }]);
    expect(host.artifacts.has("export-1")).toBe(true);
    expect(host.stops).toEqual(["export-1"]);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().fileExport?.state).toBe("cancelled");

    disconnect();
    controller.destroy();
  });

  /**
   * A decline kills the export, not the drag. The gesture the user is still
   * making becomes an ordinary local session with no host payload.
   */
  it("falls back to a local drag when the host declines, and never asks again", async () => {
    const host = createExportHost({ decline: true });
    const targetEl = layout(document.createElement("div"), TARGET_BOX);
    targetEl.textContent = "List";
    root.append(targetEl);

    const onDrop = vi.fn(() => ({ status: "committed" }) as const);
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));
    controller.registerTarget(
      targetEl,
      fileTargetReg({ acceptedKinds: ["item"], onDrop, label: "List" }),
    );

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    expect(controller.getSnapshot().fileExport?.state).toBe("unavailable");
    expect(sourceEl.getAttribute("draggable")).toBe("false");

    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    document.dispatchEvent(pointer("pointerup", { clientX: 40, clientY: 90 }));

    expect(onDrop).toHaveBeenCalledTimes(1);
    // One ask per gesture: the fallback session must not re-enter the bridge
    // that just declined, or it would decline forever.
    expect(host.prepares).toHaveLength(1);

    disconnect();
    controller.destroy();
  });

  it("reports a failed preparation as failed and still leaves a working local drag", async () => {
    const host = createExportHost({ failPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();

    expect(controller.getSnapshot().fileExport?.state).toBe("failed");
    expect(controller.getSnapshot().fileExport?.reason).toBe("render failed");
    expect(sourceEl.getAttribute("data-poodle-drag-export")).toBe("failed");

    document.dispatchEvent(pointer("pointermove", { clientX: 60, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("dragging");

    disconnect();
    controller.destroy();
  });

  /**
   * The receipt that arrives too late belongs to a session that no longer
   * exists. It is handed back rather than dropped on the floor — the host
   * allocated something for it — and it cannot arm the session that replaced
   * it.
   */
  it("returns a superseded preparation to the host and cannot arm its successor", async () => {
    const host = createExportHost({ deferPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointerup", { clientX: 20, clientY: 20 }));
    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    expect(host.prepares).toHaveLength(2);

    // The first host answers now, for a session that is gone.
    host.settlePrepare(host.prepared(1), 0);
    await settle();

    expect(host.cancels).toEqual([{ receiptId: "export-1", reason: "superseded" }]);
    expect(controller.getSnapshot().phase).toBe("preparing");
    expect(sourceEl.getAttribute("draggable")).toBe("false");

    host.settlePrepare(host.prepared(2), 1);
    await settle();
    expect(controller.getSnapshot().phase).toBe("armed");
    expect(sourceEl.getAttribute("draggable")).toBe("true");

    disconnect();
    controller.destroy();
  });

  /**
   * A host that returns more than it advertised has produced a drag that
   * fails somewhere far from here. The receipt is refused *and returned*, so
   * the temporary file it made for a drag that will never start is not
   * silently abandoned.
   */
  it("refuses a receipt beyond its own capabilities and hands it back", async () => {
    const host = createExportHost({ deferPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    host.settlePrepare(host.prepared(1, { fileCount: 3 }));
    await settle();

    expect(controller.getSnapshot().fileExport?.state).toBe("failed");
    expect(controller.getSnapshot().fileExport?.reason).toBe("multiple-files-unsupported");
    expect(host.cancels).toEqual([{ receiptId: "export-1", reason: "preparation-failed" }]);
    expect(host.starts).toEqual([]);
    expect(sourceEl.getAttribute("draggable")).toBe("false");

    disconnect();
    controller.destroy();
  });

  it("arms a multi-file receipt when the adapter advertises multiple files", async () => {
    const host = createExportHost({
      capabilities: { ...FILE_HOST, multipleFiles: true },
      deferPrepare: true,
    });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    host.settlePrepare(host.prepared(1, { fileCount: 3, displayName: "3 takes" }));
    await settle();

    expect(controller.getSnapshot().fileExport).toEqual({
      state: "armed",
      form: "materialized-file",
      fileCount: 3,
      displayName: "3 takes",
      reason: null,
    });

    disconnect();
    controller.destroy();
  });

  /** The one shape that must never reach presentation. */
  it("refuses a display name that is really a path", async () => {
    const host = createExportHost({ deferPrepare: true });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    host.settlePrepare(host.prepared(1, { displayName: "/tmp/poodle/take-01.wav" }));
    await settle();

    expect(controller.getSnapshot().fileExport?.reason).toBe("name-is-a-path");
    expect(controller.getSnapshot().fileExport?.displayName).toBe(null);
    expect(host.cancels).toEqual([{ receiptId: "export-1", reason: "preparation-failed" }]);

    disconnect();
    controller.destroy();
  });

  /**
   * An unsupported capability is inert, not half-armed: no preparation, no
   * advertisement, and an at-rest state a surface can show.
   */
  it("never prepares when the adapter can export nothing", async () => {
    const host = createExportHost({
      capabilities: { files: false, multipleFiles: false, promisedFiles: false, customDataTypes: [] },
    });
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    expect(sourceEl.getAttribute("data-poodle-drag-export")).toBe("unavailable");

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();

    // No early preparation at all: the gesture is an ordinary local one, so
    // it waits for the activation distance like any other source.
    expect(host.prepares).toEqual([]);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().fileExport).toBe(null);
    expect(sourceEl.getAttribute("draggable")).toBe("false");

    document.dispatchEvent(pointer("pointermove", { clientX: 60, clientY: 20 }));
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(host.prepares).toEqual([]);

    disconnect();
    controller.destroy();
  });

  /**
   * There is no keyboard route to the desktop. The export stays inert and the
   * ordinary keyboard drag is untouched.
   */
  it("does not arm an export for a keyboard pickup", async () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(
      sourceEl,
      sourceReg({ fileExportBridge: host.bridge, keyboardOrder: 0 }),
    );

    sourceEl.focus();
    sourceEl.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    await settle();

    expect(host.prepares).toEqual([]);
    expect(controller.getSnapshot().phase).toBe("dragging");

    disconnect();
    controller.destroy();
  });

  /** One gesture leaves one way. A silent precedence rule would be a choice. */
  it("refuses a source that declares both external bridges", () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);

    expect(() =>
      controller.registerSource(
        sourceEl,
        sourceReg({
          fileExportBridge: host.bridge,
          crossWindowSourceBridge: {
            capabilities: { pointer: true, touch: false, keyboardTargetPicker: false },
            prepare: () => Promise.resolve(null),
            start: () => () => {},
            cancel: () => {},
          },
        }),
      ),
    ).toThrow(/declares both/);

    disconnect();
    controller.destroy();
  });

  /** Disconnecting mid-export returns the live receipt to its host, once. */
  it("returns a live receipt when the surface goes away", async () => {
    const host = createExportHost();
    const controller = createDragDropController();
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg({ fileExportBridge: host.bridge }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    await settle();
    sourceEl.dispatchEvent(drag("dragstart", new DataTransfer()));

    disconnect();
    expect(host.cancels).toHaveLength(1);
    expect(host.cancels[0]?.receiptId).toBe("export-1");
    expect(host.artifacts.has("export-1")).toBe(true);
    expect(host.subscribed).toBe(false);

    controller.destroy();
    expect(host.cancels).toHaveLength(1);
  });
});

describe("inbound files", () => {
  let root: HTMLElement;
  let targetEl: HTMLElement;

  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    root = document.createElement("div");
    targetEl = layout(document.createElement("div"), TARGET_BOX);
    targetEl.textContent = "Drop zone";
    root.append(targetEl);
    document.body.append(root);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  /**
   * An inbound file is an ordinary subject: the same hit test, the same
   * arbitration, the same eligibility, the same commit. There is no second
   * file-drop callback to keep in step with the first.
   */
  it("drives the common target path and commits through onDrop", () => {
    const host = createInboundHost();
    const onDrop = vi.fn(() => ({ status: "committed" }) as const);
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg({ onDrop }));

    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });
    expect(controller.getSnapshot().phase).toBe("dragging");
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    expect(controller.getSnapshot().inboundFiles?.files).toHaveLength(1);
    expect(targetEl.getAttribute("data-poodle-drop-target")).toBe("accepted");

    host.send({ type: "dropped", batch: hostBatch(), x: 40, y: 90 });

    expect(onDrop).toHaveBeenCalledWith(
      { targetId: "dst", position: "inside", operation: "copy" },
      {
        subject: { kind: INBOUND_FILE_SUBJECT_KIND, id: "batch-1" },
        inboundFiles: hostBatch(),
      },
    );
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "committed" }]);
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().inboundFiles).toBe(null);

    disconnect();
    controller.destroy();
  });

  /**
   * Validation runs before the consumer's resolver, so a hostile batch is
   * refused by the boundary rather than by a `canDrop` that had to defend
   * itself. The reason is presentation text, not a silent no.
   */
  it("refuses an over-limit batch before the target's own eligibility runs", () => {
    const host = createInboundHost();
    const canDrop = vi.fn(() => true);
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(
      targetEl,
      fileTargetReg({ canDrop, inboundFiles: { maxFiles: 1, accept: "audio/*" } }),
    );

    host.send({
      type: "entered",
      batch: hostBatch({
        files: [
          { receiptId: "batch-1:0", name: "a.wav", mediaType: "audio/wav", size: 1 },
          { receiptId: "batch-1:1", name: "b.wav", mediaType: "audio/wav", size: 1 },
        ],
      }),
      x: 40,
      y: 90,
    });

    expect(canDrop).not.toHaveBeenCalled();
    expect(controller.getSnapshot().targetPosture).toBe("rejected");
    expect(controller.getSnapshot().rejectedReason).toBe("too-many");

    disconnect();
    controller.destroy();
  });

  /**
   * Hover cannot see sizes, so hover acceptance is provisional by
   * construction. The disclosed batch is validated again at drop, and a file
   * that is only too large once disclosed is refused there.
   */
  it("re-validates the disclosed batch at drop and refuses what hover could not see", () => {
    const host = createInboundHost();
    const onDrop = vi.fn(() => ({ status: "committed" }) as const);
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(
      targetEl,
      fileTargetReg({ onDrop, inboundFiles: { maxSize: 1_000 } }),
    );

    host.send({
      type: "entered",
      batch: hostBatch({
        files: [{ receiptId: "batch-1:0", name: null, mediaType: "audio/wav", size: null }],
      }),
      x: 40,
      y: 90,
    });
    expect(controller.getSnapshot().targetPosture).toBe("accepted");

    host.send({
      type: "dropped",
      batch: hostBatch({
        files: [{ receiptId: "batch-1:0", name: "take.wav", mediaType: "audio/wav", size: 9_999 }],
      }),
      x: 40,
      y: 90,
    });

    expect(onDrop).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "cancelled" }]);

    disconnect();
    controller.destroy();
  });

  /** A batch this window cannot carry never becomes a session at all. */
  it("refuses a malformed batch before a session exists and tells the host once", () => {
    const host = createInboundHost();
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());

    host.send({
      type: "entered",
      batch: hostBatch({
        files: [{ receiptId: "batch-1:0", name: "/etc/passwd", mediaType: "text/plain", size: 1 }],
      }),
      x: 40,
      y: 90,
    });

    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().inboundFiles).toBe(null);
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "rejected" }]);

    disconnect();
    controller.destroy();
  });

  it("releases a rejected commit as rejected, exactly once", () => {
    const host = createInboundHost();
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(
      targetEl,
      fileTargetReg({ onDrop: () => ({ status: "rejected", reason: "library is full" }) }),
    );

    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });
    host.send({ type: "dropped", batch: hostBatch(), x: 40, y: 90 });

    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "rejected" }]);

    // The host repeating itself cannot produce a second release.
    host.send({ type: "cancelled", batchId: "batch-1" });
    expect(host.released).toHaveLength(1);

    disconnect();
    controller.destroy();
  });

  it("ends and releases when the host cancels the drag", () => {
    const host = createInboundHost();
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());

    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });
    host.send({ type: "cancelled", batchId: "batch-1" });

    expect(controller.getSnapshot().phase).toBe("idle");
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "cancelled" }]);

    disconnect();
    controller.destroy();
  });

  /**
   * The user's own pointer owns this controller — but a refusal is still an
   * *answer*. A batch this window silently ignored would leave the host
   * holding material for a gesture nobody will ever finish.
   */
  it("refuses an inbound batch while a local gesture is live, and tells the host", async () => {
    const host = createInboundHost();
    const sourceEl = layout(document.createElement("button"), SOURCE_BOX);
    sourceEl.textContent = "Alpha";
    root.append(sourceEl);

    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerSource(sourceEl, sourceReg());
    controller.registerTarget(targetEl, fileTargetReg({ acceptedKinds: ["item", INBOUND_FILE_SUBJECT_KIND] }));

    sourceEl.dispatchEvent(pointer("pointerdown", { clientX: 20, clientY: 20 }));
    document.dispatchEvent(pointer("pointermove", { clientX: 40, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");

    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });

    expect(controller.getSnapshot().session?.subject).toEqual({ kind: "item", id: "a" });
    expect(controller.getSnapshot().inboundFiles).toBe(null);
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "rejected" }]);

    disconnect();
    controller.destroy();
  });

  /**
   * Exactly one release per observed batch. A second batch is refused while
   * the first is live, the first is unaffected, and a repeat of either id
   * cannot produce a second answer.
   */
  it("owns one batch at a time and answers every other one exactly once", () => {
    const host = createInboundHost();
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());

    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });
    expect(controller.getSnapshot().inboundFiles?.batchId).toBe("batch-1");

    // The same batch again is one observation, not two.
    host.send({ type: "entered", batch: hostBatch(), x: 41, y: 90 });
    expect(host.released).toEqual([]);

    // A second, different batch is refused — and the live one keeps going.
    host.send({ type: "entered", batch: hostBatch({ batchId: "batch-2" }), x: 41, y: 90 });
    expect(host.released).toEqual([{ batchId: "batch-2", outcome: "rejected" }]);
    expect(controller.getSnapshot().inboundFiles?.batchId).toBe("batch-1");
    expect(controller.getSnapshot().phase).toBe("dragging");

    // News for a batch this window refused cannot start or end anything.
    host.send({ type: "dropped", batch: hostBatch({ batchId: "batch-2" }), x: 40, y: 90 });
    host.send({ type: "cancelled", batchId: "batch-2" });
    expect(host.released).toHaveLength(1);
    expect(controller.getSnapshot().phase).toBe("dragging");

    host.send({ type: "dropped", batch: hostBatch(), x: 40, y: 90 });
    expect(host.released).toEqual([
      { batchId: "batch-2", outcome: "rejected" },
      { batchId: "batch-1", outcome: "committed" },
    ]);

    // A late repeat of the finished id cannot resurrect it or release twice.
    host.send({ type: "dropped", batch: hostBatch(), x: 40, y: 90 });
    host.send({ type: "cancelled", batchId: "batch-1" });
    expect(host.released).toHaveLength(2);
    expect(controller.getSnapshot().phase).toBe("idle");

    disconnect();
    controller.destroy();
  });

  /** A batch observed after the surface is gone is still answered. */
  it("answers a batch that arrives after the controller disconnected", () => {
    const host = createInboundHost();
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());
    host.rememberListener();

    disconnect();
    host.send({ type: "entered", batch: hostBatch(), x: 40, y: 90 });

    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "rejected" }]);
    expect(controller.getSnapshot().phase).toBe("idle");

    controller.destroy();
  });

  /** A batch this build cannot read is refused before any other field is. */
  it("refuses a batch from another protocol version before eligibility", () => {
    const host = createInboundHost();
    const canDrop = vi.fn(() => true);
    const controller = createDragDropController({ inboundFileBridge: host.bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg({ canDrop }));

    host.send({
      type: "entered",
      batch: hostBatch({ protocolVersion: INBOUND_FILE_PROTOCOL_VERSION + 1 }),
      x: 40,
      y: 90,
    });

    expect(canDrop).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(host.released).toEqual([{ batchId: "batch-1", outcome: "rejected" }]);

    disconnect();
    controller.destroy();
  });

  /**
   * The transport claim is exclusive. A window that listened to both a native
   * capture and its own drag events would take one gesture as two drops, so
   * the mismatch is loud rather than a silently ignored half.
   */
  it("refuses a bridge whose transport claim does not match what it can observe", () => {
    const hostWithDocument = {
      ...createInboundHost().bridge,
      connect: () => () => {},
    };
    const controller = createDragDropController({ inboundFileBridge: hostWithDocument });
    expect(() => controller.connect(root)).toThrow(/must not also bind document drag events/);
    controller.destroy();

    const domWithoutConnect: InboundFileHostBridge = {
      capabilities: { files: true, multipleFiles: true, transport: "data-transfer", customDataTypes: [] },
      subscribe: () => () => {},
      release: () => {},
    };
    const second = createDragDropController({ inboundFileBridge: domWithoutConnect });
    expect(() => second.connect(root)).toThrow(/cannot connect to a document/);
    second.destroy();
  });
});

describe("the browser's own file drag", () => {
  let root: HTMLElement;
  let targetEl: HTMLElement;

  function fileDataTransfer(files: File[]): DataTransfer {
    return {
      types: files.length > 0 ? ["Files"] : [],
      items: files.map((file) => ({ kind: "file", type: file.type })),
      files,
      dropEffect: "none",
      getData: () => "",
      setData: () => {},
    } as unknown as DataTransfer;
  }

  function makeFile(name: string, type: string, size: number): File {
    return { name, type, size } as File;
  }

  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
    root = document.createElement("div");
    targetEl = layout(document.createElement("div"), TARGET_BOX);
    targetEl.textContent = "Drop zone";
    root.append(targetEl);
    document.body.append(root);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    document.body.replaceChildren();
  });

  /**
   * Hover discloses kinds and types; the drop discloses everything. Both go
   * through the same target, and the `File` objects stay behind the adapter —
   * a consumer that wants them says so with `project`, and gets exactly what
   * it asked for.
   */
  it("hovers on declared types, commits on disclosed files, and never exposes one by default", () => {
    const bridge = createInboundFileDataTransferBridge();
    const seen: unknown[] = [];
    let resolvedDuringDrop: unknown = null;
    const controller = createDragDropController({ inboundFileBridge: bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(
      targetEl,
      fileTargetReg({
        inboundFiles: { accept: "audio/*" },
        onDrop: (_intent, context) => {
          seen.push(context.inboundFiles);
          // The projection is live exactly while the commit is running: this
          // is the moment a consumer has to be able to read it.
          resolvedDuringDrop = bridge.resolve("inbound-1");
          return { status: "committed" };
        },
      }),
    );

    const hoverTransfer = fileDataTransfer([makeFile("", "audio/wav", 0)]);
    targetEl.dispatchEvent(drag("dragenter", hoverTransfer, { clientX: 40, clientY: 90 }));
    const over = drag("dragover", hoverTransfer, { clientX: 40, clientY: 90 });
    targetEl.dispatchEvent(over);

    // Claimed: an unclaimed file drop navigates the window to the file.
    expect(over.defaultPrevented).toBe(true);
    expect(controller.getSnapshot().targetPosture).toBe("accepted");
    expect(controller.getSnapshot().inboundFiles?.files[0]).toEqual({
      receiptId: "inbound-1:0",
      name: null,
      mediaType: "audio/wav",
      size: null,
    });

    const dropTransfer = fileDataTransfer([makeFile("take-01.wav", "audio/wav", 2_048)]);
    targetEl.dispatchEvent(drag("drop", dropTransfer, { clientX: 40, clientY: 90 }));

    expect(seen).toEqual([
      {
        protocolVersion: INBOUND_FILE_PROTOCOL_VERSION,
        batchId: "inbound-1",
        transport: "data-transfer",
        files: [
          { receiptId: "inbound-1:0", name: "take-01.wav", mediaType: "audio/wav", size: 2_048 },
        ],
      },
    ]);
    // Default projection is the receipt itself: no `File` leaves this adapter
    // unless a consumer explicitly asked for one.
    expect(resolvedDuringDrop).toEqual([
      { receiptId: "inbound-1:0", name: "take-01.wav", mediaType: "audio/wav", size: 2_048 },
    ]);
    expect(controller.getSnapshot().phase).toBe("idle");
    // Released with the terminal: the adapter holds nothing after the drop.
    expect(bridge.heldBatches()).toEqual([]);
    expect(bridge.resolve("inbound-1")).toBe(null);

    disconnect();
    controller.destroy();
  });

  it("hands a consumer exactly the projection it asked for", () => {
    const bridge = createInboundFileDataTransferBridge({
      project: (file) => ({ upload: file.name, bytes: file.size }),
    });
    const controller = createDragDropController({ inboundFileBridge: bridge });
    const disconnect = controller.connect(root);
    let projected: unknown = null;
    controller.registerTarget(
      targetEl,
      fileTargetReg({
        onDrop: (_intent, context) => {
          projected = bridge.resolve(context.inboundFiles?.batchId ?? "");
          return { status: "committed" };
        },
      }),
    );

    const transfer = fileDataTransfer([makeFile("take-01.wav", "audio/wav", 2_048)]);
    targetEl.dispatchEvent(drag("dragenter", transfer, { clientX: 40, clientY: 90 }));
    targetEl.dispatchEvent(drag("drop", transfer, { clientX: 40, clientY: 90 }));

    expect(projected).toEqual([{ upload: "take-01.wav", bytes: 2_048 }]);

    disconnect();
    controller.destroy();
  });

  /**
   * `dragenter` and `dragleave` fire per element, not per window. A session
   * that ended on the first leave would die the moment the pointer crossed a
   * row inside the very target it was aimed at.
   */
  it("survives per-element leaves and ends only when the drag leaves the window", () => {
    const bridge = createInboundFileDataTransferBridge();
    const controller = createDragDropController({ inboundFileBridge: bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());

    const transfer = fileDataTransfer([makeFile("", "audio/wav", 0)]);
    targetEl.dispatchEvent(drag("dragenter", transfer, { clientX: 40, clientY: 90 }));
    // Into a child, and back out of it.
    targetEl.dispatchEvent(drag("dragenter", transfer, { clientX: 40, clientY: 92 }));
    targetEl.dispatchEvent(drag("dragleave", transfer, { clientX: 40, clientY: 92 }));
    expect(controller.getSnapshot().phase).toBe("dragging");

    targetEl.dispatchEvent(drag("dragleave", transfer, { clientX: 0, clientY: 0 }));
    expect(controller.getSnapshot().phase).toBe("idle");

    disconnect();
    controller.destroy();
  });

  /**
   * A consumer's projection is consumer code, and it can throw. Leaving the
   * exception to escape the drop listener would leave the controller dragging
   * a batch that can never be dropped — the surface would look stuck with no
   * way out.
   */
  it("ends the drag cleanly when the consumer's projection throws", () => {
    const bridge = createInboundFileDataTransferBridge({
      project: () => {
        throw new Error("upload queue is closed");
      },
    });
    const onDrop = vi.fn(() => ({ status: "committed" }) as const);
    const controller = createDragDropController({ inboundFileBridge: bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg({ onDrop }));

    const transfer = fileDataTransfer([makeFile("take-01.wav", "audio/wav", 2_048)]);
    targetEl.dispatchEvent(drag("dragenter", transfer, { clientX: 40, clientY: 90 }));
    expect(controller.getSnapshot().phase).toBe("dragging");

    const dropEvent = drag("drop", transfer, { clientX: 40, clientY: 90 });
    expect(() => targetEl.dispatchEvent(dropEvent)).not.toThrow();

    expect(onDrop).not.toHaveBeenCalled();
    expect(controller.getSnapshot().phase).toBe("idle");
    expect(controller.getSnapshot().inboundFiles).toBe(null);
    expect(bridge.heldBatches()).toEqual([]);

    disconnect();
    controller.destroy();
  });

  it("ignores a drag that carries no files at all", () => {
    const bridge = createInboundFileDataTransferBridge();
    const controller = createDragDropController({ inboundFileBridge: bridge });
    const disconnect = controller.connect(root);
    controller.registerTarget(targetEl, fileTargetReg());

    const textTransfer = fileDataTransfer([]);
    const enter = drag("dragenter", textTransfer, { clientX: 40, clientY: 90 });
    targetEl.dispatchEvent(enter);
    const over = drag("dragover", textTransfer, { clientX: 40, clientY: 90 });
    targetEl.dispatchEvent(over);

    expect(controller.getSnapshot().phase).toBe("idle");
    expect(over.defaultPrevented).toBe(false);

    disconnect();
    controller.destroy();
  });
});

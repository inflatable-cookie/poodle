/**
 * External file fixture — inbound files and native drag-out in a real engine.
 *
 * Both halves run against the engine's own `DataTransfer`, `DataTransferItem`,
 * and `File`, which is the part that cannot be proved in a DOM emulation: the
 * types list, what `items` discloses during `dragover` versus `drop`, and
 * whether the engine let the page claim the drop.
 *
 * What this fixture deliberately does *not* claim: that the operating system
 * handed the page a real file drag, or that a destination consumed an exported
 * one. Playwright cannot originate an OS drag into a page, and no browser API
 * reports what a desktop did with a dragged file. Those remain manual platform
 * evidence.
 */

import {
  createDragDropController,
  createInboundFileDataTransferBridge,
  INBOUND_FILE_SUBJECT_KIND,
  type DragExportBridge,
  type DragExportTerminal,
  type DropCommitContext,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/drag-drop.css";

const root = document.getElementById("root") as HTMLElement;
const clipEl = document.getElementById("clip") as HTMLButtonElement;
const zoneEl = document.getElementById("zone") as HTMLElement;
const probe = document.getElementById("probe") as HTMLElement;

/** The location only the host is allowed to know. */
const HOST_PATH = "/var/tmp/poodle-42/take-01.wav";

interface Log {
  prepares: string[];
  starts: string[];
  stops: string[];
  cancels: string[];
  outcomes: string[];
  drops: string[];
  artifacts: string[];
}

const log: Log = {
  prepares: [],
  starts: [],
  stops: [],
  cancels: [],
  outcomes: [],
  drops: [],
  artifacts: [],
};

let settlePrepare: ((prepared: Parameters<typeof resolveNothing>[0]) => void) | null = null;
let terminalSink: ((terminal: DragExportTerminal) => void) | null = null;
function resolveNothing(_value: unknown): void {}

const exportBridge: DragExportBridge = {
  capabilities: { files: true, multipleFiles: false, promisedFiles: false, customDataTypes: [] },
  prepare(request) {
    log.prepares.push(request.sessionId);
    // A real shell renders the clip to `HOST_PATH` here. The path never
    // leaves this object.
    log.artifacts.push(HOST_PATH);
    paint();
    return new Promise((resolve) => {
      settlePrepare = resolve as (prepared: unknown) => void;
    });
  },
  start(prepared, onTerminal) {
    log.starts.push(prepared.receiptId);
    terminalSink = onTerminal;
    paint();
    return () => {
      log.stops.push(prepared.receiptId);
      terminalSink = null;
      paint();
    };
  },
  cancel(prepared, reason) {
    log.cancels.push(`${prepared.receiptId}:${reason}`);
    paint();
  },
};

const inboundBridge = createInboundFileDataTransferBridge({
  multipleFiles: true,
  // The consumer asks for what it needs; without this the batch resolves to
  // receipts and no `File` leaves the adapter at all.
  project: (file) => ({ name: file.name, size: file.size }),
});

const controller = createDragDropController({ inboundFileBridge: inboundBridge });
controller.connect(root);

controller.registerSource(clipEl, {
  sourceId: "clip-1",
  subject: { kind: "clip", id: "clip-1" },
  allowedOperations: ["copy"],
  label: "Intro clip",
  fileExportBridge: exportBridge,
  onDragEnd: (outcome) => {
    log.outcomes.push(
      outcome.status === "committed"
        ? `committed:${outcome.intent.targetId}`
        : `${outcome.status}:${"reason" in outcome ? outcome.reason ?? "" : ""}`,
    );
    paint();
  },
});

controller.registerTarget(zoneEl, {
  targetId: "library",
  acceptedKinds: [INBOUND_FILE_SUBJECT_KIND],
  label: "Sample library",
  inboundFiles: { accept: "audio/*", maxFiles: 2, maxSize: 4_096 },
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (_intent, context: DropCommitContext) => {
    const batchId = context.inboundFiles?.batchId ?? "";
    const projected = inboundBridge.resolve(batchId) as
      | ReadonlyArray<{ name: string; size: number }>
      | null;
    log.drops.push(
      (projected ?? []).map((file) => `${file.name}:${file.size}`).join("|") || "none",
    );
    paint();
    return { status: "committed" };
  },
});

function paint(): void {
  const snapshot = controller.getSnapshot();
  probe.dataset.phase = snapshot.phase;
  probe.dataset.posture = snapshot.targetPosture ?? "";
  probe.dataset.reason = snapshot.rejectedReason ?? "";
  probe.dataset.export = snapshot.fileExport?.state ?? "idle";
  probe.dataset.exportName = snapshot.fileExport?.displayName ?? "";
  probe.dataset.draggable = clipEl.getAttribute("draggable") ?? "";
  probe.dataset.offered = String(snapshot.inboundFiles?.files.length ?? 0);
  probe.dataset.names = (snapshot.inboundFiles?.files ?? [])
    .map((file) => file.name ?? "?")
    .join("|");
  // Attributes only: writing the log into the element would resize it, and a
  // layout change abandons a gesture that has not activated yet.
}

controller.subscribe(paint);
paint();

function fileDrag(
  type: "dragenter" | "dragover" | "dragleave" | "drop",
  dataTransfer: DataTransfer,
  point: { x: number; y: number },
  element: Element = zoneEl,
): boolean {
  const event = new DragEvent(type, {
    bubbles: true,
    cancelable: true,
    composed: true,
    clientX: point.x,
    clientY: point.y,
  });
  Object.defineProperty(event, "dataTransfer", { value: dataTransfer, configurable: true });
  return element.dispatchEvent(event);
}

function zonePoint(): { x: number; y: number } {
  const rect = zoneEl.getBoundingClientRect();
  return { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
}

/**
 * A `DataTransfer` carrying real engine `File` objects.
 *
 * `items.add(file)` is what makes `types` report `Files` and `items[i].kind`
 * report `file` — the engine's own answers, not a stub's.
 */
function transferWith(files: Array<{ name: string; type: string; bytes: number }>): DataTransfer {
  const dataTransfer = new DataTransfer();
  for (const file of files) {
    dataTransfer.items.add(new File([new Uint8Array(file.bytes)], file.name, { type: file.type }));
  }
  return dataTransfer;
}

const host = {
  state: () => ({ ...log, phase: controller.getSnapshot().phase }),
  probe: () => ({
    phase: probe.dataset.phase ?? "",
    posture: probe.dataset.posture ?? "",
    reason: probe.dataset.reason ?? "",
    export: probe.dataset.export ?? "",
    exportName: probe.dataset.exportName ?? "",
    draggable: probe.dataset.draggable ?? "",
    offered: probe.dataset.offered ?? "",
    names: probe.dataset.names ?? "",
  }),

  /** Answer the pending export preparation, or decline it. */
  arm(receiptId: string | null, fileCount = 1): void {
    const settle = settlePrepare;
    settlePrepare = null;
    settle?.(
      receiptId === null
        ? null
        : ({
            receiptId,
            displayName: "take-01.wav",
            form: "materialized-file",
            fileCount,
          } as never),
    );
  },

  /** Start the browser's own drag on the export source. */
  startNativeDrag(): { prevented: boolean; types: string[] } {
    const dataTransfer = new DataTransfer();
    const event = new DragEvent("dragstart", { bubbles: true, cancelable: true, composed: true });
    Object.defineProperty(event, "dataTransfer", { value: dataTransfer, configurable: true });
    const proceeded = clipEl.dispatchEvent(event);
    return { prevented: !proceeded, types: [...dataTransfer.types] };
  },

  endNativeDrag(): void {
    const event = new DragEvent("dragend", { bubbles: true, cancelable: true, composed: true });
    Object.defineProperty(event, "dataTransfer", {
      value: new DataTransfer(),
      configurable: true,
    });
    clipEl.dispatchEvent(event);
  },

  reportExport(terminal: DragExportTerminal): void {
    terminalSink?.(terminal);
  },

  /** Hover the zone with real files. Returns whether the page claimed it. */
  hoverFiles(files: Array<{ name: string; type: string; bytes: number }>): {
    claimed: boolean;
    types: string[];
    kinds: string[];
  } {
    const dataTransfer = transferWith(files);
    const point = zonePoint();
    fileDrag("dragenter", dataTransfer, point);
    const claimed = !fileDrag("dragover", dataTransfer, point);
    return {
      claimed,
      types: [...dataTransfer.types],
      kinds: [...dataTransfer.items].map((item) => item.kind),
    };
  },

  dropFiles(files: Array<{ name: string; type: string; bytes: number }>): void {
    const dataTransfer = transferWith(files);
    fileDrag("drop", dataTransfer, zonePoint());
  },

  /** Leave the element, then leave the window. */
  leaveOnce(): void {
    fileDrag("dragleave", transferWith([{ name: "a.wav", type: "audio/wav", bytes: 8 }]), {
      x: 0,
      y: 0,
    });
  },

  heldBatches: () => inboundBridge.heldBatches(),
  hostPath: () => HOST_PATH,
};

(window as unknown as { __poodleFiles: typeof host }).__poodleFiles = host;

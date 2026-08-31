/**
 * The browser's own file drag, as a Poodle inbound host bridge.
 *
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md — Inbound Files.
 *
 * This is the whole of what Poodle knows about `File`, `DataTransfer`, and
 * HTML drag events on the inbound side. It turns a webview's file drag into
 * opaque receipts plus display metadata, and it keeps the `File` objects on
 * this side of the boundary: a consumer that genuinely needs them supplies its
 * own `project`, and one that does not never sees them at all. Nothing
 * downstream — target registration, eligibility, commit, snapshot — learns
 * that a browser was involved.
 *
 * Three platform facts shape everything here:
 *
 * 1. `dragover` exposes item *kinds and declared types only*. Names and sizes
 *    are hidden until `drop`, so the hover batch says `null` for both rather
 *    than inventing them, and the drop batch is validated again.
 * 2. `dragenter` and `dragleave` fire per element, not per window. A depth
 *    counter is the only reliable way to see the drag actually leave.
 * 3. A file dropped on a page that did not call `preventDefault` navigates
 *    the window to that file. A surface with an inbound bridge installed has
 *    claimed the gesture, so the default is prevented for the whole document
 *    while a batch is live — a missed target must leave the application
 *    standing, not replace it.
 */

import {
  type InboundFileBatch,
  type InboundFileCapabilities,
  type InboundFileDomBridge,
  type InboundFileEvent,
  type InboundFileOutcome,
  type InboundFileReceipt,
} from "../external-file-drag";

export interface InboundFileDataTransferOptions<T> {
  /** Whether this window will take more than one file at a time. */
  readonly multipleFiles?: boolean;
  /** Custom types this window is willing to consider beside files. */
  readonly customDataTypes?: readonly string[];
  /**
   * The consumer's own view of a dropped file.
   *
   * Absent, the batch resolves to its receipts and a `File` never leaves this
   * module. Present, the consumer has explicitly asked for whatever it needs
   * — a `File`, an object URL, an upload task — and owns that value's
   * lifetime. Poodle neither stores nor interprets the result.
   */
  readonly project?: (file: File, receipt: InboundFileReceipt) => T;
}

export interface InboundFileDataTransferBridge<T = InboundFileReceipt>
  extends InboundFileDomBridge {
  /**
   * The consumer-authored projection of one batch, valid until the batch is
   * released. `null` once released, or for a batch this bridge never held.
   */
  resolve(batchId: string): readonly T[] | null;
  /** Live batches this bridge is still holding. Cleanup evidence. */
  heldBatches(): readonly string[];
}

function carriesFiles(dataTransfer: DataTransfer | null): boolean {
  if (!dataTransfer) return false;
  if (Array.from(dataTransfer.types).includes("Files")) return true;
  return Array.from(dataTransfer.items ?? []).some((item) => item.kind === "file");
}

/**
 * The hover view: one receipt per file item, with only what `dragover`
 * actually discloses.
 */
function describeHover(dataTransfer: DataTransfer, batchId: string): InboundFileReceipt[] {
  const items = Array.from(dataTransfer.items ?? []).filter((item) => item.kind === "file");
  if (items.length > 0) {
    return items.map((item, index) => ({
      receiptId: `${batchId}:${index}`,
      name: null,
      mediaType: item.type ?? "",
      size: null,
    }));
  }
  // Some engines expose `types` but no `items` collection during `dragover`.
  // One undisclosed file is a truthful floor: the count is corrected at drop,
  // where the platform finally answers.
  return [{ receiptId: `${batchId}:0`, name: null, mediaType: "", size: null }];
}

function describeDrop(files: readonly File[], batchId: string): InboundFileReceipt[] {
  return files.map((file, index) => ({
    receiptId: `${batchId}:${index}`,
    name: file.name,
    mediaType: file.type,
    size: file.size,
  }));
}

/**
 * Create the inbound bridge for a webview's own file drag.
 *
 * The returned bridge advertises `transport: "data-transfer"`, which is an
 * exclusive claim: a controller given this bridge will not also listen to a
 * native host's file-drop capture. A shell whose platform delivers both must
 * choose one and say so, rather than taking one gesture as two drops.
 */
export function createInboundFileDataTransferBridge<T = InboundFileReceipt>(
  options: InboundFileDataTransferOptions<T> = {},
): InboundFileDataTransferBridge<T> {
  const capabilities: InboundFileCapabilities = {
    files: true,
    multipleFiles: options.multipleFiles ?? true,
    transport: "data-transfer",
    customDataTypes: options.customDataTypes ?? [],
  };

  let listener: ((event: InboundFileEvent) => void) | null = null;
  let connected: Document | null = null;
  let depth = 0;
  let issued = 0;
  let liveBatchId: string | null = null;
  const held = new Map<string, readonly T[]>();

  function emit(event: InboundFileEvent): void {
    listener?.(event);
  }

  function endLive(): void {
    const batchId = liveBatchId;
    liveBatchId = null;
    depth = 0;
    if (batchId !== null) emit({ type: "cancelled", batchId });
  }

  function onDragEnter(event: Event): void {
    if (!(event instanceof DragEvent)) return;
    depth += 1;
    if (!carriesFiles(event.dataTransfer) || liveBatchId !== null || !event.dataTransfer) return;

    issued += 1;
    const batchId = `inbound-${issued}`;
    liveBatchId = batchId;
    const batch: InboundFileBatch = {
      batchId,
      transport: "data-transfer",
      files: describeHover(event.dataTransfer, batchId),
    };
    emit({ type: "entered", batch, x: event.clientX, y: event.clientY });
  }

  function onDragOver(event: Event): void {
    if (!(event instanceof DragEvent) || liveBatchId === null) return;
    if (!carriesFiles(event.dataTransfer)) return;

    // Claimed: without this the browser refuses the drop and then navigates to
    // the file, destroying the surface the user was dragging onto. Poodle's
    // own target posture — not the OS cursor — is where a refusal is shown.
    event.preventDefault();
    if (event.dataTransfer) event.dataTransfer.dropEffect = "copy";
    emit({ type: "moved", batchId: liveBatchId, x: event.clientX, y: event.clientY });
  }

  function onDragLeave(event: Event): void {
    if (!(event instanceof DragEvent)) return;
    depth = Math.max(0, depth - 1);
    // Per-element leaves are noise; only the last one is the drag leaving the
    // window, and a session that ended on the first would die crossing a row.
    if (depth === 0) endLive();
  }

  function onDrop(event: Event): void {
    if (!(event instanceof DragEvent) || liveBatchId === null) return;
    const dataTransfer = event.dataTransfer;
    if (!carriesFiles(dataTransfer) || !dataTransfer) return;

    event.preventDefault();
    const batchId = liveBatchId;
    liveBatchId = null;
    depth = 0;

    const files = Array.from(dataTransfer.files ?? []);
    const receipts = describeDrop(files, batchId);
    const project = options.project;
    held.set(
      batchId,
      project
        ? files.map((file, index) => project(file, receipts[index] as InboundFileReceipt))
        : (receipts as readonly unknown[] as readonly T[]),
    );

    emit({
      type: "dropped",
      batch: { batchId, transport: "data-transfer", files: receipts },
      x: event.clientX,
      y: event.clientY,
    });
  }

  return {
    capabilities,

    connect(document) {
      if (connected) throw new Error("Inbound file bridge is already connected");
      connected = document;
      document.addEventListener("dragenter", onDragEnter, true);
      document.addEventListener("dragover", onDragOver, true);
      document.addEventListener("dragleave", onDragLeave, true);
      document.addEventListener("drop", onDrop, true);

      return () => {
        document.removeEventListener("dragenter", onDragEnter, true);
        document.removeEventListener("dragover", onDragOver, true);
        document.removeEventListener("dragleave", onDragLeave, true);
        document.removeEventListener("drop", onDrop, true);
        connected = null;
        liveBatchId = null;
        depth = 0;
        // Disconnecting is not a drop: whatever a consumer projected is
        // released with the listeners that could have used it.
        held.clear();
      };
    },

    subscribe(next) {
      listener = next;
      return () => {
        if (listener === next) listener = null;
      };
    },

    release(batchId: string, _outcome: InboundFileOutcome) {
      // The outcome is not this adapter's business: a browser `File` is not an
      // artifact Poodle created and not one it may delete, so every outcome
      // releases the same reference and nothing else happens. A native host
      // that *did* materialize something is the one that decides retention.
      held.delete(batchId);
      if (liveBatchId === batchId) {
        liveBatchId = null;
        depth = 0;
      }
    },

    resolve(batchId) {
      return held.get(batchId) ?? null;
    },

    heldBatches() {
      return [...held.keys()];
    },
  };
}

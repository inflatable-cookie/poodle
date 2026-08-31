/**
 * The fake host both frameworks' external-file specimens run against.
 *
 * It holds what a real shell would hold — a temporary path, the browser's
 * files — and hands Poodle only receipts and display names. That split is what
 * makes the specimen tests able to assert the thing that matters: the path the
 * host knows never appears anywhere in the rendered surface.
 */

import type {
  DragExportBridge,
  DragExportTerminal,
  InboundFileBatch,
  InboundFileEvent,
  InboundFileHostBridge,
  InboundFileOutcome,
  InboundFileReceipt,
} from "@inflatable-cookie/poodle-core";

/** The location only the host is allowed to know. */
export const HOST_PATH = "/var/tmp/poodle-42/take-01.wav";

export function createExportHost() {
  const cancels: string[] = [];
  let terminal: ((terminal: DragExportTerminal) => void) | null = null;

  const bridge: DragExportBridge = {
    capabilities: { files: true, multipleFiles: false, promisedFiles: false, customDataTypes: [] },
    prepare() {
      // A real host renders the clip to `HOST_PATH` here and answers with an
      // identifier for it. The path stays on this side.
      return Promise.resolve({
        receiptId: "export-1",
        displayName: "take-01.wav",
        form: "materialized-file" as const,
      });
    },
    start(_prepared, onTerminal) {
      terminal = onTerminal;
      return () => {
        terminal = null;
      };
    },
    cancel(prepared) {
      cancels.push(prepared.receiptId);
    },
  };

  return {
    bridge,
    cancels,
    report(value: DragExportTerminal) {
      terminal?.(value);
    },
  };
}

export function createInboundHost() {
  const released: Array<{ batchId: string; outcome: InboundFileOutcome }> = [];
  let listener: ((event: InboundFileEvent) => void) | null = null;

  const bridge: InboundFileHostBridge = {
    capabilities: { files: true, multipleFiles: true, transport: "host", customDataTypes: [] },
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

  return {
    bridge,
    released,
    send(event: InboundFileEvent) {
      listener?.(event);
    },
  };
}

export function inboundFile(name: string, size = 1_024): InboundFileReceipt {
  return { receiptId: `batch-1:${name}`, name, mediaType: "audio/wav", size };
}

export function inboundBatch(files: InboundFileReceipt[]): InboundFileBatch {
  return { batchId: "batch-1", transport: "host", files };
}

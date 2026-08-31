/**
 * The bounded web codec for the cross-window receipt.
 *
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md — Native
 * DataTransfer Adapter.
 *
 * `DataTransfer` is a wire, not a session store. This module writes exactly
 * `{ protocolVersion, token }` and reads exactly that back; there is no place
 * to put a panel record, geometry, an event, or a mutable session, because
 * there is no field for one. The live target bridge stays hover authority —
 * the browser hides the body during `dragover` and exposes only the types
 * list, so `accepts` can answer nothing more than "this drag carries our
 * envelope".
 *
 * Everything read here is untrusted: a page in another origin can put any
 * string under any MIME type. Size, shape, version, and token bounds are all
 * checked before the value is handed to anything that could act on it.
 */

import {
  CROSS_WINDOW_DRAG_MIME_TYPE,
  CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
  isCrossWindowDragReceipt,
  type CrossWindowDragReceipt,
} from "../cross-window-drag";

/**
 * The largest envelope this codec will parse.
 *
 * A valid envelope is under a hundred bytes. Reading megabytes of hostile
 * JSON to discover it is not a receipt is the denial the bound exists to
 * prevent, so the length is checked before `JSON.parse` runs.
 */
export const CROSS_WINDOW_DRAG_MAX_ENVELOPE_LENGTH = 1024;

export interface CrossWindowDataTransferAdapterOptions {
  /**
   * A host protocol may use its own MIME type. It cannot change the
   * normalized receipt shape, and it cannot make `DataTransfer` the session
   * store.
   */
  readonly mimeType?: string;
}

/** A minimal structural view of `DataTransfer`, so the codec stays testable. */
export interface CrossWindowDataTransferLike {
  readonly types: readonly string[];
  getData(format: string): string;
  setData(format: string, data: string): void;
}

export interface CrossWindowDataTransferAdapter {
  readonly mimeType: string;
  /**
   * Write the receipt during a native `dragstart`. Valid only there: outside
   * that event the browser refuses the write, and a codec that pretended
   * otherwise would report a payload nobody can read.
   */
  write(dataTransfer: CrossWindowDataTransferLike, receipt: CrossWindowDragReceipt): void;
  /**
   * The `dragover` question, and the only one answerable then: does this drag
   * declare our envelope? The body is unreadable during `dragover`, so this
   * cannot and must not claim the receipt matches a live projection.
   */
  accepts(dataTransfer: CrossWindowDataTransferLike): boolean;
  /**
   * The `drop` question. Returns the receipt only when the envelope is
   * present, bounded, well-formed, this protocol version, and carries a token
   * within bounds. Everything else is `null` — malformed, oversized, future,
   * and mismatched all refuse identically, because a partially-understood
   * receipt is not a receipt.
   */
  read(dataTransfer: CrossWindowDataTransferLike): CrossWindowDragReceipt | null;
}

export function createCrossWindowDataTransferAdapter(
  options: CrossWindowDataTransferAdapterOptions = {},
): CrossWindowDataTransferAdapter {
  const mimeType = options.mimeType ?? CROSS_WINDOW_DRAG_MIME_TYPE;

  return {
    mimeType,

    write(dataTransfer, receipt) {
      if (!isCrossWindowDragReceipt(receipt)) {
        throw new Error("Cross-window receipt is not writable: bad protocol version or token");
      }
      dataTransfer.setData(
        mimeType,
        JSON.stringify({
          protocolVersion: CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
          token: receipt.token,
        }),
      );
    },

    accepts(dataTransfer) {
      return dataTransfer.types.includes(mimeType);
    },

    read(dataTransfer) {
      if (!dataTransfer.types.includes(mimeType)) return null;

      const raw = dataTransfer.getData(mimeType);
      if (typeof raw !== "string" || raw.length === 0) return null;
      if (raw.length > CROSS_WINDOW_DRAG_MAX_ENVELOPE_LENGTH) return null;

      let parsed: unknown;
      try {
        parsed = JSON.parse(raw);
      } catch {
        return null;
      }

      if (!isCrossWindowDragReceipt(parsed)) return null;

      // Normalized, not passed through: the returned value carries exactly the
      // two fields, so an envelope with extra keys cannot smuggle them onward.
      return {
        protocolVersion: parsed.protocolVersion,
        token: parsed.token,
      };
    },
  };
}

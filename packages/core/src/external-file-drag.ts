/**
 * External-file boundaries — inbound files and native file drag-out.
 *
 * Architecture: docs/architecture/011-drag-and-drop-substrate.md.
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md.
 * Rust mirror: packages/contracts/headless/src/external_file_drag.rs.
 *
 * Two directions, one rule: the artifact never crosses into Poodle. Going
 * out, a consumer names an opaque subject and its host answers with a
 * *receipt* — never a path, descriptor, temp directory, or `File`. Coming in,
 * the host resolves the platform's own drag into *receipts* plus display
 * metadata, and Poodle validates that metadata before any target is allowed
 * to say yes.
 *
 * Nothing here imports Electron, Tauri, a shell, or a filesystem, and nothing
 * here deletes anything. Materialization, naming, retention, and cleanup are
 * host policy; Poodle reports lifecycle and presents state. A native drag
 * ending is not permission to remove a temporary file — the destination may
 * still be reading it, and Poodle has no way to know whether it was consumed
 * at all.
 */

import type {
  DragCancelReason,
  DragOperation,
  DragSubject,
} from "./drag-drop";
import { fileTypeAccepted } from "./file-upload";

// ── Shared bounds ──────────────────────────────────────────────────────────

/**
 * The longest opaque receipt id this build will carry.
 *
 * A receipt is an identifier, not a payload. The same bound as the
 * cross-window token, for the same reason: an id long enough to smuggle a
 * document through is not an id.
 */
export const EXTERNAL_FILE_MAX_RECEIPT_LENGTH = 512;

/** The longest display name this build will present. */
export const EXTERNAL_FILE_MAX_NAME_LENGTH = 255;

/** The most files one inbound batch or prepared export may name. */
export const EXTERNAL_FILE_MAX_COUNT = 1024;

/**
 * Whether a name is presentable rather than a location.
 *
 * The whole point of the boundary is that Poodle never receives a path, and
 * `displayName` is the one field with a plausible-looking excuse to carry
 * one. A separator, a drive letter, a parent-directory hop, or a URL scheme
 * means the host handed over a location, so the value is refused rather than
 * trimmed down to its last segment — quietly presenting `secret.wav` for
 * `/Users/tom/private/secret.wav` would hide the leak instead of stopping it.
 */
export function isPresentableFileName(name: string): boolean {
  if (name.length === 0 || name.length > EXTERNAL_FILE_MAX_NAME_LENGTH) return false;
  if (name.includes("/") || name.includes("\\")) return false;
  if (name.includes("\0")) return false;
  if (name === "." || name === "..") return false;
  // `C:` and `file:` are locations wearing a name's clothes.
  if (/^[a-zA-Z]:/.test(name) || /^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(name)) return false;
  return name.trim().length > 0;
}

function isOpaqueId(value: string): boolean {
  return value.length > 0 && value.length <= EXTERNAL_FILE_MAX_RECEIPT_LENGTH;
}

// ── Native file drag-out ───────────────────────────────────────────────────

/**
 * What this host can actually carry out of the surface.
 *
 * Resolved once per adapter rather than negotiated per gesture, and never
 * inferred from a user agent or a shell name. `files` is the portable
 * baseline; the rest are advertised extensions, and a capability that is
 * false stays inert — the source keeps its ordinary local drag instead of
 * advertising a transfer that cannot happen.
 */
export interface DragExportCapabilities {
  readonly files: boolean;
  readonly multipleFiles: boolean;
  readonly promisedFiles: boolean;
  readonly customDataTypes: readonly string[];
}

/**
 * Which of the four distinct export forms a preparation produced.
 *
 * They are distinct because they have different costs and different
 * lifetimes, and collapsing them would let an adapter answer a capability it
 * does not have: an existing file needs no cleanup at all, a materialized one
 * is a temporary artifact the host must retain past the gesture, a promised
 * one is not written until the destination asks, and custom data is not a
 * file in the first place.
 */
export type DragExportForm =
  | "existing-file"
  | "materialized-file"
  | "promised-file"
  | "custom-data";

/**
 * The armed export. Opaque by construction.
 *
 * `receiptId` is the host's own name for whatever it prepared; Poodle
 * compares it, hands it back, and never parses it. `displayName` is
 * presentation only and is refused when it looks like a location. There is no
 * field for a path, a descriptor, a directory, or a byte, because there is no
 * honest use for one on this side of the boundary.
 */
export interface PreparedFileExport {
  readonly receiptId: string;
  readonly displayName?: string;
  readonly form: DragExportForm;
  /** How many files this receipt stands for. Defaults to one. */
  readonly fileCount?: number;
  /** Declared types for `custom-data`; every one must be advertised. */
  readonly dataTypes?: readonly string[];
}

/** What Poodle knows when it asks a host to prepare an export. */
export interface DragExportPrepareRequest {
  readonly sessionId: string;
  readonly sourceId: string;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
  readonly allowedOperations: readonly DragOperation[];
}

/**
 * The end of a native drag-out, in the only three qualities that are honest.
 *
 * There is deliberately no "committed": a native drag ending does not prove a
 * destination consumed anything, and neither `dropEffect` nor an OS drag
 * operation is authority Poodle is willing to relay. `ended` says the gesture
 * finished and nothing more; whether a DAW took the file is downstream
 * evidence, not a callback.
 */
export type DragExportTerminal =
  | { readonly status: "ended" }
  | { readonly status: "cancelled"; readonly reason: DragCancelReason }
  | { readonly status: "failed"; readonly reason?: string };

/**
 * Per draggable source. Optional on one
 * `DragSourceRegistration.fileExportBridge`.
 *
 * `prepare` runs on the accepted pre-drag gesture, *before* activation,
 * because rendering a clip or writing a temporary file is not something that
 * can happen inside a synchronous native drag start. It is abortable for the
 * same reason: a preparation that is superseded or cancelled has to reach the
 * host so it can stop the work and release what it allocated.
 *
 * `start` is the moment the native drag begins. The host owns it —
 * `webContents.startDrag`, a Tauri plugin, an `NSDraggingSession` — and its
 * terminal callback is the only thing that ends the export. `cancel` runs
 * only while the receipt is still live and never after a terminal, so one
 * receipt receives exactly one closing command.
 *
 * Neither call authorizes deletion. Retention and cleanup stay with the host
 * that made the artifact.
 */
export interface DragExportBridge {
  readonly capabilities: DragExportCapabilities;
  prepare(
    request: DragExportPrepareRequest,
    signal: AbortSignal,
  ): Promise<PreparedFileExport | null>;
  start(
    prepared: PreparedFileExport,
    onTerminal: (terminal: DragExportTerminal) => void,
  ): () => void;
  cancel(prepared: PreparedFileExport, reason: DragCancelReason): void | Promise<void>;
}

/** Why a prepared export cannot arm a native drag. */
export type DragExportRefusal =
  | "no-receipt"
  | "files-unsupported"
  | "multiple-files-unsupported"
  | "promised-files-unsupported"
  | "custom-data-unsupported"
  | "count-out-of-range"
  | "name-is-a-path";

export type DragExportValidation =
  | { readonly accepted: true; readonly prepared: PreparedFileExport }
  | { readonly accepted: false; readonly reason: DragExportRefusal };

/**
 * Check an armed receipt against what its own adapter said it could do.
 *
 * The adapter is not the adversary here so much as the drift: a host that
 * advertises single files and then returns three, or advertises no promised
 * files and then returns a promise, has produced a drag that will fail
 * somewhere far away from here. Refusing at the boundary keeps an unsupported
 * capability inert instead of half-armed, and the same check catches the one
 * shape that must never pass — a display name that is really a path.
 */
export function validateFileExport(
  prepared: PreparedFileExport,
  capabilities: DragExportCapabilities,
): DragExportValidation {
  if (!isOpaqueId(prepared.receiptId)) {
    return { accepted: false, reason: "no-receipt" };
  }

  if (prepared.displayName !== undefined && !isPresentableFileName(prepared.displayName)) {
    return { accepted: false, reason: "name-is-a-path" };
  }

  const count = prepared.fileCount ?? 1;
  if (!Number.isInteger(count) || count < 1 || count > EXTERNAL_FILE_MAX_COUNT) {
    return { accepted: false, reason: "count-out-of-range" };
  }

  if (prepared.form === "custom-data") {
    const types = prepared.dataTypes ?? [];
    if (types.length === 0) {
      return { accepted: false, reason: "custom-data-unsupported" };
    }
    // Both sides opt in explicitly: the adapter by advertising the type, the
    // consumer by asking for it. One alone is not a negotiated format.
    for (const type of types) {
      if (!capabilities.customDataTypes.includes(type)) {
        return { accepted: false, reason: "custom-data-unsupported" };
      }
    }
    return { accepted: true, prepared };
  }

  if (!capabilities.files) {
    return { accepted: false, reason: "files-unsupported" };
  }
  if (count > 1 && !capabilities.multipleFiles) {
    return { accepted: false, reason: "multiple-files-unsupported" };
  }
  if (prepared.form === "promised-file" && !capabilities.promisedFiles) {
    return { accepted: false, reason: "promised-files-unsupported" };
  }

  return { accepted: true, prepared };
}

/** Whether this adapter can export anything at all. */
export function canExportAnything(capabilities: DragExportCapabilities): boolean {
  return capabilities.files || capabilities.customDataTypes.length > 0;
}

/**
 * The export's own visible lifecycle, beside the semantic session phase.
 *
 * The session says what the *drag* is doing; this says what the *artifact* is
 * doing, and they are not the same story. A source whose host cannot export
 * is `unavailable` before any gesture exists, a slow materialization is
 * `preparing` while the pointer is already down, and `ended` is the honest
 * close of a drag that left for the operating system — where the kernel, which
 * only knows about Poodle targets, correctly records that nothing local
 * committed.
 */
export type DragExportState =
  | "unavailable"
  | "idle"
  | "preparing"
  | "armed"
  | "dragging"
  | "ended"
  | "cancelled"
  | "failed";

/** An immutable presentation read of the current export. */
export interface DragExportSnapshot {
  readonly state: DragExportState;
  readonly form: DragExportForm | null;
  readonly fileCount: number;
  /** Presentation only, and never a path. */
  readonly displayName: string | null;
  /** A refusal or failure reason suitable for presentation. */
  readonly reason: string | null;
}

// ── Inbound files ──────────────────────────────────────────────────────────

/**
 * The subject family every inbound external drag uses.
 *
 * A target opts into external files by accepting this kind, exactly the way
 * it opts into any other kind. There is no second eligibility path, no
 * `onFileDrop`, and no bypass: an inbound file reaches `canDrop` and `onDrop`
 * through the same arbitration as a reordered row.
 */
export const INBOUND_FILE_SUBJECT_KIND = "poodle.external-file";

/**
 * Which transport owns inbound files in this window.
 *
 * Not a preference — an exclusive claim. A Tauri window's native file-drop
 * capture and the webview's own HTML drag events can both be live at once on
 * some platforms, and a surface that enabled both would take one user gesture
 * as two drops. The host declares the owner and Poodle listens to exactly
 * that one.
 */
export type InboundFileTransport = "data-transfer" | "host";

export interface InboundFileCapabilities {
  readonly files: boolean;
  readonly multipleFiles: boolean;
  readonly transport: InboundFileTransport;
  readonly customDataTypes: readonly string[];
}

/**
 * One inbound file, as far as Poodle is ever allowed to know it.
 *
 * A host-issued opaque id, a display name, a declared media type, and a size.
 * `name` and `size` are `null` while the platform is still hiding them — a
 * browser exposes nothing but item kinds and declared types during
 * `dragover`, so a hover-time batch can honestly answer count and type
 * questions and cannot answer name or size ones.
 *
 * The unknown is modelled rather than guessed. Inventing `"file"` and `0`
 * would make a hover refusal or acceptance that the drop then contradicts;
 * `null` defers exactly the rules that cannot be decided yet, and the full
 * check runs again at drop where every answer exists.
 */
export interface InboundFileReceipt {
  readonly receiptId: string;
  readonly name: string | null;
  readonly mediaType: string;
  readonly size: number | null;
}

/** One inbound gesture's files, named by one host-issued batch id. */
export interface InboundFileBatch {
  readonly batchId: string;
  readonly transport: InboundFileTransport;
  readonly files: readonly InboundFileReceipt[];
}

/**
 * What one target will take. Declared per target, not per window.
 *
 * `accept` is the same vocabulary the file-upload surfaces already use
 * (`.ext`, `type/*`, an exact media type, or `*`), because a consumer should
 * not have to learn a second one to describe the same thing.
 */
export interface InboundFileConstraints {
  readonly maxFiles?: number;
  /** Per file, in bytes. */
  readonly maxSize?: number;
  readonly accept?: string;
}

/** Why an inbound batch cannot be offered to a target. */
export type InboundFileRefusal =
  | "files-unsupported"
  | "empty"
  | "malformed"
  | "unidentified"
  | "name-is-a-path"
  | "too-many"
  | "too-large"
  | "unsupported-type";

export type InboundFileValidation =
  | { readonly accepted: true; readonly batch: InboundFileBatch }
  | { readonly accepted: false; readonly reason: InboundFileRefusal };

/**
 * Validate an inbound batch before any target is asked whether it wants it.
 *
 * External data is untrusted input: the count, the sizes, the declared types,
 * the names, and the host's own identifiers all arrive from outside and are
 * all checked here, before eligibility, before hover posture, and again
 * before commit. A batch that fails is refused with a reason a surface can
 * announce — not dropped silently, and not passed through for `canDrop` to
 * discover.
 *
 * A `null` size passes the size rule rather than failing it: at hover the
 * platform has not said, and refusing an unknown would reject every browser
 * file drag before it could be inspected. The drop-time batch carries real
 * sizes and is validated again, which is where an oversized file is caught.
 */
export function validateInboundFiles(
  batch: InboundFileBatch,
  constraints: InboundFileConstraints,
  capabilities: InboundFileCapabilities,
): InboundFileValidation {
  if (!capabilities.files) {
    return { accepted: false, reason: "files-unsupported" };
  }
  if (batch.transport !== capabilities.transport) {
    // A batch from a transport this window did not hand ownership to is not a
    // batch this window agreed to receive.
    return { accepted: false, reason: "malformed" };
  }
  if (!isOpaqueId(batch.batchId)) {
    return { accepted: false, reason: "unidentified" };
  }
  if (batch.files.length === 0) {
    return { accepted: false, reason: "empty" };
  }
  if (batch.files.length > EXTERNAL_FILE_MAX_COUNT) {
    return { accepted: false, reason: "too-many" };
  }

  const maxFiles = constraints.maxFiles ?? (capabilities.multipleFiles ? undefined : 1);
  if (maxFiles !== undefined && batch.files.length > maxFiles) {
    return { accepted: false, reason: "too-many" };
  }
  if (batch.files.length > 1 && !capabilities.multipleFiles) {
    return { accepted: false, reason: "too-many" };
  }

  const seen = new Set<string>();
  for (const file of batch.files) {
    if (!isOpaqueId(file.receiptId) || seen.has(file.receiptId)) {
      return { accepted: false, reason: "unidentified" };
    }
    seen.add(file.receiptId);

    if (file.name !== null && !isPresentableFileName(file.name)) {
      return { accepted: false, reason: "name-is-a-path" };
    }
    if (typeof file.mediaType !== "string" || file.mediaType.length > EXTERNAL_FILE_MAX_NAME_LENGTH) {
      return { accepted: false, reason: "malformed" };
    }
    if (file.size !== null && (!Number.isFinite(file.size) || file.size < 0)) {
      return { accepted: false, reason: "malformed" };
    }
    if (
      constraints.maxSize !== undefined &&
      file.size !== null &&
      file.size > constraints.maxSize
    ) {
      return { accepted: false, reason: "too-large" };
    }
    if (constraints.accept !== undefined && !acceptsInboundFile(constraints.accept, file)) {
      return { accepted: false, reason: "unsupported-type" };
    }
  }

  return { accepted: true, batch };
}

/**
 * The accept rule over a receipt that may still be half-disclosed.
 *
 * With a name in hand this is the ordinary file-upload matcher. Without one —
 * hover — an extension rule cannot be decided, and neither can any rule at all
 * when the platform declared no media type. Undecidable defers to the drop
 * check rather than guessing either way: refusing would show a refusal the
 * drop contradicts, and matching would claim a rule was satisfied when nothing
 * was compared.
 */
function acceptsInboundFile(accept: string, file: InboundFileReceipt): boolean {
  if (file.name !== null) return fileTypeAccepted(accept, file.name, file.mediaType);

  return accept
    .split(",")
    .map((token) => token.trim())
    .some((token) => {
      if (token === "*") return true;
      // An extension rule needs the name the platform has not disclosed.
      if (token.startsWith(".")) return true;
      if (file.mediaType.length === 0) return true;
      if (token.endsWith("/*")) return file.mediaType.startsWith(token.slice(0, -1));
      return file.mediaType === token;
    });
}

/**
 * What the host tells this window about an external drag it is carrying.
 *
 * Coordinates are the window's own client space, because the host is the only
 * thing that observed the pointer — a native file drag delivers no Pointer
 * Events to the page it is over. Poodle hit-tests its own registered targets
 * with them and arbitrates exactly as it would for a local gesture.
 */
export type InboundFileEvent =
  | {
      readonly type: "entered";
      readonly batch: InboundFileBatch;
      readonly x: number;
      readonly y: number;
    }
  | { readonly type: "moved"; readonly batchId: string; readonly x: number; readonly y: number }
  | {
      readonly type: "dropped";
      /** The final batch: names, types, and the sizes hover could not see. */
      readonly batch: InboundFileBatch;
      readonly x: number;
      readonly y: number;
    }
  | { readonly type: "cancelled"; readonly batchId: string };

/** How one inbound batch finished, from Poodle's side of the boundary. */
export type InboundFileOutcome = "committed" | "rejected" | "failed" | "cancelled";

/**
 * Per document or native window. Optional on one controller.
 *
 * `subscribe` is the host's live account of an external drag. `release` is
 * the single terminal notification, delivered exactly once per batch, and it
 * is a *notification*: the host decides whether the temporary copy it made
 * survives, whether a rejected batch is discarded, and when. Poodle does not
 * hold the files and does not remove them.
 */
export interface InboundFileHostBridge {
  readonly capabilities: InboundFileCapabilities;
  subscribe(listener: (event: InboundFileEvent) => void): () => void;
  release(batchId: string, outcome: InboundFileOutcome): void;
}

/**
 * A host bridge that needs the connected document to observe its transport.
 *
 * The DOM adapter is the only kind that does: a native host already sees the
 * pointer, while a webview's file drag is delivered as document events that
 * cannot be bound before the controller has a document to bind them to.
 */
export interface InboundFileDomBridge extends InboundFileHostBridge {
  connect(document: Document): () => void;
}

/** Whether this bridge binds document events rather than observing natively. */
export function isInboundFileDomBridge(
  bridge: InboundFileHostBridge,
): bridge is InboundFileDomBridge {
  return typeof (bridge as InboundFileDomBridge).connect === "function";
}

/**
 * Cross-window drag host bridge — the capability-based boundary between one
 * Poodle surface and a host that owns windows.
 *
 * Architecture: docs/architecture/011-drag-and-drop-substrate.md.
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md.
 * Rust mirror: packages/contracts/headless/src/cross_window_drag.rs.
 *
 * The bridge is split by ownership, and the split is the whole point. A
 * *source* preparation belongs to one draggable subject: it is armed, started,
 * and cancelled with that subject's gesture. Incoming *projection*, commit,
 * and accessible target picking belong to one document or native window: they
 * outlive any one subject and there may be no local source at all. Combining
 * them into one controller-wide object would tie two different lifetimes to
 * one handle, and a host would have to invent a null half.
 *
 * Only {@link CrossWindowDragReceipt} crosses the wire — a protocol version
 * and an opaque token. Everything else in this file is a *host-local
 * projection*: the host resolves a receipt into semantic values inside the
 * window that is going to render them. Poodle never serializes a subject, a
 * label, geometry, or a session, and never stores the authoritative
 * transaction. Leases, window geometry, target resolution, authorization,
 * mutation, rollback, expiry, and recovery are all host-owned.
 */

import type {
  DragCancelReason,
  DragDropCommitResult,
  DragOperation,
  DragSubject,
  DragTerminalOutcome,
  DropIntent,
  DropPosition,
} from "./drag-drop";

/**
 * What the host can actually observe, resolved once rather than negotiated per
 * gesture.
 *
 * A source does not advertise a cross-window affordance the host cannot carry:
 * `touch` is true only when the host can follow a touch contact *outside* the
 * source window, which a page's own Pointer Events cannot do. Internal
 * same-document touch is unaffected — it has its own capability report on the
 * controller.
 */
export interface CrossWindowDragCapabilities {
  readonly pointer: boolean;
  readonly touch: boolean;
  readonly keyboardTargetPicker: boolean;
}

/**
 * The entire portable payload of a cross-window drag.
 *
 * `token` is opaque: Poodle compares it, carries it, and hands it back. It is
 * not parsed, not a path, not a record, and not authority on its own — the
 * host resolves it against its own live transaction, which is what makes an
 * expired or forged token safe to receive.
 */
export interface CrossWindowDragReceipt {
  readonly protocolVersion: number;
  readonly token: string;
}

/**
 * How the host is carrying this gesture between windows.
 *
 * `data-transfer` is the browser's own drag, with the receipt written into a
 * bounded envelope. `window-capture` is a host that follows the OS pointer
 * itself. `keyboard-picker` is the accessible route, which never has a
 * pointer at all.
 */
export type CrossWindowDragTransport = "data-transfer" | "window-capture" | "keyboard-picker";

/** What Poodle knows when it asks the host to allocate a transaction. */
export interface CrossWindowDragPrepareRequest {
  readonly sessionId: string;
  readonly sourceId: string;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
  readonly allowedOperations: readonly DragOperation[];
}

/**
 * The host's answer to "what is over this window right now".
 *
 * Every field is resolved by the host *in the receiving window* — none of it
 * travels beside the receipt. `targetId` names at most one registered Poodle
 * target, so the projection can never produce two simultaneous drops, and
 * Poodle still re-runs that target's own kind, disabled, and `canDrop` gates
 * before anything commits.
 */
export interface CrossWindowDragProjection {
  readonly receipt: CrossWindowDragReceipt;
  readonly sourceId: string;
  readonly sourceLabel: string;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
  readonly inputKind: "pointer" | "touch" | "keyboard";
  readonly targetId: string | null;
  readonly position: DropPosition | null;
}

/**
 * What a window-owned bridge publishes to its subscriber.
 *
 * There is no "dropped" event: a drop is a local observation (the native drop
 * envelope, or the keyboard picker's own commit), and the host answers it
 * through {@link CrossWindowDragTargetBridge.commit}. Making the host announce
 * the drop as well would give one gesture two commit paths.
 */
export type CrossWindowDragTargetEvent =
  | { readonly type: "projection"; readonly projection: CrossWindowDragProjection }
  | { readonly type: "left"; readonly receipt: CrossWindowDragReceipt }
  | {
      readonly type: "cancelled";
      readonly receipt: CrossWindowDragReceipt;
      readonly reason: DragCancelReason;
    };

/** The one revalidated drop Poodle asks the host to make durable. */
export interface CrossWindowDragCommitRequest {
  readonly receipt: CrossWindowDragReceipt;
  readonly subject: DragSubject;
  readonly intent: DropIntent;
}

/**
 * Per draggable source. Optional on one
 * `DragSourceRegistration.crossWindowSourceBridge`.
 *
 * `prepare` runs on the accepted pre-drag gesture, *before* activation, so a
 * host that must allocate a lease has somewhere to do it that is not inside a
 * synchronous native `dragstart`. A source with this bridge cannot advertise
 * or start a cross-window gesture until its own receipt is armed; a decline or
 * failure cancels only that attempt.
 *
 * `start` installs the one authoritative terminal subscription and returns its
 * cleanup. Native `dragend`, pointer release, and `dropEffect` never
 * manufacture a committed result — only `onTerminal` does.
 */
export interface CrossWindowDragSourceBridge {
  readonly capabilities: CrossWindowDragCapabilities;
  prepare(
    request: CrossWindowDragPrepareRequest,
    signal: AbortSignal,
  ): Promise<CrossWindowDragReceipt | null>;
  start(
    receipt: CrossWindowDragReceipt,
    transport: CrossWindowDragTransport,
    onTerminal: (outcome: DragTerminalOutcome) => void,
  ): () => void;
  cancel(receipt: CrossWindowDragReceipt, reason: DragCancelReason): void | Promise<void>;
}

/**
 * Per document or native window. Optional on one controller.
 *
 * `subscribe` is live host projection; `commit` is the authoritative durable
 * step, run only after Poodle revalidates the exact live target the projection
 * named. `pickTarget` is required exactly when `keyboardTargetPicker` is true
 * and reaches the same revalidation, commit, announcement, and terminal path
 * as the pointer route — a second keyboard-only callback would be a second
 * transaction.
 */
export interface CrossWindowDragTargetBridge {
  readonly capabilities: CrossWindowDragCapabilities;
  subscribe(listener: (event: CrossWindowDragTargetEvent) => void): () => void;
  commit(
    request: CrossWindowDragCommitRequest,
    signal: AbortSignal,
  ): Promise<DragDropCommitResult>;
  pickTarget?(
    receipt: CrossWindowDragReceipt,
    signal: AbortSignal,
  ): Promise<CrossWindowDragProjection | null>;
}

/** The protocol version this build writes and the only one it accepts. */
export const CROSS_WINDOW_DRAG_PROTOCOL_VERSION = 1;

/** The default bounded envelope MIME type. */
export const CROSS_WINDOW_DRAG_MIME_TYPE = "application/x-poodle-cross-window-drag+json";

/**
 * The longest token this build will write or read.
 *
 * External data is untrusted and a `DataTransfer` body is attacker-shaped
 * input, so the codec bounds it before parsing rather than after. The limit is
 * generous for an opaque host id and small enough that a hostile payload
 * cannot be smuggled through as one.
 */
export const CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH = 512;

/**
 * Whether `value` is a receipt this build can carry.
 *
 * Deliberately strict: an unknown or future protocol version is rejected
 * rather than best-effort parsed, because a receipt Poodle cannot fully
 * understand is one it cannot honestly claim to have matched.
 */
export function isCrossWindowDragReceipt(value: unknown): value is CrossWindowDragReceipt {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as { protocolVersion?: unknown; token?: unknown };
  return (
    candidate.protocolVersion === CROSS_WINDOW_DRAG_PROTOCOL_VERSION &&
    typeof candidate.token === "string" &&
    candidate.token.length > 0 &&
    candidate.token.length <= CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH
  );
}

/** Two receipts name the same host transaction. */
export function sameCrossWindowDragReceipt(
  left: CrossWindowDragReceipt | null | undefined,
  right: CrossWindowDragReceipt | null | undefined,
): boolean {
  if (!left || !right) return false;
  return left.protocolVersion === right.protocolVersion && left.token === right.token;
}

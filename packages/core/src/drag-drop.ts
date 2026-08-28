/**
 * Drag-and-drop semantic kernel — the renderer-neutral drag session.
 *
 * Architecture: docs/architecture/011-drag-and-drop-substrate.md.
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md.
 * Rust mirror: packages/contracts/headless/src/drag_drop.rs.
 * Shared vectors: packages/contracts/headless/vectors/machines.json (`dragDrop`).
 *
 * The kernel owns lifecycle, session identity, semantic intent, cancellation,
 * nested-target arbitration, and exactly-once terminal effects. It owns nothing
 * else: no pointer, keyboard, DOM, GPUI, geometry, timer, transport, file, or
 * host vocabulary appears here. Adapters translate their platform into these
 * events and execute the effect intents.
 *
 * Exactly-once is a property of the phase, not of a flag: `emitDragStart` can
 * only be emitted on `armed -> dragging`, `requestDrop` only on
 * `dragging -> dropping`, and the terminal quartet only on the single
 * transition into `ended` or `cancelled`. A repeat of any of those events
 * arrives in a phase that no longer accepts it and is inert.
 *
 * Effects are intents, not payloads. `announce` carries only the announcement
 * kind because the adapter already holds the session (target, position,
 * operation) and, for a terminal announcement, the `emitDropResult` that
 * immediately precedes it.
 *
 * Session identity is caller-supplied and single-use. See `DragSession.sessionId`
 * — it is the one rule the kernel cannot enforce for you.
 */

import type { TransitionResult } from "./machine";

export type DragOperation = "move" | "copy" | "link";

/** The positions every target vocabulary shares; consumers may define more. */
export type StandardDropPosition = "before" | "inside" | "after";

export type DropPosition = StandardDropPosition | (string & {});

/**
 * The whole portable payload. `kind` selects a consumer-defined subject family
 * and `id` resolves the live subject through consumer state. Neither is
 * display text, a path, a record, or authority.
 */
export interface DragSubject {
  kind: string;
  id: string;
}

export interface DropIntent {
  targetId: string;
  position: DropPosition;
  operation: DragOperation;
}

export type DropEligibility =
  | { accepted: true; intent: DropIntent }
  | { accepted: false; reason?: string };

export type DragSessionPhase =
  | "idle"
  | "preparing"
  | "armed"
  | "dragging"
  | "dropping"
  | "ended"
  | "cancelled";

export type DragCancelReason =
  | "preparation-declined"
  | "preparation-failed"
  | "superseded"
  | "escape"
  | "explicit"
  | "source-lost"
  | "target-lost"
  | "transport-lost"
  | "window-lost";

/**
 * `ended` carries an authoritative drop result — committed, rejected, or
 * failed. `cancelled` carries the reason the session aborted without one.
 */
export type DragTerminalOutcome =
  | { status: "committed"; intent: DropIntent }
  | { status: "rejected"; reason?: string }
  | { status: "failed"; reason?: string }
  | { status: "cancelled"; reason: DragCancelReason };

export type DragAnnouncementKind =
  | "pickup"
  | "intentChanged"
  | "intentCleared"
  | "dropped"
  | "rejected"
  | "failed"
  | "cancelled";

export interface DragSession {
  /**
   * Caller-supplied and single-use. A `sessionId` must stay unique for as long
   * as any asynchronous completion created for it can still arrive — in
   * practice, for the lifetime of the surface. The kernel rejects a stale
   * completion by comparing this id, so it has no way to tell two sessions
   * apart that share one: reusing an id after a session ended, cancelled, and
   * reset lets a late `PREPARED` from the first arm the second. Mint a fresh
   * id for every `PREPARE`; never recycle one.
   */
  sessionId: string;
  sourceId: string;
  subject: DragSubject;
  operation: DragOperation;
  allowedOperations: readonly DragOperation[];
  intent: DropIntent | null;
}

export interface DragSessionContext {
  /** Present from `preparing` through the terminal phases; cleared by RESET. */
  session: DragSession | null;
}

export type DragSessionEvent =
  | {
      /** `sessionId` must be freshly minted; see `DragSession.sessionId`. */
      type: "PREPARE";
      sessionId: string;
      sourceId: string;
      subject: DragSubject;
      operation: DragOperation;
      allowedOperations: readonly DragOperation[];
    }
  | { type: "PREPARED"; sessionId: string }
  | { type: "PREPARE_DECLINED"; sessionId: string }
  | { type: "PREPARE_FAILED"; sessionId: string }
  | { type: "ACTIVATE"; sessionId: string }
  | { type: "TARGET_INTENT"; sessionId: string; intent: DropIntent }
  | { type: "TARGET_CLEARED"; sessionId: string }
  | { type: "OPERATION_CHANGED"; sessionId: string; operation: DragOperation }
  | { type: "DROP_REQUESTED"; sessionId: string }
  | { type: "DROP_COMMITTED"; sessionId: string; intent: DropIntent }
  | { type: "DROP_REJECTED"; sessionId: string; reason?: string }
  | { type: "DROP_FAILED"; sessionId: string; reason?: string }
  | { type: "ESCAPE"; sessionId: string }
  | { type: "CANCEL"; sessionId: string }
  | { type: "SOURCE_LOST"; sessionId: string }
  | { type: "TARGET_LOST"; sessionId: string; targetId: string }
  | { type: "TRANSPORT_LOST"; sessionId: string }
  | { type: "WINDOW_LOST"; sessionId: string }
  | { type: "RESET"; sessionId: string };

export type DragSessionEffect =
  | { type: "prepareSession"; sessionId: string; sourceId: string; subject: DragSubject }
  | {
      type: "emitDragStart";
      sessionId: string;
      sourceId: string;
      subject: DragSubject;
      operation: DragOperation;
    }
  | { type: "requestDrop"; sessionId: string; intent: DropIntent }
  | { type: "emitDropResult"; sessionId: string; outcome: DragTerminalOutcome }
  | { type: "announce"; kind: DragAnnouncementKind }
  | { type: "returnFocus"; sessionId: string; subject: DragSubject }
  | { type: "cleanupSession"; sessionId: string };

export type DragSessionResult = TransitionResult<
  DragSessionPhase,
  DragSessionContext,
  DragSessionEffect
>;

/**
 * One already-measured nested-target candidate. Geometry is adapter-owned:
 * the adapter decides `containsPoint` and `depth`, the kernel decides which
 * candidate wins.
 */
export interface DropTargetCandidate {
  targetId: string;
  /** Registration depth; deeper wins. */
  depth: number;
  /** Stable registration order; lower wins the final tie-break. */
  order: number;
  /** Explicit priority, applied only among equal-depth candidates. */
  priority?: number;
  containsPoint: boolean;
  eligibility: DropEligibility;
}

function isTerminalPhase(phase: DragSessionPhase): boolean {
  return phase === "ended" || phase === "cancelled";
}

function isActivePhase(phase: DragSessionPhase): boolean {
  return phase === "preparing" || phase === "armed" || phase === "dragging" || phase === "dropping";
}

function currentSession(context: DragSessionContext, sessionId: string): DragSession | null {
  return context.session !== null && context.session.sessionId === sessionId ? context.session : null;
}

function sameIntent(left: DropIntent | null, right: DropIntent): boolean {
  return (
    left !== null &&
    left.targetId === right.targetId &&
    left.position === right.position &&
    left.operation === right.operation
  );
}

/**
 * The single transition into a terminal phase: result, announcement,
 * focus-return (only when a pickup actually happened), then cleanup.
 */
function terminal(
  phase: DragSessionPhase,
  session: DragSession,
  outcome: DragTerminalOutcome,
  kind: DragAnnouncementKind,
  next: "ended" | "cancelled",
): DragSessionResult {
  const effects: DragSessionEffect[] = [
    { type: "emitDropResult", sessionId: session.sessionId, outcome },
    { type: "announce", kind },
  ];

  if (phase === "dragging" || phase === "dropping") {
    effects.push({ type: "returnFocus", sessionId: session.sessionId, subject: session.subject });
  }

  effects.push({ type: "cleanupSession", sessionId: session.sessionId });

  return { state: next, context: { session }, effects };
}

function cancel(
  phase: DragSessionPhase,
  session: DragSession,
  reason: DragCancelReason,
): DragSessionResult {
  return terminal(phase, session, { status: "cancelled", reason }, "cancelled", "cancelled");
}

function prepare(
  phase: DragSessionPhase,
  context: DragSessionContext,
  event: Extract<DragSessionEvent, { type: "PREPARE" }>,
): DragSessionResult {
  const inert: DragSessionResult = { state: phase, context, effects: [] };

  if (!event.allowedOperations.includes(event.operation)) {
    return inert;
  }

  // An active gesture owns its session; a terminal one must be reset first.
  if (phase === "dragging" || phase === "dropping" || isTerminalPhase(phase)) {
    return inert;
  }

  const session: DragSession = {
    sessionId: event.sessionId,
    sourceId: event.sourceId,
    subject: event.subject,
    operation: event.operation,
    allowedOperations: [...event.allowedOperations],
    intent: null,
  };
  const begin: DragSessionEffect = {
    type: "prepareSession",
    sessionId: session.sessionId,
    sourceId: session.sourceId,
    subject: session.subject,
  };

  if (phase === "idle") {
    return { state: "preparing", context: { session }, effects: [begin] };
  }

  const superseded = context.session;

  if (superseded === null || superseded.sessionId === event.sessionId) {
    return inert;
  }

  return {
    state: "preparing",
    context: { session },
    effects: [
      {
        type: "emitDropResult",
        sessionId: superseded.sessionId,
        outcome: { status: "cancelled", reason: "superseded" },
      },
      { type: "announce", kind: "cancelled" },
      { type: "cleanupSession", sessionId: superseded.sessionId },
      begin,
    ],
  };
}

export function dragSessionTransition(
  phase: DragSessionPhase,
  context: DragSessionContext,
  event: DragSessionEvent,
): DragSessionResult {
  const inert: DragSessionResult = { state: phase, context, effects: [] };

  if (event.type === "PREPARE") {
    return prepare(phase, context, event);
  }

  const session = currentSession(context, event.sessionId);

  // A stale event naming a session that is not the current one is inert.
  if (session === null) {
    return inert;
  }

  switch (event.type) {
    case "PREPARED":
      return phase === "preparing" ? { state: "armed", context, effects: [] } : inert;

    case "PREPARE_DECLINED":
      return phase === "preparing" ? cancel(phase, session, "preparation-declined") : inert;

    case "PREPARE_FAILED":
      return phase === "preparing" ? cancel(phase, session, "preparation-failed") : inert;

    case "ACTIVATE":
      return phase === "armed"
        ? {
            state: "dragging",
            context,
            effects: [
              {
                type: "emitDragStart",
                sessionId: session.sessionId,
                sourceId: session.sourceId,
                subject: session.subject,
                operation: session.operation,
              },
              { type: "announce", kind: "pickup" },
            ],
          }
        : inert;

    case "TARGET_INTENT": {
      if (phase !== "dragging") return inert;
      if (!session.allowedOperations.includes(event.intent.operation)) return inert;
      if (sameIntent(session.intent, event.intent)) return inert;

      return {
        state: "dragging",
        context: {
          session: { ...session, operation: event.intent.operation, intent: event.intent },
        },
        effects: [{ type: "announce", kind: "intentChanged" }],
      };
    }

    case "TARGET_CLEARED":
      return phase === "dragging" && session.intent !== null
        ? {
            state: "dragging",
            context: { session: { ...session, intent: null } },
            effects: [{ type: "announce", kind: "intentCleared" }],
          }
        : inert;

    case "OPERATION_CHANGED": {
      if (phase !== "dragging") return inert;
      if (!session.allowedOperations.includes(event.operation)) return inert;
      if (session.operation === event.operation) return inert;

      const intent =
        session.intent === null ? null : { ...session.intent, operation: event.operation };

      return {
        state: "dragging",
        context: { session: { ...session, operation: event.operation, intent } },
        effects: intent === null ? [] : [{ type: "announce", kind: "intentChanged" }],
      };
    }

    case "DROP_REQUESTED":
      return phase === "dragging" && session.intent !== null
        ? {
            state: "dropping",
            context,
            effects: [{ type: "requestDrop", sessionId: session.sessionId, intent: session.intent }],
          }
        : inert;

    case "DROP_COMMITTED":
      return phase === "dropping"
        ? terminal(
            phase,
            { ...session, operation: event.intent.operation, intent: event.intent },
            { status: "committed", intent: event.intent },
            "dropped",
            "ended",
          )
        : inert;

    case "DROP_REJECTED":
      return phase === "dropping"
        ? terminal(
            phase,
            session,
            event.reason === undefined
              ? { status: "rejected" }
              : { status: "rejected", reason: event.reason },
            "rejected",
            "ended",
          )
        : inert;

    case "DROP_FAILED":
      return phase === "dropping"
        ? terminal(
            phase,
            session,
            event.reason === undefined
              ? { status: "failed" }
              : { status: "failed", reason: event.reason },
            "failed",
            "ended",
          )
        : inert;

    case "ESCAPE":
      return isActivePhase(phase) ? cancel(phase, session, "escape") : inert;

    case "CANCEL":
      return isActivePhase(phase) ? cancel(phase, session, "explicit") : inert;

    case "SOURCE_LOST":
      return isActivePhase(phase) ? cancel(phase, session, "source-lost") : inert;

    case "TARGET_LOST":
      return isActivePhase(phase) && session.intent?.targetId === event.targetId
        ? cancel(phase, session, "target-lost")
        : inert;

    case "TRANSPORT_LOST":
      return isActivePhase(phase) ? cancel(phase, session, "transport-lost") : inert;

    case "WINDOW_LOST":
      return isActivePhase(phase) ? cancel(phase, session, "window-lost") : inert;

    case "RESET":
      return isTerminalPhase(phase) ? { state: "idle", context: { session: null }, effects: [] } : inert;
  }
}

function outranks(candidate: DropTargetCandidate, best: DropTargetCandidate): boolean {
  if (candidate.depth !== best.depth) {
    return candidate.depth > best.depth;
  }

  const candidatePriority = candidate.priority ?? 0;
  const bestPriority = best.priority ?? 0;

  if (candidatePriority !== bestPriority) {
    return candidatePriority > bestPriority;
  }

  return candidate.order < best.order;
}

/**
 * Deterministic nested-target arbitration over already-measured candidates:
 * discard non-containing and ineligible candidates, prefer the deepest, then
 * explicit priority among equal depth, then stable registration order. Returns
 * at most one intent — never several simultaneous drops.
 */
export function resolveDropTarget(candidates: readonly DropTargetCandidate[]): DropIntent | null {
  let best: DropTargetCandidate | null = null;
  let bestIntent: DropIntent | null = null;

  for (const candidate of candidates) {
    if (!candidate.containsPoint || !candidate.eligibility.accepted) {
      continue;
    }

    if (best === null || outranks(candidate, best)) {
      best = candidate;
      bestIntent = candidate.eligibility.intent;
    }
  }

  return bestIntent;
}

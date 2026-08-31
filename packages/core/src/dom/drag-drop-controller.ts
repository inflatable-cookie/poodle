/**
 * Same-document drag-and-drop web controller.
 *
 * Architecture: docs/architecture/011-drag-and-drop-substrate.md.
 * Spec: docs/specs/069-dependable-drag-and-drop-substrate.md.
 *
 * Owns pointer/keyboard sensors, cached geometry, effect execution, and
 * presentation snapshots. Session phase, arbitration, and exactly-once
 * terminals stay in `dragSessionTransition` / `resolveDropTarget`.
 */

import { collectScrollParents } from "./anchor";
import {
  createCrossWindowDataTransferAdapter,
  type CrossWindowDataTransferAdapter,
} from "./cross-window-data-transfer";
import {
  isCrossWindowDragReceipt,
  sameCrossWindowDragReceipt,
  type CrossWindowDragProjection,
  type CrossWindowDragReceipt,
  type CrossWindowDragSourceBridge,
  type CrossWindowDragTargetBridge,
  type CrossWindowDragTargetEvent,
  type CrossWindowDragTransport,
} from "../cross-window-drag";
import {
  resolveAutoScroll,
  type AutoScrollCandidate,
  type AutoScrollMetrics,
} from "./drag-drop-auto-scroll";
import {
  dragSessionTransition,
  resolveDropTarget,
  type DragAnnouncementKind,
  type DragCancelReason,
  type DragDropCommitResult,
  type DragOperation,
  type DragSession,
  type DragSessionContext,
  type DragSessionEffect,
  type DragSessionEvent,
  type DragSessionPhase,
  type DragSubject,
  type DragTerminalOutcome,
  type DropEligibility,
  type DropIntent,
  type DropPosition,
  type DropTargetCandidate,
} from "../drag-drop";

export type { DragDropCommitResult };

export type DragInputKind = "mouse" | "pen" | "touch" | "keyboard";

export interface DragPointerPosition {
  readonly x: number;
  readonly y: number;
}

export interface DragActivationDistance {
  readonly distance: number;
}

export interface DragActivationHold {
  readonly holdMs: number;
  readonly tolerance: number;
}

export interface DragActivationConstraints {
  readonly mouse?: DragActivationDistance;
  readonly pen?: DragActivationDistance;
  readonly touch?: DragActivationHold;
}

export interface DragPositionResolverInput {
  readonly x: number;
  readonly y: number;
  readonly rect: DOMRectReadOnly;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
  readonly inputKind: DragInputKind;
}

export interface DragSourceRegistration {
  readonly sourceId: string;
  readonly subject: DragSubject;
  readonly allowedOperations: readonly DragOperation[];
  readonly operation?: DragOperation;
  readonly disabled?: boolean;
  readonly label: string;
  readonly instructions?: string;
  readonly handle?: Element | string;
  readonly activation?: DragActivationConstraints;
  /** When set, Space/Enter pick up this focused source. Also the origin for ordered logical keyboard traversal. */
  readonly keyboardOrder?: number;
  /**
   * Host preparation for a drag that may leave this window.
   *
   * Optional and per source, because a lease belongs to the subject being
   * dragged. Preparation runs on the accepted pre-drag gesture, before
   * activation, and the source cannot advertise or start a cross-window
   * gesture until its own receipt is armed. A source without one keeps the
   * internal transport's immediate preparation, and so does a source whose
   * bridge does not advertise the capability for the input kind in hand.
   */
  readonly crossWindowSourceBridge?: CrossWindowDragSourceBridge;
  readonly onDragStart?: (session: DragSession) => void;
  readonly onDragEnd?: (outcome: DragTerminalOutcome) => void;
}

export interface DropTargetRegistration {
  readonly targetId: string;
  readonly acceptedKinds: readonly string[];
  readonly disabled?: boolean;
  readonly priority?: number;
  readonly label: string;
  readonly resolvePosition: (input: DragPositionResolverInput) => DropPosition | null;
  readonly canDrop: (intent: DropIntent, subject: DragSubject) => boolean | DropEligibility;
  readonly onDrop: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
  /** When true, this element is an auto-scroll owner in addition to overflow ancestors. */
  readonly autoScroll?: boolean;
}

export type KeyboardDropDirection = "previous" | "next" | "first" | "last";

export interface KeyboardPositionResolverInput {
  readonly direction: KeyboardDropDirection;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
}

export interface KeyboardDropTargetRegistration {
  readonly targetId: string;
  readonly acceptedKinds: readonly string[];
  readonly disabled?: boolean;
  readonly priority?: number;
  readonly label: string;
  readonly order: number;
  readonly resolvePosition: (input: KeyboardPositionResolverInput) => DropPosition | null;
  readonly canDrop: (intent: DropIntent, subject: DragSubject) => boolean | DropEligibility;
  readonly onDrop: (intent: DropIntent) => DragDropCommitResult | Promise<DragDropCommitResult>;
}

export interface KeyboardDropCommand {
  readonly sourceId: string;
  readonly targetId: string;
  readonly position: DropPosition;
}

export interface DragSourceHandle {
  update(registration: DragSourceRegistration): void;
  unregister(): void;
}

export interface DropTargetHandle {
  update(registration: DropTargetRegistration): void;
  unregister(): void;
}

export interface KeyboardDropTargetHandle {
  update(registration: KeyboardDropTargetRegistration): void;
  unregister(): void;
}

export interface DragPreviewSnapshot {
  readonly sourceId: string;
  readonly subject: DragSubject;
  readonly operation: DragOperation;
  readonly x: number;
  readonly y: number;
  readonly label: string;
}

export type DragDropTargetPosture = "accepted" | "rejected" | null;

export interface DragDropSnapshot {
  readonly phase: DragSessionPhase;
  readonly session: DragSession | null;
  readonly inputKind: DragInputKind | null;
  readonly pointer: DragPointerPosition | null;
  readonly sourceId: string | null;
  readonly targetId: string | null;
  readonly targetPosture: DragDropTargetPosture;
  readonly rejectedReason: string | undefined;
  readonly preview: DragPreviewSnapshot | null;
  readonly announcement: string | null;
}

export interface DragAnnouncementEvent {
  readonly kind: DragAnnouncementKind;
  readonly sourceLabel: string;
  readonly targetLabel?: string;
  readonly position?: DropPosition;
  readonly operation?: DragOperation;
  readonly reason?: string;
}

export interface DragDropCapabilities {
  readonly pointer: boolean;
  readonly touch: boolean;
  readonly keyboard: boolean;
}

export interface DragDropControllerOptions {
  readonly describeAnnouncement?: (event: DragAnnouncementEvent) => string | null;
  readonly createSessionId?: () => string;
  /**
   * Incoming host projection, commit, and accessible target picking for this
   * one document.
   *
   * Per window rather than per source: a projection arrives with no local
   * source at all, and outlives any one subject. It is installed when the
   * controller connects and torn down when it disconnects.
   */
  readonly crossWindowTargetBridge?: CrossWindowDragTargetBridge;
}

export interface DragDropController {
  readonly capabilities: DragDropCapabilities;
  connect(root: Element): () => void;
  registerSource(element: Element, registration: DragSourceRegistration): DragSourceHandle;
  registerTarget(element: Element, registration: DropTargetRegistration): DropTargetHandle;
  registerKeyboardTarget(registration: KeyboardDropTargetRegistration): KeyboardDropTargetHandle;
  requestKeyboardDrop(command: KeyboardDropCommand): boolean;
  getSnapshot(): DragDropSnapshot;
  subscribe(listener: () => void): () => void;
  invalidateLayout(): void;
  cancel(): void;
  destroy(): void;
}

const DEFAULT_MOUSE_DISTANCE = 4;
const DEFAULT_PEN_DISTANCE = 4;
const DEFAULT_TOUCH_HOLD_MS = 250;
const DEFAULT_TOUCH_TOLERANCE = 12;
const ANNOUNCE_THROTTLE_MS = 400;
const PREVIEW_OFFSET = 12;

const CAPABILITIES: DragDropCapabilities = Object.freeze({
  pointer: true,
  touch: true,
  keyboard: true,
});

const SOURCE_ATTR = "data-poodle-drag-source";
const TARGET_ATTR = "data-poodle-drop-target";
const POSITION_ATTR = "data-poodle-drop-position";
const TOUCH_ACTION = "touch-action";

interface CachedRect {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly left: number;
}

interface SourceEntry {
  element: Element;
  registration: DragSourceRegistration;
  order: number;
  authoredTabIndex: string | null;
  authoredAriaLabel: string | null;
  authoredAriaDescription: string | null;
  authoredDraggable: string | null;
  addedTabIndex: boolean;
  addedAriaLabel: boolean;
}

interface TargetEntry {
  element: Element;
  registration: DropTargetRegistration;
  order: number;
}

interface KeyboardTargetEntry {
  registration: KeyboardDropTargetRegistration;
  order: number;
}

interface PointerGesture {
  pointerId: number;
  pointerType: DragInputKind;
  originX: number;
  originY: number;
  x: number;
  y: number;
  sourceId: string;
  sessionId: string;
  activated: boolean;
  /**
   * The activation constraint is already satisfied.
   *
   * Distinct from `activated` only for a cross-window source, whose host
   * preparation may still be in flight when the pointer passes its distance
   * or hold: the gesture is committed, but the session cannot enter
   * `dragging` until its receipt is armed. Every other source arms
   * synchronously and the two flip together.
   */
  thresholdReached: boolean;
  holdTimer: ReturnType<typeof setTimeout> | null;
  captureElement: Element | null;
  restoredTouchAction: string | null;
}

let sessionSeq = 0;

function defaultSessionId(): string {
  sessionSeq += 1;
  const uuid = globalThis.crypto?.randomUUID?.();
  return uuid ?? `poodle-drag-${sessionSeq}`;
}

function asInputKind(pointerType: string): DragInputKind {
  if (pointerType === "touch" || pointerType === "pen") return pointerType;
  return "mouse";
}

function distance(ax: number, ay: number, bx: number, by: number): number {
  return Math.hypot(bx - ax, by - ay);
}

function copyRect(rect: DOMRectReadOnly): CachedRect {
  return {
    x: rect.x,
    y: rect.y,
    width: rect.width,
    height: rect.height,
    top: rect.top,
    right: rect.right,
    bottom: rect.bottom,
    left: rect.left,
  };
}

function containsPoint(rect: CachedRect, x: number, y: number): boolean {
  return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
}

function eventPath(event: Event): EventTarget[] {
  const path: EventTarget[] = [];
  if (typeof event.composedPath === "function") {
    for (const node of event.composedPath()) {
      if (node !== undefined) path.push(node);
    }
    if (path.length > 0) return path;
  }

  let node: Node | null = event.target as Node | null;
  while (node) {
    path.push(node);
    node = node.parentNode;
  }
  return path;
}

function isDisabled(element: Element): boolean {
  return element.hasAttribute("disabled") || element.getAttribute("aria-disabled") === "true";
}

function freezeSnapshot(snapshot: DragDropSnapshot): DragDropSnapshot {
  if (snapshot.session) Object.freeze(snapshot.session);
  if (snapshot.pointer) Object.freeze(snapshot.pointer);
  if (snapshot.preview) Object.freeze(snapshot.preview);
  return Object.freeze(snapshot);
}

function emptySnapshot(phase: DragSessionPhase, session: DragSession | null): DragDropSnapshot {
  return freezeSnapshot({
    phase,
    session: session ? Object.freeze({ ...session, allowedOperations: [...session.allowedOperations] }) : null,
    inputKind: null,
    pointer: null,
    sourceId: session?.sourceId ?? null,
    targetId: session?.intent?.targetId ?? null,
    targetPosture: null,
    rejectedReason: undefined,
    preview: null,
    announcement: null,
  });
}

function defaultAnnouncement(event: DragAnnouncementEvent): string {
  switch (event.kind) {
    case "pickup":
      return `Picked up ${event.sourceLabel}`;
    case "intentChanged":
      return event.targetLabel
        ? `${event.sourceLabel}, ${event.position ?? "over"} ${event.targetLabel}`
        : `${event.sourceLabel} over a drop target`;
    case "intentCleared":
      return `${event.sourceLabel}, no drop target`;
    case "dropped":
      return event.targetLabel
        ? `Dropped ${event.sourceLabel} on ${event.targetLabel}`
        : `Dropped ${event.sourceLabel}`;
    case "rejected":
      return event.reason ? `Drop rejected: ${event.reason}` : "Drop rejected";
    case "failed":
      return event.reason ? `Drop failed: ${event.reason}` : "Drop failed";
    case "cancelled":
      return `Cancelled dragging ${event.sourceLabel}`;
  }
}

function eligibilityFromCanDrop(
  result: boolean | DropEligibility,
  intent: DropIntent,
): DropEligibility {
  if (typeof result === "boolean") {
    return result ? { accepted: true, intent } : { accepted: false };
  }
  return result;
}

function styledElement(element: Element): (Element & { style: CSSStyleDeclaration }) | null {
  return "style" in element ? (element as Element & { style: CSSStyleDeclaration }) : null;
}

function isFocusableHost(element: Element): element is HTMLElement | SVGElement {
  return element instanceof HTMLElement || element instanceof SVGElement;
}

function focusHost(element: Element): void {
  if (isFocusableHost(element)) element.focus();
}

function canPointerCapture(element: Element): boolean {
  return typeof (element as HTMLElement).setPointerCapture === "function";
}

const NO_DRAG_ATTR = "data-poodle-no-drag";
const INTERACTIVE_SELECTOR =
  `button, input, textarea, select, a[href], [role='button'], [contenteditable]:not([contenteditable='false']), [${NO_DRAG_ATTR}]`;

function resolveHandle(element: Element, handle: Element | string | undefined): Element {
  if (handle === undefined) return element;
  if (typeof handle === "string") {
    return (element as Element).querySelector(handle) ?? element;
  }
  return handle;
}

function interactiveHost(target: EventTarget | null): Element | null {
  if (!(target instanceof Element)) return null;
  return target.closest(INTERACTIVE_SELECTOR);
}

const SCROLL_OVERFLOW = /(auto|scroll|overlay)/;

function hitElementFromPoint(doc: Document, x: number, y: number): Element | null {
  const stack =
    typeof doc.elementsFromPoint === "function" ? doc.elementsFromPoint(x, y) : [doc.elementFromPoint(x, y)];
  for (const node of stack) {
    if (!(node instanceof Element)) continue;
    if (node.classList.contains("poodle-drag-overlay") || node.classList.contains("poodle-drag-preview")) {
      continue;
    }
    const style = node.ownerDocument.defaultView?.getComputedStyle(node);
    if (style?.pointerEvents === "none") continue;
    return node;
  }
  return null;
}

function isScrollOwner(element: Element): element is HTMLElement {
  if (!(element instanceof HTMLElement)) return false;
  const style = element.ownerDocument.defaultView?.getComputedStyle(element);
  if (!style) return false;
  return SCROLL_OVERFLOW.test(`${style.overflowY} ${style.overflowX} ${style.overflow}`);
}

function measureScrollMetrics(element: HTMLElement): AutoScrollMetrics {
  const rect = element.getBoundingClientRect();
  return {
    scrollTop: element.scrollTop,
    scrollLeft: element.scrollLeft,
    scrollHeight: element.scrollHeight,
    scrollWidth: element.scrollWidth,
    clientHeight: element.clientHeight,
    clientWidth: element.clientWidth,
    rect: { top: rect.top, right: rect.right, bottom: rect.bottom, left: rect.left },
  };
}

function activationFor(
  registration: DragSourceRegistration,
  kind: DragInputKind,
): DragActivationDistance | DragActivationHold {
  const authored = registration.activation;
  if (kind === "touch") {
    return {
      holdMs: authored?.touch?.holdMs ?? DEFAULT_TOUCH_HOLD_MS,
      tolerance: authored?.touch?.tolerance ?? DEFAULT_TOUCH_TOLERANCE,
    };
  }
  if (kind === "pen") {
    return { distance: authored?.pen?.distance ?? DEFAULT_PEN_DISTANCE };
  }
  return { distance: authored?.mouse?.distance ?? DEFAULT_MOUSE_DISTANCE };
}

/**
 * One source's live host transaction.
 *
 * Held beside the kernel session rather than inside it: the kernel owns
 * lifecycle and knows nothing about transports, and a receipt is the one thing
 * that must survive independently of which phase the session happens to be in
 * when the host answers.
 */
interface CrossWindowSourceTransaction {
  readonly sessionId: string;
  readonly sourceId: string;
  readonly bridge: CrossWindowDragSourceBridge;
  readonly abort: AbortController;
  receipt: CrossWindowDragReceipt | null;
  transport: CrossWindowDragTransport | null;
  stopTerminal: (() => void) | null;
  /** The host already delivered its authoritative terminal for this receipt. */
  settled: boolean;
}

/** The incoming host transaction this window is currently projecting. */
interface CrossWindowTargetTransaction {
  readonly sessionId: string;
  readonly receipt: CrossWindowDragReceipt;
  projection: CrossWindowDragProjection;
  readonly abort: AbortController;
  /** A commit is in flight; a second drop cannot start another. */
  committing: boolean;
}

export function createDragDropController(options: DragDropControllerOptions = {}): DragDropController {
  const createSessionId = options.createSessionId ?? defaultSessionId;
  let describeAnnouncement = options.describeAnnouncement;
  const crossWindowTargetBridge = options.crossWindowTargetBridge;
  /**
   * One codec for both directions.
   *
   * The source writes the envelope at `dragstart` and the target reads it at
   * `drop`; they are the same bounded format, and two instances would be two
   * places for the MIME type and the bounds to drift apart.
   */
  const crossWindowCodec: CrossWindowDataTransferAdapter = createCrossWindowDataTransferAdapter();

  let destroyed = false;
  let connectedRoot: Element | null = null;
  let connectedDocument: Document | null = null;
  let connectedWindow: Window | null = null;

  let phase: DragSessionPhase = "idle";
  let context: DragSessionContext = { session: null };
  let inputKind: DragInputKind | null = null;
  let pointerPosition: DragPointerPosition | null = null;
  let announcement: string | null = null;
  let rejectedReason: string | undefined;
  let rejectedTargetId: string | null = null;
  let lastAnnounceAt = 0;
  let pendingIntentAnnouncement: DragAnnouncementEvent | null = null;
  let announceTimer: ReturnType<typeof setTimeout> | null = null;

  const sources = new Map<string, SourceEntry>();
  const sourcesByElement = new Map<Element, string>();
  const targets = new Map<string, TargetEntry>();
  const targetsByElement = new Map<Element, string>();
  const keyboardTargets = new Map<string, KeyboardTargetEntry>();
  let nextOrder = 0;
  let lastKeyboardDirection: KeyboardDropDirection | null = null;

  const rects = new Map<Element, CachedRect>();
  let layoutDirty = true;

  const listeners = new Set<() => void>();
  let snapshot = emptySnapshot("idle", null);

  let gesture: PointerGesture | null = null;
  let moveFrame: number | null = null;
  let pendingMove: { x: number; y: number } | null = null;
  let autoScrollFrame: number | null = null;
  let autoScrollRunning = false;
  let lastAutoScrollTs: number | null = null;
  const scrollOwnerIds = new WeakMap<Element, string>();
  let scrollOwnerSeq = 0;
  let dropGeneration = 0;
  let lastOutcome: DragTerminalOutcome | undefined;
  let keyboardSourceId: string | null = null;
  let keyboardCommandSession = false;
  let keyboardLogicalSession = false;
  let keyboardTargetIndex = -1;

  let crossWindowSource: CrossWindowSourceTransaction | null = null;
  /**
   * A session that must not re-enter its source's host bridge.
   *
   * The local fallback reuses the same registration, so without this the
   * declined bridge would be asked again and decline again forever.
   */
  let bridgeBypassSessionId: string | null = null;
  /**
   * The host's own name for a subject this window has no source for.
   *
   * Announcements need a name and an incoming projection is the only place one
   * exists, so it is held for the session rather than looked up in a registry
   * that will never contain it.
   */
  let crossWindowSourceLabel: string | null = null;
  let crossWindowTarget: CrossWindowTargetTransaction | null = null;
  let crossWindowUnsubscribe: (() => void) | null = null;
  /** The native gesture handed to the browser for a `data-transfer` source. */
  let nativeDragSessionId: string | null = null;

  const documentListeners: Array<[string, EventListener, AddEventListenerOptions | boolean | undefined]> = [];
  let resizeObserver: ResizeObserver | null = null;
  let restoredRootUserSelect: string | null | undefined;

  /**
   * Read the phase through a call so a guard earlier in the same function does
   * not narrow it. `dispatch` reassigns `phase`, which control-flow analysis
   * cannot see through.
   */
  function currentPhase(): DragSessionPhase {
    return phase;
  }

  function assertLive(): void {
    if (destroyed) {
      throw new Error("DragDropController has been destroyed");
    }
  }

  function notify(): void {
    snapshot = buildSnapshot();
    for (const listener of listeners) listener();
  }

  function currentSource(): SourceEntry | undefined {
    const id = context.session?.sourceId;
    return id === undefined ? undefined : sources.get(id);
  }

  function currentTarget(): TargetEntry | undefined {
    const id = context.session?.intent?.targetId;
    return id === undefined ? undefined : targets.get(id);
  }

  function measure(element: Element): CachedRect {
    const cached = rects.get(element);
    if (!layoutDirty && cached) return cached;
    const next = copyRect(element.getBoundingClientRect());
    rects.set(element, next);
    return next;
  }

  function refreshLayout(): void {
    if (!layoutDirty) return;
    rects.clear();
    for (const source of sources.values()) measure(source.element);
    for (const target of targets.values()) measure(target.element);
    layoutDirty = false;
  }

  function targetDepth(element: Element): number {
    let depth = 0;
    let current: Element | null = element.parentElement;
    while (current) {
      if (targetsByElement.has(current)) depth += 1;
      current = current.parentElement;
    }
    return depth;
  }

  function projectAttributes(): void {
    const session = context.session;
    const activeSourceId = session?.sourceId ?? null;
    const acceptedId = phase === "dragging" || phase === "dropping" ? session?.intent?.targetId ?? null : null;

    for (const [id, source] of sources) {
      if (activeSourceId === id && (phase === "preparing" || phase === "armed" || phase === "dragging" || phase === "dropping")) {
        source.element.setAttribute(SOURCE_ATTR, phase);
      } else {
        source.element.removeAttribute(SOURCE_ATTR);
      }
    }

    for (const [id, target] of targets) {
      if (acceptedId === id) {
        target.element.setAttribute(TARGET_ATTR, "accepted");
        const position = session?.intent?.position;
        if (position !== undefined) target.element.setAttribute(POSITION_ATTR, position);
        else target.element.removeAttribute(POSITION_ATTR);
      } else if (rejectedTargetId === id) {
        target.element.setAttribute(TARGET_ATTR, "rejected");
        target.element.removeAttribute(POSITION_ATTR);
      } else {
        target.element.removeAttribute(TARGET_ATTR);
        target.element.removeAttribute(POSITION_ATTR);
      }
    }
  }

  function buildSnapshot(): DragDropSnapshot {
    const session = context.session
      ? Object.freeze({
          ...context.session,
          allowedOperations: Object.freeze([...context.session.allowedOperations]),
          subject: Object.freeze({ ...context.session.subject }),
          intent: context.session.intent ? Object.freeze({ ...context.session.intent }) : null,
        })
      : null;

    const source = session ? sources.get(session.sourceId) : undefined;
    const dragging = phase === "dragging" || phase === "dropping";
    const preview =
      dragging && session && pointerPosition
        ? Object.freeze({
            sourceId: session.sourceId,
            subject: session.subject,
            operation: session.operation,
            x: pointerPosition.x + PREVIEW_OFFSET,
            y: pointerPosition.y + PREVIEW_OFFSET,
            label: source?.registration.label ?? crossWindowSourceLabel ?? session.subject.id,
          })
        : null;

    let targetPosture: DragDropTargetPosture = null;
    if (dragging && session?.intent) targetPosture = "accepted";
    else if (dragging && rejectedTargetId) targetPosture = "rejected";

    return freezeSnapshot({
      phase,
      session,
      inputKind,
      pointer: pointerPosition ? Object.freeze({ ...pointerPosition }) : null,
      sourceId: session?.sourceId ?? null,
      targetId: session?.intent?.targetId ?? rejectedTargetId,
      targetPosture,
      rejectedReason,
      preview,
      announcement,
    });
  }

  function announce(kind: DragAnnouncementKind, outcome?: DragTerminalOutcome): void {
    const session = context.session;
    const source = session ? sources.get(session.sourceId) : undefined;
    const target = session?.intent ? targetForAnnouncement(session.intent.targetId) : undefined;
    const reason =
      outcome && (outcome.status === "rejected" || outcome.status === "failed" || outcome.status === "cancelled")
        ? "reason" in outcome
          ? outcome.reason
          : undefined
        : rejectedReason;

    const event: DragAnnouncementEvent = {
      kind,
      sourceLabel:
        source?.registration.label ?? crossWindowSourceLabel ?? session?.subject.id ?? "item",
      targetLabel: target?.registration.label,
      position: session?.intent?.position,
      operation: session?.operation,
      reason,
    };

    const throttle =
      inputKind !== "keyboard" && (kind === "intentChanged" || kind === "rejected");
    if (throttle) {
      pendingIntentAnnouncement = event;
      const now = Date.now();
      const wait = Math.max(0, ANNOUNCE_THROTTLE_MS - (now - lastAnnounceAt));
      if (announceTimer !== null) return;
      announceTimer = setTimeout(() => {
        announceTimer = null;
        const pending = pendingIntentAnnouncement;
        pendingIntentAnnouncement = null;
        if (pending) publishAnnouncement(pending);
      }, wait);
      return;
    }

    if (announceTimer !== null) {
      clearTimeout(announceTimer);
      announceTimer = null;
      pendingIntentAnnouncement = null;
    }
    publishAnnouncement(event);
  }

  function publishAnnouncement(event: DragAnnouncementEvent): void {
    const text = describeAnnouncement ? describeAnnouncement(event) : defaultAnnouncement(event);
    if (text === null) return;
    announcement = text;
    lastAnnounceAt = Date.now();
    notify();
  }

  function dispatch(event: DragSessionEvent): void {
    if (destroyed) return;

    const queue: DragSessionEvent[] = [event];
    let changed = false;

    while (queue.length > 0) {
      const next = queue.shift();
      if (next === undefined) break;

      const result = dragSessionTransition(phase, context, next);
      const inert =
        result.state === phase && result.context === context && result.effects.length === 0;
      if (inert) continue;

      phase = result.state;
      context = result.context;
      changed = true;

      for (const effect of result.effects) {
        for (const follow of runEffect(effect)) queue.push(follow);
      }

      if (phase === "ended" || phase === "cancelled") {
        const sessionId = context.session?.sessionId ?? next.sessionId;
        queue.push({ type: "RESET", sessionId });
      }
    }

    if (phase === "idle") {
      rejectedReason = undefined;
      rejectedTargetId = null;
      if (gesture === null && keyboardSourceId === null) {
        inputKind = null;
        pointerPosition = null;
      }
    }

    if (changed) {
      projectAttributes();
      notify();
    }
  }

  function runEffect(effect: DragSessionEffect): DragSessionEvent[] {
    switch (effect.type) {
      case "prepareSession": {
        // An incoming host projection has no local source to prepare — the
        // preparation already happened in the window the drag came from.
        if (crossWindowTarget?.sessionId === effect.sessionId) {
          return [{ type: "PREPARED", sessionId: effect.sessionId }];
        }

        if (bridgeBypassSessionId === effect.sessionId) {
          bridgeBypassSessionId = null;
          return [{ type: "PREPARED", sessionId: effect.sessionId }];
        }

        const source = sources.get(effect.sourceId);
        const bridge = source?.registration.crossWindowSourceBridge;
        if (!source || !bridge || inputKind === null || !crossWindowCarries(bridge, inputKind)) {
          return [{ type: "PREPARED", sessionId: effect.sessionId }];
        }

        // Stays in `preparing` until the host answers: an armed receipt is the
        // precondition for advertising or starting a cross-window gesture.
        beginCrossWindowPreparation(effect.sessionId, source, bridge);
        return [];
      }

      case "emitDragStart": {
        const source = sources.get(effect.sourceId);
        const session = context.session;
        if (source && session) source.registration.onDragStart?.(session);
        return [];
      }

      case "requestDrop":
        requestDrop(effect.sessionId, effect.intent);
        return [];

      case "emitDropResult": {
        lastOutcome = effect.outcome;
        const source = currentSource();
        source?.registration.onDragEnd?.(effect.outcome);
        if (effect.outcome.status === "rejected") {
          rejectedReason = effect.outcome.reason;
        }
        return [];
      }

      case "announce":
        announce(effect.kind, lastOutcome);
        return [];

      case "returnFocus":
        returnFocus(effect.subject);
        return [];

      case "cleanupSession":
        stopAutoScroll();
        releasePointerHardware();
        if (crossWindowSource?.sessionId === effect.sessionId) {
          releaseCrossWindowSource(releaseReasonFor(lastOutcome));
        }
        if (crossWindowTarget?.sessionId === effect.sessionId) {
          releaseCrossWindowTarget();
        }
        crossWindowSourceLabel = null;
        gesture = null;
        dropGeneration += 1;
        keyboardSourceId = null;
        keyboardTargetIndex = -1;
        keyboardLogicalSession = false;
        keyboardCommandSession = false;
        lastKeyboardDirection = null;
        lastOutcome = undefined;
        return [];
    }
  }


  // ── Cross-window host bridge ───────────────────────────────────────────

  /**
   * Whether this host can carry the input kind in hand.
   *
   * Capability is resolved before the affordance claims support, and it is
   * per input class rather than per bridge: a host that follows a mouse
   * across windows may have no way to see a touch contact outside the source
   * window, and saying otherwise would arm a gesture that can never leave.
   * A `false` answer is not a failure — the source falls back to the internal
   * transport's immediate preparation and stays a perfectly good local drag.
   */
  function crossWindowCarries(bridge: CrossWindowDragSourceBridge, kind: DragInputKind): boolean {
    if (kind === "keyboard") return bridge.capabilities.keyboardTargetPicker;
    if (kind === "touch") return bridge.capabilities.touch;
    return bridge.capabilities.pointer;
  }

  /**
   * The transport Poodle is actually using, never what the host would prefer.
   *
   * A web page cannot observe a pointer in another window, so mouse and pen
   * transfer is the browser's own drag with the receipt in a bounded envelope.
   * `window-capture` is left for touch, where a host that advertises the
   * capability is claiming an out-of-window observation the page itself does
   * not have; keyboard has no pointer at all.
   */
  function crossWindowTransport(kind: DragInputKind): CrossWindowDragTransport {
    if (kind === "keyboard") return "keyboard-picker";
    if (kind === "touch") return "window-capture";
    return "data-transfer";
  }

  /** Mouse and pen hand the gesture to the browser rather than to the sensor. */
  function crossWindowUsesNativeDrag(bridge: CrossWindowDragSourceBridge, kind: DragInputKind): boolean {
    return (kind === "mouse" || kind === "pen") && bridge.capabilities.pointer;
  }

  /**
   * The cross-window attempt failed; the local drag has not.
   *
   * A decline or failure cancels only the transfer, so the gesture the user is
   * still making falls back to the ordinary local lifecycle with a fresh
   * session and no host payload. Without this, a host that says "not this one"
   * would also break same-window reorder.
   */
  function endCrossWindowAttempt(
    sessionId: string,
    event: "PREPARE_DECLINED" | "PREPARE_FAILED",
    sourceId: string,
  ): void {
    // The kernel's cancellation clears the gesture along with the session,
    // which is right for a real terminal and wrong here: the pointer is still
    // down and the user is still dragging. The gesture is held across the
    // dispatch, hold timer included, so a declined touch source does not also
    // lose the hold it was in the middle of.
    const pending = gesture;
    const holdTimer = pending?.holdTimer ?? null;
    if (pending) pending.holdTimer = null;
    dispatch({ type: event, sessionId });
    if (pending) pending.holdTimer = holdTimer;
    fallBackToLocalSession(pending, sourceId);
  }

  function fallBackToLocalSession(pending: PointerGesture | null, sourceId: string): void {
    if (!pending || pending.sourceId !== sourceId || pending.activated) return;
    if (phase !== "idle") return;
    const source = sources.get(sourceId);
    if (!source || source.registration.disabled || !source.element.isConnected) return;
    const kind = pending.pointerType;

    gesture = pending;
    inputKind = kind;
    pointerPosition = { x: pending.x, y: pending.y };

    const sessionId = createSessionId();
    bridgeBypassSessionId = sessionId;
    pending.sessionId = beginSession(source, kind, pending.x, pending.y, sessionId);
    if (!pending.thresholdReached) return;
    const handle = resolveHandle(source.element, source.registration.handle);
    if (phase === "armed") {
      activate(pending.sessionId, handle, pending.pointerId);
      if (gesture) hitTest(pending.x, pending.y);
    }
  }

  /**
   * Ask the host for a lease, without letting the answer arm the wrong
   * session.
   *
   * Every guard here is the same guard: the completion is bound to the session
   * it was created for. A receipt that arrives after supersession is handed
   * straight back to the host rather than dropped on the floor, because the
   * host allocated something for it.
   */
  function beginCrossWindowPreparation(
    sessionId: string,
    source: SourceEntry,
    bridge: CrossWindowDragSourceBridge,
  ): void {
    const abort = new AbortController();
    const transaction: CrossWindowSourceTransaction = {
      sessionId,
      sourceId: source.registration.sourceId,
      bridge,
      abort,
      receipt: null,
      transport: null,
      stopTerminal: null,
      settled: false,
    };
    crossWindowSource = transaction;

    const operation = source.registration.operation ?? source.registration.allowedOperations[0];
    if (operation === undefined) {
      dispatch({ type: "PREPARE_FAILED", sessionId });
      return;
    }

    let pending: Promise<CrossWindowDragReceipt | null>;
    try {
      pending = bridge.prepare(
        {
          sessionId,
          sourceId: source.registration.sourceId,
          subject: source.registration.subject,
          operation,
          allowedOperations: source.registration.allowedOperations,
        },
        abort.signal,
      );
    } catch {
      dispatch({ type: "PREPARE_FAILED", sessionId });
      return;
    }

    void Promise.resolve(pending).then(
      (receipt) => {
        const stale = crossWindowSource !== transaction || abort.signal.aborted;
        if (stale) {
          // The host still allocated something for a session that no longer
          // exists. Hand it back rather than leaking it.
          if (receipt && isCrossWindowDragReceipt(receipt)) {
            try {
              void bridge.cancel(receipt, "superseded");
            } catch {
              // A host that throws on cleanup cannot break the local session.
            }
          }
          return;
        }
        if (receipt === null) {
          endCrossWindowAttempt(sessionId, "PREPARE_DECLINED", transaction.sourceId);
          return;
        }
        if (!isCrossWindowDragReceipt(receipt)) {
          endCrossWindowAttempt(sessionId, "PREPARE_FAILED", transaction.sourceId);
          return;
        }
        transaction.receipt = receipt;
        dispatch({ type: "PREPARED", sessionId });
        // An armed receipt may arrive after the gesture already passed its
        // activation threshold; the pending activation is honoured here rather
        // than waiting for another pointer move that may never come.
        if (gesture?.sessionId === sessionId && gesture.thresholdReached && phase === "armed") {
          const live = sources.get(transaction.sourceId);
          if (live) {
            const handle = resolveHandle(live.element, live.registration.handle);
            activate(sessionId, handle, gesture.pointerId);
            hitTest(gesture.x, gesture.y);
          }
        }
      },
      () => {
        if (crossWindowSource !== transaction || abort.signal.aborted) return;
        endCrossWindowAttempt(sessionId, "PREPARE_FAILED", transaction.sourceId);
      },
    );
  }

  /**
   * The gesture is live: install the one authoritative terminal subscription.
   *
   * `start` is called once per receipt. Its callback is the only thing in this
   * controller that can end a cross-window session with a drop result — native
   * `dragend`, pointer release, and `dropEffect` cannot, which is the whole
   * reason the host owns this subscription rather than the DOM.
   */
  function startCrossWindowTransport(sessionId: string, transport: CrossWindowDragTransport): void {
    const transaction = crossWindowSource;
    if (!transaction || transaction.sessionId !== sessionId) return;
    if (transaction.stopTerminal !== null || transaction.receipt === null) return;

    transaction.transport = transport;
    const receipt = transaction.receipt;
    try {
      transaction.stopTerminal = transaction.bridge.start(receipt, transport, (outcome) => {
        if (crossWindowSource !== transaction || transaction.settled) return;
        transaction.settled = true;
        dispatch({ type: "HOST_TERMINAL", sessionId, outcome });
      });
    } catch {
      transaction.stopTerminal = null;
      dispatch({ type: "TRANSPORT_LOST", sessionId });
    }
  }

  /**
   * Release the host transaction exactly once, on the single terminal.
   *
   * `cancel` runs only while the receipt is still live — a host that already
   * reported its own terminal has closed the transaction, and telling it to
   * cancel afterwards would be a second command against one session id.
   */
  function releaseCrossWindowSource(reason: DragCancelReason): void {
    const transaction = crossWindowSource;
    if (!transaction) return;
    crossWindowSource = null;
    nativeDragSessionId = null;

    if (!transaction.abort.signal.aborted) transaction.abort.abort(reason);

    const stop = transaction.stopTerminal;
    transaction.stopTerminal = null;
    if (stop) {
      try {
        stop();
      } catch {
        // Host cleanup cannot break local cleanup.
      }
    }

    if (transaction.receipt && !transaction.settled) {
      try {
        void transaction.bridge.cancel(transaction.receipt, reason);
      } catch {
        // Same: the local session is already ending either way.
      }
    }
  }

  /** The cancel reason to hand the host, taken from the terminal that ran. */
  function releaseReasonFor(outcome: DragTerminalOutcome | undefined): DragCancelReason {
    return outcome?.status === "cancelled" ? outcome.reason : "explicit";
  }

  function releaseCrossWindowTarget(): void {
    const transaction = crossWindowTarget;
    if (!transaction) return;
    crossWindowTarget = null;
    if (!transaction.abort.signal.aborted) transaction.abort.abort();
  }


  // ── Cross-window target projection ─────────────────────────────────────

  /** The host reports a device class; this window never observed the device. */
  function projectedInputKind(kind: CrossWindowDragProjection["inputKind"]): DragInputKind {
    if (kind === "keyboard") return "keyboard";
    if (kind === "touch") return "touch";
    return "mouse";
  }

  /**
   * The live registration a projected target id names, if it still exists.
   *
   * A projection can name a DOM target or, on the keyboard route, a logical
   * one. It cannot name both and it cannot name two, which is what keeps one
   * host gesture from producing two simultaneous drops.
   */
  function projectedRegistration(targetId: string): {
    disabled?: boolean;
    acceptedKinds: readonly string[];
    canDrop: DropTargetRegistration["canDrop"];
    label: string;
  } | null {
    const dom = targets.get(targetId);
    if (dom) {
      if (isDisabled(dom.element)) return null;
      return dom.registration;
    }
    const logical = keyboardTargets.get(targetId);
    return logical ? logical.registration : null;
  }

  /**
   * Re-run this window's own gates over a host-supplied projection.
   *
   * The host decided *which* target the gesture is over; it does not decide
   * whether that target will take it. Kind, disabled posture, and `canDrop`
   * are consumer state the host cannot see, and they are checked here on every
   * projection and again at commit — hover acceptance never authorizes a
   * durable move.
   */
  function applyProjection(projection: CrossWindowDragProjection): void {
    const session = context.session;
    if (!session || phase !== "dragging") return;

    if (projection.targetId === null || projection.position === null) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      if (session.intent) dispatch({ type: "TARGET_CLEARED", sessionId: session.sessionId });
      else {
        projectAttributes();
        notify();
      }
      return;
    }

    const registration = projectedRegistration(projection.targetId);
    const intent: DropIntent = {
      targetId: projection.targetId,
      position: projection.position,
      operation: projection.operation,
    };

    let eligibility: DropEligibility = { accepted: false };
    if (
      registration &&
      !registration.disabled &&
      registration.acceptedKinds.includes(projection.subject.kind)
    ) {
      eligibility = eligibilityFromCanDrop(
        registration.canDrop(intent, projection.subject),
        intent,
      );
    }

    if (eligibility.accepted) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      dispatch({ type: "TARGET_INTENT", sessionId: session.sessionId, intent: eligibility.intent });
      return;
    }

    rejectedTargetId = registration ? projection.targetId : null;
    rejectedReason = eligibility.accepted === false ? eligibility.reason : undefined;
    if (session.intent) dispatch({ type: "TARGET_CLEARED", sessionId: session.sessionId });
    else {
      projectAttributes();
      notify();
    }
  }

  /**
   * Begin, update, or refuse the one incoming host transaction.
   *
   * A local gesture always wins: the user's own pointer or keyboard owns this
   * controller, and a projection arriving mid-drag would otherwise supersede a
   * drag the user is still making. The host is free to project again once the
   * local gesture ends.
   */
  function onCrossWindowTargetEvent(event: CrossWindowDragTargetEvent): void {
    if (destroyed || !connectedRoot) return;

    if (event.type === "projection") {
      const projection = event.projection;
      if (!isCrossWindowDragReceipt(projection.receipt)) return;

      const live = crossWindowTarget;
      if (live && sameCrossWindowDragReceipt(live.receipt, projection.receipt)) {
        if (context.session?.sessionId !== live.sessionId) return;
        live.projection = projection;
        applyProjection(projection);
        return;
      }

      if (live) releaseCrossWindowTargetSession("superseded");
      if (gesture !== null || keyboardSourceId !== null || phase !== "idle") return;

      const sessionId = createSessionId();
      const transaction: CrossWindowTargetTransaction = {
        sessionId,
        receipt: projection.receipt,
        projection,
        abort: new AbortController(),
        committing: false,
      };
      crossWindowTarget = transaction;
      crossWindowSourceLabel = projection.sourceLabel;
      inputKind = projectedInputKind(projection.inputKind);
      // No local pointer was ever observed, so there is no local preview: a
      // cross-window drag's preview belongs to whoever owns the cursor.
      pointerPosition = null;

      dispatch({
        type: "PREPARE",
        sessionId,
        sourceId: projection.sourceId,
        subject: projection.subject,
        operation: projection.operation,
        allowedOperations: [projection.operation],
      });
      if (context.session?.sessionId !== sessionId) {
        releaseCrossWindowTarget();
        return;
      }
      dispatch({ type: "ACTIVATE", sessionId });
      if (currentPhase() !== "dragging") {
        releaseCrossWindowTarget();
        return;
      }
      applyProjection(projection);
      maybePickTarget(transaction);
      return;
    }

    const live = crossWindowTarget;
    if (!live || !sameCrossWindowDragReceipt(live.receipt, event.receipt)) return;

    if (event.type === "left") {
      rejectedTargetId = null;
      rejectedReason = undefined;
      if (context.session?.sessionId === live.sessionId && context.session.intent) {
        dispatch({ type: "TARGET_CLEARED", sessionId: live.sessionId });
      } else {
        projectAttributes();
        notify();
      }
      return;
    }

    releaseCrossWindowTargetSession(event.reason);
  }

  /** End the incoming session with the host's reason, exactly once. */
  function releaseCrossWindowTargetSession(reason: DragCancelReason): void {
    const live = crossWindowTarget;
    if (!live) return;
    if (context.session?.sessionId === live.sessionId && phase !== "idle") {
      dispatch({ type: "HOST_TERMINAL", sessionId: live.sessionId, outcome: { status: "cancelled", reason } });
      return;
    }
    releaseCrossWindowTarget();
  }

  /**
   * The accessible cross-window route.
   *
   * The picker is how a keyboard transfer resolves its destination: the host
   * owns the chooser, returns a projection, and Poodle runs the same
   * revalidation, commit, announcement, and terminal path the pointer takes.
   * A `null` choice leaves the session with no intent rather than inventing
   * one, and a target that goes stale between the choice and the commit is
   * refused by ordinary revalidation.
   */
  function maybePickTarget(transaction: CrossWindowTargetTransaction): void {
    const bridge = crossWindowTargetBridge;
    if (!bridge || transaction.projection.inputKind !== "keyboard") return;
    if (!bridge.capabilities.keyboardTargetPicker) return;
    const pick = bridge.pickTarget;
    if (!pick) return;

    void Promise.resolve(pick.call(bridge, transaction.receipt, transaction.abort.signal)).then(
      (projection) => {
        if (crossWindowTarget !== transaction || transaction.abort.signal.aborted) return;
        if (context.session?.sessionId !== transaction.sessionId || phase !== "dragging") return;
        if (projection === null) return;
        if (!sameCrossWindowDragReceipt(projection.receipt, transaction.receipt)) return;
        transaction.projection = projection;
        applyProjection(projection);
        if (context.session?.intent) {
          dispatch({ type: "DROP_REQUESTED", sessionId: transaction.sessionId });
        }
      },
      () => {
        if (crossWindowTarget !== transaction) return;
        releaseCrossWindowTargetSession("transport-lost");
      },
    );
  }

  /**
   * `dragover` is the only place the browser will let us claim the drop, and
   * the only place it hides the body. The declared type says the drag carries
   * our envelope; the live projection says whether this window wants it. Both
   * are required, because either alone would accept a drop it cannot honour.
   */
  function onNativeDragOver(event: Event): void {
    if (!(event instanceof DragEvent) || !event.dataTransfer) return;
    const live = crossWindowTarget;
    if (!live || context.session?.sessionId !== live.sessionId) return;
    if (!crossWindowCodec.accepts(event.dataTransfer)) return;
    if (!context.session.intent) return;

    event.preventDefault();
    event.dataTransfer.dropEffect = nativeEffectFor(context.session.operation);
  }

  /**
   * The drop envelope has to name the transaction this window is projecting.
   *
   * A receipt that is absent, malformed, or simply different is a drag this
   * window never agreed to take, and hover acceptance does not carry over to
   * it: the intent is cleared and no commit runs. The host's own terminal
   * still ends the session, because the host is the only thing that knows what
   * became of its lease.
   */
  function onNativeDrop(event: Event): void {
    if (!(event instanceof DragEvent) || !event.dataTransfer) return;
    const live = crossWindowTarget;
    if (!live || context.session?.sessionId !== live.sessionId) return;
    if (!crossWindowCodec.accepts(event.dataTransfer)) return;

    event.preventDefault();
    const receipt = crossWindowCodec.read(event.dataTransfer);
    if (!receipt || !sameCrossWindowDragReceipt(receipt, live.receipt)) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      if (context.session.intent) {
        dispatch({ type: "TARGET_CLEARED", sessionId: live.sessionId });
      }
      return;
    }

    if (!context.session.intent || live.committing) return;
    dispatch({ type: "DROP_REQUESTED", sessionId: live.sessionId });
  }

  /**
   * Ask the host to make the projected drop durable, after this window has
   * re-checked its own gates one last time.
   */
  function requestCrossWindowCommit(
    transaction: CrossWindowTargetTransaction,
    sessionId: string,
    intent: DropIntent,
  ): void {
    const bridge = crossWindowTargetBridge;
    const session = context.session;
    if (!bridge || !session) {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
      return;
    }

    const registration = projectedRegistration(intent.targetId);
    if (
      !registration ||
      registration.disabled ||
      !registration.acceptedKinds.includes(session.subject.kind)
    ) {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
      return;
    }

    const eligibility = eligibilityFromCanDrop(
      registration.canDrop(intent, session.subject),
      intent,
    );
    if (eligibility.accepted === false) {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: eligibility.reason });
      return;
    }

    const generation = dropGeneration;
    transaction.committing = true;
    let pending: Promise<DragDropCommitResult>;
    try {
      pending = bridge.commit(
        {
          receipt: transaction.receipt,
          subject: session.subject,
          intent: eligibility.intent,
        },
        transaction.abort.signal,
      );
    } catch (error) {
      dispatch({
        type: "DROP_FAILED",
        sessionId,
        reason: error instanceof Error ? error.message : String(error),
      });
      return;
    }

    void Promise.resolve(pending).then(
      (commit) => {
        if (generation !== dropGeneration) return;
        applyCommit(sessionId, eligibility.intent, commit);
      },
      (error: unknown) => {
        if (generation !== dropGeneration) return;
        dispatch({
          type: "DROP_FAILED",
          sessionId,
          reason: error instanceof Error ? error.message : String(error),
        });
      },
    );
  }

  function returnFocus(subject: DragSubject): void {
    const source = currentSource();
    const surviving = source?.element;
    if (surviving?.isConnected) {
      focusHost(surviving);
      return;
    }

    for (const entry of sources.values()) {
      if (entry.registration.subject.kind === subject.kind && entry.registration.subject.id === subject.id) {
        if (entry.element.isConnected) {
          focusHost(entry.element);
          return;
        }
      }
    }
  }

  function releasePointerHardware(): void {
    if (moveFrame !== null && connectedWindow) {
      connectedWindow.cancelAnimationFrame(moveFrame);
      moveFrame = null;
    }
    pendingMove = null;

    if (gesture?.holdTimer !== null && gesture?.holdTimer !== undefined) {
      clearTimeout(gesture.holdTimer);
      gesture.holdTimer = null;
    }

    const captureStyle = gesture?.captureElement ? styledElement(gesture.captureElement) : null;
    if (captureStyle) {
      if (gesture?.restoredTouchAction === null) captureStyle.style.removeProperty(TOUCH_ACTION);
      else if (gesture?.restoredTouchAction) {
        captureStyle.style.setProperty(TOUCH_ACTION, gesture.restoredTouchAction);
      }
    }

    if (gesture?.captureElement && canPointerCapture(gesture.captureElement)) {
      try {
        (gesture.captureElement as HTMLElement).releasePointerCapture(gesture.pointerId);
      } catch {
        // Capture may already have been released by the browser.
      }
    }

    restoreRootUserSelect();
  }

  function restoreRootUserSelect(): void {
    if (!connectedRoot || restoredRootUserSelect === undefined) return;
    const style = styledElement(connectedRoot);
    if (style) {
      if (restoredRootUserSelect === null || restoredRootUserSelect === "") {
        style.style.removeProperty("user-select");
      } else {
        style.style.setProperty("user-select", restoredRootUserSelect);
      }
    }
    restoredRootUserSelect = undefined;
  }

  /**
   * Give up a gesture that never became a drag.
   *
   * A local source has nothing to give up — no session exists before
   * activation. A cross-window source does: its preparation started at
   * pointer-down, so a tap, a scroll, an Escape, or a pointer cancel has to
   * release the host lease rather than leave it allocated for a drag that will
   * never happen.
   */
  function abandonUnarmedGesture(): void {
    const pending = gesture;
    if (
      pending &&
      context.session?.sessionId === pending.sessionId &&
      phase !== "idle" &&
      phase !== "ended" &&
      phase !== "cancelled"
    ) {
      dispatch({ type: "CANCEL", sessionId: pending.sessionId });
    }
    stopAutoScroll();
    releasePointerHardware();
    gesture = null;
    inputKind = null;
    pointerPosition = null;
  }

  function stopCandidate(): void {
    if (gesture && !gesture.activated) {
      abandonUnarmedGesture();
    }
  }

  function beginSession(
    source: SourceEntry,
    kind: DragInputKind,
    x: number,
    y: number,
    reuseSessionId?: string,
  ): string {
    const operation = source.registration.operation ?? source.registration.allowedOperations[0];
    if (operation === undefined) {
      throw new Error(`Source "${source.registration.sourceId}" has no allowed operations`);
    }

    // A cross-window gesture prepares at pointer-down, so the session it
    // creates is the one the gesture already carries: minting a second id here
    // would leave the host's completion naming a session nobody is waiting on.
    const sessionId = reuseSessionId ?? createSessionId();
    inputKind = kind;
    keyboardLogicalSession = kind === "keyboard" && matchingLogicalKeyboard(source);
    pointerPosition = { x, y };
    dispatch({
      type: "PREPARE",
      sessionId,
      sourceId: source.registration.sourceId,
      subject: source.registration.subject,
      operation,
      allowedOperations: source.registration.allowedOperations,
    });
    return sessionId;
  }

  function armAndActivate(
    source: SourceEntry,
    kind: DragInputKind,
    x: number,
    y: number,
    captureElement: Element | null,
    pointerId: number | null,
  ): void {
    const sessionId = beginSession(source, kind, x, y);
    if (gesture) gesture.sessionId = sessionId;
    activate(sessionId, captureElement, pointerId);
  }

  /**
   * The activation constraint is satisfied. Enter `dragging` if the session is
   * ready, and otherwise wait for the host.
   *
   * A local source arms synchronously, so this is `armAndActivate` under
   * another name. A cross-window source may still be `preparing`: the gesture
   * is committed but the receipt is not armed, and starting the drag anyway
   * would advertise a transfer the host has not agreed to. The preparation's
   * own resolution activates it instead.
   */
  function reachThreshold(
    source: SourceEntry,
    kind: DragInputKind,
    x: number,
    y: number,
    captureElement: Element | null,
    pointerId: number | null,
  ): void {
    const pending = gesture;
    if (pending && context.session?.sessionId === pending.sessionId) {
      if (phase === "preparing") return;
      const bridge = source.registration.crossWindowSourceBridge;
      // The browser's own `dragstart` activates a native cross-window gesture,
      // because that is the only moment its envelope can be written. Arming
      // here as well would start the session with the wrong transport.
      if (bridge && crossWindowUsesNativeDrag(bridge, kind) && crossWindowSource?.sessionId === pending.sessionId) {
        return;
      }
      if (phase === "armed") {
        activate(pending.sessionId, captureElement, pointerId);
        if (gesture) hitTest(x, y);
      }
      return;
    }

    armAndActivate(source, kind, x, y, captureElement, pointerId);
    if (gesture) hitTest(x, y);
  }

  function activate(sessionId: string, captureElement: Element | null, pointerId: number | null): void {
    dispatch({ type: "ACTIVATE", sessionId });
    if (phase !== "dragging") return;

    if (gesture) gesture.activated = true;

    const rootStyle = connectedRoot ? styledElement(connectedRoot) : null;
    if (rootStyle && restoredRootUserSelect === undefined) {
      restoredRootUserSelect = rootStyle.style.getPropertyValue("user-select");
      rootStyle.style.setProperty("user-select", "none");
    }

    if (captureElement && pointerId !== null) {
      const style = styledElement(captureElement);
      const restored = style?.style.getPropertyValue(TOUCH_ACTION) || null;
      style?.style.setProperty(TOUCH_ACTION, "none");
      if (canPointerCapture(captureElement)) {
        try {
          (captureElement as HTMLElement).setPointerCapture(pointerId);
        } catch {
          // jsdom and some test doubles do not implement capture.
        }
      }
      if (gesture) {
        gesture.captureElement = captureElement;
        gesture.restoredTouchAction = restored;
      }
    }
    startAutoScroll();
    if (crossWindowSource?.sessionId === sessionId) {
      startCrossWindowTransport(sessionId, crossWindowTransport(inputKind ?? "mouse"));
    }
    notify();
  }

  function scrollOwnerId(element: Element): string {
    const existing = scrollOwnerIds.get(element);
    if (existing) return existing;
    scrollOwnerSeq += 1;
    const id = `scroll-${scrollOwnerSeq}`;
    scrollOwnerIds.set(element, id);
    return id;
  }

  function scrollOwnerDepth(element: Element): number {
    let depth = isScrollOwner(element) ? 1 : 0;
    let current: HTMLElement | null = element.parentElement;
    while (current) {
      if (isScrollOwner(current)) depth += 1;
      current = current.parentElement;
    }
    return depth;
  }

  function considerScrollOwner(
    element: Element | null,
    seen: Set<Element>,
    list: Array<AutoScrollCandidate & { element: HTMLElement }>,
    explicit = false,
  ): void {
    if (!(element instanceof HTMLElement) || seen.has(element)) return;
    if (!explicit && !isScrollOwner(element)) return;
    seen.add(element);
    list.push({
      id: scrollOwnerId(element),
      depth: scrollOwnerDepth(element),
      metrics: measureScrollMetrics(element),
      element,
    });
  }

  function collectAutoScrollCandidates(): Array<AutoScrollCandidate & { element: HTMLElement }> {
    const seen = new Set<Element>();
    const list: Array<AutoScrollCandidate & { element: HTMLElement }> = [];
    const origins: Array<Element | null> = [];

    if (connectedDocument && pointerPosition) {
      origins.push(hitElementFromPoint(connectedDocument, pointerPosition.x, pointerPosition.y));
    }
    origins.push(connectedRoot);
    for (const target of targets.values()) {
      if (target.registration.autoScroll) origins.push(target.element);
    }

    for (const origin of origins) {
      const explicit = origin instanceof Element && [...targets.values()].some((target) => target.registration.autoScroll && target.element === origin);
      considerScrollOwner(origin, seen, list, explicit);
      if (origin instanceof Element) {
        for (const parent of collectScrollParents(origin)) {
          considerScrollOwner(parent, seen, list);
        }
      }
    }

    const scrolling = connectedDocument?.scrollingElement ?? null;
    considerScrollOwner(scrolling, seen, list);
    return list;
  }

  function applyAutoScroll(id: string, dx: number, dy: number, owners: Array<AutoScrollCandidate & { element: HTMLElement }>): boolean {
    const owner = owners.find((entry) => entry.id === id);
    if (!owner) return false;
    const beforeTop = owner.element.scrollTop;
    const beforeLeft = owner.element.scrollLeft;
    if (dy !== 0) owner.element.scrollTop = beforeTop + dy;
    if (dx !== 0) owner.element.scrollLeft = beforeLeft + dx;
    return owner.element.scrollTop !== beforeTop || owner.element.scrollLeft !== beforeLeft;
  }

  function stopAutoScroll(): void {
    autoScrollRunning = false;
    lastAutoScrollTs = null;
    if (autoScrollFrame !== null && connectedWindow) {
      connectedWindow.cancelAnimationFrame(autoScrollFrame);
    }
    autoScrollFrame = null;
  }

  function startAutoScroll(): void {
    if (phase !== "dragging" || inputKind === "keyboard") return;
    autoScrollRunning = true;
    scheduleAutoScroll();
  }

  function scheduleAutoScroll(): void {
    if (!autoScrollRunning || autoScrollFrame !== null || !connectedWindow) return;
    autoScrollFrame = -1;
    let sync = true;
    const frame = connectedWindow.requestAnimationFrame((now) => {
      if (autoScrollFrame === -1 || autoScrollFrame === frame) autoScrollFrame = null;
      const keepGoing = onAutoScrollFrame(now);
      if (!sync && keepGoing) scheduleAutoScroll();
    });
    sync = false;
    if (autoScrollFrame === -1) autoScrollFrame = frame;
  }

  function onAutoScrollFrame(now: number): boolean {
    if (!autoScrollRunning || phase !== "dragging" || !pointerPosition) {
      stopAutoScroll();
      return false;
    }
    const dt = lastAutoScrollTs === null ? 16 : Math.min(Math.max(now - lastAutoScrollTs, 0), 64);
    if (dt === 0) return true;
    lastAutoScrollTs = now;
    const owners = collectAutoScrollCandidates();
    const intent = resolveAutoScroll(owners, pointerPosition, dt);
    if (!intent || !applyAutoScroll(intent.id, intent.dx, intent.dy, owners)) {
      lastAutoScrollTs = null;
      return false;
    }
    layoutDirty = true;
    hitTest(pointerPosition.x, pointerPosition.y);
    return true;
  }

  function evaluateTarget(
    entry: TargetEntry,
    x: number,
    y: number,
    subject: DragSubject,
    operation: DragOperation,
    kind: DragInputKind,
  ): { candidate: DropTargetCandidate; rejected?: { reason?: string } } {
    const rect = measure(entry.element);
    const inside = containsPoint(rect, x, y);
    const position = inside
      ? entry.registration.resolvePosition({
          x,
          y,
          rect: rect as DOMRectReadOnly,
          subject,
          operation,
          inputKind: kind,
        })
      : null;

    let eligibility: DropEligibility = { accepted: false };
    if (
      position !== null &&
      !entry.registration.disabled &&
      entry.registration.acceptedKinds.includes(subject.kind)
    ) {
      const intent: DropIntent = {
        targetId: entry.registration.targetId,
        position,
        operation,
      };
      eligibility = eligibilityFromCanDrop(entry.registration.canDrop(intent, subject), intent);
    }

    return {
      candidate: {
        targetId: entry.registration.targetId,
        depth: targetDepth(entry.element),
        order: entry.order,
        priority: entry.registration.priority,
        containsPoint: inside && position !== null,
        eligibility,
      },
      rejected:
        inside && position !== null && eligibility.accepted === false
          ? { reason: eligibility.reason }
          : undefined,
    };
  }

  function hitTest(x: number, y: number): void {
    const session = context.session;
    if (!session || phase !== "dragging" || inputKind === null) return;

    refreshLayout();
    const candidates: DropTargetCandidate[] = [];
    let rejected: { targetId: string; reason?: string } | null = null;

    for (const entry of targets.values()) {
      const { candidate, rejected: local } = evaluateTarget(
        entry,
        x,
        y,
        session.subject,
        session.operation,
        inputKind,
      );
      candidates.push(candidate);
      if (local && rejected === null) {
        rejected = { targetId: entry.registration.targetId, reason: local.reason };
      }
    }

    const intent = resolveDropTarget(candidates);
    rejectedTargetId = intent ? null : rejected?.targetId ?? null;
    rejectedReason = intent ? undefined : rejected?.reason;

    if (intent) {
      dispatch({ type: "TARGET_INTENT", sessionId: session.sessionId, intent });
    } else if (session.intent) {
      dispatch({ type: "TARGET_CLEARED", sessionId: session.sessionId });
      if (rejectedTargetId) {
        announce("rejected", { status: "rejected", reason: rejectedReason });
      }
    } else if (
      snapshot.targetPosture !== (rejectedTargetId ? "rejected" : null) ||
      snapshot.rejectedReason !== rejectedReason ||
      snapshot.targetId !== rejectedTargetId
    ) {
      projectAttributes();
      notify();
      if (rejectedTargetId) {
        announce("rejected", { status: "rejected", reason: rejectedReason });
      }
    } else if (rejectedTargetId) {
      announce("rejected", { status: "rejected", reason: rejectedReason });
    }
  }

  function hasMatchingLogicalTarget(kind: string): boolean {
    for (const entry of keyboardTargets.values()) {
      if (entry.registration.acceptedKinds.includes(kind)) return true;
    }
    return false;
  }

  function matchingLogicalKeyboard(source?: SourceEntry | null): boolean {
    const kind = source?.registration.subject.kind;
    return source?.registration.keyboardOrder !== undefined && kind !== undefined && hasMatchingLogicalTarget(kind);
  }

  function requestKeyboardDrop(command: KeyboardDropCommand): boolean {
    if (destroyed || phase !== "idle") return false;
    const source = sources.get(command.sourceId);
    if (!source || source.registration.disabled || isDisabled(source.element)) return false;
    if (source.registration.allowedOperations.length === 0) return false;

    const logical = keyboardTargets.get(command.targetId);
    const dom = targets.get(command.targetId);
    const registration = logical?.registration ?? dom?.registration;
    if (!registration || registration.disabled) return false;
    if (!logical && dom && isDisabled(dom.element)) return false;
    if (!registration.acceptedKinds.includes(source.registration.subject.kind)) return false;

    const rect = measure(source.element);
    const sessionId = beginSession(source, "keyboard", rect.left + rect.width / 2, rect.top + rect.height / 2);
    keyboardLogicalSession = logical !== undefined;
    keyboardCommandSession = true;
    keyboardSourceId = source.registration.sourceId;
    lastKeyboardDirection = null;
    activate(sessionId, null, null);
    const session = context.session;
    if (!session || session.sessionId !== sessionId) return false;

    const operation = session.operation;
    const intent: DropIntent = {
      targetId: command.targetId,
      position: command.position,
      operation,
    };
    dispatch({ type: "TARGET_INTENT", sessionId, intent });
    if (context.session?.intent) {
      dispatch({ type: "DROP_REQUESTED", sessionId });
    } else {
      dispatch({ type: "CANCEL", sessionId });
    }
    return true;
  }

  function usesLogicalKeyboard(): boolean {
    return keyboardLogicalSession;
  }

  function targetForAnnouncement(targetId: string): KeyboardTargetEntry | TargetEntry | undefined {
    if (inputKind === "keyboard" && usesLogicalKeyboard()) {
      return keyboardTargets.get(targetId) ?? targets.get(targetId);
    }
    return targets.get(targetId);
  }

  function liveDropRegistration(intent: DropIntent): {
    disabled?: boolean;
    canDrop: DropTargetRegistration["canDrop"];
    onDrop: DropTargetRegistration["onDrop"];
  } | null {
    if (keyboardCommandSession || (inputKind === "keyboard" && usesLogicalKeyboard())) {
      const logical = keyboardTargets.get(intent.targetId);
      if (logical) return logical.registration;
      if (keyboardCommandSession) {
        const dom = targets.get(intent.targetId);
        return dom?.registration ?? null;
      }
      return null;
    }
    const target = targets.get(intent.targetId);
    if (!target) return null;
    return target.registration;
  }

  function commandTargetUnavailable(intent: DropIntent): boolean {
    const logical = keyboardTargets.get(intent.targetId);
    if (logical) return Boolean(logical.registration.disabled);
    const dom = targets.get(intent.targetId);
    if (!dom) return true;
    return Boolean(dom.registration.disabled) || isDisabled(dom.element);
  }

  function requestDrop(sessionId: string, intent: DropIntent): void {
    const generation = dropGeneration;
    const session = context.session;

    // An incoming host transaction revalidates semantically, not spatially:
    // the position came from the host's own geometry, and this window never
    // measured a pointer for it.
    if (crossWindowTarget?.sessionId === sessionId) {
      requestCrossWindowCommit(crossWindowTarget, sessionId, intent);
      return;
    }

    const registration = liveDropRegistration(intent);

    if (!session || session.sessionId !== sessionId || !registration || registration.disabled) {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
      return;
    }

    let accepted: DropIntent = intent;
    if (keyboardCommandSession) {
      if (commandTargetUnavailable(intent)) {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
        return;
      }
      const eligibility = eligibilityFromCanDrop(registration.canDrop(intent, session.subject), intent);
      if (eligibility.accepted === false) {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: eligibility.reason });
        return;
      }
      accepted = eligibility.intent;
    } else if (inputKind === "keyboard" && usesLogicalKeyboard()) {
      const logical = keyboardTargets.get(intent.targetId);
      let position = intent.position;
      if (lastKeyboardDirection !== null) {
        const resolved = logical
          ? logical.registration.resolvePosition({
              direction: lastKeyboardDirection,
              subject: session.subject,
              operation: session.operation,
            })
          : intent.position;
        if (resolved === null) {
          dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
          return;
        }
        position = resolved;
      }
      const revalidated: DropIntent = {
        targetId: intent.targetId,
        position,
        operation: session.operation,
      };
      const eligibility = eligibilityFromCanDrop(registration.canDrop(revalidated, session.subject), revalidated);
      if (eligibility.accepted === false) {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: eligibility.reason });
        return;
      }
      accepted = eligibility.intent;
    } else {
      const target = targets.get(intent.targetId);
      if (!target) {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
        return;
      }
      refreshLayout();
      const pointer = pointerPosition ?? { x: 0, y: 0 };
      const { candidate } = evaluateTarget(
        target,
        pointer.x,
        pointer.y,
        session.subject,
        intent.operation,
        inputKind ?? "mouse",
      );
      const eligibility = candidate.eligibility;
      if (eligibility.accepted === false) {
        dispatch({
          type: "DROP_REJECTED",
          sessionId,
          reason: eligibility.reason,
        });
        return;
      }
      accepted = eligibility.intent;
    }

    try {
      const result = registration.onDrop(accepted);
      if (result !== undefined && typeof (result as Promise<DragDropCommitResult>).then === "function") {
        void Promise.resolve(result).then(
          (commit) => {
            if (generation !== dropGeneration) return;
            applyCommit(sessionId, accepted, commit);
          },
          (error: unknown) => {
            if (generation !== dropGeneration) return;
            dispatch({
              type: "DROP_FAILED",
              sessionId,
              reason: error instanceof Error ? error.message : String(error),
            });
          },
        );
        return;
      }

      applyCommit(sessionId, accepted, result as DragDropCommitResult);
    } catch (error) {
      dispatch({
        type: "DROP_FAILED",
        sessionId,
        reason: error instanceof Error ? error.message : String(error),
      });
    }
  }

  function applyCommit(sessionId: string, intent: DropIntent, commit: DragDropCommitResult): void {
    if (crossWindowTarget?.sessionId === sessionId) {
      const projected = projectedRegistration(intent.targetId);
      if (!projected || projected.disabled) {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
        return;
      }
      if (commit.status === "committed") {
        dispatch({ type: "DROP_COMMITTED", sessionId, intent });
      } else if (commit.status === "rejected") {
        dispatch({ type: "DROP_REJECTED", sessionId, reason: commit.reason });
      } else {
        dispatch({ type: "DROP_FAILED", sessionId, reason: commit.reason });
      }
      return;
    }

    const liveTarget = liveDropRegistration(intent);
    if (!liveTarget || liveTarget.disabled || (keyboardCommandSession && commandTargetUnavailable(intent))) {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: "target-unavailable" });
      return;
    }

    if (commit.status === "committed") {
      dispatch({ type: "DROP_COMMITTED", sessionId, intent });
      return;
    }
    if (commit.status === "rejected") {
      dispatch({ type: "DROP_REJECTED", sessionId, reason: commit.reason });
      return;
    }
    dispatch({ type: "DROP_FAILED", sessionId, reason: commit.reason });
  }

  function sourceFromEvent(event: Event): SourceEntry | null {
    for (const node of eventPath(event)) {
      if (!(node instanceof Element)) continue;
      const id = sourcesByElement.get(node);
      if (id === undefined) continue;
      const entry = sources.get(id);
      if (!entry || entry.registration.disabled || isDisabled(entry.element)) continue;
      const handle = resolveHandle(entry.element, entry.registration.handle);
      const target = event.target;
      if (target instanceof Node && !handle.contains(target) && handle !== target) continue;
      const interactive = interactiveHost(target);
      if (interactive && interactive !== handle) continue;
      return entry;
    }
    return null;
  }

  function isPrimaryPointer(event: PointerEvent): boolean {
    if (event.pointerType === "touch") return event.isPrimary !== false;
    return event.button === 0;
  }

  function onPointerDown(event: Event): void {
    if (!(event instanceof PointerEvent)) return;
    if (!isPrimaryPointer(event)) return;
    if (gesture || phase !== "idle") return;

    const source = sourceFromEvent(event);
    if (!source) return;
    if (connectedRoot && !connectedRoot.contains(source.element) && connectedRoot !== source.element) {
      return;
    }

    const kind = asInputKind(event.pointerType);
    const sessionId = createSessionId();
    inputKind = kind;
    pointerPosition = { x: event.clientX, y: event.clientY };
    gesture = {
      pointerId: event.pointerId,
      pointerType: kind,
      originX: event.clientX,
      originY: event.clientY,
      x: event.clientX,
      y: event.clientY,
      sourceId: source.registration.sourceId,
      sessionId,
      activated: false,
      thresholdReached: false,
      holdTimer: null,
      captureElement: null,
      restoredTouchAction: null,
    };

    // The accepted pre-drag gesture, which is where host preparation belongs:
    // a lease allocated inside the activation threshold would have to be
    // allocated synchronously, and a host that cannot answer synchronously
    // would have to be refused.
    const bridge = source.registration.crossWindowSourceBridge;
    if (bridge && crossWindowCarries(bridge, kind)) {
      beginSession(source, kind, event.clientX, event.clientY, sessionId);
    }

    if (kind === "touch") {
      const touch = activationFor(source.registration, "touch") as DragActivationHold;
      gesture.holdTimer = setTimeout(() => {
        if (destroyed || !connectedRoot) return;
        if (!gesture || gesture.sessionId !== sessionId || gesture.activated) return;
        const live = sources.get(source.registration.sourceId);
        if (!live || live.registration.disabled) return;
        const capture = resolveHandle(live.element, live.registration.handle);
        gesture.thresholdReached = true;
        reachThreshold(live, kind, gesture.x, gesture.y, capture, event.pointerId);
      }, touch.holdMs);
      return;
    }

    // Mouse/pen wait for distance. Do not capture yet so scrolling/selection
    // still win on a tap.
  }

  function flushMove(x: number, y: number): void {
    if (!gesture) return;
    gesture.x = x;
    gesture.y = y;
    pointerPosition = { x, y };

    if (!gesture.activated) {
      const source = sources.get(gesture.sourceId);
      if (!source) return;
      const kind = gesture.pointerType;
      const origin = { x: gesture.originX, y: gesture.originY };
      const travelled = distance(origin.x, origin.y, x, y);

      if (kind === "touch") {
        const touch = activationFor(source.registration, "touch") as DragActivationHold;
        if (travelled > touch.tolerance) {
          abandonUnarmedGesture();
        }
        return;
      }

      const constraint = activationFor(source.registration, kind) as DragActivationDistance;
      if (travelled >= constraint.distance) {
        const handle = resolveHandle(source.element, source.registration.handle);
        gesture.thresholdReached = true;
        reachThreshold(source, kind, x, y, handle, gesture.pointerId);
      }
      return;
    }

    hitTest(x, y);
    if (gesture?.activated) scheduleAutoScroll();
  }

  function suppressScroll(event: Event): void {
    if (gesture?.activated) event.preventDefault();
  }

  function onPointerMove(event: Event): void {
    if (!(event instanceof PointerEvent) || !gesture || event.pointerId !== gesture.pointerId) return;

    if (gesture.activated) {
      event.preventDefault();
    }

    pendingMove = { x: event.clientX, y: event.clientY };
    if (!connectedWindow) {
      flushMove(event.clientX, event.clientY);
      return;
    }
    if (moveFrame !== null) return;

    moveFrame = -1;
    const frame = connectedWindow.requestAnimationFrame(() => {
      moveFrame = null;
      const pending = pendingMove;
      pendingMove = null;
      if (pending) flushMove(pending.x, pending.y);
    });
    if (moveFrame === -1) moveFrame = frame;
  }

  function onPointerUp(event: Event): void {
    if (!(event instanceof PointerEvent) || !gesture || event.pointerId !== gesture.pointerId) return;

    pendingMove = null;
    if (moveFrame !== null && connectedWindow) {
      connectedWindow.cancelAnimationFrame(moveFrame);
      moveFrame = null;
    }
    flushMove(event.clientX, event.clientY);
    if (!gesture) return;

    const sessionId = gesture.sessionId;
    const activated = gesture.activated;
    const intent = context.session?.intent;

    if (!activated) {
      abandonUnarmedGesture();
      return;
    }

    releasePointerHardware();
    gesture = null;

    if (intent) {
      dispatch({ type: "DROP_REQUESTED", sessionId });
      return;
    }

    dispatch({ type: "CANCEL", sessionId });
  }

  function onPointerCancel(event: Event): void {
    if (!(event instanceof PointerEvent) || !gesture || event.pointerId !== gesture.pointerId) return;
    if (!gesture.activated) {
      abandonUnarmedGesture();
      return;
    }
    const sessionId = gesture.sessionId;
    dispatch({ type: "TRANSPORT_LOST", sessionId });
  }

  function onLostCapture(event: Event): void {
    if (!(event instanceof PointerEvent) || !gesture || event.pointerId !== gesture.pointerId) return;
    if (!gesture.activated) return;
    const sessionId = gesture.sessionId;
    dispatch({ type: "TRANSPORT_LOST", sessionId });
  }

  function onVisibility(): void {
    if (!connectedDocument || connectedDocument.visibilityState !== "hidden") return;
    stopCandidate();
    const sessionId = context.session?.sessionId;
    if (!sessionId) return;
    dispatch({ type: "WINDOW_LOST", sessionId });
  }

  function onScrollOrResize(): void {
    layoutDirty = true;
    if (gesture && !gesture.activated) {
      stopCandidate();
    }
    if (phase === "dragging" && pointerPosition) {
      hitTest(pointerPosition.x, pointerPosition.y);
      scheduleAutoScroll();
    }
  }

  function spatialTargets(): TargetEntry[] {
    refreshLayout();
    return [...targets.values()].sort((left, right) => {
      const a = measure(left.element);
      const b = measure(right.element);
      if (a.top !== b.top) return a.top - b.top;
      if (a.left !== b.left) return a.left - b.left;
      return left.order - right.order;
    });
  }

  function eligibleKeyboardTargets(subject: DragSubject, operation: DragOperation): TargetEntry[] {
    const listed: TargetEntry[] = [];
    for (const entry of spatialTargets()) {
      const rect = measure(entry.element);
      const x = rect.left + rect.width / 2;
      const y = rect.top + rect.height / 2;
      const { candidate } = evaluateTarget(entry, x, y, subject, operation, "keyboard");
      if (candidate.eligibility.accepted) listed.push(entry);
    }
    return listed;
  }

  function spatialCompare(left: CachedRect, right: CachedRect): number {
    if (left.top !== right.top) return left.top - right.top;
    if (left.left !== right.left) return left.left - right.left;
    return 0;
  }

  function firstTargetAfterSource(listed: TargetEntry[], source: SourceEntry | undefined): number {
    if (!source) return listed.length > 0 ? 0 : -1;
    const origin = measure(source.element);
    for (let index = 0; index < listed.length; index += 1) {
      const entry = listed[index];
      if (entry && spatialCompare(measure(entry.element), origin) > 0) return index;
    }
    return -1;
  }

  function firstTargetBeforeSource(listed: TargetEntry[], source: SourceEntry | undefined): number {
    if (!source) return listed.length - 1;
    const origin = measure(source.element);
    for (let index = listed.length - 1; index >= 0; index -= 1) {
      const entry = listed[index];
      if (entry && spatialCompare(measure(entry.element), origin) < 0) return index;
    }
    return -1;
  }

  function applyKeyboardIntent(entry: TargetEntry, session: DragSession): void {
    const rect = measure(entry.element);
    const x = rect.left + rect.width / 2;
    const y = rect.top + rect.height / 2;
    pointerPosition = { x, y };
    const { candidate } = evaluateTarget(entry, x, y, session.subject, session.operation, "keyboard");
    if (candidate.eligibility.accepted) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      dispatch({ type: "TARGET_INTENT", sessionId: session.sessionId, intent: candidate.eligibility.intent });
    }
  }

  function focusedSource(): SourceEntry | null {
    const active = connectedDocument?.activeElement;
    if (!(active instanceof Element)) return null;
    const id = sourcesByElement.get(active);
    if (id === undefined) return null;
    const entry = sources.get(id);
    if (!entry || entry.registration.disabled) return null;
    return entry;
  }

  function onKeyDown(event: Event): void {
    if (!(event instanceof KeyboardEvent)) return;

    if (event.key === "Escape") {
      if (gesture && !gesture.activated) {
        event.preventDefault();
        abandonUnarmedGesture();
        return;
      }
      if (context.session) {
        event.preventDefault();
        dispatch({ type: "ESCAPE", sessionId: context.session.sessionId });
      }
      return;
    }

    if (phase === "idle") {
      if (event.key !== " " && event.key !== "Enter") return;
      const source = focusedSource();
      if (!source || source.registration.keyboardOrder === undefined) return;
      event.preventDefault();
      const rect = measure(source.element);
      const x = rect.left + rect.width / 2;
      const y = rect.top + rect.height / 2;
      const sessionId = beginSession(source, "keyboard", x, y);
      activate(sessionId, null, null);
      keyboardSourceId = source.registration.sourceId;
      keyboardTargetIndex = -1;
      return;
    }

    if (phase !== "dragging" || inputKind !== "keyboard" || !context.session) return;

    const session = context.session;
    const source = keyboardSourceId ? sources.get(keyboardSourceId) : undefined;

    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      if (session.intent) dispatch({ type: "DROP_REQUESTED", sessionId: session.sessionId });
      else dispatch({ type: "CANCEL", sessionId: session.sessionId });
      return;
    }

    if (usesLogicalKeyboard()) {
      onLogicalKeyboardMove(event, session, source);
      return;
    }

    const listed = eligibleKeyboardTargets(session.subject, session.operation);
    if (listed.length === 0) return;

    let next = keyboardTargetIndex;
    if (event.key === "Home") {
      event.preventDefault();
      next = 0;
    } else if (event.key === "End") {
      event.preventDefault();
      next = listed.length - 1;
    } else if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      if (keyboardTargetIndex < 0) {
        next = firstTargetAfterSource(listed, source);
        if (next < 0) return;
      } else if (keyboardTargetIndex >= listed.length - 1) {
        return;
      } else {
        next = keyboardTargetIndex + 1;
      }
    } else if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      if (keyboardTargetIndex < 0) {
        next = firstTargetBeforeSource(listed, source);
        if (next < 0) return;
      } else if (keyboardTargetIndex <= 0) {
        return;
      } else {
        next = keyboardTargetIndex - 1;
      }
    } else {
      return;
    }

    const entry = listed[next];
    if (!entry) return;
    keyboardTargetIndex = next;
    applyKeyboardIntent(entry, session);
  }

  function eligibleLogicalTargets(subject: DragSubject, operation: DragOperation): KeyboardTargetEntry[] {
    return [...keyboardTargets.values()]
      .sort((left, right) => {
        if (left.registration.order !== right.registration.order) {
          return left.registration.order - right.registration.order;
        }
        return left.order - right.order;
      })
      .filter((entry) => {
        if (entry.registration.disabled) return false;
        return entry.registration.acceptedKinds.includes(subject.kind);
      });
  }

  function applyLogicalKeyboardIntent(
    entry: KeyboardTargetEntry,
    session: DragSession,
    direction: KeyboardDropDirection,
  ): void {
    lastKeyboardDirection = direction;
    const position = entry.registration.resolvePosition({
      direction,
      subject: session.subject,
      operation: session.operation,
    });
    if (position === null) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      if (session.intent) dispatch({ type: "TARGET_CLEARED", sessionId: session.sessionId });
      else {
        projectAttributes();
        notify();
      }
      return;
    }
    const intent: DropIntent = {
      targetId: entry.registration.targetId,
      position,
      operation: session.operation,
    };
    const eligibility = eligibilityFromCanDrop(entry.registration.canDrop(intent, session.subject), intent);
    if (eligibility.accepted) {
      rejectedTargetId = null;
      rejectedReason = undefined;
      dispatch({ type: "TARGET_INTENT", sessionId: session.sessionId, intent: eligibility.intent });
    } else {
      rejectedTargetId = entry.registration.targetId;
      rejectedReason = eligibility.accepted === false ? eligibility.reason : undefined;
      if (session.intent) dispatch({ type: "TARGET_CLEARED", sessionId: session.sessionId });
      else {
        projectAttributes();
        notify();
      }
    }
  }

  function onLogicalKeyboardMove(event: KeyboardEvent, session: DragSession, source: SourceEntry | undefined): void {
    const listed = eligibleLogicalTargets(session.subject, session.operation);
    if (listed.length === 0) return;

    const origin = source?.registration.keyboardOrder;
    let next = keyboardTargetIndex;
    let direction: KeyboardDropDirection;

    if (event.key === "Home") {
      event.preventDefault();
      next = 0;
      direction = "first";
    } else if (event.key === "End") {
      event.preventDefault();
      next = listed.length - 1;
      direction = "last";
    } else if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      direction = "next";
      if (keyboardTargetIndex < 0) {
        next = listed.findIndex((entry) => origin === undefined || entry.registration.order > origin);
        if (next < 0) return;
      } else if (keyboardTargetIndex >= listed.length - 1) {
        return;
      } else {
        next = keyboardTargetIndex + 1;
      }
    } else if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      direction = "previous";
      if (keyboardTargetIndex < 0) {
        next = -1;
        for (let index = listed.length - 1; index >= 0; index -= 1) {
          const entry = listed[index];
          if (entry && (origin === undefined || entry.registration.order < origin)) {
            next = index;
            break;
          }
        }
        if (next < 0) return;
      } else if (keyboardTargetIndex <= 0) {
        return;
      } else {
        next = keyboardTargetIndex - 1;
      }
    } else {
      return;
    }

    const entry = listed[next];
    if (!entry) return;
    keyboardTargetIndex = next;
    applyLogicalKeyboardIntent(entry, session, direction);
  }

  /** `effectAllowed` is presentation for the OS; the operation stays semantic. */
  function nativeEffectFor(operation: DragOperation): "move" | "copy" | "link" {
    return operation;
  }

  /**
   * The one moment a cross-window envelope can be written.
   *
   * A native drag is refused unless this source's own receipt is already
   * armed: an unarmed gesture would leave the window advertising a transfer
   * the host never agreed to, and `DataTransfer` cannot be written later.
   * Every other native drag stays refused exactly as before, so an ordinary
   * source never starts a second, browser-owned gesture beside the sensor's.
   */
  function onNativeDragStart(event: Event): void {
    if (!(event instanceof DragEvent)) return;
    const source = sourceFromEvent(event);
    if (!source) return;

    const transaction = crossWindowSource;
    const session = context.session;
    const armed =
      transaction !== null &&
      transaction.receipt !== null &&
      transaction.sourceId === source.registration.sourceId &&
      session?.sessionId === transaction.sessionId &&
      (phase === "armed" || phase === "dragging") &&
      (inputKind === "mouse" || inputKind === "pen") &&
      crossWindowUsesNativeDrag(transaction.bridge, inputKind) &&
      event.dataTransfer !== null;

    if (!armed || !transaction || !session || !transaction.receipt || !event.dataTransfer) {
      event.preventDefault();
      return;
    }

    try {
      crossWindowCodec.write(event.dataTransfer, transaction.receipt);
    } catch {
      event.preventDefault();
      return;
    }
    event.dataTransfer.effectAllowed = nativeEffectFor(session.operation);

    if (phase === "armed") activate(transaction.sessionId, null, null);
    if (phase !== "dragging") {
      event.preventDefault();
      return;
    }

    startCrossWindowTransport(transaction.sessionId, "data-transfer");
    nativeDragSessionId = transaction.sessionId;

    // The browser owns the gesture from here: our capture, touch-action, and
    // move coalescing would only fight it, and no pointer event will arrive to
    // release them.
    releasePointerHardware();
    gesture = null;
  }

  /**
   * The native transport closed. It is not a result.
   *
   * `dropEffect` at this point reports what the OS believes happened, which is
   * exactly the thing that must not become a commit: the host still owns the
   * lease and its terminal subscription is the only authority on the outcome.
   * A session whose host has already answered is long gone by now, so there is
   * nothing left to do but drop the local gesture state.
   */
  function onNativeDragEnd(event: Event): void {
    if (!(event instanceof DragEvent)) return;
    if (nativeDragSessionId === null) return;
    if (context.session?.sessionId !== nativeDragSessionId) {
      nativeDragSessionId = null;
      return;
    }
    nativeDragSessionId = null;
    releasePointerHardware();
    gesture = null;
  }

  function bindDocument(doc: Document, win: Window): void {
    const add = (
      type: string,
      listener: EventListener,
      options?: AddEventListenerOptions | boolean,
    ) => {
      doc.addEventListener(type, listener, options);
      documentListeners.push([type, listener, options]);
    };

    add("pointerdown", onPointerDown, true);
    add("pointermove", onPointerMove, { capture: true, passive: false });
    add("pointerup", onPointerUp, true);
    add("pointercancel", onPointerCancel, true);
    add("lostpointercapture", onLostCapture, true);
    add("touchmove", suppressScroll, { capture: true, passive: false });
    add("keydown", onKeyDown, true);
    add("dragstart", onNativeDragStart, true);
    add("dragend", onNativeDragEnd, true);
    if (crossWindowTargetBridge) {
      add("dragover", onNativeDragOver, true);
      add("drop", onNativeDrop, true);
    }
    add("scroll", onScrollOrResize, true);
    add("visibilitychange", onVisibility);

    win.addEventListener("resize", onScrollOrResize);
    documentListeners.push(["resize", onScrollOrResize as EventListener, undefined]);
  }

  function unbindDocument(): void {
    if (!connectedDocument) {
      documentListeners.length = 0;
      return;
    }

    for (const [type, listener, options] of documentListeners) {
      if (type === "resize" && connectedWindow) {
        connectedWindow.removeEventListener("resize", listener);
      } else {
        connectedDocument.removeEventListener(type, listener, options);
      }
    }
    documentListeners.length = 0;
  }

  function applySourceDom(entry: SourceEntry): void {
    const element = entry.element;
    if (
      !entry.addedTabIndex &&
      entry.authoredTabIndex === null &&
      isFocusableHost(element) &&
      element.tabIndex < 0
    ) {
      entry.addedTabIndex = true;
      element.tabIndex = 0;
    }
    if (
      !entry.addedAriaLabel &&
      entry.authoredAriaLabel === null &&
      !element.getAttribute("aria-label") &&
      !element.textContent?.trim()
    ) {
      entry.addedAriaLabel = true;
      element.setAttribute("aria-label", entry.registration.label);
    }
    if (entry.registration.instructions) {
      element.setAttribute("aria-description", entry.registration.instructions);
    } else if (entry.authoredAriaDescription === null) {
      element.removeAttribute("aria-description");
    } else {
      element.setAttribute("aria-description", entry.authoredAriaDescription);
    }
    // The browser's own drag is the web's only cross-window transport, so a
    // source whose host can carry a pointer transfer must advertise it. Every
    // other source stays non-draggable: the sensor owns the gesture and a
    // parallel native drag would fight it.
    const bridge = entry.registration.crossWindowSourceBridge;
    element.setAttribute(
      "draggable",
      bridge && bridge.capabilities.pointer ? "true" : "false",
    );
  }

  function restoreSourceDom(entry: SourceEntry): void {
    const element = entry.element;
    element.removeAttribute(SOURCE_ATTR);
    if (entry.addedTabIndex) {
      if (entry.authoredTabIndex === null) element.removeAttribute("tabindex");
      else element.setAttribute("tabindex", entry.authoredTabIndex);
    }
    if (entry.addedAriaLabel) {
      if (entry.authoredAriaLabel === null) element.removeAttribute("aria-label");
      else element.setAttribute("aria-label", entry.authoredAriaLabel);
    }
    if (entry.authoredAriaDescription === null) element.removeAttribute("aria-description");
    else element.setAttribute("aria-description", entry.authoredAriaDescription);
    if (entry.authoredDraggable === null) element.removeAttribute("draggable");
    else element.setAttribute("draggable", entry.authoredDraggable);
  }

  function unregisterSource(id: string): void {
    const entry = sources.get(id);
    if (!entry) return;
    if (gesture?.sourceId === id) stopCandidate();
    if (context.session?.sourceId === id && phase !== "idle" && phase !== "ended" && phase !== "cancelled") {
      dispatch({ type: "SOURCE_LOST", sessionId: context.session.sessionId });
    }
    restoreSourceDom(entry);
    resizeObserver?.unobserve(entry.element);
    sources.delete(id);
    sourcesByElement.delete(entry.element);
    rects.delete(entry.element);
    layoutDirty = true;
  }

  function loseIntentTarget(id: string): void {
    if (context.session?.intent?.targetId !== id) return;
    if (phase === "dropping") {
      dispatch({
        type: "DROP_REJECTED",
        sessionId: context.session.sessionId,
        reason: "target-unavailable",
      });
    } else if (phase === "dragging") {
      dispatch({ type: "TARGET_LOST", sessionId: context.session.sessionId, targetId: id });
    }
  }

  function unregisterTarget(id: string): void {
    const entry = targets.get(id);
    if (!entry) return;
    if (!(inputKind === "keyboard" && usesLogicalKeyboard() && keyboardTargets.has(id))) {
      loseIntentTarget(id);
    }
    entry.element.removeAttribute(TARGET_ATTR);
    entry.element.removeAttribute(POSITION_ATTR);
    resizeObserver?.unobserve(entry.element);
    targets.delete(id);
    targetsByElement.delete(entry.element);
    rects.delete(entry.element);
    layoutDirty = true;
  }

  function unregisterKeyboardTarget(id: string): void {
    const entry = keyboardTargets.get(id);
    if (!entry) return;
    if (inputKind === "keyboard" && usesLogicalKeyboard()) {
      loseIntentTarget(id);
    }
    keyboardTargets.delete(id);
  }

  return {
    capabilities: CAPABILITIES,

    connect(root: Element): () => void {
      assertLive();
      if (connectedRoot) {
        throw new Error("DragDropController is already connected");
      }

      const doc = root.ownerDocument;
      const win = doc.defaultView;
      if (!win) {
        throw new Error("DragDropController.connect requires a window");
      }

      connectedRoot = root;
      connectedDocument = doc;
      connectedWindow = win;
      bindDocument(doc, win);
      // Installed with the document, not with the controller: a projection
      // arriving before there is a window to render it into has nowhere to go.
      if (crossWindowTargetBridge) {
        if (crossWindowTargetBridge.capabilities.keyboardTargetPicker && !crossWindowTargetBridge.pickTarget) {
          throw new Error(
            "crossWindowTargetBridge advertises keyboardTargetPicker but implements no pickTarget",
          );
        }
        crossWindowUnsubscribe = crossWindowTargetBridge.subscribe(onCrossWindowTargetEvent);
      }
      if (typeof ResizeObserver === "function") {
        resizeObserver = new ResizeObserver(() => onScrollOrResize());
        resizeObserver.observe(root);
        for (const source of sources.values()) resizeObserver.observe(source.element);
        for (const target of targets.values()) resizeObserver.observe(target.element);
      }

      return () => {
        if (connectedRoot !== root) return;
        stopAutoScroll();
        stopCandidate();
        if (context.session) dispatch({ type: "CANCEL", sessionId: context.session.sessionId });
        const unsubscribe = crossWindowUnsubscribe;
        crossWindowUnsubscribe = null;
        unsubscribe?.();
        releaseCrossWindowSource("transport-lost");
        releaseCrossWindowTarget();
        unbindDocument();
        resizeObserver?.disconnect();
        resizeObserver = null;
        connectedRoot = null;
        connectedDocument = null;
        connectedWindow = null;
      };
    },

    registerSource(element: Element, registration: DragSourceRegistration): DragSourceHandle {
      assertLive();
      if (sources.has(registration.sourceId)) {
        throw new Error(`Duplicate drag source id "${registration.sourceId}"`);
      }
      if (sourcesByElement.has(element)) {
        throw new Error("Element is already registered as a drag source");
      }

      const entry: SourceEntry = {
        element,
        registration,
        order: nextOrder++,
        authoredTabIndex: element.getAttribute("tabindex"),
        authoredAriaLabel: element.getAttribute("aria-label"),
        authoredAriaDescription: element.getAttribute("aria-description"),
        authoredDraggable: element.getAttribute("draggable"),
        addedTabIndex: false,
        addedAriaLabel: false,
      };
      sources.set(registration.sourceId, entry);
      sourcesByElement.set(element, registration.sourceId);
      applySourceDom(entry);
      resizeObserver?.observe(element);
      layoutDirty = true;

      let live = true;
      return {
        update(next: DragSourceRegistration) {
          if (!live || destroyed) return;
          if (next.sourceId !== entry.registration.sourceId) {
            throw new Error(
              `Drag source id "${entry.registration.sourceId}" is immutable on a live handle`,
            );
          }
          const id = entry.registration.sourceId;
          const wasDisabled = entry.registration.disabled;
          entry.registration = next;
          applySourceDom(entry);
          if (!wasDisabled && next.disabled) {
            if (gesture?.sourceId === id) stopCandidate();
            if (context.session?.sourceId === id) {
              dispatch({ type: "SOURCE_LOST", sessionId: context.session.sessionId });
            }
          }
          layoutDirty = true;
        },
        unregister() {
          if (!live) return;
          live = false;
          if (destroyed) return;
          unregisterSource(entry.registration.sourceId);
        },
      };
    },

    registerTarget(element: Element, registration: DropTargetRegistration): DropTargetHandle {
      assertLive();
      if (targets.has(registration.targetId)) {
        throw new Error(`Duplicate drop target id "${registration.targetId}"`);
      }
      if (targetsByElement.has(element)) {
        throw new Error("Element is already registered as a drop target");
      }

      const entry: TargetEntry = {
        element,
        registration,
        order: nextOrder++,
      };
      targets.set(registration.targetId, entry);
      targetsByElement.set(element, registration.targetId);
      resizeObserver?.observe(element);
      layoutDirty = true;

      let live = true;
      return {
        update(next: DropTargetRegistration) {
          if (!live || destroyed) return;
          if (next.targetId !== entry.registration.targetId) {
            throw new Error(
              `Drop target id "${entry.registration.targetId}" is immutable on a live handle`,
            );
          }
          const id = entry.registration.targetId;
          const wasDisabled = entry.registration.disabled;
          entry.registration = next;
          layoutDirty = true;
          if (!wasDisabled && next.disabled && phase === "dropping" && context.session?.intent?.targetId === id) {
            dispatch({
              type: "DROP_REJECTED",
              sessionId: context.session.sessionId,
              reason: "target-unavailable",
            });
          }
        },
        unregister() {
          if (!live) return;
          live = false;
          if (destroyed) return;
          unregisterTarget(entry.registration.targetId);
        },
      };
    },

    requestKeyboardDrop,

    registerKeyboardTarget(registration: KeyboardDropTargetRegistration): KeyboardDropTargetHandle {
      assertLive();
      if (keyboardTargets.has(registration.targetId)) {
        throw new Error(`Duplicate keyboard drop target id "${registration.targetId}"`);
      }

      const entry: KeyboardTargetEntry = {
        registration,
        order: nextOrder++,
      };
      keyboardTargets.set(registration.targetId, entry);

      let live = true;
      return {
        update(next: KeyboardDropTargetRegistration) {
          if (!live || destroyed) return;
          if (next.targetId !== entry.registration.targetId) {
            throw new Error(
              `Keyboard drop target id "${entry.registration.targetId}" is immutable on a live handle`,
            );
          }
          const id = entry.registration.targetId;
          const wasDisabled = entry.registration.disabled;
          entry.registration = next;
          if (
            !wasDisabled &&
            next.disabled &&
            phase === "dropping" &&
            context.session?.intent?.targetId === id &&
            inputKind === "keyboard" &&
            usesLogicalKeyboard()
          ) {
            dispatch({
              type: "DROP_REJECTED",
              sessionId: context.session.sessionId,
              reason: "target-unavailable",
            });
          }
        },
        unregister() {
          if (!live) return;
          live = false;
          if (destroyed) return;
          unregisterKeyboardTarget(entry.registration.targetId);
        },
      };
    },

    getSnapshot() {
      return snapshot;
    },

    subscribe(listener: () => void): () => void {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },

    invalidateLayout() {
      layoutDirty = true;
      if (phase === "dragging" && pointerPosition) hitTest(pointerPosition.x, pointerPosition.y);
    },

    cancel() {
      stopAutoScroll();
      stopCandidate();
      if (!context.session) return;
      dispatch({ type: "CANCEL", sessionId: context.session.sessionId });
    },

    destroy() {
      if (destroyed) return;
      stopAutoScroll();
      stopCandidate();
      if (context.session) dispatch({ type: "CANCEL", sessionId: context.session.sessionId });
      const unsubscribe = crossWindowUnsubscribe;
      crossWindowUnsubscribe = null;
      unsubscribe?.();
      releaseCrossWindowSource("transport-lost");
      releaseCrossWindowTarget();
      for (const id of [...sources.keys()]) unregisterSource(id);
      for (const id of [...targets.keys()]) unregisterTarget(id);
      for (const id of [...keyboardTargets.keys()]) unregisterKeyboardTarget(id);
      unbindDocument();
      if (announceTimer !== null) clearTimeout(announceTimer);
      releasePointerHardware();
      gesture = null;
      listeners.clear();
      connectedRoot = null;
      connectedDocument = null;
      connectedWindow = null;
      destroyed = true;
    },
  };
}

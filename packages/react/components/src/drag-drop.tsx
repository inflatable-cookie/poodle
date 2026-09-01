import {
  createContext,
  useCallback,
  useContext,
  useLayoutEffect,
  useRef,
  useState,
  useSyncExternalStore,
  type CSSProperties,
  type HTMLAttributes,
  type ReactNode,
  type Ref,
} from "react";

import "@inflatable-cookie/poodle-core/styles/drag-drop.css";
import {
  createDragDropController,
  type CrossWindowDragTargetBridge,
  type DragAnnouncementEvent,
  type DragDropController,
  type DragDropSnapshot,
  type DragPreviewSnapshot,
  type DragSourceHandle,
  type DragSourceRegistration,
  type DropTargetHandle,
  type DropTargetRegistration,
  type InboundFileHostBridge,
  type KeyboardDropCommand,
  type KeyboardDropTargetHandle,
  type KeyboardDropTargetRegistration,
} from "@inflatable-cookie/poodle-core";

export type {
  CrossWindowDragCapabilities,
  CrossWindowDragCommitRequest,
  CrossWindowDragPrepareRequest,
  CrossWindowDragProjection,
  CrossWindowDragReceipt,
  CrossWindowDragSourceBridge,
  CrossWindowDragTargetBridge,
  CrossWindowDragTargetEvent,
  CrossWindowDragTransport,
  DragActivationConstraints,
  DragAnnouncementEvent,
  DragDropCommitResult,
  DragDropController,
  DragDropSnapshot,
  DragPreviewSnapshot,
  DragExportBridge,
  DragExportCapabilities,
  DragExportForm,
  DragExportPrepareRequest,
  DragExportSnapshot,
  DragExportState,
  DragExportTerminal,
  DragSourceRegistration,
  DropCommitContext,
  DropTargetRegistration,
  InboundFileBatch,
  InboundFileCapabilities,
  InboundFileConstraints,
  InboundFileEvent,
  InboundFileHostBridge,
  InboundFileOutcome,
  InboundFileReceipt,
  PreparedFileExport,
  KeyboardDropCommand,
  KeyboardDropDirection,
  KeyboardDropTargetHandle,
  KeyboardDropTargetRegistration,
  KeyboardPositionResolverInput,
} from "@inflatable-cookie/poodle-core";

interface DragDropContextValue {
  controller: DragDropController;
  snapshot: DragDropSnapshot;
}

const DragDropContext = createContext<DragDropContextValue | null>(null);

function composeRefs<T>(...refs: Array<Ref<T> | undefined>): (node: T | null) => void {
  return (node) => {
    for (const ref of refs) {
      if (typeof ref === "function") ref(node);
      else if (ref) (ref as { current: T | null }).current = node;
    }
  };
}

function composeHandler<E>(theirs?: (event: E) => void, ours?: (event: E) => void): (event: E) => void {
  return (event) => {
    theirs?.(event);
    ours?.(event);
  };
}

function presentationKey(snapshot: DragDropSnapshot): string {
  return [
    snapshot.phase,
    snapshot.sourceId ?? "",
    snapshot.targetId ?? "",
    snapshot.targetPosture ?? "",
    snapshot.announcement ?? "",
    snapshot.preview?.label ?? "",
    snapshot.preview ? "1" : "0",
  ].join("|");
}

function previewTransform(preview: DragPreviewSnapshot): string {
  return `translate3d(${preview.x}px, ${preview.y}px, 0)`;
}

function previewPoseKey(preview: DragPreviewSnapshot | null): string {
  if (!preview) return "";
  return `${preview.x}|${preview.y}|${preview.label}|${preview.sourceId}`;
}

export interface DragDropProviderProps {
  controller?: DragDropController;
  describeAnnouncement?: (event: DragAnnouncementEvent) => string | null;
  /**
   * Incoming cross-window host projection, commit, and accessible target
   * picking for this document. Ignored when an explicit `controller` is
   * supplied, because that controller already owns its own bridge.
   */
  crossWindowTargetBridge?: CrossWindowDragTargetBridge;
  /**
   * Inbound external files for this document. Ignored when an explicit
   * `controller` is supplied, because that controller already owns its own
   * bridge, and exclusive by construction: the bridge names the one transport
   * that owns inbound files here.
   */
  inboundFileBridge?: InboundFileHostBridge;
  preview?: (snapshot: DragPreviewSnapshot) => ReactNode;
  children?: ReactNode;
}

export function DragDropProvider({
  controller,
  describeAnnouncement,
  crossWindowTargetBridge,
  inboundFileBridge,
  preview,
  children,
}: DragDropProviderProps) {
  const ownedRef = useRef(controller === undefined);
  const [ctrl] = useState(() =>
    ownedRef.current
      ? createDragDropController({
          describeAnnouncement,
          crossWindowTargetBridge,
          inboundFileBridge,
        })
      : controller!,
  );
  const rootRef = useRef<HTMLDivElement>(null);
  const previewRef = useRef<HTMLDivElement>(null);
  const [snapshot, setSnapshot] = useState(() => ctrl.getSnapshot());
  const [previewPose, setPreviewPose] = useState<DragPreviewSnapshot | null>(
    () => ctrl.getSnapshot().preview,
  );
  const presentationKeyRef = useRef(presentationKey(snapshot));
  const previewPoseKeyRef = useRef(previewPoseKey(snapshot.preview));
  const connectGenerationRef = useRef(0);

  useLayoutEffect(() => {
    const generation = ++connectGenerationRef.current;
    const unsub = ctrl.subscribe(() => {
      const next = ctrl.getSnapshot();
      const node = previewRef.current;
      if (node && next.preview) {
        node.style.transform = previewTransform(next.preview);
      }
      const pose = previewPoseKey(next.preview);
      if (pose !== previewPoseKeyRef.current) {
        previewPoseKeyRef.current = pose;
        setPreviewPose(next.preview ? { ...next.preview } : null);
      }
      const key = presentationKey(next);
      if (key === presentationKeyRef.current) return;
      presentationKeyRef.current = key;
      setSnapshot(next);
    });
    const root = rootRef.current;
    if (!root) {
      unsub();
      return;
    }
    const disconnect = ctrl.connect(root);
    return () => {
      unsub();
      disconnect();
      if (!ownedRef.current) return;
      const owned = ctrl;
      queueMicrotask(() => {
        if (connectGenerationRef.current !== generation) return;
        owned.destroy();
      });
    };
  }, [ctrl]);

  const previewStyle: CSSProperties | undefined = previewPose
    ? { transform: previewTransform(previewPose) }
    : undefined;

  return (
    <DragDropContext.Provider value={{ controller: ctrl, snapshot }}>
      <div ref={rootRef} className="poodle-drag-drop-provider">
        {children}
        <div className="poodle-drag-overlay" aria-hidden="true">
          {previewPose ? (
            <div ref={previewRef} className="poodle-drag-preview" style={previewStyle}>
              {preview ? preview(previewPose) : previewPose.label}
            </div>
          ) : null}
        </div>
        <div className="poodle-drag-live-region" aria-live="polite" aria-atomic="true">
          {snapshot.announcement ?? ""}
        </div>
      </div>
    </DragDropContext.Provider>
  );
}

/**
 * The nearest drag-drop context, or `null`.
 *
 * Internal to the component package and deliberately not re-exported: a
 * component that *joins* an ambient provider when one exists needs to ask
 * without throwing, but a consumer reaching for the controller should still
 * get the loud `useDragDrop` error rather than a silent null.
 */
export function useOptionalDragDrop(): DragDropContextValue | null {
  return useContext(DragDropContext);
}

function useDragDropContext(): DragDropContextValue {
  const ctx = useContext(DragDropContext);
  if (!ctx) {
    throw new Error("useDragDrop must be used inside DragDropProvider");
  }
  return ctx;
}

export function useDragDrop(): {
  snapshot: DragDropSnapshot;
  cancel: () => void;
  requestKeyboardDrop: (command: KeyboardDropCommand) => boolean;
} {
  const ctx = useDragDropContext();
  const snapshot = useSyncExternalStore(
    (listener) => ctx.controller.subscribe(listener),
    () => ctx.controller.getSnapshot(),
    () => ctx.controller.getSnapshot(),
  );
  return {
    snapshot,
    cancel: () => ctx.controller.cancel(),
    requestKeyboardDrop: (command) => ctx.controller.requestKeyboardDrop(command),
  };
}

export type SourcePropGetter = (
  props?: HTMLAttributes<HTMLElement> & { ref?: Ref<HTMLElement> },
) => HTMLAttributes<HTMLElement> & { ref: (node: HTMLElement | null) => void };

export type TargetPropGetter = (
  props?: HTMLAttributes<HTMLElement> & { ref?: Ref<HTMLElement> },
) => HTMLAttributes<HTMLElement> & { ref: (node: HTMLElement | null) => void };

export function useDragSource(registration: DragSourceRegistration): {
  getSourceProps: SourcePropGetter;
  dragging: boolean;
  phase: DragDropSnapshot["phase"];
} {
  const ctx = useDragDropContext();
  const [node, setNode] = useState<HTMLElement | null>(null);
  const handleRef = useRef<DragSourceHandle | null>(null);
  const registrationRef = useRef(registration);
  registrationRef.current = registration;

  useLayoutEffect(() => {
    if (!node) return;
    const handle = ctx.controller.registerSource(node, registrationRef.current);
    handleRef.current = handle;
    return () => {
      handle.unregister();
      handleRef.current = null;
    };
  }, [ctx.controller, registration.sourceId, node]);

  useLayoutEffect(() => {
    handleRef.current?.update(registration);
  });

  const getSourceProps = useCallback<SourcePropGetter>((props = {}) => {
    const { ref, onKeyDown, ...rest } = props;
    return {
      ...rest,
      ref: composeRefs(ref, (el) => setNode(el)),
      onKeyDown: composeHandler(onKeyDown),
    };
  }, []);

  const active = ctx.snapshot.sourceId === registration.sourceId;
  return {
    getSourceProps,
    dragging: active && (ctx.snapshot.phase === "dragging" || ctx.snapshot.phase === "dropping"),
    phase: active ? ctx.snapshot.phase : "idle",
  };
}

export function useDropTarget(registration: DropTargetRegistration): {
  getTargetProps: TargetPropGetter;
  accepted: boolean;
  rejected: boolean;
} {
  const ctx = useDragDropContext();
  const [node, setNode] = useState<HTMLElement | null>(null);
  const handleRef = useRef<DropTargetHandle | null>(null);
  const registrationRef = useRef(registration);
  registrationRef.current = registration;

  useLayoutEffect(() => {
    if (!node) return;
    const handle = ctx.controller.registerTarget(node, registrationRef.current);
    handleRef.current = handle;
    return () => {
      handle.unregister();
      handleRef.current = null;
    };
  }, [ctx.controller, registration.targetId, node]);

  useLayoutEffect(() => {
    handleRef.current?.update(registration);
  });

  const getTargetProps = useCallback<TargetPropGetter>((props = {}) => {
    const { ref, onKeyDown, ...rest } = props;
    return {
      ...rest,
      ref: composeRefs(ref, (el) => setNode(el)),
      onKeyDown: composeHandler(onKeyDown),
    };
  }, []);

  return {
    getTargetProps,
    accepted: ctx.snapshot.targetId === registration.targetId && ctx.snapshot.targetPosture === "accepted",
    rejected: ctx.snapshot.targetId === registration.targetId && ctx.snapshot.targetPosture === "rejected",
  };
}

/**
 * Register a drop target against an explicit controller.
 *
 * For the one case the context hooks cannot serve: a component that *renders*
 * the provider is above it in the tree, so it cannot read its own context. A
 * DockRegion with no ambient provider is exactly that — it owns the controller
 * and registers its own region against it directly.
 */
export function useControllerDropTarget(
  controller: DragDropController,
  registration: DropTargetRegistration,
): { getTargetProps: TargetPropGetter; accepted: boolean; rejected: boolean } {
  const [node, setNode] = useState<HTMLElement | null>(null);
  const [snapshot, setSnapshot] = useState(() => controller.getSnapshot());
  const handleRef = useRef<DropTargetHandle | null>(null);
  const registrationRef = useRef(registration);
  registrationRef.current = registration;

  useLayoutEffect(() => {
    setSnapshot(controller.getSnapshot());
    return controller.subscribe(() => setSnapshot(controller.getSnapshot()));
  }, [controller]);

  useLayoutEffect(() => {
    if (!node) return;
    const handle = controller.registerTarget(node, registrationRef.current);
    handleRef.current = handle;
    return () => {
      handle.unregister();
      handleRef.current = null;
    };
  }, [controller, registration.targetId, node]);

  useLayoutEffect(() => {
    handleRef.current?.update(registration);
  });

  const getTargetProps = useCallback<TargetPropGetter>((props = {}) => {
    const { ref, onKeyDown, ...rest } = props;
    return {
      ...rest,
      ref: composeRefs(ref, (el) => setNode(el)),
      onKeyDown: composeHandler(onKeyDown),
    };
  }, []);

  return {
    getTargetProps,
    accepted: snapshot.targetId === registration.targetId && snapshot.targetPosture === "accepted",
    rejected: snapshot.targetId === registration.targetId && snapshot.targetPosture === "rejected",
  };
}

export function useKeyboardDropTarget(registration: KeyboardDropTargetRegistration): {
  accepted: boolean;
  rejected: boolean;
} {
  const ctx = useDragDropContext();
  const handleRef = useRef<KeyboardDropTargetHandle | null>(null);
  const registrationRef = useRef(registration);
  registrationRef.current = registration;

  useLayoutEffect(() => {
    const handle = ctx.controller.registerKeyboardTarget(registrationRef.current);
    handleRef.current = handle;
    return () => {
      handle.unregister();
      handleRef.current = null;
    };
  }, [ctx.controller, registration.targetId]);

  useLayoutEffect(() => {
    handleRef.current?.update(registration);
  });

  return {
    accepted: ctx.snapshot.targetId === registration.targetId && ctx.snapshot.targetPosture === "accepted",
    rejected: ctx.snapshot.targetId === registration.targetId && ctx.snapshot.targetPosture === "rejected",
  };
}

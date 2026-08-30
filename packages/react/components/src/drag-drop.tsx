import {
  createContext,
  useCallback,
  useContext,
  useLayoutEffect,
  useRef,
  useState,
  type CSSProperties,
  type HTMLAttributes,
  type ReactNode,
  type Ref,
} from "react";

import "@inflatable-cookie/poodle-core/styles/drag-drop.css";
import {
  createDragDropController,
  type DragAnnouncementEvent,
  type DragDropController,
  type DragDropSnapshot,
  type DragPreviewSnapshot,
  type DragSourceHandle,
  type DragSourceRegistration,
  type DropTargetHandle,
  type DropTargetRegistration,
} from "@inflatable-cookie/poodle-core";

export type {
  DragActivationConstraints,
  DragAnnouncementEvent,
  DragDropCommitResult,
  DragDropController,
  DragDropSnapshot,
  DragPreviewSnapshot,
  DragSourceRegistration,
  DropTargetRegistration,
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

export interface DragDropProviderProps {
  controller?: DragDropController;
  describeAnnouncement?: (event: DragAnnouncementEvent) => string | null;
  preview?: (snapshot: DragPreviewSnapshot) => ReactNode;
  children?: ReactNode;
}

export function DragDropProvider({
  controller,
  describeAnnouncement,
  preview,
  children,
}: DragDropProviderProps) {
  const ownedRef = useRef(controller === undefined);
  const [ctrl] = useState(() =>
    ownedRef.current ? createDragDropController({ describeAnnouncement }) : controller!,
  );
  const rootRef = useRef<HTMLDivElement>(null);
  const [snapshot, setSnapshot] = useState(() => ctrl.getSnapshot());
  const connectGenerationRef = useRef(0);

  useLayoutEffect(() => {
    const generation = ++connectGenerationRef.current;
    const unsub = ctrl.subscribe(() => setSnapshot(ctrl.getSnapshot()));
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

  const previewStyle: CSSProperties | undefined = snapshot.preview
    ? { left: snapshot.preview.x, top: snapshot.preview.y }
    : undefined;

  return (
    <DragDropContext.Provider value={{ controller: ctrl, snapshot }}>
      <div ref={rootRef} className="poodle-drag-drop-provider">
        {children}
        <div className="poodle-drag-overlay" aria-hidden="true">
          {snapshot.preview ? (
            <div className="poodle-drag-preview" style={previewStyle}>
              {preview ? preview(snapshot.preview) : snapshot.preview.label}
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
} {
  const ctx = useDragDropContext();
  return {
    snapshot: ctx.snapshot,
    cancel: () => ctx.controller.cancel(),
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

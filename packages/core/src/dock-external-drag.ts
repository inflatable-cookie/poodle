/**
 * DockRegion external drag — the host-owned drag session, framework-free.
 *
 * Contract: `docs/contracts/components/dock-region.md` §5.
 *
 * A panel can be dragged out of the dock into something Poodle knows nothing
 * about (another window, another app). The host writes its own payload, so the
 * component's job is only to run the session in the right order and to
 * guarantee that a preparation which never starts is always cancelled — a host
 * that allocates on `prepare` must not leak when the pointer is released
 * without a drag.
 *
 * Lives here rather than in a framework package because the ordering is subtle
 * and identical everywhere: two copies would be two things to keep in step, and
 * the second one would drift.
 */

export type DockEdgeLike = "left" | "right" | "top" | "bottom";

export type DockPanelLike = {
  value: string;
  [key: string]: unknown;
};

export type DockExternalDragCancelReason =
  | "superseded"
  | "pointer-released"
  | "pointer-cancelled"
  | "not-ready"
  | "unmounted";

export type DockExternalDragPrepareContext<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  panel: Panel;
  sourceEdge: Edge;
  event: PointerEvent;
  signal: AbortSignal;
};

export type DockExternalDragStartContext<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  panel: Panel;
  sourceEdge: Edge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDragEndContext<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  panel: Panel;
  sourceEdge: Edge;
  event: DragEvent;
  dropEffect: DataTransfer["dropEffect"];
};

export type DockExternalDragCancelContext<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  panel: Panel;
  sourceEdge: Edge;
  reason: DockExternalDragCancelReason;
};

export type DockExternalDragPreparation<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  start: (context: DockExternalDragStartContext<Panel, Edge>) => void;
  end?: (context: DockExternalDragEndContext<Panel, Edge>) => void | Promise<void>;
  cancel?: (context: DockExternalDragCancelContext<Panel, Edge>) => void | Promise<void>;
};

export type DockExternalDragSource<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  prepare: (
    context: DockExternalDragPrepareContext<Panel, Edge>,
  ) =>
    | DockExternalDragPreparation<Panel, Edge>
    | null
    | Promise<DockExternalDragPreparation<Panel, Edge> | null>;
  onPrepareError?: (
    error: unknown,
    context: DockExternalDragPrepareContext<Panel, Edge>,
  ) => void;
};

export type DockExternalDropEligibilityContext<Edge extends string = DockEdgeLike> = {
  phase: "over" | "drop";
  targetEdge: Edge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDropContext<Edge extends string = DockEdgeLike> = {
  targetEdge: Edge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDropTarget<Edge extends string = DockEdgeLike> = {
  canDrop: (context: DockExternalDropEligibilityContext<Edge>) => boolean;
  drop: (context: DockExternalDropContext<Edge>) => void | Promise<void>;
};

type ExternalPreparationState<Panel extends DockPanelLike, Edge extends string> = {
  panel: Panel;
  sourceEdge: Edge;
  source: DockExternalDragSource<Panel, Edge>;
  context: DockExternalDragPrepareContext<Panel, Edge>;
  controller: AbortController;
  status: "pending" | "ready" | "started";
  preparation: DockExternalDragPreparation<Panel, Edge> | null;
  cancelReason: DockExternalDragCancelReason | null;
  removePointerListeners: () => void;
};

export type DockExternalDragAccessors<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
> = {
  source: () => DockExternalDragSource<Panel, Edge> | null;
  panel: (panelId: string) => Panel | undefined;
  edge: () => Edge;
};

export type DockExternalDragController = {
  prepare: (panelId: string, event: PointerEvent) => void;
  start: (panelId: string, event: DragEvent) => boolean;
  end: (panelId: string, event: DragEvent) => void;
  cancel: (reason: DockExternalDragCancelReason) => void;
  activePanelId: () => string | null;
};

function isPromiseLike<T>(value: T | Promise<T>): value is Promise<T> {
  return typeof (value as Promise<T> | null)?.then === "function";
}

export function createDockExternalDragController<
  Panel extends DockPanelLike = DockPanelLike,
  Edge extends string = DockEdgeLike,
>(
  accessors: DockExternalDragAccessors<Panel, Edge>,
): DockExternalDragController {
  let state: ExternalPreparationState<Panel, Edge> | null = null;

  function cancelResult(
    current: ExternalPreparationState<Panel, Edge>,
    preparation: DockExternalDragPreparation<Panel, Edge>,
    reason: DockExternalDragCancelReason,
  ): void {
    void preparation.cancel?.({
      panel: current.panel,
      sourceEdge: current.sourceEdge,
      reason,
    });
  }

  function cancel(reason: DockExternalDragCancelReason): void {
    const current = state;
    if (!current || current.status === "started") return;

    state = null;
    current.cancelReason = reason;
    current.removePointerListeners();
    if (!current.controller.signal.aborted) {
      current.controller.abort(reason);
    }
    if (current.preparation) {
      cancelResult(current, current.preparation, reason);
    }
  }

  function settle(
    current: ExternalPreparationState<Panel, Edge>,
    preparation: DockExternalDragPreparation<Panel, Edge> | null,
  ): void {
    if (!preparation) {
      if (state === current) {
        state = null;
        current.removePointerListeners();
      }
      return;
    }

    if (state !== current || current.controller.signal.aborted) {
      cancelResult(current, preparation, current.cancelReason ?? "superseded");
      return;
    }

    current.preparation = preparation;
    current.status = "ready";
  }

  function fail(
    current: ExternalPreparationState<Panel, Edge>,
    error: unknown,
  ): void {
    if (state !== current || current.controller.signal.aborted) return;

    state = null;
    current.removePointerListeners();
    current.source.onPrepareError?.(error, current.context);
  }

  function prepare(panelId: string, event: PointerEvent): void {
    const source = accessors.source();
    const panel = accessors.panel(panelId);
    if (!source || !panel || event.button !== 0) return;
    if (state?.status === "started") return;

    cancel("superseded");

    const controller = new AbortController();
    const context: DockExternalDragPrepareContext<Panel, Edge> = {
      panel,
      sourceEdge: accessors.edge(),
      event,
      signal: controller.signal,
    };
    const current: ExternalPreparationState<Panel, Edge> = {
      panel,
      sourceEdge: context.sourceEdge,
      source,
      context,
      controller,
      status: "pending",
      preparation: null,
      cancelReason: null,
      removePointerListeners: () => {},
    };
    state = current;

    const ownerWindow = (event.currentTarget as Node | null)?.ownerDocument
      ?.defaultView;
    if (ownerWindow) {
      const handlePointerUp = () => {
        if (state === current && current.status !== "started") {
          cancel("pointer-released");
        }
      };
      const handlePointerCancel = () => {
        if (state === current && current.status !== "started") {
          cancel("pointer-cancelled");
        }
      };
      ownerWindow.addEventListener("pointerup", handlePointerUp, true);
      ownerWindow.addEventListener("pointercancel", handlePointerCancel, true);
      current.removePointerListeners = () => {
        ownerWindow.removeEventListener("pointerup", handlePointerUp, true);
        ownerWindow.removeEventListener(
          "pointercancel",
          handlePointerCancel,
          true,
        );
      };
    }

    let result:
      | DockExternalDragPreparation<Panel, Edge>
      | null
      | Promise<DockExternalDragPreparation<Panel, Edge> | null>;
    try {
      result = source.prepare(context);
    } catch (error) {
      fail(current, error);
      return;
    }

    if (isPromiseLike(result)) {
      void result.then(
        (preparation) => settle(current, preparation),
        (error) => fail(current, error),
      );
      return;
    }

    settle(current, result);
  }

  function start(panelId: string, event: DragEvent): boolean {
    const current = state;
    if (
      !current ||
      current.panel.value !== panelId ||
      current.status !== "ready" ||
      !current.preparation ||
      !event.dataTransfer
    ) {
      if (current && current.status !== "started") {
        cancel(current.panel.value === panelId ? "not-ready" : "superseded");
      }
      return false;
    }

    current.status = "started";
    current.removePointerListeners();
    current.preparation.start({
      panel: current.panel,
      sourceEdge: current.sourceEdge,
      event,
      dataTransfer: event.dataTransfer,
    });
    return true;
  }

  function end(panelId: string, event: DragEvent): void {
    const current = state;
    if (
      !current ||
      current.panel.value !== panelId ||
      current.status !== "started" ||
      !current.preparation
    ) {
      return;
    }

    state = null;
    current.removePointerListeners();
    void current.preparation.end?.({
      panel: current.panel,
      sourceEdge: current.sourceEdge,
      event,
      dropEffect: event.dataTransfer?.dropEffect ?? "none",
    });
  }

  return {
    prepare,
    start,
    end,
    cancel,
    activePanelId: () => state?.panel.value ?? null,
  };
}

import type {
  DockEdge,
  DockExternalDragCancelReason,
  DockExternalDragPreparation,
  DockExternalDragPrepareContext,
  DockExternalDragSource,
  PanelTabItem,
} from "./types";

type ExternalPreparationState = {
  panel: PanelTabItem;
  sourceEdge: DockEdge;
  source: DockExternalDragSource;
  context: DockExternalDragPrepareContext;
  controller: AbortController;
  status: "pending" | "ready" | "started";
  preparation: DockExternalDragPreparation | null;
  cancelReason: DockExternalDragCancelReason | null;
  removePointerListeners: () => void;
};

type DockExternalDragAccessors = {
  source: () => DockExternalDragSource | null;
  panel: (panelId: string) => PanelTabItem | undefined;
  edge: () => DockEdge;
};

export type DockExternalDragController = {
  prepare: (panelId: string, event: PointerEvent) => void;
  start: (panelId: string, event: DragEvent) => boolean;
  end: (panelId: string, event: DragEvent) => void;
  cancel: (reason: DockExternalDragCancelReason) => void;
  activePanelId: () => string | null;
};

function isPromiseLike<T>(
  value: T | Promise<T>,
): value is Promise<T> {
  return typeof (value as Promise<T> | null)?.then === "function";
}

export function createDockExternalDragController(
  accessors: DockExternalDragAccessors,
): DockExternalDragController {
  let state: ExternalPreparationState | null = null;

  function cancelResult(
    current: ExternalPreparationState,
    preparation: DockExternalDragPreparation,
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
    current: ExternalPreparationState,
    preparation: DockExternalDragPreparation | null,
  ): void {
    if (!preparation) {
      if (state === current) {
        state = null;
        current.removePointerListeners();
      }
      return;
    }

    if (state !== current || current.controller.signal.aborted) {
      cancelResult(
        current,
        preparation,
        current.cancelReason ?? "superseded",
      );
      return;
    }

    current.preparation = preparation;
    current.status = "ready";
  }

  function fail(
    current: ExternalPreparationState,
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
    const context: DockExternalDragPrepareContext = {
      panel,
      sourceEdge: accessors.edge(),
      event,
      signal: controller.signal,
    };
    const current: ExternalPreparationState = {
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
      | DockExternalDragPreparation
      | null
      | Promise<DockExternalDragPreparation | null>;
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
        cancel(
          current.panel.value === panelId ? "not-ready" : "superseded",
        );
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

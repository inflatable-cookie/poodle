import { describe, expect, test } from "bun:test";

import {
  createDockExternalDragController,
  type DockExternalDragCancelReason,
  type DockExternalDragPreparation,
} from "../src/dock-external-drag.ts";

// The contract this suite defends (dock-region.md §5): a host may allocate on
// `prepare`, so every preparation that does not go on to start must be
// cancelled with an honest reason. Nothing else in the repo checked that, and
// both web targets now run this same controller.

const panel = { value: "inspector", label: "Inspector" };

type Recorder = {
  events: string[];
  preparation: DockExternalDragPreparation;
};

function recorder(): Recorder {
  const events: string[] = [];
  return {
    events,
    preparation: {
      start: () => events.push("start"),
      end: (ctx) => events.push(`end:${ctx.dropEffect}`),
      cancel: (ctx) => events.push(`cancel:${ctx.reason}`),
    },
  };
}

function harness(
  prepare: (signal: AbortSignal) => DockExternalDragPreparation | null | Promise<DockExternalDragPreparation | null>,
  onPrepareError?: (error: unknown) => void,
) {
  return createDockExternalDragController<typeof panel, "left">({
    source: () => ({
      prepare: (context) => prepare(context.signal),
      onPrepareError: onPrepareError ? (error) => onPrepareError(error) : undefined,
    }),
    panel: (id) => (id === panel.value ? panel : undefined),
    edge: () => "left",
  });
}

/**
 * A stand-in for the window the controller listens on.
 *
 * This package is deliberately DOM-free (see the `dom/` machines, tested the
 * same way), so the events are duck-typed to exactly what the controller
 * reads: `button`, and `currentTarget.ownerDocument.defaultView` to find
 * somewhere to hang the pointerup listener.
 */
function fakeWindow() {
  const listeners = new Map<string, Set<() => void>>();
  return {
    addEventListener: (type: string, fn: () => void) => {
      if (!listeners.has(type)) listeners.set(type, new Set());
      listeners.get(type)!.add(fn);
    },
    removeEventListener: (type: string, fn: () => void) => {
      listeners.get(type)?.delete(fn);
    },
    emit: (type: string) => {
      for (const fn of [...(listeners.get(type) ?? [])]) fn();
    },
    listenerCount: () =>
      [...listeners.values()].reduce((total, set) => total + set.size, 0),
  };
}

const win = fakeWindow();

function pointerDown(button = 0): PointerEvent {
  return {
    button,
    currentTarget: { ownerDocument: { defaultView: win } },
  } as unknown as PointerEvent;
}

function dragEvent(dropEffect: DataTransfer["dropEffect"] = "move"): DragEvent {
  return {
    dataTransfer: { dropEffect, setData: () => {}, getData: () => "", types: [] },
  } as unknown as DragEvent;
}

describe("createDockExternalDragController", () => {
  test("a synchronous preparation starts, then ends with the drop effect", () => {
    const rec = recorder();
    const drag = harness(() => rec.preparation);

    drag.prepare(panel.value, pointerDown());
    expect(drag.start(panel.value, dragEvent())).toBe(true);
    expect(drag.activePanelId()).toBe(panel.value);

    drag.end(panel.value, dragEvent("copy"));
    expect(rec.events).toEqual(["start", "end:copy"]);
    expect(drag.activePanelId()).toBeNull();
  });

  test("releasing the pointer without a drag cancels the preparation", () => {
    const rec = recorder();
    const drag = harness(() => rec.preparation);

    drag.prepare(panel.value, pointerDown());
    win.emit("pointerup");

    expect(rec.events).toEqual(["cancel:pointer-released"]);
    expect(drag.activePanelId()).toBeNull();
  });

  test("a second prepare supersedes the first", () => {
    const first = recorder();
    const second = recorder();
    let call = 0;
    const drag = harness(() => (call++ === 0 ? first.preparation : second.preparation));

    drag.prepare(panel.value, pointerDown());
    drag.prepare(panel.value, pointerDown());

    expect(first.events).toEqual(["cancel:superseded"]);
    expect(second.events).toEqual([]);
  });

  test("starting before the preparation is ready cancels as not-ready", async () => {
    const rec = recorder();
    let release: (value: DockExternalDragPreparation) => void = () => {};
    const pending = new Promise<DockExternalDragPreparation>((resolve) => {
      release = resolve;
    });
    const drag = harness(() => pending);

    drag.prepare(panel.value, pointerDown());
    // dragstart is synchronous; a host that resolves late does not get to
    // write the payload, and must be told so rather than left holding it.
    expect(drag.start(panel.value, dragEvent())).toBe(false);

    release(rec.preparation);
    await pending;

    expect(rec.events).toEqual(["cancel:not-ready"]);
  });

  test("a preparation resolving after unmount is cancelled, not started", async () => {
    const rec = recorder();
    let release: (value: DockExternalDragPreparation) => void = () => {};
    const pending = new Promise<DockExternalDragPreparation>((resolve) => {
      release = resolve;
    });
    const drag = harness(() => pending);

    drag.prepare(panel.value, pointerDown());
    drag.cancel("unmounted");
    release(rec.preparation);
    await pending;

    expect(rec.events).toEqual(["cancel:unmounted"]);
  });

  test("prepare aborts its signal so the host can drop in-flight work", () => {
    const reasons: DockExternalDragCancelReason[] = [];
    let captured: AbortSignal | null = null;
    // A pending promise, so the session is still live when cancel lands —
    // returning `null` settles it immediately and there would be nothing left
    // to abort.
    const drag = harness((signal) => {
      captured = signal;
      signal.addEventListener("abort", () => reasons.push(signal.reason));
      return new Promise<DockExternalDragPreparation>(() => {});
    });

    drag.prepare(panel.value, pointerDown());
    expect(captured).not.toBeNull();
    drag.cancel("unmounted");
    expect(reasons).toEqual(["unmounted"]);
  });

  test("a throwing prepare reaches onPrepareError and leaves no session", () => {
    const errors: unknown[] = [];
    const drag = harness(
      () => {
        throw new Error("no payload");
      },
      (error) => errors.push(error),
    );

    drag.prepare(panel.value, pointerDown());
    expect(errors).toHaveLength(1);
    expect(drag.activePanelId()).toBeNull();
  });

  test("a non-primary button never opens a session", () => {
    const rec = recorder();
    const drag = harness(() => rec.preparation);

    drag.prepare(panel.value, pointerDown(2));
    expect(drag.activePanelId()).toBeNull();
    expect(rec.events).toEqual([]);
  });

  test("declining with null leaves nothing to cancel", () => {
    const drag = harness(() => null);

    drag.prepare(panel.value, pointerDown());
    expect(drag.activePanelId()).toBeNull();
    expect(drag.start(panel.value, dragEvent())).toBe(false);
  });
});

import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import DockRegion from "../src/DockRegion.svelte";
import type {
  DockExternalDragPreparation,
  DockExternalDragSource,
  DockExternalDropTarget,
  PanelTabItem,
} from "../src/types.ts";

const items: PanelTabItem[] = [
  { value: "explorer", label: "Explorer" },
  { value: "inspector", label: "Inspector" },
];

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((next) => {
    resolve = next;
  });
  return { promise, resolve };
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("DockRegion external drag source", () => {
  it("writes a ready external payload at dragstart and ends exactly once", async () => {
    const cancel = vi.fn();
    const end = vi.fn();
    const start = vi.fn(({ dataTransfer }) => {
      dataTransfer.setData("application/x-host-panel", "explorer");
    });
    const externalDragSource: DockExternalDragSource = {
      prepare: vi.fn(() => ({ start, end, cancel })),
    };
    const { getByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        externalDragSource,
      },
    });
    const tab = getByRole("tab", { name: "Explorer" });
    const dragItem = tab.parentElement!;
    const dataTransfer = new DataTransfer();

    await fireEvent.pointerDown(tab, { button: 0 });
    await fireEvent.dragStart(dragItem, { dataTransfer });

    expect(start).toHaveBeenCalledOnce();
    expect(dataTransfer.getData("application/x-host-panel")).toBe("explorer");
    expect(dataTransfer.types).not.toContain(
      "application/x-poodle-panel-drag",
    );

    await fireEvent.dragEnd(dragItem, { dataTransfer });
    await fireEvent.dragEnd(dragItem, { dataTransfer });

    expect(end).toHaveBeenCalledOnce();
    expect(cancel).not.toHaveBeenCalled();
  });

  it("does not write an external or panel payload when preparation is pending", async () => {
    const pending = deferred<DockExternalDragPreparation | null>();
    const cancel = vi.fn();
    let signal: AbortSignal | undefined;
    const externalDragSource: DockExternalDragSource = {
      prepare: vi.fn((context) => {
        signal = context.signal;
        return pending.promise;
      }),
    };
    const { getByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        externalDragSource,
      },
    });
    const tab = getByRole("tab", { name: "Explorer" });
    const dataTransfer = new DataTransfer();

    await fireEvent.pointerDown(tab, { button: 0 });
    await fireEvent.dragStart(tab.parentElement!, { dataTransfer });

    expect(signal?.aborted).toBe(true);
    expect(signal?.reason).toBe("not-ready");
    expect(dataTransfer.types).not.toContain("application/x-host-panel");
    expect(dataTransfer.types).not.toContain(
      "application/x-poodle-panel-drag",
    );

    pending.resolve({
      start: ({ dataTransfer: transfer }) => {
        transfer.setData("application/x-host-panel", "too-late");
      },
      cancel,
    });
    await Promise.resolve();
    await Promise.resolve();

    expect(cancel).toHaveBeenCalledOnce();
    expect(cancel).toHaveBeenCalledWith(
      expect.objectContaining({ reason: "not-ready" }),
    );
  });

  it("aborts and cancels a ready preparation when another panel supersedes it", async () => {
    const firstCancel = vi.fn();
    const signals: AbortSignal[] = [];
    const externalDragSource: DockExternalDragSource = {
      prepare: vi.fn((context) => {
        signals.push(context.signal);
        return {
          start: vi.fn(),
          cancel:
            context.panel.value === "explorer" ? firstCancel : vi.fn(),
        };
      }),
    };
    const { getByRole } = render(DockRegion, {
      props: { items, externalDragSource },
    });

    await fireEvent.pointerDown(
      getByRole("tab", { name: "Explorer" }),
      { button: 0 },
    );
    await fireEvent.pointerDown(
      getByRole("tab", { name: "Inspector" }),
      { button: 0 },
    );

    expect(signals[0].aborted).toBe(true);
    expect(signals[0].reason).toBe("superseded");
    expect(firstCancel).toHaveBeenCalledOnce();
    expect(firstCancel).toHaveBeenCalledWith(
      expect.objectContaining({
        panel: expect.objectContaining({ value: "explorer" }),
        reason: "superseded",
      }),
    );
  });

  it.each([
    ["pointerup", "pointer-released"],
    ["pointercancel", "pointer-cancelled"],
  ] as const)(
    "cancels a ready preparation on %s before dragstart",
    async (eventName, reason) => {
      const cancel = vi.fn();
      let signal: AbortSignal | undefined;
      const externalDragSource: DockExternalDragSource = {
        prepare: vi.fn((context) => {
          signal = context.signal;
          return { start: vi.fn(), cancel };
        }),
      };
      const { getByRole } = render(DockRegion, {
        props: { items, externalDragSource },
      });

      await fireEvent.pointerDown(
        getByRole("tab", { name: "Explorer" }),
        { button: 0 },
      );
      window.dispatchEvent(new PointerEvent(eventName));

      expect(signal?.aborted).toBe(true);
      expect(signal?.reason).toBe(reason);
      expect(cancel).toHaveBeenCalledOnce();
      expect(cancel).toHaveBeenCalledWith(
        expect.objectContaining({ reason }),
      );
    },
  );

  it("keeps same-region reorder Poodle-owned with an external source", async () => {
    const onReorder = vi.fn();
    const start = vi.fn(({ dataTransfer }) => {
      dataTransfer.setData("application/x-host-panel", "explorer");
    });
    const externalDragSource: DockExternalDragSource = {
      prepare: () => ({ start }),
    };
    const { getAllByRole } = render(DockRegion, {
      props: {
        items,
        value: "explorer",
        onReorder,
        externalDragSource,
      },
    });
    const [firstTab, secondTab] = getAllByRole("tab");
    const firstItem = firstTab.parentElement!;
    const secondItem = secondTab.parentElement!;
    const dataTransfer = new DataTransfer();

    await fireEvent.pointerDown(firstTab, { button: 0 });
    await fireEvent.dragStart(firstItem, { dataTransfer });
    await fireEvent.dragOver(secondItem, { dataTransfer });
    await fireEvent.drop(secondItem, { dataTransfer });

    expect(start).toHaveBeenCalledOnce();
    expect(onReorder).toHaveBeenCalledWith(["inspector", "explorer"]);
    expect(dataTransfer.types).toContain(
      "application/x-poodle-tab-reorder",
    );
    expect(dataTransfer.types).not.toContain(
      "application/x-poodle-panel-drag",
    );
  });
});

describe("DockRegion external drop target", () => {
  it("uses eligibility for the affordance and accepted drop", async () => {
    const drop = vi.fn();
    const canDrop = vi.fn(() => true);
    const externalDropTarget: DockExternalDropTarget = { canDrop, drop };
    const { container, getByRole } = render(DockRegion, {
      props: {
        items,
        ariaLabel: "Workspace panels",
        externalDropTarget,
      },
    });
    const region = getByRole("region", { name: "Workspace panels" });
    const dataTransfer = new DataTransfer();
    dataTransfer.setData("application/x-host-panel", "lease-1");

    const dragOverAccepted = await fireEvent.dragOver(region, {
      dataTransfer,
    });

    expect(dragOverAccepted).toBe(false);
    expect(canDrop).toHaveBeenCalledWith(
      expect.objectContaining({ phase: "over", targetEdge: "left" }),
    );
    expect(
      container.querySelector(".poodle-dock-region__drop-zone"),
    ).not.toBeNull();

    const dropAccepted = await fireEvent.drop(region, { dataTransfer });

    expect(dropAccepted).toBe(false);
    expect(drop).toHaveBeenCalledOnce();
    expect(drop).toHaveBeenCalledWith(
      expect.objectContaining({
        targetEdge: "left",
        dataTransfer,
      }),
    );
  });

  it("does not advertise an affordance or accept an ineligible drop", async () => {
    const drop = vi.fn();
    const externalDropTarget: DockExternalDropTarget = {
      canDrop: () => false,
      drop,
    };
    const { container, getByRole } = render(DockRegion, {
      props: { items, externalDropTarget },
    });
    const region = getByRole("region");
    const dataTransfer = new DataTransfer();
    dataTransfer.setData("application/x-host-panel", "lease-1");

    const dragOverAccepted = await fireEvent.dragOver(region, {
      dataTransfer,
    });
    const dropAccepted = await fireEvent.drop(region, { dataTransfer });

    expect(dragOverAccepted).toBe(true);
    expect(dropAccepted).toBe(true);
    expect(
      container.querySelector(".poodle-dock-region__drop-zone"),
    ).toBeNull();
    expect(drop).not.toHaveBeenCalled();
  });

  it("preserves region and tab accessible names with the extension enabled", () => {
    const externalDropTarget: DockExternalDropTarget = {
      canDrop: () => false,
      drop: vi.fn(),
    };
    const { getByRole } = render(DockRegion, {
      props: {
        items,
        ariaLabel: "Workspace panels",
        externalDropTarget,
      },
    });

    expect(
      getByRole("region", { name: "Workspace panels" }),
    ).toBeTruthy();
    expect(getByRole("tab", { name: "Explorer" })).toBeTruthy();
    expect(getByRole("tab", { name: "Inspector" })).toBeTruthy();
  });
});

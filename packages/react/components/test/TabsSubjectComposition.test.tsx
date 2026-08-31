/**
 * Tabs drag subject composition (g16.026).
 *
 * `dragSubjectKind` lets an owning composite put a strip in a shared semantic
 * family without taking over its reorder. The risk that creates is capture: a
 * second strip in the same family could swallow a drop meant for the composite
 * around it. These cases pin the three rules that stop it — instance-scoped
 * default families, registration ids scoped independently of subject ids, and
 * eligibility-time rejection of a subject the strip does not own.
 */

import { act, render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ReactNode } from "react";

import type { DragDropCommitResult, DropIntent } from "@inflatable-cookie/poodle-core";

import { Tabs } from "../src/Tabs";
import { DragDropProvider, useDropTarget } from "../src/drag-drop";
import type { TabItem } from "../src/types";

const left: TabItem[] = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

/** A different item set, so a subject from the left strip is genuinely foreign. */
const right: TabItem[] = [
  { value: "gamma", label: "Gamma" },
  { value: "delta", label: "Delta" },
];

/** Deliberately repeats the left strip's values: registration-id collision bait. */
const repeated: TabItem[] = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

/** A minimal composite target, standing in for what DockRegion is. */
function CompositeTarget({
  kind,
  onDropped,
  children,
}: {
  kind: string;
  onDropped: (intent: DropIntent) => void;
  children: ReactNode;
}) {
  const { getTargetProps } = useDropTarget({
    targetId: "composite",
    acceptedKinds: [kind],
    label: "Composite",
    priority: -1,
    resolvePosition: () => "inside",
    canDrop: (intent) => ({ accepted: true, intent }),
    onDrop: (intent): DragDropCommitResult => {
      onDropped(intent);
      return { status: "committed" };
    },
  });

  return <div {...getTargetProps({ "data-testid": "composite" } as never)}>{children}</div>;
}

interface HarnessProps {
  kind?: string | null;
  rightItems?: TabItem[];
  onCompositeDrop?: (intent: DropIntent) => void;
  onLeftReorder?: (order: string[]) => void;
  onRightReorder?: (order: string[]) => void;
}

function Harness({
  kind = null,
  rightItems = right,
  onCompositeDrop = () => {},
  onLeftReorder,
  onRightReorder,
}: HarnessProps) {
  const rightStrip = (
    <div data-testid="right-host">
      <Tabs
        items={rightItems}
        value={rightItems[0]?.value ?? null}
        reorderable
        dragSubjectKind={kind}
        ariaLabel="Right strip"
        onReorder={onRightReorder}
      />
    </div>
  );

  return (
    <DragDropProvider>
      <div data-testid="left-host">
        <Tabs
          items={left}
          value={left[0]?.value ?? null}
          reorderable
          dragSubjectKind={kind}
          ariaLabel="Left strip"
          onReorder={onLeftReorder}
        />
      </div>
      {kind === null ? (
        rightStrip
      ) : (
        <CompositeTarget kind={kind} onDropped={onCompositeDrop}>
          {rightStrip}
        </CompositeTarget>
      )}
    </DragDropProvider>
  );
}

function rect(element: HTMLElement, x: number, y: number, width: number, height: number): void {
  const value = {
    x,
    y,
    width,
    height,
    top: y,
    left: x,
    right: x + width,
    bottom: y + height,
    toJSON() {
      return this;
    },
  } as DOMRect;
  element.getBoundingClientRect = () => value;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  element.hasPointerCapture = () => false;
}

/**
 * Left strip at x 0-200, right host (and its composite) at x 400-800.
 *
 * happy-dom measures everything as an empty box at the origin, so without real
 * places every target contains every point and "dropped on the right" would
 * mean nothing.
 */
function layout(container: HTMLElement): void {
  const composite = container.querySelector<HTMLElement>('[data-testid="composite"]');
  if (composite) rect(composite, 400, 0, 400, 120);

  (["left-host", "right-host"] as const).forEach((testid, hostIndex) => {
    const host = container.querySelector<HTMLElement>(`[data-testid="${testid}"]`);
    if (!host) return;
    const originX = hostIndex * 400;
    rect(host, originX, 0, 200, 120);
    [...host.querySelectorAll<HTMLElement>(".poodle-tabs__item")].forEach((item, index) => {
      rect(item, originX + index * 80, 20, 80, 40);
      const tab = item.querySelector<HTMLElement>(".poodle-tabs__tab");
      if (tab) rect(tab, originX + index * 80, 20, 80, 40);
    });
  });
}

function pointer(type: string, x: number): PointerEvent {
  return new PointerEvent(type, {
    bubbles: true,
    cancelable: true,
    pointerId: 1,
    pointerType: "mouse",
    button: 0,
    buttons: type === "pointerup" ? 0 : 1,
    isPrimary: true,
    clientX: x,
    clientY: 40,
  });
}

function dragTo(source: HTMLElement, x: number): void {
  act(() => {
    source.dispatchEvent(pointer("pointerdown", 20));
  });
  act(() => {
    document.dispatchEvent(pointer("pointermove", 60));
  });
  act(() => {
    document.dispatchEvent(pointer("pointermove", x));
  });
  act(() => {
    document.dispatchEvent(pointer("pointerup", x));
  });
}

function leftTabs(container: HTMLElement): HTMLElement[] {
  const host = container.querySelector<HTMLElement>('[data-testid="left-host"]')!;
  return [...host.querySelectorAll<HTMLElement>(".poodle-tabs__tab")];
}

describe("Tabs drag subject composition", () => {
  beforeEach(() => {
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => {
      callback(0);
      return 1;
    });
    vi.stubGlobal("cancelAnimationFrame", vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("keeps ordinary sibling strips mutually ineligible under one provider", () => {
    const onLeftReorder = vi.fn();
    const onRightReorder = vi.fn();
    const { container } = render(
      <Harness onLeftReorder={onLeftReorder} onRightReorder={onRightReorder} />,
    );
    layout(container);

    // Two strips, one controller. Without an explicit kind each strip has its
    // own family, so nothing in the right strip is a candidate.
    dragTo(leftTabs(container)[0], 520);

    expect(onRightReorder).not.toHaveBeenCalled();
    expect(onLeftReorder).not.toHaveBeenCalled();
    expect(container.querySelectorAll("[data-drop-target]")).toHaveLength(0);
  });

  it("does not collide registration ids when sibling strips repeat tab values", () => {
    const onLeftReorder = vi.fn();
    // Duplicate live source or target ids are an error in the controller, so a
    // second strip with the same values would throw on registration if ids
    // were minted from the value alone. Rendering is the assertion.
    const { container } = render(
      <Harness kind="poodle.test-family" rightItems={repeated} onLeftReorder={onLeftReorder} />,
    );
    layout(container);

    expect(leftTabs(container)).toHaveLength(2);

    // And each strip still reorders itself, so the ids resolve to the right
    // registrations rather than merely being unique.
    dragTo(leftTabs(container)[0], 100);
    expect(onLeftReorder).toHaveBeenCalledWith(["beta", "alpha"]);
  });

  it("falls a foreign same-family subject through to the composite exactly once", () => {
    const onCompositeDrop = vi.fn();
    const onLeftReorder = vi.fn();
    const onRightReorder = vi.fn();
    const { container } = render(
      <Harness
        kind="poodle.test-family"
        onCompositeDrop={onCompositeDrop}
        onLeftReorder={onLeftReorder}
        onRightReorder={onRightReorder}
      />,
    );
    layout(container);

    // Aimed squarely at one of the right strip's own tabs. The subject id
    // "alpha" belongs only to the left strip, which is the capture hazard: a
    // shared family makes it *reachable* here, so the right strip must refuse
    // it during eligibility — not at commit — so the composite around it wins
    // arbitration instead.
    dragTo(leftTabs(container)[0], 520);

    expect(onCompositeDrop).toHaveBeenCalledOnce();
    expect(onCompositeDrop.mock.calls[0][0].targetId).toBe("composite");
    expect(onRightReorder).not.toHaveBeenCalled();
    expect(onLeftReorder).not.toHaveBeenCalled();
  });

  it("keeps same-strip reorder working inside a shared family", () => {
    const onLeftReorder = vi.fn();
    const onCompositeDrop = vi.fn();
    const { container } = render(
      <Harness
        kind="poodle.test-family"
        onLeftReorder={onLeftReorder}
        onCompositeDrop={onCompositeDrop}
      />,
    );
    layout(container);

    dragTo(leftTabs(container)[0], 100);

    expect(onLeftReorder).toHaveBeenCalledWith(["beta", "alpha"]);
    expect(onCompositeDrop).not.toHaveBeenCalled();
  });
});

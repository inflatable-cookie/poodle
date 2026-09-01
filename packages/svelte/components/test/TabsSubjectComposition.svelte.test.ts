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

import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import Harness from "./TabsSubjectCompositionHarness.svelte";
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

  const hosts = ["left-host", "right-host"] as const;
  hosts.forEach((testid, hostIndex) => {
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

async function dragTo(source: HTMLElement, x: number): Promise<void> {
  await fireEvent(source, pointer("pointerdown", 20));
  await fireEvent(document, pointer("pointermove", 60));
  await fireEvent(document, pointer("pointermove", x));
  await fireEvent(document, pointer("pointerup", x));
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

  it("keeps ordinary sibling strips mutually ineligible under one provider", async () => {
    const onLeftReorder = vi.fn();
    const onRightReorder = vi.fn();
    const { container } = render(Harness, {
      props: { left, right, onLeftReorder, onRightReorder },
    });
    layout(container);

    // Two strips, one controller, and the same tab values in both. Without an
    // explicit kind each strip has its own family, so nothing in the right
    // strip is a candidate for the left strip's subject.
    // Aimed at the right strip's *second* tab, whose value differs from the
    // dragged subject: a self-drop would refuse for the wrong reason and prove
    // nothing about family scoping.
    await dragTo(leftTabs(container)[0], 520);

    expect(onRightReorder).not.toHaveBeenCalled();
    expect(onLeftReorder).not.toHaveBeenCalled();
    expect(container.querySelectorAll("[data-drop-target]")).toHaveLength(0);
  });

  it("does not collide registration ids when sibling strips repeat tab values", async () => {
    const onLeftReorder = vi.fn();
    // Duplicate live source or target ids are an error in the controller, so a
    // second strip with the same values would throw on registration if ids
    // were minted from the value alone. Rendering is the assertion.
    const { container } = render(Harness, {
      props: { kind: "poodle.test-family", left, right: repeated, onLeftReorder },
    });
    layout(container);

    expect(leftTabs(container)).toHaveLength(2);

    // And each strip still reorders itself, so the ids resolve to the right
    // registrations rather than merely being unique.
    await dragTo(leftTabs(container)[0], 140);
    expect(onLeftReorder).toHaveBeenCalledWith(["beta", "alpha"]);
  });

  it("falls a foreign same-family subject through to the composite exactly once", async () => {
    const onCompositeDrop = vi.fn();
    const onLeftReorder = vi.fn();
    const onRightReorder = vi.fn();
    const { container } = render(Harness, {
      props: {
        kind: "poodle.test-family",
        left,
        right,
        onCompositeDrop,
        onLeftReorder,
        onRightReorder,
      },
    });
    layout(container);

    // Aimed squarely at one of the right strip's own tabs. The subject id
    // "alpha" belongs only to the left strip, which is the capture hazard: a
    // shared family makes it *reachable* here, so the right strip must refuse
    // it during eligibility — not at commit — so the composite around it wins
    // arbitration instead.
    await dragTo(leftTabs(container)[0], 520);

    expect(onCompositeDrop).toHaveBeenCalledOnce();
    expect(onCompositeDrop.mock.calls[0][0].targetId).toBe("composite");
    expect(onRightReorder).not.toHaveBeenCalled();
    expect(onLeftReorder).not.toHaveBeenCalled();
  });

  it("keeps same-strip reorder working inside a shared family", async () => {
    const onLeftReorder = vi.fn();
    const onCompositeDrop = vi.fn();
    const { container } = render(Harness, {
      props: { kind: "poodle.test-family", left, right, onLeftReorder, onCompositeDrop },
    });
    layout(container);

    await dragTo(leftTabs(container)[0], 140);

    expect(onLeftReorder).toHaveBeenCalledWith(["beta", "alpha"]);
    expect(onCompositeDrop).not.toHaveBeenCalled();
  });
});

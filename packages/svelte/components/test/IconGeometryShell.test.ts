import { tick } from "svelte";
import { fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

import IconGeometryShell from "./IconGeometryShell.svelte";
import IconGeometryShellFocusHarness from "./IconGeometryShellFocusHarness.svelte";

afterEach(() => {
  vi.useRealTimers();
});

const PAIR = "chevron-left-to-chevron-right";

function svgOf(container: HTMLElement): SVGSVGElement {
  const svg = container.querySelector<SVGSVGElement>("[data-poodle-icon-geometry]");
  if (!svg) throw new Error("missing geometry svg");
  return svg;
}

describe("IconGeometryShell (svelte, private)", () => {
  it("keeps one svg root across start, mid, end, reverse, and frozen", async () => {
    const view = render(IconGeometryShell, {
      props: { pairId: PAIR, target: "to", progress: 0 },
    });
    const root = svgOf(view.container);
    expect(root.getAttribute("viewBox")).toBe("0 0 24 24");
    expect(root.querySelectorAll("path").length).toBeGreaterThan(0);
    const startD = root.querySelector("path")?.getAttribute("d");

    await view.rerender({ pairId: PAIR, target: "to", progress: 0.5 });
    expect(svgOf(view.container)).toBe(root);
    const midD = root.querySelector("path")?.getAttribute("d");
    expect(midD).not.toBe(startD);

    await view.rerender({ pairId: PAIR, target: "to", progress: 1 });
    expect(svgOf(view.container)).toBe(root);
    const endD = root.querySelector("path")?.getAttribute("d");
    expect(endD).not.toBe(midD);

    await view.rerender({ pairId: PAIR, target: "from", progress: 0.4 });
    expect(svgOf(view.container)).toBe(root);

    await view.rerender({ pairId: PAIR, target: "to", policy: "frozen", progress: 0.4 });
    expect(svgOf(view.container)).toBe(root);
    expect(root.querySelector("path")?.getAttribute("d")).toBe(endD);
  });

  it("does not change sibling focus or layout when the sampled frame updates", async () => {
    const view = render(IconGeometryShellFocusHarness, { props: { progress: 0 } });
    const button = view.getByRole("button", { name: "Keep focus" });
    const svg = svgOf(view.container);
    button.focus();
    const before = {
      active: document.activeElement,
      button: button.getBoundingClientRect().toJSON(),
      svg: svg.getBoundingClientRect().toJSON(),
    };

    await view.rerender({ progress: 0.6 });
    await tick();

    expect(document.activeElement).toBe(before.active);
    expect(button.getBoundingClientRect().toJSON()).toEqual(before.button);
    expect(svgOf(view.container).getBoundingClientRect().toJSON()).toEqual(before.svg);
  });

  it("teardown leaves no late rAF write", async () => {
    let nextId = 1;
    const raf = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation(() => nextId++);
    const cancel = vi.spyOn(globalThis, "cancelAnimationFrame");
    const view = render(IconGeometryShell, {
      props: { pairId: PAIR, target: "to" },
    });
    await tick();
    expect(svgOf(view.container).querySelector("path")).not.toBeNull();
    expect(raf).toHaveBeenCalled();
    const scheduledId = raf.mock.results[0]?.value as number;
    view.unmount();
    expect(cancel).toHaveBeenCalledWith(scheduledId);
    const callsAfterUnmount = raf.mock.calls.length;
    await fireEvent.animationEnd(document.body);
    await tick();
    expect(raf.mock.calls.length).toBe(callsAfterUnmount);
    cancel.mockRestore();
    raf.mockRestore();
  });

  it("keeps inert progress, reverses proportionally, and cancels when frozen", async () => {
    let now = 0;
    let nextId = 1;
    const callbacks = new Map<number, FrameRequestCallback>();
    const clock = vi.spyOn(performance, "now").mockImplementation(() => now);
    const raf = vi.spyOn(globalThis, "requestAnimationFrame").mockImplementation((callback) => {
      const id = nextId++;
      callbacks.set(id, callback);
      return id;
    });
    const cancel = vi.spyOn(globalThis, "cancelAnimationFrame").mockImplementation((id) => {
      callbacks.delete(id);
    });
    const run = async (at: number) => {
      now = at;
      const [id, callback] = callbacks.entries().next().value as [number, FrameRequestCallback];
      callbacks.delete(id);
      callback(at);
      await tick();
    };

    const view = render(IconGeometryShell, { props: { pairId: PAIR, target: "to" } });
    await tick();
    const start = svgOf(view.container).querySelector("path")?.getAttribute("d");
    await run(72);
    const partial = svgOf(view.container).querySelector("path")?.getAttribute("d");
    expect(partial).not.toBe(start);

    await view.rerender({ pairId: PAIR, target: "to", initial: true });
    expect(callbacks.size).toBe(1);
    await view.rerender({ pairId: PAIR, target: "from", initial: false });
    await run(144);
    expect(svgOf(view.container).querySelector("path")?.getAttribute("d")).toBe(start);
    expect(callbacks.size).toBe(0);

    await view.rerender({ pairId: PAIR, target: "to", initial: false });
    expect(callbacks.size).toBe(1);
    await view.rerender({ pairId: PAIR, target: "to", policy: "frozen" });
    expect(callbacks.size).toBe(0);
    expect(cancel).toHaveBeenCalled();
    view.unmount();
    cancel.mockRestore();
    raf.mockRestore();
    clock.mockRestore();
  });
});

import { act } from "react";
import { hydrateRoot } from "react-dom/client";
import { renderToString } from "react-dom/server";
import { fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { IconGeometryShell } from "./IconGeometryShell";

afterEach(() => {
  vi.useRealTimers();
});

const PAIR = "chevron-left-to-chevron-right";

function svgOf(container: HTMLElement): SVGSVGElement {
  const svg = container.querySelector<SVGSVGElement>("[data-poodle-icon-geometry]");
  if (!svg) throw new Error("missing geometry svg");
  return svg;
}

describe("IconGeometryShell (react, private)", () => {
  it("keeps one svg root across start, mid, end, reverse, and frozen", () => {
    const view = render(<IconGeometryShell pairId={PAIR} target="to" progress={0} />);
    const root = svgOf(view.container);
    expect(root.getAttribute("viewBox")).toBe("0 0 24 24");
    expect(root.querySelectorAll("path").length).toBeGreaterThan(0);
    const startD = root.querySelector("path")?.getAttribute("d");

    view.rerender(<IconGeometryShell pairId={PAIR} target="to" progress={0.5} />);
    expect(svgOf(view.container)).toBe(root);
    const midD = root.querySelector("path")?.getAttribute("d");
    expect(midD).not.toBe(startD);

    view.rerender(<IconGeometryShell pairId={PAIR} target="to" progress={1} />);
    expect(svgOf(view.container)).toBe(root);
    const endD = root.querySelector("path")?.getAttribute("d");
    expect(endD).not.toBe(midD);

    view.rerender(<IconGeometryShell pairId={PAIR} target="from" progress={0.4} />);
    expect(svgOf(view.container)).toBe(root);

    view.rerender(
      <IconGeometryShell pairId={PAIR} target="to" policy="frozen" progress={0.4} />,
    );
    expect(svgOf(view.container)).toBe(root);
    expect(root.querySelector("path")?.getAttribute("d")).toBe(endD);
  });

  it("does not change sibling focus or layout when the sampled frame updates", () => {
    const view = render(
      <div>
        <button type="button">Keep focus</button>
        <IconGeometryShell pairId={PAIR} target="to" progress={0} />
      </div>,
    );
    const button = view.getByRole("button", { name: "Keep focus" });
    button.focus();
    const before = {
      active: document.activeElement,
      button: button.getBoundingClientRect().toJSON(),
      svg: svgOf(view.container).getBoundingClientRect().toJSON(),
    };

    view.rerender(
      <div>
        <button type="button">Keep focus</button>
        <IconGeometryShell pairId={PAIR} target="to" progress={0.6} />
      </div>,
    );

    expect(document.activeElement).toBe(before.active);
    expect(button.getBoundingClientRect().toJSON()).toEqual(before.button);
    expect(svgOf(view.container).getBoundingClientRect().toJSON()).toEqual(before.svg);
  });

  it("emits SSR endpoint HTML and hydrates without swapping the svg root", async () => {
    const element = <IconGeometryShell pairId={PAIR} target="from" initial />;
    const html = renderToString(element);
    expect(html).toContain("data-poodle-icon-geometry");
    expect(html).toContain('viewBox="0 0 24 24"');
    expect(html).toContain("<path");
    expect(html).not.toContain("chevron-left-to-chevron-right");

    const container = document.createElement("div");
    document.body.appendChild(container);
    container.innerHTML = html;
    const before = container.querySelector("[data-poodle-icon-geometry]");

    let root: ReturnType<typeof hydrateRoot> | undefined;
    await act(async () => {
      root = hydrateRoot(container, element);
    });
    const after = container.querySelector("[data-poodle-icon-geometry]");
    expect(after).toBe(before);
    await act(async () => {
      root?.unmount();
    });
    container.remove();
  });

  it("teardown leaves no late rAF write", () => {
    const raf = vi.spyOn(globalThis, "requestAnimationFrame");
    const view = render(<IconGeometryShell pairId={PAIR} target="to" progress={0.3} />);
    expect(svgOf(view.container).querySelector("path")).not.toBeNull();
    const callsAtUnmount = raf.mock.calls.length;
    view.unmount();
    fireEvent.animationEnd(document.body);
    expect(raf.mock.calls.length).toBe(callsAtUnmount);
    raf.mockRestore();
  });

  it("rejected pair paints no path and keeps the svg root", () => {
    const view = render(
      <IconGeometryShell pairId="menu-to-ellipsis" target="to" progress={0.5} />,
    );
    const root = svgOf(view.container);
    expect(root.querySelectorAll("path")).toHaveLength(0);
  });
});

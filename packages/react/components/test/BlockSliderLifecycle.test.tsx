/**
 * Block Slider and RangeSlider adapter terminal ownership (g16.046 repair).
 * Mirrors packages/react/components/test/AudioControlsLifecycle.test.tsx.
 */

import { fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { RangeSlider, Slider } from "../src";

const TRACK = { left: 0, top: 0, width: 200, height: 44, right: 200, bottom: 44, x: 0, y: 0, toJSON: () => ({}) };

function mockRect(element: HTMLElement, rect: Partial<DOMRect> = {}): HTMLElement {
  const box = { ...TRACK, ...rect };
  element.getBoundingClientRect = () => box as DOMRect;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  return element;
}

function mockRangeGesture(container: HTMLElement, thumb: HTMLElement): HTMLElement {
  mockRect(container.querySelector(".poodle-range-slider") as HTMLElement);
  return mockRect(thumb);
}

function BoundSlider({ initial, ...props }: { initial: number } & Record<string, unknown>) {
  const [value, setValue] = useState(initial);
  return (
    <Slider
      {...props}
      appearance="block"
      value={value}
      onValueChange={(next: number) => {
        setValue(next);
        (props.onValueChange as ((v: number) => void) | undefined)?.(next);
      }}
    />
  );
}

function BoundRangeSlider({ initial, ...props }: { initial: [number, number] } & Record<string, unknown>) {
  const [value, setValue] = useState<[number, number]>(initial);
  return (
    <RangeSlider
      {...props}
      appearance="block"
      value={value}
      onValueChange={(next: [number, number]) => {
        setValue(next);
        (props.onValueChange as ((v: [number, number]) => void) | undefined)?.(next);
      }}
    />
  );
}

describe("block slider terminal ownership (react)", () => {
  it("Slider unmount after callback swap commits through the latest onValueCommit", () => {
    const first = vi.fn();
    const second = vi.fn();
    const view = render(
      <BoundSlider initial={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={first} />,
    );
    const slider = mockRect(view.getByRole("slider", { name: "Gain" }));
    fireEvent.pointerDown(slider, { button: 0, pointerId: 1, clientX: 100, clientY: 22 });
    fireEvent.pointerMove(slider, { pointerId: 1, clientX: 140, clientY: 22 });
    view.rerender(
      <BoundSlider initial={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={second} />,
    );
    view.unmount();
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledOnce();
    expect(second).toHaveBeenCalledWith(70);
  });

  it("Slider unmount after controlledness swap uses the live callback", () => {
    const first = vi.fn();
    const second = vi.fn();
    const view = render(
      <Slider appearance="block" value={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={first} />,
    );
    const slider = mockRect(view.getByRole("slider", { name: "Gain" }));
    fireEvent.pointerDown(slider, { button: 0, pointerId: 1, clientX: 100, clientY: 22 });
    fireEvent.pointerMove(slider, { pointerId: 1, clientX: 140, clientY: 22 });
    view.rerender(
      <Slider appearance="block" defaultValue={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={second} />,
    );
    view.unmount();
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledOnce();
    expect(second).toHaveBeenCalledWith(70);
  });

  it("Slider closes an open gesture once on teardown and ignores stale pointer ids", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundSlider initial={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={onValueCommit} />);
    const slider = mockRect(view.getByRole("slider", { name: "Gain" }));
    fireEvent.pointerDown(slider, { button: 0, pointerId: 1, clientX: 100, clientY: 22 });
    fireEvent.pointerMove(slider, { pointerId: 9, clientX: 160, clientY: 22 });
    fireEvent.pointerCancel(slider, { pointerId: 1 });
    fireEvent.pointerCancel(slider, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
    view.unmount();
    expect(onValueCommit).toHaveBeenCalledOnce();
  });

  it("Slider disabled during drag terminates with the live callback", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundSlider initial={50} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={onValueCommit} />);
    const slider = mockRect(view.getByRole("slider", { name: "Gain" }));
    fireEvent.pointerDown(slider, { button: 0, pointerId: 1, clientX: 100, clientY: 22 });
    view.rerender(
      <BoundSlider initial={50} min={0} max={100} step={10} ariaLabel="Gain" disabled onValueCommit={onValueCommit} />,
    );
    expect(onValueCommit).toHaveBeenCalledOnce();
  });

  it("RangeSlider unmount after callback swap commits through the latest onValueCommit", () => {
    const first = vi.fn();
    const second = vi.fn();
    const view = render(
      <BoundRangeSlider initial={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={first} />,
    );
    const lower = mockRangeGesture(view.container, view.getByRole("slider", { name: "Gain minimum" }));
    fireEvent.pointerDown(lower, { button: 0, pointerId: 1, clientX: 40, clientY: 22 });
    fireEvent.pointerMove(lower, { pointerId: 1, clientX: 80, clientY: 22 });
    view.rerender(
      <BoundRangeSlider initial={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={second} />,
    );
    view.unmount();
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledOnce();
    expect(second).toHaveBeenCalledWith([40, 80]);
  });

  it("RangeSlider unmount after controlledness swap uses the live callback", () => {
    const first = vi.fn();
    const second = vi.fn();
    const view = render(
      <RangeSlider appearance="block" value={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={first} />,
    );
    const lower = mockRangeGesture(view.container, view.getByRole("slider", { name: "Gain minimum" }));
    fireEvent.pointerDown(lower, { button: 0, pointerId: 1, clientX: 40, clientY: 22 });
    fireEvent.pointerMove(lower, { pointerId: 1, clientX: 80, clientY: 22 });
    view.rerender(
      <RangeSlider appearance="block" defaultValue={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={second} />,
    );
    view.unmount();
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledOnce();
    expect(second).toHaveBeenCalledWith([40, 80]);
  });

  it("RangeSlider lost capture ends the gesture once", () => {
    const onValueCommit = vi.fn();
    const view = render(
      <BoundRangeSlider initial={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={onValueCommit} />,
    );
    const upper = mockRangeGesture(view.container, view.getByRole("slider", { name: "Gain maximum" }));
    fireEvent.pointerDown(upper, { button: 0, pointerId: 1, clientX: 160, clientY: 22 });
    fireEvent.lostPointerCapture(upper, { pointerId: 1 });
    fireEvent.lostPointerCapture(upper, { pointerId: 1 });
    expect(onValueCommit).toHaveBeenCalledOnce();
  });

  it("RangeSlider disabled during drag terminates with the live callback", () => {
    const onValueCommit = vi.fn();
    const view = render(
      <BoundRangeSlider initial={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" onValueCommit={onValueCommit} />,
    );
    const lower = mockRangeGesture(view.container, view.getByRole("slider", { name: "Gain minimum" }));
    fireEvent.pointerDown(lower, { button: 0, pointerId: 1, clientX: 40, clientY: 22 });
    view.rerender(
      <BoundRangeSlider initial={[20, 80]} min={0} max={100} step={10} ariaLabel="Gain" disabled onValueCommit={onValueCommit} />,
    );
    expect(onValueCommit).toHaveBeenCalledOnce();
  });
});

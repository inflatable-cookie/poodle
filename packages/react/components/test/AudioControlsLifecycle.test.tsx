/**
 * Mounted pointer and entry lifecycle for the continuous audio controls
 * (g16.031). The pure transitions are pinned by the shared `audioControls`
 * vectors; these tests prove the React adapter's own responsibilities —
 * primary-pointer ownership, capture loss, teardown, and entry focus
 * transitions — through real DOM events. The claims and expected traces mirror
 * packages/svelte/components/test/AudioControlsLifecycle.svelte.test.ts.
 */

import { act, fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import { Fader, Knob, XYPad } from "../src";

const SQUARE = { left: 0, top: 0, width: 100, height: 100, right: 100, bottom: 100, x: 0, y: 0, toJSON: () => ({}) };

function measurable(element: HTMLElement): HTMLElement {
  element.getBoundingClientRect = () => SQUARE as DOMRect;
  element.setPointerCapture = vi.fn();
  element.releasePointerCapture = vi.fn();
  return element;
}

function gestureSpies() {
  return { onValueChange: vi.fn(), onValueCommit: vi.fn(), onGestureBegin: vi.fn(), onGestureEnd: vi.fn() };
}

/**
 * The Svelte mirror binds `value`, so the React cases hold the same value in
 * host state. A fixed `value` prop would report a stale commit and would not
 * be the same trace.
 */
function BoundKnob({ initial, ...props }: { initial: number } & Record<string, unknown>) {
  const [value, setValue] = useState(initial);
  return <Knob {...props} value={value} onValueChange={(next: number) => { setValue(next); (props.onValueChange as ((v: number) => void) | undefined)?.(next); }} />;
}

function BoundFader({ initial, ...props }: { initial: number } & Record<string, unknown>) {
  const [value, setValue] = useState(initial);
  return <Fader {...props} value={value} onValueChange={(next: number) => { setValue(next); (props.onValueChange as ((v: number) => void) | undefined)?.(next); }} />;
}

function BoundXYPad(props: Record<string, unknown>) {
  const [pair, setPair] = useState({ x: 0, y: 0 });
  return <XYPad {...props} x={pair.x} y={pair.y} onValueChange={(x: number, y: number) => { setPair({ x, y }); (props.onValueChange as ((x: number, y: number) => void) | undefined)?.(x, y); }} />;
}

describe("audio control pointer lifecycle (react)", () => {
  it("Knob pairs one begin and one end and refuses a second pointer", () => {
    const spies = gestureSpies();
    const view = render(<BoundKnob initial={0.5} ariaLabel="Gain" {...spies} />);
    const knob = measurable(view.getByRole("slider", { name: "Gain" }));

    fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    fireEvent.pointerDown(knob, { button: 0, pointerId: 2, clientX: 50, clientY: 40 });
    fireEvent.pointerMove(knob, { pointerId: 2, clientX: 50, clientY: 10 });
    fireEvent.pointerMove(knob, { pointerId: 1, clientX: 50, clientY: 44 });
    fireEvent.pointerUp(knob, { pointerId: 1 });
    fireEvent.pointerUp(knob, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.6]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.6]]);
  });

  it("Knob closes the gesture once when pointer capture is lost", () => {
    const spies = gestureSpies();
    const view = render(<BoundKnob initial={0.5} ariaLabel="Gain" {...spies} />);
    const knob = measurable(view.getByRole("slider", { name: "Gain" }));

    fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    fireEvent.lostPointerCapture(knob, { pointerId: 9 });
    fireEvent.lostPointerCapture(knob, { pointerId: 1 });
    fireEvent.lostPointerCapture(knob, { pointerId: 1 });
    fireEvent.pointerUp(knob, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
  });

  it("Knob closes an open gesture exactly once on teardown", () => {
    const spies = gestureSpies();
    const view = render(<BoundKnob initial={0.5} ariaLabel="Gain" {...spies} />);
    const knob = measurable(view.getByRole("slider", { name: "Gain" }));

    fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    view.unmount();

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.5]]);
  });

  it("Fader cancellation ends the gesture and stale pointer ids stay inert", () => {
    const spies = gestureSpies();
    const view = render(<BoundFader initial={0.25} orientation="horizontal" ariaLabel="Mix" {...spies} />);
    const fader = measurable(view.getByRole("slider", { name: "Mix" }));

    fireEvent.pointerDown(fader, { button: 0, pointerId: 1, clientX: 25, clientY: 50 });
    fireEvent.pointerMove(fader, { pointerId: 7, clientX: 90, clientY: 50 });
    fireEvent.pointerUp(fader, { pointerId: 7 });
    fireEvent.pointerMove(fader, { pointerId: 1, clientX: 80, clientY: 50 });
    fireEvent.pointerCancel(fader, { pointerId: 1 });
    fireEvent.pointerCancel(fader, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.25], [0.8]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.8]]);
  });

  it("XYPad presses at the pointer position and refuses a second pointer", () => {
    const spies = gestureSpies();
    const view = render(<BoundXYPad ariaLabel="Position" {...spies} />);
    view.getByRole("group", { name: "Position" });
    const pad = measurable(view.container.querySelector<HTMLElement>("[data-scope='xy-pad']")!);

    fireEvent.pointerDown(pad, { button: 0, pointerId: 1, clientX: 25, clientY: 25 });
    fireEvent.pointerDown(pad, { button: 0, pointerId: 2, clientX: 90, clientY: 90 });
    fireEvent.pointerMove(pad, { pointerId: 1, clientX: 50, clientY: 50 });
    fireEvent.pointerUp(pad, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.25, 0.75], [0.5, 0.5]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.5, 0.5]]);
  });
});

describe("audio control entry lifecycle (react)", () => {
  /**
   * Enter and Escape return focus to the root, which blurs the entry. The
   * blur listener proves that transition really happened, so a passing
   * commit count cannot come from a blur that never fired.
   */
  function watchBlur(entry: HTMLElement) {
    const blurred = vi.fn();
    entry.addEventListener("blur", blurred);
    // Focusing the entry blurs whatever held focus, which is a real state
    // update in the control that just handed focus back to its root.
    act(() => entry.focus());
    return blurred;
  }

  it("Knob Enter commits once and the blur it causes cannot commit again", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundKnob initial={0.5} format={{ type: "percent" }} ariaLabel="Gain" onValueCommit={onValueCommit} />);

    fireEvent.keyDown(view.getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = view.getByRole("textbox", { name: "Gain value" });
    const blurred = watchBlur(entry);
    fireEvent.change(entry, { target: { value: "75%" } });
    fireEvent.keyDown(entry, { key: "Enter" });

    expect(blurred).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.75]]);
  });

  it("Knob Escape commits nothing and the blur it causes cannot commit the draft", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundKnob initial={0.5} format={{ type: "percent" }} ariaLabel="Gain" onValueCommit={onValueCommit} />);

    fireEvent.keyDown(view.getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = view.getByRole("textbox", { name: "Gain value" });
    const blurred = watchBlur(entry);
    fireEvent.change(entry, { target: { value: "75%" } });
    fireEvent.keyDown(entry, { key: "Escape" });

    expect(blurred).toHaveBeenCalledTimes(1);
    expect(onValueCommit).not.toHaveBeenCalled();
    expect(view.getByRole("slider", { name: "Gain" }).getAttribute("aria-valuenow")).toBe("0.5");
  });

  it("Knob an unresolved blur still commits the draft", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundKnob initial={0.5} format={{ type: "percent" }} ariaLabel="Gain" onValueCommit={onValueCommit} />);

    fireEvent.keyDown(view.getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = view.getByRole("textbox", { name: "Gain value" });
    fireEvent.change(entry, { target: { value: "75%" } });
    fireEvent.blur(entry);

    expect(onValueCommit.mock.calls).toEqual([[0.75]]);
  });

  it("Fader Enter commits once and a following Escape reverses nothing", () => {
    const onValueCommit = vi.fn();
    const view = render(<BoundFader initial={0.25} ariaLabel="Mix" onValueCommit={onValueCommit} />);
    const fader = view.getByRole("slider", { name: "Mix" });

    fireEvent.keyDown(fader, { key: "Enter" });
    const entry = view.getByRole("textbox", { name: "Mix value" });
    const committed = watchBlur(entry);
    fireEvent.change(entry, { target: { value: "0.8" } });
    fireEvent.keyDown(entry, { key: "Enter" });
    expect(committed).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.8]]);

    fireEvent.keyDown(fader, { key: "Enter" });
    const reopened = view.getByRole("textbox", { name: "Mix value" });
    const cancelled = watchBlur(reopened);
    fireEvent.change(reopened, { target: { value: "0.1" } });
    fireEvent.keyDown(reopened, { key: "Escape" });
    expect(cancelled).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.8]]);
    expect(fader.getAttribute("aria-valuenow")).toBe("0.8");
  });
});

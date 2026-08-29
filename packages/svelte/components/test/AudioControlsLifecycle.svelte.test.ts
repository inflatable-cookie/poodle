/**
 * Mounted pointer and entry lifecycle for the continuous audio controls
 * (g16.031). The pure transitions are pinned by the shared `audioControls`
 * vectors; these tests prove the Svelte adapter's own responsibilities —
 * primary-pointer ownership, capture loss, teardown, and entry focus
 * transitions — through real DOM events. The React mirror asserts the same
 * traces (packages/react/components/test/AudioControlsLifecycle.test.tsx).
 */

import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import Fader from "../src/Fader.svelte";
import Knob from "../src/Knob.svelte";
import XYPad from "../src/XYPad.svelte";

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

describe("audio control pointer lifecycle (svelte)", () => {
  it("Knob pairs one begin and one end and refuses a second pointer", async () => {
    const spies = gestureSpies();
    const { getByRole } = render(Knob, { props: { value: 0.5, ariaLabel: "Gain", ...spies } });
    const knob = measurable(getByRole("slider", { name: "Gain" }));

    await fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    await fireEvent.pointerDown(knob, { button: 0, pointerId: 2, clientX: 50, clientY: 40 });
    await fireEvent.pointerMove(knob, { pointerId: 2, clientX: 50, clientY: 10 });
    await fireEvent.pointerMove(knob, { pointerId: 1, clientX: 50, clientY: 44 });
    await fireEvent.pointerUp(knob, { pointerId: 1 });
    // A browser releases implicit capture after pointerup; that lost-capture
    // event must not close the gesture a second time.
    await fireEvent.lostPointerCapture(knob, { pointerId: 1 });
    await fireEvent.pointerUp(knob, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.6]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.6]]);
  });

  it("Knob closes the gesture once when pointer capture is lost", async () => {
    const spies = gestureSpies();
    const { getByRole } = render(Knob, { props: { value: 0.5, ariaLabel: "Gain", ...spies } });
    const knob = measurable(getByRole("slider", { name: "Gain" }));

    await fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    await fireEvent.lostPointerCapture(knob, { pointerId: 9 });
    await fireEvent.lostPointerCapture(knob, { pointerId: 1 });
    await fireEvent.lostPointerCapture(knob, { pointerId: 1 });
    await fireEvent.pointerUp(knob, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
  });

  it("Knob closes the gesture when the host removes it from onGestureBegin", async () => {
    // The host tears the control down inside the begin callback, before the
    // component's own reactive state is observable anywhere else. The terminal
    // must still run, and exactly once.
    const spies = gestureSpies();
    let removeHost: () => void = () => {};
    const onGestureBegin = vi.fn(() => { spies.onGestureBegin(); removeHost(); });

    const { getByRole, unmount } = render(Knob, {
      props: { value: 0.5, ariaLabel: "Gain", ...spies, onGestureBegin },
    });
    removeHost = unmount;
    const knob = measurable(getByRole("slider", { name: "Gain" }));
    await fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.5]]);
  });

  it("Fader keeps its press batch in order when the host removes it from onGestureBegin", async () => {
    // A coarse press emits `beginGesture` then `emitValueChange`. Tearing down
    // from the first callback must not let the terminal overtake the second:
    // the accepted transition's own effects come first, then the terminal.
    const trace: string[] = [];
    let removeHost: () => void = () => {};

    const { getByRole, unmount } = render(Fader, { props: {
      value: 0, orientation: "horizontal", ariaLabel: "Mix",
      onGestureBegin: () => { trace.push("begin"); removeHost(); },
      onValueChange: (next: number) => trace.push(`change:${next}`),
      onValueCommit: (next: number) => trace.push(`commit:${next}`),
      onGestureEnd: () => trace.push("end"),
    } });
    removeHost = unmount;
    const fader = measurable(getByRole("slider", { name: "Mix" }));
    await fireEvent.pointerDown(fader, { button: 0, pointerId: 1, clientX: 25, clientY: 50 });

    expect(trace).toEqual(["begin", "change:0.25", "commit:0.25", "end"]);
  });

  it("XYPad keeps its press batch in order when the host removes it from onGestureBegin", async () => {
    const trace: string[] = [];
    let removeHost: () => void = () => {};

    const { container, unmount } = render(XYPad, { props: {
      x: 0, y: 0, ariaLabel: "Position",
      onGestureBegin: () => { trace.push("begin"); removeHost(); },
      onValueChange: (x: number, y: number) => trace.push(`change:${x},${y}`),
      onValueCommit: (x: number, y: number) => trace.push(`commit:${x},${y}`),
      onGestureEnd: () => trace.push("end"),
    } });
    removeHost = unmount;
    const pad = measurable(container.querySelector<HTMLElement>("[data-scope='xy-pad']")!);
    await fireEvent.pointerDown(pad, { button: 0, pointerId: 1, clientX: 25, clientY: 25 });

    expect(trace).toEqual(["begin", "change:0.25,0.75", "commit:0.25,0.75", "end"]);
  });

  it("Knob closes an open gesture exactly once on teardown", async () => {
    const spies = gestureSpies();
    const { getByRole, unmount } = render(Knob, { props: { value: 0.5, ariaLabel: "Gain", ...spies } });
    const knob = measurable(getByRole("slider", { name: "Gain" }));

    await fireEvent.pointerDown(knob, { button: 0, pointerId: 1, clientX: 50, clientY: 60 });
    unmount();

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.5]]);
  });

  it("Fader cancellation ends the gesture and stale pointer ids stay inert", async () => {
    const spies = gestureSpies();
    const { getByRole } = render(Fader, { props: { value: 0.25, orientation: "horizontal", ariaLabel: "Mix", ...spies } });
    const fader = measurable(getByRole("slider", { name: "Mix" }));

    await fireEvent.pointerDown(fader, { button: 0, pointerId: 1, clientX: 25, clientY: 50 });
    await fireEvent.pointerMove(fader, { pointerId: 7, clientX: 90, clientY: 50 });
    await fireEvent.pointerUp(fader, { pointerId: 7 });
    await fireEvent.pointerMove(fader, { pointerId: 1, clientX: 80, clientY: 50 });
    await fireEvent.pointerCancel(fader, { pointerId: 1 });
    await fireEvent.pointerCancel(fader, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.25], [0.8]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.8]]);
  });

  it("XYPad presses at the pointer position and refuses a second pointer", async () => {
    const spies = { onValueChange: vi.fn(), onValueCommit: vi.fn(), onGestureBegin: vi.fn(), onGestureEnd: vi.fn() };
    const { container, getByRole } = render(XYPad, { props: { x: 0, y: 0, ariaLabel: "Position", ...spies } });
    getByRole("group", { name: "Position" });
    const pad = measurable(container.querySelector<HTMLElement>("[data-scope='xy-pad']")!);

    await fireEvent.pointerDown(pad, { button: 0, pointerId: 1, clientX: 25, clientY: 25 });
    await fireEvent.pointerDown(pad, { button: 0, pointerId: 2, clientX: 90, clientY: 90 });
    await fireEvent.pointerMove(pad, { pointerId: 1, clientX: 50, clientY: 50 });
    await fireEvent.pointerUp(pad, { pointerId: 1 });

    expect(spies.onGestureBegin).toHaveBeenCalledTimes(1);
    expect(spies.onGestureEnd).toHaveBeenCalledTimes(1);
    expect(spies.onValueChange.mock.calls).toEqual([[0.25, 0.75], [0.5, 0.5]]);
    expect(spies.onValueCommit.mock.calls).toEqual([[0.5, 0.5]]);
  });
});

describe("audio control entry lifecycle (svelte)", () => {
  /**
   * Enter and Escape return focus to the root, which blurs the entry. The
   * blur listener proves that transition really happened, so a passing
   * commit count cannot come from a blur that never fired.
   */
  function watchBlur(entry: HTMLElement) {
    const blurred = vi.fn();
    entry.addEventListener("blur", blurred);
    entry.focus();
    return blurred;
  }

  it("Knob Enter commits once and the blur it causes cannot commit again", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Knob, { props: { value: 0.5, format: { type: "percent" }, ariaLabel: "Gain", onValueCommit } });

    await fireEvent.keyDown(getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = getByRole("textbox", { name: "Gain value" });
    const blurred = watchBlur(entry);
    await fireEvent.input(entry, { target: { value: "75%" } });
    await fireEvent.keyDown(entry, { key: "Enter" });

    expect(blurred).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.75]]);
  });

  it("Knob Escape commits nothing and the blur it causes cannot commit the draft", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Knob, { props: { value: 0.5, format: { type: "percent" }, ariaLabel: "Gain", onValueCommit } });

    await fireEvent.keyDown(getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = getByRole("textbox", { name: "Gain value" });
    const blurred = watchBlur(entry);
    await fireEvent.input(entry, { target: { value: "75%" } });
    await fireEvent.keyDown(entry, { key: "Escape" });

    expect(blurred).toHaveBeenCalledTimes(1);
    expect(onValueCommit).not.toHaveBeenCalled();
    expect(getByRole("slider", { name: "Gain" }).getAttribute("aria-valuenow")).toBe("0.5");
  });

  it("Knob an unresolved blur still commits the draft", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Knob, { props: { value: 0.5, format: { type: "percent" }, ariaLabel: "Gain", onValueCommit } });

    await fireEvent.keyDown(getByRole("slider", { name: "Gain" }), { key: "Enter" });
    const entry = getByRole("textbox", { name: "Gain value" });
    await fireEvent.input(entry, { target: { value: "75%" } });
    await fireEvent.blur(entry);

    expect(onValueCommit.mock.calls).toEqual([[0.75]]);
  });

  it("Fader Enter commits once and a following Escape reverses nothing", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Fader, { props: { value: 0.25, ariaLabel: "Mix", onValueCommit } });
    const fader = getByRole("slider", { name: "Mix" });

    await fireEvent.keyDown(fader, { key: "Enter" });
    const entry = getByRole("textbox", { name: "Mix value" });
    const committed = watchBlur(entry);
    await fireEvent.input(entry, { target: { value: "0.8" } });
    await fireEvent.keyDown(entry, { key: "Enter" });
    expect(committed).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.8]]);

    await fireEvent.keyDown(fader, { key: "Enter" });
    const reopened = getByRole("textbox", { name: "Mix value" });
    const cancelled = watchBlur(reopened);
    await fireEvent.input(reopened, { target: { value: "0.1" } });
    await fireEvent.keyDown(reopened, { key: "Escape" });
    expect(cancelled).toHaveBeenCalledTimes(1);
    expect(onValueCommit.mock.calls).toEqual([[0.8]]);
    expect(fader.getAttribute("aria-valuenow")).toBe("0.8");
  });
});

import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";

import { createAudioMeterContext, createGainReductionMeterContext } from "@inflatable-cookie/poodle-core";
import AudioMeter from "../src/AudioMeter.svelte";
import AudioSwitch from "../src/AudioSwitch.svelte";
import DragNumberField from "../src/DragNumberField.svelte";
import EnvelopeEditor from "../src/EnvelopeEditor.svelte";
import Fader from "../src/Fader.svelte";
import GainReductionMeter from "../src/GainReductionMeter.svelte";
import Keyboard from "../src/Keyboard.svelte";
import Knob from "../src/Knob.svelte";
import ModMatrixGrid from "../src/ModMatrixGrid.svelte";
import ValueReadout from "../src/ValueReadout.svelte";
import WaveformDisplay from "../src/WaveformDisplay.svelte";
import XYPad from "../src/XYPad.svelte";

describe("audio controls", () => {
  it("Knob exposes formatted slider semantics and keyboard commits", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Knob, { props: {
      value: -12, min: -60, max: 6, keyboardStep: 1,
      format: { type: "db" }, ariaLabel: "Gain", onValueCommit,
    } });
    const knob = getByRole("slider", { name: "Gain" });
    expect(knob.getAttribute("aria-valuetext")).toBe("-12 dB");
    await fireEvent.keyDown(knob, { key: "ArrowUp" });
    expect(onValueCommit).toHaveBeenCalledWith(-11);
    expect(knob.getAttribute("aria-valuenow")).toBe("-11");
  });

  it("Fader reports orientation and supports reset", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(Fader, { props: {
      value: 0.25, defaultValue: 0.5, orientation: "horizontal",
      ariaLabel: "Mix", onValueCommit,
    } });
    const fader = getByRole("slider", { name: "Mix" });
    expect(fader.getAttribute("aria-orientation")).toBe("horizontal");
    await fireEvent.dblClick(fader);
    expect(onValueCommit).toHaveBeenCalledWith(0.5);
  });

  it("DragNumberField type-in uses the shared formatter parser", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(DragNumberField, { props: {
      value: 250, min: 0, max: 5000, format: { type: "milliseconds" },
      ariaLabel: "Attack", onValueCommit,
    } });
    const spinbutton = getByRole("spinbutton", { name: "Attack" });
    await fireEvent.keyDown(spinbutton, { key: "Enter" });
    const entry = getByRole("textbox", { name: "Attack value" });
    await fireEvent.input(entry, { target: { value: "1.5 s" } });
    await fireEvent.keyDown(entry, { key: "Enter" });
    expect(onValueCommit).toHaveBeenCalledWith(1500);
  });

  it("DragNumberField click-to-edit does not open an automation gesture", async () => {
    const onGestureBegin = vi.fn();
    const onGestureEnd = vi.fn();
    const { getByRole } = render(DragNumberField, { props: {
      value: 12, ariaLabel: "Value", onGestureBegin, onGestureEnd,
    } });
    const spinbutton = getByRole("spinbutton", { name: "Value" });
    Object.defineProperty(spinbutton, "setPointerCapture", { value: vi.fn() });
    await fireEvent.pointerDown(spinbutton, { button: 0, pointerId: 1, clientX: 20 });
    await fireEvent.pointerUp(spinbutton, { pointerId: 1, clientX: 20 });
    expect(getByRole("textbox", { name: "Value value" })).toBeTruthy();
    expect(onGestureBegin).not.toHaveBeenCalled();
    expect(onGestureEnd).not.toHaveBeenCalled();
  });

  it("AudioMeter and ValueReadout keep visuals out of the accessibility tree", () => {
    const rightContext = createAudioMeterContext({ ballisticDb: -12 });
    const meterRender = render(AudioMeter, { props: { ariaLabel: "Master", rightContext } });
    const meter = meterRender.getByRole("meter", { name: "Master" });
    expect(meter.getAttribute("aria-valuetext")).toBe("Left -60 dB, right -12 dB");
    expect(meterRender.container.querySelector(".poodle-audio-meter-visual")?.getAttribute("aria-hidden")).toBe("true");

    const readoutRender = render(ValueReadout, { props: { value: 440, format: { type: "hz" }, ariaLabel: "Frequency" } });
    expect(readoutRender.getByText("440 Hz")).toBeTruthy();
    expect(readoutRender.container.querySelector(".poodle-value-readout-visual")?.getAttribute("aria-hidden")).toBe("true");
  });

  it("EnvelopeEditor exposes keyboard-operable point adapters", async () => {
    const onPointsCommit = vi.fn();
    const { getByRole } = render(EnvelopeEditor, { props: {
      points: [{ id: "attack", x: 0.25, y: 0.5, curve: 0 }],
      ariaLabel: "Envelope", onPointsCommit,
    } });
    const point = getByRole("button", { name: /Point 1,/ });
    await fireEvent.keyDown(point, { key: "ArrowUp" });
    expect(onPointsCommit.mock.calls.at(-1)?.[0][0]).toMatchObject({ id: "attack", y: 0.51 });
  });

  it("XYPad axis sliders commit atomic pairs", async () => {
    const onValueCommit = vi.fn();
    const { getByRole } = render(XYPad, { props: { x: 0.25, y: 0.75, ariaLabel: "Position", onValueCommit } });
    await fireEvent.keyDown(getByRole("slider", { name: "Position X" }), { key: "ArrowRight" });
    expect(onValueCommit).toHaveBeenCalledWith(0.26, 0.75);
  });

  it("AudioSwitch keyboard operation preserves independent lamp state", async () => {
    const onStateCommit = vi.fn();
    const { getByRole, container } = render(AudioSwitch, { props: {
      mode: "latch", state: 0, lampOn: true, ariaLabel: "Bypass", onStateCommit,
    } });
    const button = getByRole("button", { name: "Bypass" });
    await fireEvent.keyDown(button, { key: "Enter" });
    await fireEvent.keyUp(button, { key: "Enter" });
    expect(onStateCommit).toHaveBeenCalledWith(1);
    expect(container.querySelector(".poodle-audio-switch-visual")?.getAttribute("data-lamp")).toBe("true");
  });

  it("GainReductionMeter reports positive reduction and hides its renderer", () => {
    const context = createGainReductionMeterContext({ reductionDb: 12, ballisticDb: 12 });
    const { getByRole, container } = render(GainReductionMeter, { props: { context, ariaLabel: "Compression" } });
    expect(getByRole("meter", { name: "Compression" }).getAttribute("aria-valuetext")).toBe("12 dB reduction");
    expect(container.querySelector(".poodle-gain-reduction-meter-visual")?.getAttribute("aria-hidden")).toBe("true");
  });

  it("Keyboard pairs computer-key note effects", async () => {
    const onNoteOn = vi.fn(); const onNoteOff = vi.fn();
    const { getByRole } = render(Keyboard, { props: { firstNote: 48, lastNote: 72, ariaLabel: "Keys", onNoteOn, onNoteOff } });
    const keyboard = getByRole("toolbar", { name: "Keys" });
    await fireEvent.keyDown(keyboard, { key: "a" }); await fireEvent.keyUp(keyboard, { key: "a" });
    expect(onNoteOn).toHaveBeenCalledWith(60, 100); expect(onNoteOff).toHaveBeenCalledWith(60);
  });

  it("WaveformDisplay owns keyboard cursor state", async () => {
    const onCursorChange = vi.fn(); const pyramid = { sampleCount: 4, levels: [{ samplesPerPeak: 1, peaks: [{ min: -.2, max: .3 }, { min: -.5, max: .6 }, { min: -.1, max: .2 }, { min: -.7, max: .8 }] }] };
    const { getByRole } = render(WaveformDisplay, { props: { pyramid, cursorSample: 1, ariaLabel: "Clip", onCursorChange } });
    await fireEvent.keyDown(getByRole("slider"), { key: "ArrowRight" }); expect(onCursorChange).toHaveBeenCalledWith(2);
  });

  it("ModMatrixGrid toggles a focused generic cell", async () => {
    const onCellCommit = vi.fn(); const { getByRole } = render(ModMatrixGrid, { props: { sources: [{ id: "one", label: "Source 1" }], destinations: [{ id: "a", label: "Dest A" }], ariaLabel: "Routes", onCellCommit } });
    const cell = getByRole("gridcell"); await fireEvent.focus(cell); await fireEvent.keyDown(cell, { key: " " });
    expect(onCellCommit.mock.calls.at(-1)?.[0]).toMatchObject({ sourceId: "one", destinationId: "a", enabled: true });
  });
});

import { fireEvent, render } from "@testing-library/react";
import axe from "axe-core";
import { describe, expect, it, vi } from "vitest";
import { createAudioMeterContext, createGainReductionMeterContext } from "@inflatable-cookie/poodle-core";
import { AudioMeter, AudioSwitch, DragNumberField, EnvelopeEditor, Fader, GainReductionMeter, Keyboard, Knob, ModMatrixGrid, ValueReadout, WaveformDisplay, XYPad } from "../src";

const waveform = { sampleCount: 4, levels: [{ samplesPerPeak: 1, peaks: [{ min: -.2, max: .3 }, { min: -.5, max: .6 }, { min: -.1, max: .2 }, { min: -.7, max: .8 }] }] };
const matrixSources = [{ id: "one", label: "Source 1" }];
const matrixDestinations = [{ id: "a", label: "Dest A" }];

describe("audio controls (react)", () => {
  it("wires formatted knob keyboard commits", () => {
    const onValueCommit = vi.fn();
    const view = render(<Knob value={-12} min={-60} max={6} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" onValueCommit={onValueCommit} />);
    const knob = view.getByRole("slider", { name: "Gain" });
    expect(knob.getAttribute("aria-valuetext")).toBe("-12 dB");
    fireEvent.keyDown(knob, { key: "ArrowUp" });
    expect(onValueCommit).toHaveBeenCalledWith(-11);
  });

  it("wires fader orientation and reset", () => {
    const onValueCommit = vi.fn();
    const view = render(<Fader value={0.25} defaultValue={0.5} orientation="horizontal" ariaLabel="Mix" onValueCommit={onValueCommit} />);
    const fader = view.getByRole("slider", { name: "Mix" });
    expect(fader.getAttribute("aria-orientation")).toBe("horizontal");
    fireEvent.doubleClick(fader);
    expect(onValueCommit).toHaveBeenCalledWith(0.5);
  });

  it("uses shared parsing for drag-number direct entry", () => {
    const onValueCommit = vi.fn();
    const view = render(<DragNumberField value={250} min={0} max={5000} format={{ type: "milliseconds" }} ariaLabel="Attack" onValueCommit={onValueCommit} />);
    fireEvent.keyDown(view.getByRole("spinbutton", { name: "Attack" }), { key: "Enter" });
    const entry = view.getByRole("textbox", { name: "Attack value" });
    fireEvent.change(entry, { target: { value: "1.5 s" } });
    fireEvent.keyDown(entry, { key: "Enter" });
    expect(onValueCommit).toHaveBeenCalledWith(1500);
  });

  it("keeps meter and readout visuals hidden", () => {
    const meter = render(<AudioMeter ariaLabel="Master" rightContext={createAudioMeterContext({ ballisticDb: -12 })} />);
    expect(meter.getByRole("meter", { name: "Master" }).getAttribute("aria-valuetext")).toBe("Left -60 dB, right -12 dB");
    expect(meter.container.querySelector(".poodle-audio-meter-visual")?.getAttribute("aria-hidden")).toBe("true");
    const readout = render(<ValueReadout value={440} format={{ type: "hz" }} ariaLabel="Frequency" />);
    expect(readout.getByText("440 Hz")).toBeTruthy();
  });

  it("exposes keyboard-operable envelope points", () => {
    const onPointsCommit = vi.fn();
    const view = render(<EnvelopeEditor points={[{ id: "attack", x: 0.25, y: 0.5, curve: 0 }]} ariaLabel="Envelope" onPointsCommit={onPointsCommit} />);
    fireEvent.keyDown(view.getByRole("button", { name: /Point 1,/ }), { key: "ArrowUp" });
    expect(onPointsCommit.mock.calls.at(-1)?.[0][0]).toMatchObject({ id: "attack", y: 0.51 });
  });

  it("commits XY axes atomically", () => {
    const onValueCommit = vi.fn();
    const view = render(<XYPad x={0.25} y={0.75} ariaLabel="Position" onValueCommit={onValueCommit} />);
    fireEvent.keyDown(view.getByRole("slider", { name: "Position X" }), { key: "ArrowRight" });
    expect(onValueCommit).toHaveBeenCalledWith(0.26, 0.75);
  });

  it("preserves independent switch lamp state", () => {
    const onStateCommit = vi.fn();
    const view = render(<AudioSwitch mode="latch" state={0} lampOn ariaLabel="Bypass" onStateCommit={onStateCommit} />);
    const button = view.getByRole("button", { name: "Bypass" });
    fireEvent.keyDown(button, { key: "Enter" }); fireEvent.keyUp(button, { key: "Enter" });
    expect(onStateCommit).toHaveBeenCalledWith(1);
    expect(view.container.querySelector(".poodle-audio-switch-visual")?.getAttribute("data-lamp")).toBe("true");
  });

  it("reports positive gain reduction", () => {
    const context = createGainReductionMeterContext({ reductionDb: 12, ballisticDb: 12 });
    const view = render(<GainReductionMeter context={context} ariaLabel="Compression" />);
    expect(view.getByRole("meter", { name: "Compression" }).getAttribute("aria-valuetext")).toBe("12 dB reduction");
  });

  it("pairs computer-key notes and separates external highlights", () => {
    const onNoteOn = vi.fn(); const onNoteOff = vi.fn();
    const view = render(<Keyboard firstNote={48} lastNote={72} externalHeldNotes={[64]} onNoteOn={onNoteOn} onNoteOff={onNoteOff} ariaLabel="Keys" />);
    const keyboard = view.getByRole("toolbar", { name: "Keys" });
    fireEvent.keyDown(keyboard, { key: "a" }); fireEvent.keyUp(keyboard, { key: "a" });
    expect(onNoteOn).toHaveBeenCalledWith(60, 100); expect(onNoteOff).toHaveBeenCalledWith(60);
    expect(view.getByRole("button", { name: "E4" }).getAttribute("aria-pressed")).toBe("true");
  });

  it("retargets a captured pointer across keyboard keys", () => {
    const onNoteOn = vi.fn(); const onNoteOff = vi.fn();
    const view = render(<Keyboard firstNote={60} lastNote={62} onNoteOn={onNoteOn} onNoteOff={onNoteOff} ariaLabel="Keys" />);
    const keyboard = view.getByRole("toolbar", { name: "Keys" }) as HTMLDivElement;
    keyboard.getBoundingClientRect = () => ({ left: 0, top: 0, width: 100, height: 100, right: 100, bottom: 100, x: 0, y: 0, toJSON: () => ({}) });
    keyboard.setPointerCapture = vi.fn();
    fireEvent.pointerDown(keyboard, { pointerId: 1, button: 0, clientX: 10, clientY: 90 });
    fireEvent.pointerMove(keyboard, { pointerId: 1, clientX: 90, clientY: 90 });
    expect(onNoteOn.mock.calls).toEqual([[60, 114], [62, 114]]);
    expect(onNoteOff).toHaveBeenCalledWith(60);
  });

  it("moves the waveform cursor and exposes selection text", () => {
    const onCursorChange = vi.fn();
    const view = render(<WaveformDisplay pyramid={waveform} cursorSample={1} selection={{ start: 1, end: 2 }} onCursorChange={onCursorChange} ariaLabel="Clip" />);
    const slider = view.getByRole("slider"); fireEvent.keyDown(slider, { key: "ArrowRight" });
    expect(onCursorChange).toHaveBeenCalledWith(2); expect(slider.getAttribute("aria-valuetext")).toContain("selection 1 to 2");
  });

  it("navigates and toggles generic matrix cells", () => {
    const onCellCommit = vi.fn();
    const view = render(<ModMatrixGrid sources={matrixSources} destinations={matrixDestinations} onCellCommit={onCellCommit} ariaLabel="Routes" />);
    const cell = view.getByRole("gridcell"); fireEvent.focus(cell); fireEvent.keyDown(cell, { key: " " });
    expect(onCellCommit.mock.calls.at(-1)?.[0]).toMatchObject({ sourceId: "one", destinationId: "a", enabled: true });
  });

  it("resolves both presentation axes across the complete family", () => {
    const view = render(<>
      <Knob size="xl" density="comfortable" ariaLabel="Gain" />
      <Fader size="xl" density="comfortable" ariaLabel="Level" />
      <AudioMeter size="xl" density="comfortable" ariaLabel="Master meter" />
      <ValueReadout size="xl" density="comfortable" ariaLabel="Frequency" />
      <DragNumberField size="xl" density="comfortable" ariaLabel="Attack" />
      <EnvelopeEditor size="xl" density="comfortable" ariaLabel="Envelope" />
      <XYPad size="xl" density="comfortable" ariaLabel="Position" />
      <AudioSwitch size="xl" density="comfortable" ariaLabel="Bypass" />
      <GainReductionMeter size="xl" density="comfortable" ariaLabel="Compression" />
      <Keyboard size="xl" density="comfortable" ariaLabel="Keys" />
      <WaveformDisplay size="xl" density="comfortable" pyramid={waveform} ariaLabel="Clip" />
      <ModMatrixGrid size="xl" density="comfortable" sources={matrixSources} destinations={matrixDestinations} ariaLabel="Routes" />
    </>);
    const roots = [...view.container.querySelectorAll<HTMLElement>("[data-scope][data-part='root']")];
    expect(roots).toHaveLength(12);
    expect(roots.every((root) => root.dataset.size === "xl" && root.dataset.density === "comfortable")).toBe(true);
  });

  it("passes a runtime axe sweep for the complete family", async () => {
    const view = render(<>
      <Knob value={0.5} ariaLabel="Gain" />
      <Fader value={0.5} ariaLabel="Level" />
      <AudioMeter context={createAudioMeterContext()} ariaLabel="Master meter" />
      <ValueReadout value={440} format={{ type: "hz" }} ariaLabel="Frequency" />
      <DragNumberField value={250} ariaLabel="Attack" />
      <EnvelopeEditor points={[{ id: "attack", x: 0.25, y: 0.5, curve: 0 }]} ariaLabel="Envelope" />
      <XYPad x={0.25} y={0.75} ariaLabel="Position" />
      <AudioSwitch state={1} ariaLabel="Bypass" />
      <GainReductionMeter context={createGainReductionMeterContext()} ariaLabel="Compression" />
      <Keyboard ariaLabel="Keys" />
      <WaveformDisplay pyramid={waveform} ariaLabel="Clip" />
      <ModMatrixGrid sources={matrixSources} destinations={matrixDestinations} ariaLabel="Routes" />
    </>);
    const results = await axe.run(view.container, {
      resultTypes: ["violations"],
      rules: { region: { enabled: false }, "color-contrast": { enabled: false } },
    });
    expect(results.violations.map((violation) => violation.id)).toEqual([]);
  });
});

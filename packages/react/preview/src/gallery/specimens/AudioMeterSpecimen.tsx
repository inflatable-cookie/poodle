import { audioMeterTransition, createAudioMeterContext, type AudioMeterMode } from "@inflatable-cookie/poodle-core";
import { AudioMeter } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
const make = (mode: AudioMeterMode, peak: number, atMs = 300) => audioMeterTransition(createAudioMeterContext({ mode }), { type: "PUSH_FRAME", frame: { atMs, peak, meanSquare: peak * peak * .6, durationMs: atMs } }).context;
const vu = make("vu", .72); const ppm = make("ppm", .72, 10); const samplePeak = make("sample-peak", .86, 16); const rms = make("rms", .55); const clipped = make("sample-peak", 1.1, 16); const reset = audioMeterTransition(clipped, { type: "RESET_CLIP" }).context;
export function AudioMeterSpecimen() { return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <AudioMeter context={ppm} size={size} ariaLabel={`Meter ${size} size`} />}
      densities={(density) => <AudioMeter context={ppm} density={density} ariaLabel={`Meter ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <p style={{ margin: 0, color: "var(--poodle-color-text-secondary)", fontSize: "0.8125rem" }}>High-count consoles opt into the <a href="#components/meter-surface">MeterSurface specimen</a>; this page stays the standalone evidence.</p>
      <SpecimenGroup label="VU — 300 ms integration"><AudioMeter context={vu} ariaLabel="VU meter" /></SpecimenGroup>
  <SpecimenGroup label="PPM"><AudioMeter context={ppm} ariaLabel="PPM meter" /></SpecimenGroup>
  <SpecimenGroup label="Sample peak"><AudioMeter context={samplePeak} ariaLabel="Sample peak meter" /></SpecimenGroup>
  <SpecimenGroup label="RMS window"><AudioMeter context={rms} ariaLabel="RMS meter" /></SpecimenGroup>
  <SpecimenGroup label="Bar and segment styles"><Row><AudioMeter context={ppm} style="bar" orientation="horizontal" ariaLabel="Bar meter" /><AudioMeter context={ppm} style="segments" orientation="horizontal" ariaLabel="Segment meter" /></Row></SpecimenGroup>
  <SpecimenGroup label="Mono and stereo"><Row><AudioMeter context={ppm} ariaLabel="Mono meter" /><AudioMeter context={ppm} rightContext={rms} ariaLabel="Stereo meter" /></Row></SpecimenGroup>
  <SpecimenGroup label="Vertical and horizontal"><Row><AudioMeter context={ppm} ariaLabel="Vertical meter" /><AudioMeter context={ppm} orientation="horizontal" ariaLabel="Horizontal meter" /></Row></SpecimenGroup>
  <SpecimenGroup label="Peak hold"><AudioMeter context={samplePeak} ariaLabel="Peak hold meter" /></SpecimenGroup>
  <SpecimenGroup label="Clip latch and manual reset"><Row><AudioMeter context={clipped} ariaLabel="Clipped meter" /><AudioMeter context={reset} ariaLabel="Reset clip meter" /></Row></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

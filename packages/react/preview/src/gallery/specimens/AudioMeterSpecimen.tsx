import { audioMeterTransition, createAudioMeterContext, type AudioMeterMode } from "@inflatable-cookie/poodle-core";
import { AudioMeter } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
const make = (mode: AudioMeterMode, peak: number, atMs = 300) => audioMeterTransition(createAudioMeterContext({ mode }), { type: "PUSH_FRAME", frame: { atMs, peak, meanSquare: peak * peak * .6, durationMs: atMs } }).context;
const vu = make("vu", .72); const ppm = make("ppm", .72, 10); const samplePeak = make("sample-peak", .86, 16); const rms = make("rms", .55); const clipped = make("sample-peak", 1.1, 16); const reset = audioMeterTransition(clipped, { type: "RESET_CLIP" }).context;
export function AudioMeterSpecimen() { return <Page>
  <Group title="VU — 300 ms integration"><AudioMeter context={vu} ariaLabel="VU meter" /></Group>
  <Group title="PPM"><AudioMeter context={ppm} ariaLabel="PPM meter" /></Group>
  <Group title="Sample peak"><AudioMeter context={samplePeak} ariaLabel="Sample peak meter" /></Group>
  <Group title="RMS window"><AudioMeter context={rms} ariaLabel="RMS meter" /></Group>
  <Group title="Bar and segment styles"><Row><AudioMeter context={ppm} style="bar" orientation="horizontal" ariaLabel="Bar meter" /><AudioMeter context={ppm} style="segments" orientation="horizontal" ariaLabel="Segment meter" /></Row></Group>
  <Group title="Mono and stereo"><Row><AudioMeter context={ppm} ariaLabel="Mono meter" /><AudioMeter context={ppm} rightContext={rms} ariaLabel="Stereo meter" /></Row></Group>
  <Group title="Vertical and horizontal"><Row><AudioMeter context={ppm} ariaLabel="Vertical meter" /><AudioMeter context={ppm} orientation="horizontal" ariaLabel="Horizontal meter" /></Row></Group>
  <Group title="Peak hold"><AudioMeter context={samplePeak} ariaLabel="Peak hold meter" /></Group>
  <Group title="Clip latch and manual reset"><Row><AudioMeter context={clipped} ariaLabel="Clipped meter" /><AudioMeter context={reset} ariaLabel="Reset clip meter" /></Row></Group>
  <AudioAxes render={(props, label) => <AudioMeter {...props} context={ppm} ariaLabel={`Meter ${label}`} />} />
</Page>; }

import { createGainReductionMeterContext, gainReductionMeterTransition } from "@inflatable-cookie/poodle-core";
import { GainReductionMeter } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
const context = (value: number, enabled = true) => createGainReductionMeterContext({ reductionDb: value, ballisticDb: value, enabled });
const attacked = gainReductionMeterTransition(createGainReductionMeterContext(), { type: "PUSH_FRAME", frame: { atMs: 10, reductionDb: 18, durationMs: 10 } }).context;
const released = gainReductionMeterTransition(attacked, { type: "PUSH_FRAME", frame: { atMs: 310, reductionDb: 4, durationMs: 300 } }).context;
const invalid = gainReductionMeterTransition(context(12), { type: "PUSH_FRAME", frame: { atMs: Number.NaN, reductionDb: -1, durationMs: 0 } }).context;
export function GainReductionMeterSpecimen() { return <Page>
  <Group title="No reduction"><GainReductionMeter context={context(0)} ariaLabel="No gain reduction" /></Group>
  <Group title="Attack"><GainReductionMeter context={attacked} ariaLabel="Attack response" /></Group>
  <Group title="Release"><GainReductionMeter context={released} ariaLabel="Release response" /></Group>
  <Group title="Maximum reduction"><GainReductionMeter context={context(30)} ariaLabel="Maximum reduction" /></Group>
  <Group title="Bar and segment styles"><Row><GainReductionMeter context={context(12)} style="bar" orientation="horizontal" ariaLabel="Bar reduction" /><GainReductionMeter context={context(12)} style="segments" orientation="horizontal" ariaLabel="Segment reduction" /></Row></Group>
  <Group title="Vertical and horizontal"><Row><GainReductionMeter context={context(12)} ariaLabel="Vertical reduction" /><GainReductionMeter context={context(12)} orientation="horizontal" ariaLabel="Horizontal reduction" /></Row></Group>
  <Group title="Invalid-frame rejection"><GainReductionMeter context={invalid} ariaLabel="Invalid frame rejected" /></Group>
  <Group title="Reset"><GainReductionMeter context={context(0)} ariaLabel="Reset reduction" /></Group>
  <Group title="Disabled"><GainReductionMeter context={context(12, false)} ariaLabel="Disabled reduction" /></Group>
  <AudioAxes render={(props, label) => <GainReductionMeter {...props} context={context(12)} ariaLabel={`Gain reduction ${label}`} />} />
</Page>; }

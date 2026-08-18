import { createGainReductionMeterContext, gainReductionMeterTransition } from "@inflatable-cookie/poodle-core";
import { GainReductionMeter } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
const context = (value: number, enabled = true) => createGainReductionMeterContext({ reductionDb: value, ballisticDb: value, enabled });
const attacked = gainReductionMeterTransition(createGainReductionMeterContext(), { type: "PUSH_FRAME", frame: { atMs: 10, reductionDb: 18, durationMs: 10 } }).context;
const released = gainReductionMeterTransition(attacked, { type: "PUSH_FRAME", frame: { atMs: 310, reductionDb: 4, durationMs: 300 } }).context;
const invalid = gainReductionMeterTransition(context(12), { type: "PUSH_FRAME", frame: { atMs: Number.NaN, reductionDb: -1, durationMs: 0 } }).context;
export function GainReductionMeterSpecimen() { return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <GainReductionMeter context={context(12)} size={size} ariaLabel={`Gain reduction ${size} size`} />}
      densities={(density) => <GainReductionMeter context={context(12)} density={density} ariaLabel={`Gain reduction ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="No reduction"><GainReductionMeter context={context(0)} ariaLabel="No gain reduction" /></SpecimenGroup>
  <SpecimenGroup label="Attack"><GainReductionMeter context={attacked} ariaLabel="Attack response" /></SpecimenGroup>
  <SpecimenGroup label="Release"><GainReductionMeter context={released} ariaLabel="Release response" /></SpecimenGroup>
  <SpecimenGroup label="Maximum reduction"><GainReductionMeter context={context(30)} ariaLabel="Maximum reduction" /></SpecimenGroup>
  <SpecimenGroup label="Bar and segment styles"><Row><GainReductionMeter context={context(12)} style="bar" orientation="horizontal" ariaLabel="Bar reduction" /><GainReductionMeter context={context(12)} style="segments" orientation="horizontal" ariaLabel="Segment reduction" /></Row></SpecimenGroup>
  <SpecimenGroup label="Vertical and horizontal"><Row><GainReductionMeter context={context(12)} ariaLabel="Vertical reduction" /><GainReductionMeter context={context(12)} orientation="horizontal" ariaLabel="Horizontal reduction" /></Row></SpecimenGroup>
  <SpecimenGroup label="Invalid-frame rejection"><GainReductionMeter context={invalid} ariaLabel="Invalid frame rejected" /></SpecimenGroup>
  <SpecimenGroup label="Reset"><GainReductionMeter context={context(0)} ariaLabel="Reset reduction" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><GainReductionMeter context={context(12, false)} ariaLabel="Disabled reduction" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

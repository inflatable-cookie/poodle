import { ValueReadout } from "@inflatable-cookie/poodle-react";
import {AudioAxes, AudioSpecimenPage as Page, AudioSpecimenRow as Row} from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
export function ValueReadoutSpecimen() { return <Page>
  <SpecimenGroup label="Number"><ValueReadout value={12.345} min={-20_000} max={20_000} format={{ type: "number", decimals: 2 }} ariaLabel="Number" /></SpecimenGroup>
  <SpecimenGroup label="dB"><ValueReadout value={-12.4} min={-60} max={6} format={{ type: "db" }} ariaLabel="Gain" /></SpecimenGroup>
  <SpecimenGroup label="Hz / kHz"><ValueReadout value={12_500} min={20} max={20_000} format={{ type: "hz" }} ariaLabel="Frequency" /></SpecimenGroup>
  <SpecimenGroup label="Percent"><ValueReadout value={0.625} format={{ type: "percent" }} ariaLabel="Mix" /></SpecimenGroup>
  <SpecimenGroup label="Ratio"><ValueReadout value={4} min={1} max={20} format={{ type: "ratio" }} ariaLabel="Ratio" /></SpecimenGroup>
  <SpecimenGroup label="Milliseconds"><ValueReadout value={1250} min={0} max={5000} format={{ type: "milliseconds" }} ariaLabel="Time" /></SpecimenGroup>
  <SpecimenGroup label="Note name"><ValueReadout value={60} min={0} max={127} format={{ type: "note" }} ariaLabel="Note" /></SpecimenGroup>
  <SpecimenGroup label="Semitones"><ValueReadout value={-7} min={-24} max={24} format={{ type: "semitones" }} ariaLabel="Transpose" /></SpecimenGroup>
  <SpecimenGroup label="Negative / boundary / disabled"><Row><ValueReadout value={-1} min={-1} max={1} /><ValueReadout value={1} min={-1} max={1} /><ValueReadout value={0} disabled ariaLabel="Disabled readout" /></Row></SpecimenGroup>
  <AudioAxes render={(props, label) => <ValueReadout {...props} value={-12.4} format={{ type: "db" }} ariaLabel={`Readout ${label}`} />} />
</Page>; }

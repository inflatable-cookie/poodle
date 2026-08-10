import { ValueReadout } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function ValueReadoutSpecimen() { return <Page>
  <Group title="Number"><ValueReadout value={12.345} min={-20_000} max={20_000} format={{ type: "number", decimals: 2 }} ariaLabel="Number" /></Group>
  <Group title="dB"><ValueReadout value={-12.4} min={-60} max={6} format={{ type: "db" }} ariaLabel="Gain" /></Group>
  <Group title="Hz / kHz"><ValueReadout value={12_500} min={20} max={20_000} format={{ type: "hz" }} ariaLabel="Frequency" /></Group>
  <Group title="Percent"><ValueReadout value={0.625} format={{ type: "percent" }} ariaLabel="Mix" /></Group>
  <Group title="Ratio"><ValueReadout value={4} min={1} max={20} format={{ type: "ratio" }} ariaLabel="Ratio" /></Group>
  <Group title="Milliseconds"><ValueReadout value={1250} min={0} max={5000} format={{ type: "milliseconds" }} ariaLabel="Time" /></Group>
  <Group title="Note name"><ValueReadout value={60} min={0} max={127} format={{ type: "note" }} ariaLabel="Note" /></Group>
  <Group title="Semitones"><ValueReadout value={-7} min={-24} max={24} format={{ type: "semitones" }} ariaLabel="Transpose" /></Group>
  <Group title="Negative / boundary / disabled"><Row><ValueReadout value={-1} min={-1} max={1} /><ValueReadout value={1} min={-1} max={1} /><ValueReadout value={0} disabled ariaLabel="Disabled readout" /></Row></Group>
  <AudioAxes render={(props, label) => <ValueReadout {...props} value={-12.4} format={{ type: "db" }} ariaLabel={`Readout ${label}`} />} />
</Page>; }

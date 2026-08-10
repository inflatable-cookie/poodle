import { useState } from "react";
import { Knob } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function KnobSpecimen() {
  const [gain, setGain] = useState(-12); const [frequency, setFrequency] = useState(1000); const [stepped, setStepped] = useState(4);
  return <Page>
    <Group title="Linear / default reset"><Knob value={gain} onValueChange={setGain} min={-60} max={6} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" /></Group>
    <Group title="Logarithmic frequency"><Knob value={frequency} onValueChange={setFrequency} min={20} max={20_000} law={{ type: "logarithmic" }} defaultValue={440} keyboardStep={10} format={{ type: "hz" }} ariaLabel="Frequency" /></Group>
    <Group title="Bipolar center"><Knob value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} keyboardStep={0.1} ariaLabel="Pan" /></Group>
    <Group title="Stepped values"><Knob value={stepped} onValueChange={setStepped} min={0} max={10} law={{ type: "stepped", step: 1 }} keyboardStep={1} ariaLabel="Stepped value" /></Group>
    <Group title="Fine drag (Shift)"><Knob value={0.42} ariaLabel="Fine drag" /></Group>
    <Group title="Circular mode"><Knob value={0.8} dragMode="circular" ariaLabel="Circular drag" /></Group>
    <Group title="Automation state"><Knob value={0.7} automation="writing" ariaLabel="Automated value" /></Group>
    <Group title="Type-in (Enter)"><Knob value={0.25} ariaLabel="Type-in value" /></Group>
    <Group title="Keyboard bounds (Home / End)"><Row><Knob value={0} ariaLabel="Minimum" /><Knob value={1} ariaLabel="Maximum" /></Row></Group>
    <Group title="Disabled"><Knob value={0.5} disabled ariaLabel="Disabled knob" /></Group>
    <AudioAxes render={(props, label) => <Knob {...props} value={0.6} ariaLabel={`Knob ${label}`} />} />
  </Page>;
}

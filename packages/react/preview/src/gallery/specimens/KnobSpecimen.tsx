import { useState } from "react";
import { Knob } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function KnobSpecimen() {
  const [gain, setGain] = useState(-12); const [frequency, setFrequency] = useState(1000); const [stepped, setStepped] = useState(4);
  return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <Knob value={0.6} size={size} ariaLabel={`Knob ${size} size`} />}
      densities={(density) => <Knob value={0.6} density={density} ariaLabel={`Knob ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Linear / default reset"><Knob value={gain} onValueChange={setGain} min={-60} max={6} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" /></SpecimenGroup>
    <SpecimenGroup label="Logarithmic frequency"><Knob value={frequency} onValueChange={setFrequency} min={20} max={20_000} law={{ type: "logarithmic" }} defaultValue={440} keyboardStep={10} format={{ type: "hz" }} ariaLabel="Frequency" /></SpecimenGroup>
    <SpecimenGroup label="Bipolar center"><Knob value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} keyboardStep={0.1} ariaLabel="Pan" /></SpecimenGroup>
    <SpecimenGroup label="Stepped values"><Knob value={stepped} onValueChange={setStepped} min={0} max={10} law={{ type: "stepped", step: 1 }} keyboardStep={1} ariaLabel="Stepped value" /></SpecimenGroup>
    <SpecimenGroup label="Fine drag (Shift)"><Knob value={0.42} ariaLabel="Fine drag" /></SpecimenGroup>
    <SpecimenGroup label="Circular mode"><Knob value={0.8} dragMode="circular" ariaLabel="Circular drag" /></SpecimenGroup>
    <SpecimenGroup label="Automation state"><Knob value={0.7} automation="writing" ariaLabel="Automated value" /></SpecimenGroup>
    <SpecimenGroup label="Type-in (Enter)"><Knob value={0.25} ariaLabel="Type-in value" /></SpecimenGroup>
    <SpecimenGroup label="Keyboard bounds (Home / End)"><Row><Knob value={0} ariaLabel="Minimum" /><Knob value={1} ariaLabel="Maximum" /></Row></SpecimenGroup>
    <SpecimenGroup label="Disabled"><Knob value={0.5} disabled ariaLabel="Disabled knob" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

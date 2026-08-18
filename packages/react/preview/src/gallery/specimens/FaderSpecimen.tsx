import { useState } from "react";
import { Fader } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function FaderSpecimen() {
  const [gain, setGain] = useState(0); const [frequency, setFrequency] = useState(1000);
  return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <Fader value={0.6} size={size} ariaLabel={`Fader ${size} size`} />}
      densities={(density) => <Fader value={0.6} density={density} ariaLabel={`Fader ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Vertical and horizontal"><Row><Fader value={gain} onValueChange={setGain} min={-60} max={12} format={{ type: "db" }} ariaLabel="Vertical gain" /><Fader value={0.65} orientation="horizontal" ariaLabel="Horizontal fader" /></Row></SpecimenGroup>
    <SpecimenGroup label="Linear / log / bipolar laws"><Row><Fader value={0.4} ariaLabel="Linear fader" /><Fader value={frequency} onValueChange={setFrequency} min={20} max={20_000} law={{ type: "logarithmic" }} format={{ type: "hz" }} ariaLabel="Log fader" /><Fader value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} ariaLabel="Bipolar fader" /></Row></SpecimenGroup>
    <SpecimenGroup label="Detents"><Fader value={0.5} detents={[0.25, 0.5, 0.75]} detentSnap={0.03} ariaLabel="Detented fader" /></SpecimenGroup>
    <SpecimenGroup label="Coarse / fine drag (Shift)"><Fader value={0.4} ariaLabel="Fine-adjust fader" /></SpecimenGroup>
    <SpecimenGroup label="Automation touch"><Fader value={0.7} automation="touched" ariaLabel="Touched automation" /></SpecimenGroup>
    <SpecimenGroup label="Type-in (Enter)"><Fader value={0.25} ariaLabel="Type-in fader" /></SpecimenGroup>
    <SpecimenGroup label="Keyboard bounds"><Row><Fader value={0} ariaLabel="Minimum fader" /><Fader value={1} ariaLabel="Maximum fader" /></Row></SpecimenGroup>
    <SpecimenGroup label="Disabled"><Fader value={0.5} disabled ariaLabel="Disabled fader" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

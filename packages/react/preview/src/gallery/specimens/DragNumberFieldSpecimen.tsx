import { useState } from "react";
import { DragNumberField } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function DragNumberFieldSpecimen() { const [value, setValue] = useState(.5); const [integer, setInteger] = useState(4); const [gain, setGain] = useState(-12.4); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <DragNumberField value={-12.4} format={{ type: "db" }} size={size} ariaLabel={`Drag number ${size} size`} />}
      densities={(density) => <DragNumberField value={-12.4} format={{ type: "db" }} density={density} ariaLabel={`Drag number ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Default"><DragNumberField value={value} onValueChange={setValue} min={0} max={1} step={.01} ariaLabel="Default value" /></SpecimenGroup>
  <SpecimenGroup label="Integer step"><DragNumberField value={integer} onValueChange={setInteger} min={0} max={10} step={1} ariaLabel="Integer value" /></SpecimenGroup>
  <SpecimenGroup label="Formatted dB"><DragNumberField value={gain} onValueChange={setGain} min={-60} max={12} step={.1} format={{ type: "db" }} ariaLabel="Gain" /></SpecimenGroup>
  <SpecimenGroup label="Coarse / fine drag (Shift)"><DragNumberField value={.4} min={0} max={1} step={.01} ariaLabel="Drag value" /></SpecimenGroup>
  <SpecimenGroup label="Direct entry (click)"><DragNumberField value={250} min={0} max={5000} step={1} format={{ type: "milliseconds" }} ariaLabel="Attack time" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard bounds (Home / End)"><Row><DragNumberField value={0} min={0} max={1} step={.1} ariaLabel="Minimum" /><DragNumberField value={1} min={0} max={1} step={.1} ariaLabel="Maximum" /></Row></SpecimenGroup>
  <SpecimenGroup label="Negative range"><DragNumberField value={-7} min={-24} max={24} step={1} format={{ type: "semitones" }} ariaLabel="Transpose" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><DragNumberField value={.5} disabled ariaLabel="Disabled value" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

import { useState } from "react";
import { DragNumberField } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function DragNumberFieldSpecimen() { const [value, setValue] = useState(.5); const [integer, setInteger] = useState(4); const [gain, setGain] = useState(-12.4); return <Page>
  <Group title="Default"><DragNumberField value={value} onValueChange={setValue} min={0} max={1} step={.01} ariaLabel="Default value" /></Group>
  <Group title="Integer step"><DragNumberField value={integer} onValueChange={setInteger} min={0} max={10} step={1} ariaLabel="Integer value" /></Group>
  <Group title="Formatted dB"><DragNumberField value={gain} onValueChange={setGain} min={-60} max={12} step={.1} format={{ type: "db" }} ariaLabel="Gain" /></Group>
  <Group title="Coarse / fine drag (Shift)"><DragNumberField value={.4} min={0} max={1} step={.01} ariaLabel="Drag value" /></Group>
  <Group title="Direct entry (click)"><DragNumberField value={250} min={0} max={5000} step={1} format={{ type: "milliseconds" }} ariaLabel="Attack time" /></Group>
  <Group title="Keyboard bounds (Home / End)"><Row><DragNumberField value={0} min={0} max={1} step={.1} ariaLabel="Minimum" /><DragNumberField value={1} min={0} max={1} step={.1} ariaLabel="Maximum" /></Row></Group>
  <Group title="Negative range"><DragNumberField value={-7} min={-24} max={24} step={1} format={{ type: "semitones" }} ariaLabel="Transpose" /></Group>
  <Group title="Disabled"><DragNumberField value={.5} disabled ariaLabel="Disabled value" /></Group>
  <AudioAxes render={(props, label) => <DragNumberField {...props} value={-12.4} format={{ type: "db" }} ariaLabel={`Drag number ${label}`} />} />
</Page>; }

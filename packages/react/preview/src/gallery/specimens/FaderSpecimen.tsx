import { useState } from "react";
import { Fader } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function FaderSpecimen() {
  const [gain, setGain] = useState(0); const [frequency, setFrequency] = useState(1000);
  return <Page>
    <Group title="Vertical and horizontal"><Row><Fader value={gain} onValueChange={setGain} min={-60} max={12} format={{ type: "db" }} ariaLabel="Vertical gain" /><Fader value={0.65} orientation="horizontal" ariaLabel="Horizontal fader" /></Row></Group>
    <Group title="Linear / log / bipolar laws"><Row><Fader value={0.4} ariaLabel="Linear fader" /><Fader value={frequency} onValueChange={setFrequency} min={20} max={20_000} law={{ type: "logarithmic" }} format={{ type: "hz" }} ariaLabel="Log fader" /><Fader value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} ariaLabel="Bipolar fader" /></Row></Group>
    <Group title="Detents"><Fader value={0.5} detents={[0.25, 0.5, 0.75]} detentSnap={0.03} ariaLabel="Detented fader" /></Group>
    <Group title="Coarse / fine drag (Shift)"><Fader value={0.4} ariaLabel="Fine-adjust fader" /></Group>
    <Group title="Automation touch"><Fader value={0.7} automation="touched" ariaLabel="Touched automation" /></Group>
    <Group title="Type-in (Enter)"><Fader value={0.25} ariaLabel="Type-in fader" /></Group>
    <Group title="Keyboard bounds"><Row><Fader value={0} ariaLabel="Minimum fader" /><Fader value={1} ariaLabel="Maximum fader" /></Row></Group>
    <Group title="Disabled"><Fader value={0.5} disabled ariaLabel="Disabled fader" /></Group>
    <AudioAxes render={(props, label) => <Fader {...props} value={0.6} ariaLabel={`Fader ${label}`} />} />
  </Page>;
}

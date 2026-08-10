import { useState } from "react";
import { AudioSwitch } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
export function AudioSwitchSpecimen() { const [latch, setLatch] = useState(0); const [momentary, setMomentary] = useState(0); const [multi, setMulti] = useState(1); return <Page>
  <Group title="Off / on latch"><Row><AudioSwitch state={0} ariaLabel="Off latch" /><AudioSwitch state={latch} onStateChange={setLatch} ariaLabel="Interactive latch" /><AudioSwitch state={1} ariaLabel="On latch" /></Row></Group>
  <Group title="Held / released momentary"><AudioSwitch mode="momentary" state={momentary} onStateChange={setMomentary} ariaLabel="Momentary trigger" /></Group>
  <Group title="Three-state cycle with labels"><AudioSwitch mode="multi" state={multi} onStateChange={setMulti} stateCount={3} stateLabels={["Low", "Mid", "High"]} ariaLabel="Range" /></Group>
  <Group title="Lamp override"><Row><AudioSwitch state={0} lampOn ariaLabel="Off with lamp" /><AudioSwitch state={1} lampOn={false} ariaLabel="On without lamp" /></Row></Group>
  <Group title="Pressed / focused"><AudioSwitch state={1} ariaLabel="Focus and press switch" /></Group>
  <Group title="Disabled"><AudioSwitch state={1} disabled ariaLabel="Disabled switch" /></Group>
  <AudioAxes render={(props, label) => <AudioSwitch {...props} state={1} ariaLabel={`Audio switch ${label}`} />} />
</Page>; }

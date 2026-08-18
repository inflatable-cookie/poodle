import { useState } from "react";
import { AudioSwitch } from "@inflatable-cookie/poodle-react";
import {AudioAxes, AudioSpecimenPage as Page, AudioSpecimenRow as Row} from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
export function AudioSwitchSpecimen() { const [latch, setLatch] = useState(0); const [momentary, setMomentary] = useState(0); const [multi, setMulti] = useState(1); return <Page>
  <SpecimenGroup label="Off / on latch"><Row><AudioSwitch state={0} ariaLabel="Off latch" /><AudioSwitch state={latch} onStateChange={setLatch} ariaLabel="Interactive latch" /><AudioSwitch state={1} ariaLabel="On latch" /></Row></SpecimenGroup>
  <SpecimenGroup label="Held / released momentary"><AudioSwitch mode="momentary" state={momentary} onStateChange={setMomentary} ariaLabel="Momentary trigger" /></SpecimenGroup>
  <SpecimenGroup label="Three-state cycle with labels"><AudioSwitch mode="multi" state={multi} onStateChange={setMulti} stateCount={3} stateLabels={["Low", "Mid", "High"]} ariaLabel="Range" /></SpecimenGroup>
  <SpecimenGroup label="Lamp override"><Row><AudioSwitch state={0} lampOn ariaLabel="Off with lamp" /><AudioSwitch state={1} lampOn={false} ariaLabel="On without lamp" /></Row></SpecimenGroup>
  <SpecimenGroup label="Pressed / focused"><AudioSwitch state={1} ariaLabel="Focus and press switch" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><AudioSwitch state={1} disabled ariaLabel="Disabled switch" /></SpecimenGroup>
  <AudioAxes render={(props, label) => <AudioSwitch {...props} state={1} ariaLabel={`Audio switch ${label}`} />} />
</Page>; }

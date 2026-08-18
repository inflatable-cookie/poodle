import { useState } from "react";
import { AudioSwitch } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenRow as Row } from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function AudioSwitchSpecimen() { const [latch, setLatch] = useState(0); const [momentary, setMomentary] = useState(0); const [multi, setMulti] = useState(1); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <AudioSwitch state={1} size={size} ariaLabel={`Audio switch ${size} size`} />}
      densities={(density) => <AudioSwitch state={1} density={density} ariaLabel={`Audio switch ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Off / on latch"><Row><AudioSwitch state={0} ariaLabel="Off latch" /><AudioSwitch state={latch} onStateChange={setLatch} ariaLabel="Interactive latch" /><AudioSwitch state={1} ariaLabel="On latch" /></Row></SpecimenGroup>
  <SpecimenGroup label="Held / released momentary"><AudioSwitch mode="momentary" state={momentary} onStateChange={setMomentary} ariaLabel="Momentary trigger" /></SpecimenGroup>
  <SpecimenGroup label="Three-state cycle with labels"><AudioSwitch mode="multi" state={multi} onStateChange={setMulti} stateCount={3} stateLabels={["Low", "Mid", "High"]} ariaLabel="Range" /></SpecimenGroup>
  <SpecimenGroup label="Lamp override"><Row><AudioSwitch state={0} lampOn ariaLabel="Off with lamp" /><AudioSwitch state={1} lampOn={false} ariaLabel="On without lamp" /></Row></SpecimenGroup>
  <SpecimenGroup label="Pressed / focused"><AudioSwitch state={1} ariaLabel="Focus and press switch" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><AudioSwitch state={1} disabled ariaLabel="Disabled switch" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

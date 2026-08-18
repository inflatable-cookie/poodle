import { useState } from "react";
import { Keyboard } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
export function KeyboardSpecimen() { const [held, setHeld] = useState([60, 64, 67]); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} size={size} ariaLabel={`Keyboard ${size} size`} />}
      densities={(density) => <Keyboard firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} density={density} ariaLabel={`Keyboard ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Horizontal input / local chord"><Keyboard externalHeldNotes={held} onNoteOn={(note) => setHeld((notes) => [...new Set([...notes, note])])} onNoteOff={(note) => setHeld((notes) => notes.filter((value) => value !== note))} ariaLabel="Playable keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Vertical piano-roll gutter"><Keyboard orientation="vertical" firstNote={48} lastNote={60} externalHeldNotes={[52, 55]} ariaLabel="Pitch gutter" /></SpecimenGroup>
  <SpecimenGroup label="Velocity depth"><Keyboard firstNote={60} lastNote={72} ariaLabel="Velocity keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Computer keys / octave shift"><Keyboard firstNote={60} lastNote={84} octaveShift={1} ariaLabel="Computer-key octave" /></SpecimenGroup>
  <SpecimenGroup label="External playback highlight"><Keyboard externalHeldNotes={[61, 65, 68]} ariaLabel="Host playback" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><Keyboard disabled externalHeldNotes={[60, 64]} ariaLabel="Disabled keyboard" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

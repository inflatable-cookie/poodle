import { useState } from "react";
import { Keyboard } from "@inflatable-cookie/poodle-react";
import {AudioAxes, AudioSpecimenPage as Page} from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
export function KeyboardSpecimen() { const [held, setHeld] = useState([60, 64, 67]); return <Page>
  <SpecimenGroup label="Horizontal input / local chord"><Keyboard externalHeldNotes={held} onNoteOn={(note) => setHeld((notes) => [...new Set([...notes, note])])} onNoteOff={(note) => setHeld((notes) => notes.filter((value) => value !== note))} ariaLabel="Playable keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Vertical piano-roll gutter"><Keyboard orientation="vertical" firstNote={48} lastNote={60} externalHeldNotes={[52, 55]} ariaLabel="Pitch gutter" /></SpecimenGroup>
  <SpecimenGroup label="Velocity depth"><Keyboard firstNote={60} lastNote={72} ariaLabel="Velocity keyboard" /></SpecimenGroup>
  <SpecimenGroup label="Computer keys / octave shift"><Keyboard firstNote={60} lastNote={84} octaveShift={1} ariaLabel="Computer-key octave" /></SpecimenGroup>
  <SpecimenGroup label="External playback highlight"><Keyboard externalHeldNotes={[61, 65, 68]} ariaLabel="Host playback" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><Keyboard disabled externalHeldNotes={[60, 64]} ariaLabel="Disabled keyboard" /></SpecimenGroup>
  <AudioAxes render={(props, label) => <Keyboard {...props} firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} ariaLabel={`Keyboard ${label}`} />} />
</Page>; }

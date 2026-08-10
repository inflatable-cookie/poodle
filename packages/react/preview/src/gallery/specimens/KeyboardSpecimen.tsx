import { useState } from "react";
import { Keyboard } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page } from "./AudioSpecimen";
export function KeyboardSpecimen() { const [held, setHeld] = useState([60, 64, 67]); return <Page>
  <Group title="Horizontal input / local chord"><Keyboard externalHeldNotes={held} onNoteOn={(note) => setHeld((notes) => [...new Set([...notes, note])])} onNoteOff={(note) => setHeld((notes) => notes.filter((value) => value !== note))} ariaLabel="Playable keyboard" /></Group>
  <Group title="Vertical piano-roll gutter"><Keyboard orientation="vertical" firstNote={48} lastNote={60} externalHeldNotes={[52, 55]} ariaLabel="Pitch gutter" /></Group>
  <Group title="Velocity depth"><Keyboard firstNote={60} lastNote={72} ariaLabel="Velocity keyboard" /></Group>
  <Group title="Computer keys / octave shift"><Keyboard firstNote={60} lastNote={84} octaveShift={1} ariaLabel="Computer-key octave" /></Group>
  <Group title="External playback highlight"><Keyboard externalHeldNotes={[61, 65, 68]} ariaLabel="Host playback" /></Group>
  <Group title="Disabled"><Keyboard disabled externalHeldNotes={[60, 64]} ariaLabel="Disabled keyboard" /></Group>
  <AudioAxes render={(props, label) => <Keyboard {...props} firstNote={60} lastNote={72} externalHeldNotes={[60, 64]} ariaLabel={`Keyboard ${label}`} />} />
</Page>; }

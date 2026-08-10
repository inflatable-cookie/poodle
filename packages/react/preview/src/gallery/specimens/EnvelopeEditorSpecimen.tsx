import { useState } from "react";
import type { EnvelopePoint } from "@inflatable-cookie/poodle-core";
import { EnvelopeEditor } from "@inflatable-cookie/poodle-react";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page, AudioSpecimenRow as Row } from "./AudioSpecimen";
const adsr: EnvelopePoint[] = [{ id: "start", x: 0, y: 0, curve: .35 }, { id: "attack", x: .18, y: 1, curve: -.25 }, { id: "decay", x: .48, y: .62, curve: .2 }, { id: "sustain", x: .82, y: .62, curve: 0 }, { id: "end", x: 1, y: 0, curve: 0 }];
const curved: EnvelopePoint[] = [{ id: "a", x: 0, y: 0, curve: .7 }, { id: "b", x: .5, y: 1, curve: -.7 }, { id: "c", x: 1, y: 0, curve: 0 }];
const flat: EnvelopePoint[] = [{ id: "a", x: 0, y: .5, curve: .8 }, { id: "b", x: 1, y: .5, curve: 0 }];
export function EnvelopeEditorSpecimen() { const [points, setPoints] = useState(adsr); return <Page>
  <Group title="ADSR-like default"><EnvelopeEditor points={points} onPointsChange={setPoints} ariaLabel="ADSR envelope" /></Group>
  <Group title="Positive / negative curves"><EnvelopeEditor points={curved} ariaLabel="Curved envelope" /></Group>
  <Group title="Selected / dragging, add / remove"><Row><span>Drag points; double-click to add or remove.</span><EnvelopeEditor points={[...adsr]} ariaLabel="Editable envelope" /></Row></Group>
  <Group title="Snapped movement"><EnvelopeEditor points={[...adsr]} snapPoint={(point) => ({ x: Math.round(point.x * 20) / 20, y: Math.round(point.y * 20) / 20 })} ariaLabel="Snapped envelope" /></Group>
  <Group title="Keyboard and curve nudges"><EnvelopeEditor points={[...adsr]} step={.05} ariaLabel="Keyboard envelope" /></Group>
  <Group title="Flat-segment regression"><EnvelopeEditor points={flat} ariaLabel="Flat envelope" /></Group>
  <Group title="Disabled"><EnvelopeEditor points={adsr} disabled ariaLabel="Disabled envelope" /></Group>
  <AudioAxes render={(props, label) => <EnvelopeEditor {...props} points={adsr} ariaLabel={`Envelope ${label}`} />} />
</Page>; }

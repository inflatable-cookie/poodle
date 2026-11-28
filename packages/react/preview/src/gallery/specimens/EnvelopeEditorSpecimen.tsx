import { useState } from "react";
import type { EnvelopePoint } from "@inflatable-cookie/poodle-core";
import { EnvelopeEditor } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
const adsr: EnvelopePoint[] = [{ id: "start", x: 0, y: 0, curve: .35 }, { id: "attack", x: .18, y: 1, curve: -.25 }, { id: "decay", x: .48, y: .62, curve: .2 }, { id: "sustain", x: .82, y: .62, curve: 0 }, { id: "end", x: 1, y: 0, curve: 0 }];
const curved: EnvelopePoint[] = [{ id: "a", x: 0, y: 0, curve: .7 }, { id: "b", x: .5, y: 1, curve: -.7 }, { id: "c", x: 1, y: 0, curve: 0 }];
const flat: EnvelopePoint[] = [{ id: "a", x: 0, y: .5, curve: .8 }, { id: "b", x: 1, y: .5, curve: 0 }];
const noteStyle = { margin: 0, color: "var(--poodle-color-text-secondary)", fontSize: "0.75rem" } as const;
export function EnvelopeEditorSpecimen() { const [points, setPoints] = useState(adsr); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <EnvelopeEditor points={adsr} size={size} ariaLabel={`Envelope ${size} size`} />}
      densities={(density) => <EnvelopeEditor points={adsr} density={density} ariaLabel={`Envelope ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="ADSR-like default"><EnvelopeEditor points={points} onPointsChange={setPoints} ariaLabel="ADSR envelope" /></SpecimenGroup>
  <SpecimenGroup label="Positive / negative curves"><EnvelopeEditor points={curved} ariaLabel="Curved envelope" /></SpecimenGroup>
  <SpecimenGroup label="Selected / dragging, add / remove"><p style={noteStyle}>Drag points; double-click the surface to add and a point to remove.</p><EnvelopeEditor points={[...adsr]} ariaLabel="Editable envelope" /></SpecimenGroup>
  <SpecimenGroup label="Snapped movement"><EnvelopeEditor points={[...adsr]} snapPoint={(point) => ({ x: Math.round(point.x * 20) / 20, y: Math.round(point.y * 20) / 20 })} ariaLabel="Snapped envelope" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard and curve nudges"><p style={noteStyle}>Focus a point; use arrows, Shift+arrows, and Page Up/Down.</p><EnvelopeEditor points={[...adsr]} step={.05} ariaLabel="Keyboard envelope" /></SpecimenGroup>
  <SpecimenGroup label="Flat-segment regression"><EnvelopeEditor points={flat} ariaLabel="Flat envelope" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><EnvelopeEditor points={adsr} disabled ariaLabel="Disabled envelope" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

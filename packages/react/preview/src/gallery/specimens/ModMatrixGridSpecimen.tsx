import { useState } from "react";
import { ModMatrixGrid } from "@inflatable-cookie/poodle-react";
import type { ModMatrixCell, ModMatrixHeader } from "@inflatable-cookie/poodle-core";
import { AudioAxes, AudioSpecimenGroup as Group, AudioSpecimenPage as Page } from "./AudioSpecimen";
const sources: ModMatrixHeader[] = [{ id: "one", label: "Source 1" }, { id: "two", label: "Source 2" }, { id: "three", label: "Source 3" }];
const destinations: ModMatrixHeader[] = [{ id: "a", label: "Dest A" }, { id: "b", label: "Dest B" }, { id: "c", label: "Dest C" }];
const initial: ModMatrixCell[] = [{ sourceId: "one", destinationId: "a", amount: .75, enabled: true }, { sourceId: "one", destinationId: "b", amount: -.5, enabled: true }, { sourceId: "one", destinationId: "c", amount: .35, enabled: true, parameters: { min: 0, max: 1, step: .05 } }, { sourceId: "two", destinationId: "c", amount: 0, enabled: true }];
export function ModMatrixGridSpecimen() { const [cells] = useState(initial); return <Page>
  <Group title="Sparse generic matrix"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} ariaLabel="Generic modulation matrix" /></Group>
  <Group title="Bipolar / negative / unipolar"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} ariaLabel="Mixed parameter amounts" /></Group>
  <Group title="Keyboard navigation and toggle"><ModMatrixGrid sources={sources.slice(0, 2)} destinations={destinations} cells={cells} ariaLabel="Keyboard matrix" /></Group>
  <Group title="Empty axes"><ModMatrixGrid ariaLabel="Empty matrix" /></Group>
  <Group title="Disabled"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} disabled ariaLabel="Disabled matrix" /></Group>
  <AudioAxes render={(props, label) => <ModMatrixGrid {...props} sources={sources.slice(0,2)} destinations={destinations.slice(0,2)} cells={cells} ariaLabel={`Mod matrix ${label}`} />} />
</Page>; }

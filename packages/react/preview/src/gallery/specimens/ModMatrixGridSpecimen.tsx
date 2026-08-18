import { useState } from "react";
import { ModMatrixGrid } from "@inflatable-cookie/poodle-react";
import type { ModMatrixCell, ModMatrixHeader } from "@inflatable-cookie/poodle-core";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
const sources: ModMatrixHeader[] = [{ id: "one", label: "Source 1" }, { id: "two", label: "Source 2" }, { id: "three", label: "Source 3" }];
const destinations: ModMatrixHeader[] = [{ id: "a", label: "Dest A" }, { id: "b", label: "Dest B" }, { id: "c", label: "Dest C" }];
const initial: ModMatrixCell[] = [{ sourceId: "one", destinationId: "a", amount: .75, enabled: true }, { sourceId: "one", destinationId: "b", amount: -.5, enabled: true }, { sourceId: "one", destinationId: "c", amount: .35, enabled: true, parameters: { min: 0, max: 1, step: .05 } }, { sourceId: "two", destinationId: "c", amount: 0, enabled: true }];
export function ModMatrixGridSpecimen() { const [cells] = useState(initial); return (
    <SpecimenLayout
      variantDirection="row"
      sizes={(size) => <ModMatrixGrid sources={sources.slice(0, 2)} destinations={destinations.slice(0, 2)} cells={cells} size={size} ariaLabel={`Mod matrix ${size} size`} />}
      densities={(density) => <ModMatrixGrid sources={sources.slice(0, 2)} destinations={destinations.slice(0, 2)} cells={cells} density={density} ariaLabel={`Mod matrix ${density} density`} />}
    >
      <div style={{ display: "grid", gap: "1.5rem" }}>
        <SpecimenGroup label="Sparse generic matrix"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} ariaLabel="Generic modulation matrix" /></SpecimenGroup>
  <SpecimenGroup label="Bipolar / negative / unipolar"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} ariaLabel="Mixed parameter amounts" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard navigation and toggle"><ModMatrixGrid sources={sources.slice(0, 2)} destinations={destinations} cells={cells} ariaLabel="Keyboard matrix" /></SpecimenGroup>
  <SpecimenGroup label="Empty axes"><ModMatrixGrid ariaLabel="Empty matrix" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><ModMatrixGrid sources={sources} destinations={destinations} cells={cells} disabled ariaLabel="Disabled matrix" /></SpecimenGroup>
      </div>
    </SpecimenLayout>
  ); }

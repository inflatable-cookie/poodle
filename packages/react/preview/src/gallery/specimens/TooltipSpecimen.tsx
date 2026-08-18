import { Tooltip, Button } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const rowStyle = { display: "flex", flexWrap: "wrap", gap: "0.75rem", alignItems: "center" } as const;

export function TooltipSpecimen() {
  return (
    <SpecimenLayout>
      <SpecimenGroup label="Default">
        <Tooltip content="Save your changes">
          <Button variant="secondary">Hover me</Button>
        </Tooltip>
      </SpecimenGroup>

      <SpecimenGroup label="Placements">
        <div style={rowStyle}>
          <Tooltip content="Top tooltip" placement="top">
            <Button variant="ghost">Top</Button>
          </Tooltip>
          <Tooltip content="Bottom tooltip" placement="bottom">
            <Button variant="ghost">Bottom</Button>
          </Tooltip>
          <Tooltip content="Left tooltip" placement="left">
            <Button variant="ghost">Left</Button>
          </Tooltip>
          <Tooltip content="Right tooltip" placement="right">
            <Button variant="ghost">Right</Button>
          </Tooltip>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { Separator } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

export function SeparatorSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Horizontal (default)">
        <p>Content above</p>
        <Separator />
        <p>Content below</p>
      </SpecimenGroup>

      <SpecimenGroup label="Vertical">
        <div style={{ display: "flex", gap: "0.75rem", alignItems: "center", height: "2rem" }}>
          <span>Left</span>
          <Separator orientation="vertical" />
          <span>Center</span>
          <Separator orientation="vertical" />
          <span>Right</span>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Decorative">
        <Separator decorative />
      </SpecimenGroup>
    </div>
  );
}

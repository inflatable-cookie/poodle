import type { CSSProperties } from "react";
import { Surface } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const copyStyle: CSSProperties = {
  margin: 0,
  fontSize: "0.875rem",
};

export function SurfaceSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Panel tone (default)">
        <Surface padding="md" border="subtle">
          <p style={copyStyle}>Panel surface with subtle border — the standard container.</p>
        </Surface>
      </SpecimenGroup>

      <SpecimenGroup label="Canvas tone">
        <Surface tone="canvas" padding="md" border="subtle">
          <p style={copyStyle}>Canvas surface sits behind panels as a background layer.</p>
        </Surface>
      </SpecimenGroup>

      <SpecimenGroup label="Elevated tone">
        <Surface tone="elevated" padding="md" border="subtle" elevated>
          <p style={copyStyle}>Elevated surface with shadow for overlays and cards.</p>
        </Surface>
      </SpecimenGroup>

      <SpecimenGroup label="No border">
        <Surface padding="md" border="none">
          <p style={copyStyle}>Surface with no border — just padding and background.</p>
        </Surface>
      </SpecimenGroup>
    </div>
  );
}

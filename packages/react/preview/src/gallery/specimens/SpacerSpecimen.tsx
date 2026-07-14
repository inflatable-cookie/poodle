import type { CSSProperties } from "react";
import { Spacer, Stack, Surface } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const spanStyle: CSSProperties = { fontSize: "0.875rem", whiteSpace: "nowrap" };

export function SpacerSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Push items apart">
        <Stack direction="row" gap="md">
          <Surface padding="sm" border="subtle">
            <span style={spanStyle}>Logo</span>
          </Surface>
          <Spacer />
          <Surface padding="sm" border="subtle">
            <span style={spanStyle}>Sign in</span>
          </Surface>
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Between three items">
        <Stack direction="row" gap="md">
          <Surface padding="sm" border="subtle">
            <span style={spanStyle}>Left</span>
          </Surface>
          <Spacer />
          <Surface padding="sm" border="subtle">
            <span style={spanStyle}>Center</span>
          </Surface>
          <Spacer />
          <Surface padding="sm" border="subtle">
            <span style={spanStyle}>Right</span>
          </Surface>
        </Stack>
      </SpecimenGroup>
    </div>
  );
}

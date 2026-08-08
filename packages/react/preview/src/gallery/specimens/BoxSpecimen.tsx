import type { CSSProperties } from "react";
import { Box } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const outlineStyle: CSSProperties = {
  border: "1px dashed var(--poodle-color-border-default)",
  borderRadius: "4px",
};

export function BoxSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Default (no padding)">
        <div style={outlineStyle}>
          <Box>Content inside a Box with no padding.</Box>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With padding">
        <div style={outlineStyle}>
          <Box padding="lg">Content inside a Box with large padding.</Box>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Fixed dimensions">
        <div style={outlineStyle}>
          <Box padding="md" width="12rem" height="6rem">
            Fixed 12×6rem box.
          </Box>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Overflow hidden">
        <div style={outlineStyle}>
          <Box padding="sm" width="10rem" height="3rem" overflow="hidden">
            This text is too long and will be clipped by the overflow hidden setting on the box container.
          </Box>
        </div>
      </SpecimenGroup>
    </div>
  );
}

import type { CSSProperties } from "react";
import { Stack, Surface } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const itemStyle: CSSProperties = {
  margin: 0,
  fontSize: "0.875rem",
  whiteSpace: "nowrap",
};

export function StackSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Column (default)">
        <Stack gap="md">
          <Surface padding="sm" border="subtle"><p style={itemStyle}>First item</p></Surface>
          <Surface padding="sm" border="subtle"><p style={itemStyle}>Second item</p></Surface>
          <Surface padding="sm" border="subtle"><p style={itemStyle}>Third item</p></Surface>
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Column — large gap, center aligned">
        <Stack gap="lg" align="center">
          <Surface padding="sm" border="subtle"><p style={itemStyle}>Centered A</p></Surface>
          <Surface padding="sm" border="subtle"><p style={itemStyle}>Centered B</p></Surface>
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Row">
        <Stack direction="row" gap="md">
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Item A</span></Surface>
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Taller item B with more text</span></Surface>
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Item C</span></Surface>
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Row — justify: between">
        <Stack direction="row" gap="md" justify="between">
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Left</span></Surface>
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Center</span></Surface>
          <Surface padding="sm" border="subtle"><span style={itemStyle}>Right</span></Surface>
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Row — wrapping">
        <Stack direction="row" gap="sm" wrap>
          {Array.from({ length: 8 }).map((_, i) => (
            <Surface key={i} padding="sm" border="subtle"><span style={itemStyle}>Tag {i + 1}</span></Surface>
          ))}
        </Stack>
      </SpecimenGroup>
    </div>
  );
}

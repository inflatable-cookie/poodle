import { Grid, Surface } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const cellText = { margin: 0, fontSize: "0.875rem" } as const;

export function GridSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Three columns">
        <Grid columns="1fr 1fr 1fr" gap="md">
          <Surface padding="md" border="subtle"><p style={cellText}>Column 1</p></Surface>
          <Surface padding="md" border="subtle"><p style={cellText}>Column 2</p></Surface>
          <Surface padding="md" border="subtle"><p style={cellText}>Column 3</p></Surface>
        </Grid>
      </SpecimenGroup>

      <SpecimenGroup label="Mixed column widths">
        <Grid columns="1fr 2fr" gap="md">
          <Surface padding="md" border="subtle"><p style={cellText}>Sidebar (1fr)</p></Surface>
          <Surface padding="md" border="subtle"><p style={cellText}>Main content (2fr)</p></Surface>
        </Grid>
      </SpecimenGroup>

      <SpecimenGroup label="Auto-fit responsive">
        <Grid columns="repeat(auto-fit, minmax(8rem, 1fr))" gap="sm">
          <Surface padding="sm" border="subtle"><p style={cellText}>A</p></Surface>
          <Surface padding="sm" border="subtle"><p style={cellText}>B</p></Surface>
          <Surface padding="sm" border="subtle"><p style={cellText}>C</p></Surface>
          <Surface padding="sm" border="subtle"><p style={cellText}>D</p></Surface>
          <Surface padding="sm" border="subtle"><p style={cellText}>E</p></Surface>
        </Grid>
      </SpecimenGroup>
    </div>
  );
}

import { PickerShell, Surface } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const itemTextStyle = { margin: 0, fontSize: "0.875rem" } as const;

export function PickerShellSpecimen() {
  return (
    <SpecimenLayout showSizes={false} bareVariants>
      <SpecimenGroup label="Inline variant (ready)" bare>
        <PickerShell
          title="Select a component"
          description="Browse and select from available components."
          resultCount={12}
          variant="inline"
        >
          <Surface padding="sm" border="subtle">
            <p style={itemTextStyle}>Component A</p>
          </Surface>
          <Surface padding="sm" border="subtle">
            <p style={itemTextStyle}>Component B</p>
          </Surface>
          <Surface padding="sm" border="subtle">
            <p style={itemTextStyle}>Component C</p>
          </Surface>
        </PickerShell>
      </SpecimenGroup>

      <SpecimenGroup label="No results" bare>
        <PickerShell
          title="Select an item"
          state="no-results"
          stateTitle="No matches"
          stateMessage="Try a different search term."
          variant="inline"
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { PickerShell, Surface, UiPresentationProvider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const itemTextStyle = { margin: 0, fontSize: "0.875rem" } as const;
const densityDemoStyle = { width: "min(100%, 28rem)" } as const;

export function PickerShellSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      showDensities
      bareVariants
      densities={(density) => (
        <UiPresentationProvider density={density}>
          <div style={densityDemoStyle}>
            <PickerShell
              title="Select a component"
              description="Browse and select from available components."
              resultCount={12}
              selectionCount={2}
              variant="inline"
            >
              <Surface padding="sm" border="subtle">
                <p style={itemTextStyle}>Component A</p>
              </Surface>
              <Surface padding="sm" border="subtle">
                <p style={itemTextStyle}>Component B</p>
              </Surface>
            </PickerShell>
          </div>
        </UiPresentationProvider>
      )}
    >
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

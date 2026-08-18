import type { CSSProperties } from "react";
import { Button, Select, Surface, TextInput, UiPresentationProvider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const controls: CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  alignItems: "center",
  gap: "var(--poodle-space-stack-md)",
};

const options = [
  { value: "alpha", label: "Alpha" },
  { value: "beta", label: "Beta" },
];

export function UiPresentationProviderSpecimen() {
  return (
    <SpecimenLayout
      densities={(density) => (
        <UiPresentationProvider density={density}>
          <Surface border="subtle" padding="md">
            <div style={controls}>
              <Button>Save</Button>
              <TextInput id="provider-axis-name" value="Draft" />
              <Select id="provider-axis-select" options={options} value="alpha" />
            </div>
          </Surface>
        </UiPresentationProvider>
      )}
    >
      <SpecimenGroup label="Compact small scope">
        <UiPresentationProvider density="compact" sizeScale="sm">
          <Surface border="subtle" padding="md">
            <div style={controls}>
              <Button>Save</Button>
              <TextInput id="provider-compact-name" value="Compact" />
              <Select id="provider-compact-select" options={options} value="alpha" />
            </div>
          </Surface>
        </UiPresentationProvider>
      </SpecimenGroup>

      <SpecimenGroup label="Comfortable large scope">
        <UiPresentationProvider density="comfortable" sizeScale="lg">
          <Surface border="subtle" padding="md">
            <div style={controls}>
              <Button variant="primary">Save</Button>
              <TextInput id="provider-comfortable-name" value="Comfortable" />
              <Select id="provider-comfortable-select" options={options} value="beta" />
            </div>
          </Surface>
        </UiPresentationProvider>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

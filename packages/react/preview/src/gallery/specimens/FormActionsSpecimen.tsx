import type { CSSProperties } from "react";
import { FormActions, Button } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const dangerItems = [
  {
    label: "Discard draft",
    onSelect: () => console.log("discard"),
  },
];

const variantBlockStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.5rem",
  width: "min(22rem, 100%)",
};

const variantLabelStyle: CSSProperties = {
  color: "var(--poodle-color-text-secondary)",
  fontFamily: "var(--poodle-typography-label-family)",
  fontSize: "0.75rem",
  fontWeight: "var(--poodle-typography-label-weight)" as CSSProperties["fontWeight"],
  lineHeight: "var(--poodle-typography-label-lineHeight)",
  letterSpacing: "0.08em",
  textTransform: "uppercase",
};

export function FormActionsSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={variantBlockStyle}>
          <div style={variantLabelStyle}>{density.toUpperCase()}</div>
          <div style={{ width: "100%" }}>
            <FormActions density={density}>
              <Button variant="ghost">Cancel</Button>
              <Button variant="primary">Save changes</Button>
            </FormActions>
          </div>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="End-aligned (default)">
          <FormActions>
            <Button variant="ghost">Cancel</Button>
            <Button variant="primary">Save changes</Button>
          </FormActions>
        </SpecimenGroup>

        <SpecimenGroup label="Start-aligned">
          <FormActions align="start">
            <Button variant="ghost">Back</Button>
            <Button variant="primary">Continue</Button>
          </FormActions>
        </SpecimenGroup>

        <SpecimenGroup label="Space between">
          <FormActions align="between">
            <Button variant="secondary" tone="danger">
              Delete
            </Button>
            <Button variant="primary">Save</Button>
          </FormActions>
        </SpecimenGroup>

        <SpecimenGroup label="Responsive danger actions">
          <div style={{ maxWidth: "20rem" }}>
            <FormActions align="end" dangerItems={dangerItems} danger={<Button variant="ghost" tone="danger">Discard draft</Button>}>
              <Button variant="ghost">Back</Button>
              <Button variant="primary">Save changes</Button>
            </FormActions>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Bordered separation">
          <FormActions showTopBorder>
            <Button variant="ghost">Cancel</Button>
            <Button variant="primary">Save changes</Button>
          </FormActions>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

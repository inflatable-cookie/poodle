import { DetailItem, DetailSection, DetailSectionGroup } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DetailSectionGroupSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <DetailSectionGroup density={density} minColumnWidth="12rem">
          <DetailSection title="Density">
            <DetailItem label="Mode" value={density} />
          </DetailSection>
          <DetailSection title="Spacing">
            <DetailItem label="Inherited" value="Yes" />
          </DetailSection>
        </DetailSectionGroup>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Grid layout">
          <DetailSectionGroup ariaLabel="Project metadata">
            <DetailSection title="General" columns={2}>
              <DetailItem label="Owner" value="Platform" layout="stacked" />
              <DetailItem label="Status" value="Active" layout="stacked" />
            </DetailSection>
            <DetailSection title="Runtime" columns={2}>
              <DetailItem label="Region" value="eu-west-1" layout="stacked" />
              <DetailItem label="Tier" value="Production" layout="stacked" />
            </DetailSection>
            <DetailSection title="Policy" columns={2}>
              <DetailItem label="Retention" value="90 days" layout="stacked" />
              <DetailItem label="Review" value="Required" layout="stacked" />
            </DetailSection>
          </DetailSectionGroup>
        </SpecimenGroup>

        <SpecimenGroup label="Stack layout">
          <DetailSectionGroup layout="stack" itemMinColumnWidth="10rem">
            <DetailSection title="Access" columns={2}>
              <DetailItem label="Role" value="Editor" layout="stacked" />
              <DetailItem label="Scope" value="Workspace" layout="stacked" />
            </DetailSection>
            <DetailSection title="Billing" columns={2}>
              <DetailItem label="Plan" value="Team" layout="stacked" />
              <DetailItem label="Renewal" value="Monthly" layout="stacked" />
            </DetailSection>
          </DetailSectionGroup>
        </SpecimenGroup>

        <SpecimenGroup label="Column cap">
          <DetailSectionGroup minColumnWidth="10rem" maxColumns={2}>
            {["One", "Two", "Three", "Four"].map((title) => (
              <DetailSection key={title} title={title}>
                <DetailItem label="Value" value={title} />
              </DetailSection>
            ))}
          </DetailSectionGroup>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

import { Button, DetailItem, DetailSection } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DetailSectionSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
          <div
            style={{
              color: "var(--poodle-color-text-muted)",
              fontSize: "0.75rem",
              fontWeight: 700,
              letterSpacing: "0.16em",
              textTransform: "uppercase",
            }}
          >
            {density.toUpperCase()}
          </div>
          <div style={{ width: "min(100%, 40rem)" }}>
            <DetailSection
              title="Workspace access"
              description="Shared settings and runtime defaults."
              columns={2}
              density={density}
            >
              <DetailItem label="Default role" value="Editor" layout="stacked" />
              <DetailItem label="Approvals" value="Required" layout="stacked" />
              <DetailItem label="Region" value="eu-west-1" layout="stacked" />
              <DetailItem label="Retention" value="30 days" layout="stacked" />
            </DetailSection>
          </div>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="With title and rows">
          <DetailSection title="Project details" description="Core metadata for this project.">
            <DetailItem label="Name" value="Poodle Design System" />
            <DetailItem label="Owner" value="Clay + Aura" />
            <DetailItem label="Created" value="March 2025" />
            <DetailItem label="Status" value="Active" />
          </DetailSection>
        </SpecimenGroup>

        <SpecimenGroup label="With actions">
          <DetailSection
            title="Billing"
            actions={
              <Button variant="secondary" size="sm">
                Edit
              </Button>
            }
          >
            <DetailItem label="Plan" value="Pro" />
            <DetailItem label="Billing cycle" value="Monthly" />
            <DetailItem label="Next invoice" value="April 1, 2026" />
          </DetailSection>
        </SpecimenGroup>

        <SpecimenGroup label="DetailItem with description">
          <DetailSection title="Configuration">
            <DetailItem
              label="API endpoint"
              value="https://api.example.com/v2"
              description="The base URL for all API requests."
              truncateValue
            />
            <DetailItem label="Rate limit" value="1,000 req/min" description="Maximum requests per minute." />
          </DetailSection>
        </SpecimenGroup>

        <SpecimenGroup label="Two-column details">
          <DetailSection title="Runtime summary" description="Compact layout for denser metadata surfaces." columns={2}>
            <DetailItem label="Route" value="local-brokered" layout="stacked" />
            <DetailItem label="Posture" value="aura-local-brokered" layout="stacked" />
            <DetailItem label="Authority" value="local" layout="stacked" />
            <DetailItem label="Displays" value="2" layout="stacked" />
          </DetailSection>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

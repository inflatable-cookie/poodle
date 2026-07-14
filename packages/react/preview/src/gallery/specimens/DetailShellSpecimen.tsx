import { Button, DetailItem, DetailSection, DetailShell, PageHeader, Pill, Region, Separator } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function DetailShellSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Layout structure">
        <div style={{ marginTop: "0.75rem" }}>
          <DetailShell ariaLabel="Layout regions" header={<Region label="Header" color="#5b9bd5" minHeight="3rem" />}>
            <Region label="Section 1" color="#70ad47" minHeight="3rem" />
            <Region label="Section 2" color="#ed7d31" minHeight="3rem" />
            <Region label="Section 3" color="#a855f7" minHeight="3rem" />
          </DetailShell>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Multi-section layout with header">
        <div style={{ marginTop: "0.75rem" }}>
          <DetailShell
            ariaLabel="Project detail view"
            header={
              <PageHeader
                title="Poodle Design System"
                eyebrow="Project"
                subtitle="A comprehensive component library."
                actions={
                  <>
                    <Pill appearance="badge" tone="success">
                      Active
                    </Pill>
                    <Button variant="secondary">Edit</Button>
                  </>
                }
              />
            }
          >
            <DetailSection title="General">
              <DetailItem label="Owner" value="Clay" />
              <DetailItem label="Created" value="March 2025" />
              <DetailItem label="Repository" value="github.com/poodle-ui/poodle" />
            </DetailSection>
            <Separator />
            <DetailSection
              title="Configuration"
              actions={<Button variant="ghost">Reset</Button>}
            >
              <DetailItem label="Theme" value="Dark" />
              <DetailItem label="Density" value="Compact" />
              <DetailItem label="Default size" value="Medium" />
            </DetailSection>
            <Separator />
            <DetailSection title="Integrations">
              <DetailItem label="Figma" value="Connected" />
              <DetailItem label="Storybook" value="Not configured" />
            </DetailSection>
          </DetailShell>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading state">
        <div style={{ marginTop: "0.75rem" }}>
          <DetailShell title="Loading" state="loading" ariaLabel="Loading view" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Error state">
        <div style={{ marginTop: "0.75rem" }}>
          <DetailShell
            title="Error"
            state="error"
            stateTitle="Failed to load"
            stateMessage="Something went wrong. Please try again."
            ariaLabel="Error view"
          />
        </div>
      </SpecimenGroup>
    </div>
  );
}

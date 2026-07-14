import { Button, DetailItem, DetailSection, Pill } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DetailItemSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={{ width: "min(100%, 32rem)" }}>
          <DetailItem
            label="Storage"
            value="84.2 GB"
            description="Current usage for the active workspace."
            presentation="surface"
            density={density}
            action={
              <Button variant="secondary" size="sm">
                Manage
              </Button>
            }
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Inline layout (default)">
          <DetailSection title="Package info">
            <DetailItem label="Name" value="Poodle Design System" />
            <DetailItem label="Version" value="2.1.0" />
            <DetailItem label="License" value="MIT" />
          </DetailSection>
        </SpecimenGroup>

        <SpecimenGroup label="With description">
          <DetailItem label="API endpoint" value="https://api.example.com/v2" description="Base URL for all API requests." />
        </SpecimenGroup>

        <SpecimenGroup label="With action slot">
          <DetailItem
            label="Email"
            value="clay@example.com"
            action={
              <Button variant="secondary" size="sm">
                Change
              </Button>
            }
          />
        </SpecimenGroup>

        <SpecimenGroup label="With value slot">
          <DetailItem
            label="Status"
            valueContent={
              <Pill tone="success" appearance="badge">
                Active
              </Pill>
            }
          />
        </SpecimenGroup>

        <SpecimenGroup label="Stacked layout">
          <DetailItem
            label="Arrangement"
            value="2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A..."
            truncateValue
            layout="stacked"
          />
        </SpecimenGroup>

        <SpecimenGroup label="Surface presentation">
          <DetailSection title="Account">
            <DetailItem label="Name" value="Alice Chen" presentation="surface" />
            <DetailItem label="Role" value="Engineer" presentation="surface" />
            <DetailItem
              label="Email"
              value="alice@example.com"
              presentation="surface"
              action={
                <Button variant="secondary" size="sm">
                  Edit
                </Button>
              }
            />
          </DetailSection>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

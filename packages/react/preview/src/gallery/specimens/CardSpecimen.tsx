import type { CSSProperties } from "react";
import { Card } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const cardsGrid: CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fit, minmax(14rem, 1fr))",
  gap: "1rem",
};

const titleStyle: CSSProperties = {
  fontSize: "1rem",
  fontWeight: 600,
  margin: 0,
  color: "var(--poodle-color-text-primary)",
};

const bodyStyle: CSSProperties = {
  fontSize: "0.875rem",
  margin: 0,
  color: "var(--poodle-color-text-secondary)",
  lineHeight: 1.5,
};

const metaStyle: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

const variantStyle: CSSProperties = { width: "min(100%, 22rem)" };

export function CardSpecimen() {
  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={variantStyle}>
          <Card
            ariaLabel={`${density} density card`}
            density={density}
            header={<h3 style={titleStyle}>Project Alpha</h3>}
            footer={<span style={metaStyle}>Updated 2 days ago</span>}
          >
            <p style={bodyStyle}>A design system component library for building consistent interfaces.</p>
          </Card>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Default variant">
          <div style={cardsGrid}>
            <Card
              ariaLabel="Project card"
              header={<h3 style={titleStyle}>Project Alpha</h3>}
              footer={<span style={metaStyle}>Updated 2 days ago</span>}
            >
              <p style={bodyStyle}>A design system component library for building consistent interfaces.</p>
            </Card>

            <Card
              ariaLabel="Stats card"
              header={<h3 style={titleStyle}>Monthly report</h3>}
            >
              <p style={bodyStyle}>48 components shipped across 3 packages this month.</p>
            </Card>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Outlined variant">
          <Card
            variant="outlined"
            ariaLabel="Outlined card"
            header={<h3 style={titleStyle}>Outlined card</h3>}
          >
            <p style={bodyStyle}>This card uses a subtle border instead of elevation.</p>
          </Card>
        </SpecimenGroup>

        <SpecimenGroup label="Elevated variant">
          <Card
            variant="elevated"
            ariaLabel="Elevated card"
            header={<h3 style={titleStyle}>Elevated card</h3>}
          >
            <p style={bodyStyle}>This card uses a drop shadow for visual prominence.</p>
          </Card>
        </SpecimenGroup>

        <SpecimenGroup label="Interactive">
          <Card
            interactive
            ariaLabel="Clickable card"
            header={<h3 style={titleStyle}>Interactive card</h3>}
          >
            <p style={bodyStyle}>Hover to see the interactive state. Cursor changes to pointer.</p>
          </Card>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

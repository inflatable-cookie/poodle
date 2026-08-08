import { useState } from "react";
import { Grid, Icon, NavCard } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function NavCardSpecimen() {
  const [lastClick, setLastClick] = useState("");

  return (
    <SpecimenLayout
      showSizes={false}
      densities={(density) => (
        <div style={{ width: "min(100%, 18rem)" }}>
          <NavCard
            title="Components"
            description="Browse all available components."
            badge="New"
            density={density}
            icon={<Icon name="layers" />}
          />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <SpecimenGroup label="Navigation cards in a grid (2 columns)">
          <Grid columns="1fr 1fr" gap="md" asRole="navigation" ariaLabel="Navigation cards">
            <NavCard
              title="Getting Started"
              description="Learn the basics of the component library."
              onClick={() => setLastClick("Getting Started")}
              icon={<Icon name="house" />}
            />
            <NavCard
              title="Components"
              description="Browse all available components."
              badge="New"
              onClick={() => setLastClick("Components")}
              icon={<Icon name="layers" />}
            />
            <NavCard
              title="Tokens"
              description="Design tokens and theming system."
              onClick={() => setLastClick("Tokens")}
              icon={<Icon name="sliders-horizontal" />}
            />
            <NavCard
              title="API Reference"
              description="Complete component API documentation."
              disabled
              icon={<Icon name="file-text" />}
            />
          </Grid>
        </SpecimenGroup>

        <SpecimenGroup label="Single card (as link)">
          <NavCard title="View Documentation" description="Open the full documentation site." href="#" />
        </SpecimenGroup>

        {lastClick ? (
          <SpecimenGroup label="Last click">
            <p style={{ margin: 0 }}>{lastClick}</p>
          </SpecimenGroup>
        ) : null}
      </div>
    </SpecimenLayout>
  );
}

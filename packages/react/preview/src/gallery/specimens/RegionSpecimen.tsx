import { Region, Stack } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function RegionSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Default">
        <Region label="Content area" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom colors">
        <Stack gap="sm">
          <Region label="Header" color="#5b9bd5" minHeight="3rem" />
          <Region label="Sidebar" color="#70ad47" minHeight="6rem" />
          <Region label="Main content" color="#ed7d31" minHeight="8rem" />
          <Region label="Footer" color="#a855f7" minHeight="3rem" />
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Layout composition">
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "10rem 1fr",
            gap: "0.5rem",
            minHeight: "14rem",
          }}
        >
          <Region label="Nav" color="#5b9bd5" minHeight="100%" />
          <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
            <Region label="Toolbar" color="#70ad47" minHeight="2.5rem" />
            <Region label="Content" color="#ed7d31" minHeight="10rem" />
          </div>
        </div>
      </SpecimenGroup>
    </div>
  );
}

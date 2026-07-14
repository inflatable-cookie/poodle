import { Meter } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function MeterSpecimen() {
  return (
    <SpecimenLayout
      showSizes
      showDensities={false}
      sizes={(size) => (
        <div style={{ width: "min(100%, 20rem)" }}>
          <Meter value={50} ariaLabel={`Storage usage at ${size}`} size={size} />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem", maxWidth: "20rem" }}>
        <SpecimenGroup label="Default (50%)">
          <Meter value={50} ariaLabel="Storage usage" />
        </SpecimenGroup>

        <SpecimenGroup label="With thresholds">
          <Meter value={82} low={25} high={75} optimum={50} ariaLabel="CPU usage" />
          <p>82% — above high threshold</p>
        </SpecimenGroup>

        <SpecimenGroup label="Low value (optimal range)">
          <Meter value={30} low={25} high={75} optimum={50} ariaLabel="Memory usage" />
          <p>30% — within normal range</p>
        </SpecimenGroup>

        <SpecimenGroup label="Custom range (0–500)">
          <Meter value={350} min={0} max={500} ariaLabel="API calls" />
          <p>350 / 500 API calls used</p>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

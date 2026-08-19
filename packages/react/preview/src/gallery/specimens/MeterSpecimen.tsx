import { Meter } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const row = { display: "flex", alignItems: "center", gap: "0.75rem" } as const;

export function MeterSpecimen() {
  return (
    <SpecimenLayout
      showSizes
      showDensities={false}
      sizes={(size) => (
        <div style={{ width: "min(100%, 20rem)" }}>
          <Meter shape="ring" value={60} ariaLabel={`Context used at ${size}`} size={size} />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem", maxWidth: "20rem" }}>
        <SpecimenGroup label="Default usage">
          <Meter value={50} ariaLabel="Storage usage" />
        </SpecimenGroup>

        <SpecimenGroup label="Threshold states">
          <Meter value={82} low={25} high={75} optimum={50} ariaLabel="CPU usage" />
          <p>82% — above high threshold</p>
          <Meter value={30} low={25} high={75} optimum={50} ariaLabel="Memory usage" />
          <p>30% — within normal range</p>
        </SpecimenGroup>

        <SpecimenGroup label="Custom range">
          <Meter value={350} min={0} max={500} ariaLabel="API calls" />
          <p>350 / 500 API calls used</p>
        </SpecimenGroup>

        <SpecimenGroup label="Ring shape and readout">
          <div style={row}>
            <Meter shape="ring" value={38} ariaLabel="Context used" />
            <Meter
              shape="ring"
              value={86}
              high={80}
              ariaLabel="Context used, above warn threshold"
            />
            <Meter shape="ring" value={64} showValue size="xl" ariaLabel="Context used" />
          </div>
          <p>38% · 86% (above high) · 64% with readout</p>
        </SpecimenGroup>

        <SpecimenGroup label="Ring tones">
          <div style={row}>
            {(["success", "accent", "warning", "danger", "neutral"] as const).map((tone) => (
              <Meter key={tone} shape="ring" value={60} tone={tone} ariaLabel={`${tone} tone`} />
            ))}
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

import { useState, type CSSProperties } from "react";
import { TimeZoneSelect, Eyebrow, Surface } from "@poodle/react";
import { SpecimenLayout } from "../SpecimenLayout";

const specimenStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.75rem",
};

const itemStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "0.75rem",
};

const valueStyle: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

export function TimeZoneSelectSpecimen() {
  const [zone, setZone] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <TimeZoneSelect size={size} ariaLabel={size} />}
      densities={(density) => <TimeZoneSelect density={density} ariaLabel={density} />}
    >
      <Surface tone="panel" border="subtle" padding="md">
        <div style={specimenStyle}>
          <div style={itemStyle}>
            <Eyebrow>Default</Eyebrow>
            <TimeZoneSelect ariaLabel="Time zone" onValueChange={(value) => setZone(value)} />
            {zone && <span style={valueStyle}>{zone}</span>}
          </div>

          <div style={itemStyle}>
            <Eyebrow>Pre-selected</Eyebrow>
            <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled" />
          </div>

          <div style={itemStyle}>
            <Eyebrow>Disabled</Eyebrow>
            <TimeZoneSelect disabled ariaLabel="Disabled" />
          </div>
        </div>
      </Surface>
    </SpecimenLayout>
  );
}

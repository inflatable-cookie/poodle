import { useState, type CSSProperties } from "react";
import { TimeZoneSelect, Eyebrow, Surface } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
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
            <SpecimenGroup label="Default">
        <TimeZoneSelect ariaLabel="Time zone" onValueChange={(value) => setZone(value)} />
                    {zone && <span style={valueStyle}>{zone}</span>}
      </SpecimenGroup>

                <SpecimenGroup label="Pre-selected">
        <TimeZoneSelect defaultValue="America/New_York" ariaLabel="Pre-filled" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <TimeZoneSelect disabled ariaLabel="Disabled" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { useState } from "react";
import { TimeZoneSelect } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const itemStyle = { display: "flex", alignItems: "center", gap: "0.75rem" } as const;
const valueStyle = { fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" } as const;

export function TimeZoneSelectSpecimen() {
  const [zone, setZone] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <TimeZoneSelect size={size} ariaLabel={size} />}
      densities={(density) => <TimeZoneSelect density={density} ariaLabel={density} />}
    >
      <SpecimenGroup label="Default">
        <div style={itemStyle}>
          <TimeZoneSelect ariaLabel="Time zone" onValueChange={(value) => setZone(value)} />
          {zone ? <span style={valueStyle}>{zone}</span> : null}
        </div>
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

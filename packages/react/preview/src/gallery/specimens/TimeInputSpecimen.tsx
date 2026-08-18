import { useState } from "react";
import { TimeInput } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const itemStyle = { display: "flex", alignItems: "center", gap: "0.75rem" } as const;
const valueStyle = { fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" } as const;

export function TimeInputSpecimen() {
  const [time, setTime] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <TimeInput id={"size-" + size} size={size} ariaLabel={size} />}
      densities={(density) => <TimeInput id={"density-" + density} density={density} />}
    >
      <SpecimenGroup label="Default">
        <div style={itemStyle}>
          <TimeInput
            id="start-time"
            ariaLabel="Start time"
            onValueChange={(value) => {
              if (value) setTime(value);
            }}
          />
          {time ? <span style={valueStyle}>{time}</span> : null}
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With default value">
        <TimeInput id="meeting-time" defaultValue="14:30" ariaLabel="Meeting time" />
      </SpecimenGroup>

      <SpecimenGroup label="With min/max">
        <TimeInput id="office" defaultValue="09:00" min="08:00" max="18:00" ariaLabel="Office hours" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <TimeInput id="disabled-time" defaultValue="12:00" disabled ariaLabel="Disabled" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

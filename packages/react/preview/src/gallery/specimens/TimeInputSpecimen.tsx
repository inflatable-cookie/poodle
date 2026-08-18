import { useState, type CSSProperties } from "react";
import { TimeInput, Eyebrow, Surface } from "@inflatable-cookie/poodle-react";
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

export function TimeInputSpecimen() {
  const [time, setTime] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <TimeInput id={"size-" + size} size={size} ariaLabel={size} />}
      densities={(density) => <TimeInput id={"density-" + density} density={density} />}
    >
            <SpecimenGroup label="Default">
        <TimeInput
                      id="start-time"
                      ariaLabel="Start time"
                      onValueChange={(value) => {
                        if (value) setTime(value);
                      }}
                    />
                    {time && <span style={valueStyle}>{time}</span>}
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

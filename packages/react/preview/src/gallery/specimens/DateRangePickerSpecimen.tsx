import { useState } from "react";
import { DateRangePicker, type DateRangeValue } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DateRangePickerSpecimen() {
  const [range, setRange] = useState<DateRangeValue>({ start: "", end: "" });

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateRangePicker size={size} ariaLabel={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateRangePicker density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Default">
        <div style={{ maxWidth: "20rem" }}>
          <DateRangePicker ariaLabel="Select date range" onValueChange={(value) => setRange(value)} />
        </div>
        {range.start ? (
          <p>
            {range.start} → {range.end || "…"}
          </p>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="With default range">
        <div style={{ maxWidth: "20rem" }}>
          <DateRangePicker defaultValue={{ start: "2026-03-01", end: "2026-03-14" }} ariaLabel="Pre-filled range" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={{ maxWidth: "20rem" }}>
          <DateRangePicker disabled ariaLabel="Disabled range picker" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { DateTimeRangePicker } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DateTimeRangePickerSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateTimeRangePicker size={size} ariaLabel={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateTimeRangePicker density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Default">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimeRangePicker ariaLabel="Select date and time range" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With default range">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimeRangePicker
            defaultValue={{
              start: { date: "2026-03-10", time: "09:00" },
              end: { date: "2026-03-14", time: "17:00" },
            }}
            ariaLabel="Pre-filled range"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimeRangePicker disabled ariaLabel="Disabled range picker" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

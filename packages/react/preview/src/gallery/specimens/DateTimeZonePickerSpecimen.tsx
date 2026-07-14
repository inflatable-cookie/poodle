import type { CSSProperties } from "react";
import { DateTimeZonePicker } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const control: CSSProperties = { maxWidth: "20rem" };

export function DateTimeZonePickerSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={control}>
          <DateTimeZonePicker size={size} ariaLabel={size} />
        </div>
      )}
      densities={(density) => (
        <div style={control}>
          <DateTimeZonePicker density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Default">
        <div style={control}>
          <DateTimeZonePicker ariaLabel="Select date, time, and zone" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With default value">
        <div style={control}>
          <DateTimeZonePicker
            defaultValue={{ date: "2026-03-14", time: "10:00", timeZone: "America/Los_Angeles" }}
            ariaLabel="Pre-filled zoned date time"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={control}>
          <DateTimeZonePicker disabled ariaLabel="Disabled picker" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

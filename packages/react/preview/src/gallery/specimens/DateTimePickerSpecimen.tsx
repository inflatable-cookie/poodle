import { DateTimePicker } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DateTimePickerSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateTimePicker size={size} ariaLabel={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ maxWidth: "20rem" }}>
          <DateTimePicker density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Default">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimePicker ariaLabel="Select date and time" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With default value">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimePicker defaultValue={{ date: "2026-03-14", time: "14:30" }} ariaLabel="Pre-filled date time" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={{ maxWidth: "20rem" }}>
          <DateTimePicker disabled ariaLabel="Disabled date time picker" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

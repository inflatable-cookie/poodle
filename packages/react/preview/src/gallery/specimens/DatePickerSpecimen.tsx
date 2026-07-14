import { useState } from "react";
import { DatePicker } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DatePickerSpecimen() {
  const [selected, setSelected] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ maxWidth: "20rem" }}>
          <DatePicker size={size} ariaLabel={size} />
        </div>
      )}
      densities={(density) => (
        <div style={{ maxWidth: "20rem" }}>
          <DatePicker density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Default">
        <div style={{ maxWidth: "20rem" }}>
          <DatePicker ariaLabel="Select date" onValueChange={(value) => setSelected(value)} />
        </div>
        {selected ? (
          <p>Selected: <strong>{selected}</strong></p>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="With default value">
        <div style={{ maxWidth: "20rem" }}>
          <DatePicker defaultValue="2026-03-14" ariaLabel="Pre-filled date" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={{ maxWidth: "20rem" }}>
          <DatePicker placeholder="Disabled" disabled ariaLabel="Disabled date picker" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

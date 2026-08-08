import { useState, type CSSProperties } from "react";
import { Calendar, type DateRangeValue } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const hintStyle: CSSProperties = {
  margin: "0.5rem 0 0",
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

export function CalendarSpecimen() {
  const [selected, setSelected] = useState("");
  const [range, setRange] = useState<DateRangeValue>({ start: "", end: "" });

  return (
    <SpecimenLayout
      sizes={(size) => <Calendar size={size} ariaLabel={size + " calendar"} />}
      densities={(density) => <Calendar density={density} />}
    >
      <SpecimenGroup label="Default">
        <Calendar ariaLabel="Select a date" onValueChange={(value) => setSelected(value as string)} />
        <p style={hintStyle}>Double-click the month to choose a month, or the year to edit the year directly.</p>
        {selected ? (
          <p>Selected: <strong>{selected}</strong></p>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="With pre-selected date">
        <Calendar defaultValue="2026-03-14" ariaLabel="Calendar with default" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <Calendar defaultValue="2026-03-01" disabled ariaLabel="Disabled calendar" />
      </SpecimenGroup>

      <SpecimenGroup label="Range selection">
        <Calendar
          mode="range"
          ariaLabel="Select a date range"
          onValueChange={(value) => setRange(value as DateRangeValue)}
        />
        {range.start ? (
          <p>{range.start} &rarr; {range.end || "..."}</p>
        ) : null}
      </SpecimenGroup>

      <SpecimenGroup label="Range with pre-selected range">
        <Calendar
          mode="range"
          defaultValue={{ start: "2026-03-05", end: "2026-03-12" }}
          ariaLabel="Pre-selected range"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Range disabled">
        <Calendar mode="range" disabled ariaLabel="Disabled range calendar" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { useState } from "react";
import { DatePicker, DateRangePicker, DateTimePicker, type DateRangeValue, type DateTimeValue } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function DatePickerSpecimen() {
  const [d, setD] = useState<string | null>("2026-07-14");
  return (
    <SpecimenSection title="DatePicker">
      <Row>
        <DatePicker value={d} onValueChange={setD} />
      </Row>
      <span data-testid="dp-value">value: {d}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "date-picker", title: "DatePicker", render: () => <DatePickerSpecimen /> });

function DateRangePickerSpecimen() {
  const [r, setR] = useState<DateRangeValue>({ start: "2026-07-06", end: null });
  return (
    <SpecimenSection title="DateRangePicker">
      <Row>
        <DateRangePicker value={r} onValueChange={setR} />
      </Row>
      <span data-testid="drp-value">
        {r.start ?? "-"}..{r.end ?? "-"}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "date-range-picker", title: "DateRangePicker", render: () => <DateRangePickerSpecimen /> });

function DateTimePickerSpecimen() {
  const [v, setV] = useState<DateTimeValue>({ date: "2026-07-14", time: null });
  return (
    <SpecimenSection title="DateTimePicker">
      <Row>
        <DateTimePicker value={v} onValueChange={setV} />
      </Row>
      <span data-testid="dtp-value">
        {v.date ?? "-"} {v.time ?? "-"}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "date-time-picker", title: "DateTimePicker", render: () => <DateTimePickerSpecimen /> });

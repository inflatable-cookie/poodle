import { useState } from "react";
import { DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker, DateTimeZonePicker, type DateRangeValue, type DateTimeRangeValue, type DateTimeValue, type ZonedDateTimeValue } from "@poodle/react";
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

function DateTimeRangePickerSpecimen() {
  const [v, setV] = useState<DateTimeRangeValue>({
    start: { date: "2026-07-06", time: "09:00" },
    end: { date: null, time: null },
  });
  return (
    <SpecimenSection title="DateTimeRangePicker">
      <Row>
        <DateTimeRangePicker value={v} onValueChange={setV} />
      </Row>
      <span data-testid="dtrp-value">
        {v.start.date ?? "-"} {v.start.time ?? "-"} .. {v.end.date ?? "-"} {v.end.time ?? "-"}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "date-time-range-picker", title: "DateTimeRangePicker", render: () => <DateTimeRangePickerSpecimen /> });

function DateTimeZonePickerSpecimen() {
  const [v, setV] = useState<ZonedDateTimeValue>({ date: "2026-07-14", time: null, timeZone: null });
  return (
    <SpecimenSection title="DateTimeZonePicker">
      <Row>
        <DateTimeZonePicker value={v} onValueChange={setV} />
      </Row>
      <span data-testid="dtzp-value">
        {v.date ?? "-"} {v.time ?? "-"} {v.timeZone ?? "-"}
      </span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "date-time-zone-picker", title: "DateTimeZonePicker", render: () => <DateTimeZonePickerSpecimen /> });

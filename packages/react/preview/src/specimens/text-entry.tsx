import { useState } from "react";
import { EditableLabel, TimeInput } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

function EditableLabelSpecimen() {
  const [events, setEvents] = useState<string[]>([]);
  return (
    <SpecimenSection title="EditableLabel">
      <EditableLabel
        value="Double-click me"
        onCommit={({ value, previousValue }) => setEvents((e) => [...e, `commit:${value}(was ${previousValue})`])}
        onCancel={() => setEvents((e) => [...e, "cancel"])}
      />
      <EditableLabel value="" emptyText="Untitled" activationMode="enterOrSpace" showEditIcon />
      <span data-testid="editable-events">{events.join(" | ") || "no events"}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "editable-label", title: "EditableLabel", render: () => <EditableLabelSpecimen /> });

function TimeInputSpecimen() {
  const [t, setT] = useState<string | null>("09:30");
  return (
    <SpecimenSection title="TimeInput">
      <Row>
        <TimeInput value={t} onValueChange={setT} ariaLabel="Start time" />
        <TimeInput defaultValue="17:00" disabled ariaLabel="Disabled" />
      </Row>
      <span data-testid="time-value">time: {String(t)}</span>
    </SpecimenSection>
  );
}
registerSpecimen({ slug: "time-input", title: "TimeInput", render: () => <TimeInputSpecimen /> });

import { useState } from "react";
import { EditableLabel } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function EditableLabelSpecimen() {
  const [title, setTitle] = useState("My project title");
  const [emptyValue, setEmptyValue] = useState("");
  const [flushValue, setFlushValue] = useState("Inline heading");
  const [lastEvent, setLastEvent] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <EditableLabel value={size.toUpperCase()} size={size} ariaLabel={"Label at " + size} />}
      densities={(density) => <EditableLabel value="Edit me" density={density} />}
    >
      <SpecimenGroup label="Double-click to edit (default)">
        <EditableLabel
          value={title}
          ariaLabel="Project title"
          onCommit={(detail) => {
            setTitle(detail.value);
            setLastEvent(`Committed: "${detail.value}" (was: "${detail.previousValue}")`);
          }}
          onCancel={() => setLastEvent("Edit cancelled")}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Click to edit with icon">
        <EditableLabel
          value={title}
          ariaLabel="Project title"
          activationMode="enterOrSpace"
          showEditIcon
          onCommit={(detail) => {
            setTitle(detail.value);
            setLastEvent(`Committed: "${detail.value}"`);
          }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Empty state">
        <EditableLabel
          value={emptyValue}
          ariaLabel="Description"
          activationMode="enterOrSpace"
          emptyText="Add a description…"
          onCommit={(detail) => {
            setEmptyValue(detail.value);
            setLastEvent(`Committed: "${detail.value}"`);
          }}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Flush variant">
        <EditableLabel
          value={flushValue}
          ariaLabel="Heading"
          variant="flush"
          activationMode="enterOrSpace"
          showEditIcon
          onCommit={(detail) => setFlushValue(detail.value)}
        />
      </SpecimenGroup>

      <SpecimenGroup label="With max length">
        <EditableLabel
          value="Short text"
          ariaLabel="Short text"
          activationMode="enterOrSpace"
          maxLength={20}
          placeholder="Enter text…"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <EditableLabel
          value="Read-only value"
          ariaLabel="Read-only"
          disabled
        />
      </SpecimenGroup>

      {lastEvent ? (
        <SpecimenGroup label="Last event">
          <p style={{ margin: 0 }}>{lastEvent}</p>
        </SpecimenGroup>
      ) : null}
    </SpecimenLayout>
  );
}

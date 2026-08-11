import { useState } from "react";
import { SplitButton, Eyebrow, Surface, type MenuItem } from "@inflatable-cookie/poodle-react";
import { SpecimenLayout } from "../SpecimenLayout";

export function SplitButtonSpecimen() {
  const [lastAction, setLastAction] = useState("");

  const saveItems: MenuItem[] = [
    { value: "save-draft", label: "Save as draft" },
    { value: "save-template", label: "Save as template" },
    { value: "separator-1", label: "", kind: "separator" },
    { value: "discard", label: "Discard changes" },
  ];

  const exportItems: MenuItem[] = [
    { value: "csv", label: "Export as CSV" },
    { value: "json", label: "Export as JSON" },
    { value: "pdf", label: "Export as PDF" },
  ];

  return (
    <SpecimenLayout
      sizes={(size) => (
        <SplitButton variant="primary" items={saveItems} size={size}>
          Save
        </SplitButton>
      )}
      densities={(density) => (
        <SplitButton variant="primary" items={saveItems} density={density}>
          Save
        </SplitButton>
      )}
    >
      <Surface tone="panel" border="subtle" padding="md">
        <div className="poodle-specimen">
          <div className="poodle-specimen__row">
            <Eyebrow>Primary</Eyebrow>
            <SplitButton variant="primary" items={saveItems} onClick={() => setLastAction("Save")} onAction={(value) => setLastAction(value)}>
              Save
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Secondary</Eyebrow>
            <SplitButton variant="secondary" items={exportItems} onClick={() => setLastAction("Export")} onAction={(value) => setLastAction(value)}>
              Export
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Danger</Eyebrow>
            <SplitButton
              tone="danger"
              items={[
                { value: "delete-selected", label: "Delete selected" },
                { value: "delete-all", label: "Delete all" },
              ]}
              onClick={() => setLastAction("Delete")}
              onAction={(value) => setLastAction(value)}
            >
              Delete
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Success</Eyebrow>
            <SplitButton
              tone="success"
              items={[
                { value: "publish-now", label: "Publish now" },
                { value: "schedule-publish", label: "Schedule" },
              ]}
              onClick={() => setLastAction("Publish")}
              onAction={(value) => setLastAction(value)}
            >
              Publish
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Warning</Eyebrow>
            <SplitButton
              tone="warning"
              items={[
                { value: "archive-selected", label: "Archive selected" },
                { value: "archive-all", label: "Archive all" },
              ]}
              onClick={() => setLastAction("Archive")}
              onAction={(value) => setLastAction(value)}
            >
              Archive
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Loading</Eyebrow>
            <SplitButton variant="primary" items={saveItems} loading>
              Saving…
            </SplitButton>
          </div>

          <div className="poodle-specimen__row">
            <Eyebrow>Disabled</Eyebrow>
            <SplitButton variant="secondary" items={saveItems} disabled>
              Save
            </SplitButton>
          </div>

          {lastAction ? (
            <p style={{ margin: 0, fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" }}>
              Last action: <strong>{lastAction}</strong>
            </p>
          ) : null}
        </div>
      </Surface>
    </SpecimenLayout>
  );
}

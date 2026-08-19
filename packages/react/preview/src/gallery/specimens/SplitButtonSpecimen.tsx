import { useState } from "react";
import { SplitButton, type MenuItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const rowStyle = {
  display: "flex",
  flexWrap: "wrap",
  gap: "0.75rem",
  alignItems: "center",
} as const;

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
            <SpecimenGroup label="Save split action">
        <SplitButton variant="primary" items={saveItems} onClick={() => setLastAction("Save")} onAction={(value) => setLastAction(value)}>
                      Save
                    </SplitButton>
      </SpecimenGroup>

                <SpecimenGroup label="Secondary export">
        <SplitButton variant="secondary" items={exportItems} onClick={() => setLastAction("Export")} onAction={(value) => setLastAction(value)}>
                      Export
                    </SplitButton>
      </SpecimenGroup>

                <SpecimenGroup label="Intent tones">
        <div style={rowStyle}>
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
      </SpecimenGroup>

                <SpecimenGroup label="Loading and disabled states">
        <div style={rowStyle}>
          <SplitButton variant="primary" items={saveItems} loading>
                      Saving…
                    </SplitButton>
          <SplitButton variant="secondary" items={saveItems} disabled>
                      Save
                    </SplitButton>
        </div>
      </SpecimenGroup>

          {lastAction ? (
            <p style={{ margin: 0, fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" }}>
              Last action: <strong>{lastAction}</strong>
            </p>
          ) : null}
    </SpecimenLayout>
  );
}

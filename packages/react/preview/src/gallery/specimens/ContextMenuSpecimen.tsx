import { useState, type CSSProperties } from "react";
import { ContextMenu } from "@poodle/react";
import type { MenuItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const items: MenuItem[] = [
  { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
  { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
  { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
  { value: "sep1", label: "", kind: "separator" },
  { value: "select-all", label: "Select all", shortcutLabel: "⌘A" },
  { value: "sep2", label: "", kind: "separator" },
  { value: "delete", label: "Delete", disabled: true },
];

const targetArea: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "8rem",
  border: "2px dashed var(--poodle-color-border-default)",
  borderRadius: "4px",
};

const targetAreaSmall: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "4rem",
  padding: "0 1rem",
  border: "2px dashed var(--poodle-color-border-default)",
  borderRadius: "4px",
};

const targetText: CSSProperties = {
  margin: 0,
  fontSize: "0.875rem",
  color: "var(--poodle-color-text-secondary)",
};

export function ContextMenuSpecimen() {
  const [lastAction, setLastAction] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <ContextMenu items={items} size={size}>
          <div style={targetAreaSmall}>
            <p style={targetText}>{size.toUpperCase()}</p>
          </div>
        </ContextMenu>
      )}
      densities={(density) => (
        <ContextMenu items={items} density={density}>
          <div style={targetAreaSmall}>
            <p style={targetText}>{density}</p>
          </div>
        </ContextMenu>
      )}
    >
      <SpecimenGroup label="Right-click the area below">
        <ContextMenu items={items} onAction={(value) => setLastAction(value)}>
          <div style={targetArea}>
            <p style={targetText}>Right-click here to open context menu</p>
          </div>
        </ContextMenu>
        {lastAction ? (
          <p style={targetText}>Last action: <strong>{lastAction}</strong></p>
        ) : null}
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

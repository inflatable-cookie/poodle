import { useState } from "react";
import { ToggleGroup, type ToggleGroupOption } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const viewOptions: ToggleGroupOption[] = [
  { value: "grid", label: "Grid" },
  { value: "list", label: "List" },
  { value: "board", label: "Board" },
];

const alignOptions: ToggleGroupOption[] = [
  { value: "left", label: "Left" },
  { value: "center", label: "Center" },
  { value: "right", label: "Right" },
  { value: "justify", label: "Justify" },
];

const tagOptions: ToggleGroupOption[] = [
  { value: "design", label: "Design" },
  { value: "engineering", label: "Engineering" },
  { value: "docs", label: "Docs" },
];

const hintStyle = { margin: 0, fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" } as const;

export function ToggleGroupSpecimen() {
  const [view, setView] = useState("grid");
  const [tags, setTags] = useState<string[]>(["design", "docs"]);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <ToggleGroup options={viewOptions} defaultValue="grid" size={size} ariaLabel={size + " toggle group"} />
      )}
      densities={(density) => (
        <ToggleGroup options={viewOptions} defaultValue="grid" density={density} ariaLabel={density + " toggle group"} />
      )}
    >
      <SpecimenGroup label="Single selection">
        <ToggleGroup
          options={viewOptions}
          value={view}
          ariaLabel="View mode"
          onValueChange={(value) => setView(value as string)}
        />
        <p style={hintStyle}>View: <strong>{view}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Four options">
        <ToggleGroup options={alignOptions} defaultValue="left" ariaLabel="Text alignment" />
      </SpecimenGroup>

      <SpecimenGroup label="Multiple selection">
        <ToggleGroup
          options={tagOptions}
          value={tags}
          selectionMode="multiple"
          ariaLabel="Filter tags"
          onValueChange={(value) => setTags(value as string[])}
        />
        <p style={hintStyle}>Selected: <strong>{tags.join(", ") || "none"}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <ToggleGroup options={viewOptions} defaultValue="list" disabled ariaLabel="Disabled toggle group" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

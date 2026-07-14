import { useState } from "react";
import { SelectionSummary, type SelectionSummaryItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const variantItems: SelectionSummaryItem[] = [
  { id: "1", label: "Button" },
  { id: "2", label: "Card" },
  { id: "3", label: "Dialog" },
];

const variantStyle = { width: "min(100%, 28rem)" };

export function SelectionSummarySpecimen() {
  const [items, setItems] = useState<SelectionSummaryItem[]>([
    { id: "1", label: "Button" },
    { id: "2", label: "Card" },
    { id: "3", label: "Dialog" },
    { id: "4", label: "Table" },
    { id: "5", label: "Tabs" },
  ]);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={variantStyle}>
          <SelectionSummary items={variantItems} size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <SelectionSummary items={variantItems} density={density} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Multiple items selected">
          <SelectionSummary
            items={items}
            onRemove={(id) => setItems((prev) => prev.filter((item) => item.id !== id))}
            onClear={() => setItems([])}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Single item">
          <SelectionSummary items={[{ id: "1", label: "Primary button" }]} />
        </SpecimenGroup>

        <SpecimenGroup label="Truncated (max 3 visible)">
          <SelectionSummary
            items={[
              { id: "a", label: "Alpha" },
              { id: "b", label: "Beta" },
              { id: "c", label: "Gamma" },
              { id: "d", label: "Delta" },
              { id: "e", label: "Epsilon" },
              { id: "f", label: "Zeta" },
            ]}
            maxVisibleItems={3}
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

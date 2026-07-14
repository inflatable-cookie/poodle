import { useState } from "react";
import { CardToggleGroup } from "@poodle/react";
import type { CardToggleItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const statusItems: CardToggleItem[] = [
  { value: "pending", label: "Pending", description: "Waiting for review.", count: 42 },
  { value: "marked", label: "Marked", description: "Reviewed and complete.", count: 128 },
  { value: "void", label: "Void", description: "Removed from the active queue.", count: 6 },
  { value: "all", label: "All", description: "Every item in the collection.", count: 176 },
];

const compactItems: CardToggleItem[] = [
  { value: "draft", label: "Draft", count: 8 },
  { value: "live", label: "Live", count: 21 },
  { value: "archived", label: "Archived", count: 4, disabled: true },
];

export function CardToggleGroupSpecimen() {
  const [statusValue, setStatusValue] = useState<string | null>("pending");
  const [optionalValue, setOptionalValue] = useState<string | null>("marked");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ width: "min(100%, 42rem)" }}>
          <CardToggleGroup items={compactItems} value="live" columns={3} size={size} ariaLabel={`Card toggle group at ${size}`} />
        </div>
      )}
      densities={(density) => (
        <div style={{ width: "min(100%, 42rem)" }}>
          <CardToggleGroup items={compactItems} value="live" columns={3} density={density} ariaLabel={`Card toggle group at ${density} density`} />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <SpecimenGroup label="Query variants">
          <CardToggleGroup
            items={statusItems}
            value={statusValue}
            columns={4}
            ariaLabel="Select answer status"
            onValueChange={(value) => setStatusValue(value)}
          />
          <p style={{ margin: 0 }}>Selected: <strong>{statusValue ?? "none"}</strong></p>
        </SpecimenGroup>

        <SpecimenGroup label="Deactivation allowed">
          <CardToggleGroup
            items={compactItems}
            value={optionalValue}
            columns={3}
            allowDeactivation
            ariaLabel="Select optional status"
            onValueChange={(value) => setOptionalValue(value)}
          />
          <p style={{ margin: 0 }}>Selected: <strong>{optionalValue ?? "none"}</strong></p>
        </SpecimenGroup>

        <SpecimenGroup label="Disabled group">
          <CardToggleGroup
            items={compactItems}
            value="live"
            columns={3}
            disabled
            ariaLabel="Disabled card toggle group"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

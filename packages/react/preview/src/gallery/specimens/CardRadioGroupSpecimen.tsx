import { useState } from "react";
import { CardRadioGroup } from "@inflatable-cookie/poodle-react";
import type { CardRadioItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const planItems: CardRadioItem[] = [
  { value: "free", label: "Free", description: "Basic features for personal use. Up to 3 projects." },
  { value: "pro", label: "Pro", description: "Advanced features for professionals. Unlimited projects." },
  { value: "team", label: "Team", description: "Collaboration tools for teams. Shared workspace included." },
  { value: "enterprise", label: "Enterprise", description: "Custom solutions for large organizations.", disabled: true },
];

const sizeItems: CardRadioItem[] = [
  { value: "sm", label: "Small", description: "1 CPU, 512 MB RAM" },
  { value: "md", label: "Medium", description: "2 CPU, 2 GB RAM" },
  { value: "lg", label: "Large", description: "4 CPU, 8 GB RAM" },
];

export function CardRadioGroupSpecimen() {
  const [planValue, setPlanValue] = useState<string | null>("pro");
  const [sizeValue, setSizeValue] = useState<string | null>(null);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ width: "min(100%, 40rem)" }}>
          <CardRadioGroup items={sizeItems} value="md" columns={3} size={size} ariaLabel={`Card radio group at ${size}`} />
        </div>
      )}
      densities={(density) => (
        <div style={{ width: "min(100%, 40rem)" }}>
          <CardRadioGroup items={sizeItems} value="md" columns={3} density={density} ariaLabel={`Card radio group at ${density} density`} />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <SpecimenGroup label="Plan selection (2 columns)">
          <CardRadioGroup
            items={planItems}
            value={planValue}
            columns={2}
            ariaLabel="Select a plan"
            onValueChange={(value) => setPlanValue(value)}
          />
          {planValue ? (
            <p style={{ margin: 0 }}>Selected: <strong>{planValue}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Instance size (3 columns)">
          <CardRadioGroup
            items={sizeItems}
            value={sizeValue}
            columns={3}
            ariaLabel="Select an instance size"
            onValueChange={(value) => setSizeValue(value)}
          />
          {sizeValue ? (
            <p style={{ margin: 0 }}>Selected: <strong>{sizeValue}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Disabled group">
          <CardRadioGroup
            items={sizeItems}
            value="md"
            columns={3}
            disabled
            ariaLabel="Disabled selection"
          />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

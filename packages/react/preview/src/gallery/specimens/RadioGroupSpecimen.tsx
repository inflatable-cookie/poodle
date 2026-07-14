import { useState } from "react";
import { RadioGroup, type RadioGroupOption } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const planOptions: RadioGroupOption[] = [
  { value: "free", label: "Free" },
  { value: "pro", label: "Pro" },
  { value: "enterprise", label: "Enterprise" },
];

const sizeOptions: RadioGroupOption[] = [
  { value: "sm", label: "Small" },
  { value: "md", label: "Medium" },
  { value: "lg", label: "Large" },
  { value: "xl", label: "Extra large" },
];

export function RadioGroupSpecimen() {
  const [selectedPlan, setSelectedPlan] = useState("pro");
  const [selectedSize, setSelectedSize] = useState("md");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <RadioGroup
          options={planOptions}
          defaultValue="pro"
          orientation="horizontal"
          size={size}
          ariaLabel={`Plan at ${size}`}
        />
      )}
      densities={(density) => (
        <RadioGroup
          options={planOptions}
          defaultValue="pro"
          orientation="horizontal"
          density={density}
          ariaLabel={`Plan at ${density} density`}
        />
      )}
    >
      <SpecimenGroup label="Vertical (default)">
        <RadioGroup
          options={planOptions}
          value={selectedPlan}
          ariaLabel="Select plan"
          onValueChange={setSelectedPlan}
        />
        <p>
          Selected: <strong>{selectedPlan}</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Horizontal">
        <RadioGroup
          options={sizeOptions}
          value={selectedSize}
          orientation="horizontal"
          ariaLabel="Select size"
          onValueChange={setSelectedSize}
        />
        <p>
          Selected: <strong>{selectedSize}</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <RadioGroup options={planOptions} defaultValue="free" disabled ariaLabel="Disabled plan selector" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom selected color">
        <RadioGroup
          options={planOptions}
          value={selectedPlan}
          selectedColor="#22c55e"
          ariaLabel="Select plan with custom selected color"
          onValueChange={setSelectedPlan}
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

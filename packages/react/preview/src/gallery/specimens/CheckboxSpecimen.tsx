import { useState } from "react";
import { Checkbox } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function CheckboxSpecimen() {
  const [notifications, setNotifications] = useState(true);
  const [marketing, setMarketing] = useState(false);
  const [terms, setTerms] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => <Checkbox label="Accept terms" size={size} />}
      densities={(density) => <Checkbox id={`density-${density}`} label="Option" density={density} />}
    >
      <SpecimenGroup label="Default">
        <Checkbox label="Enable email notifications" checked={notifications} onCheckedChange={setNotifications} />
        <Checkbox label="Subscribe to marketing emails" checked={marketing} onCheckedChange={setMarketing} />
        <Checkbox label="I agree to the terms and conditions" checked={terms} onCheckedChange={setTerms} />
      </SpecimenGroup>

      <SpecimenGroup label="States">
        <Checkbox label="Disabled unchecked" disabled />
        <Checkbox label="Disabled checked" checked disabled />
        <Checkbox label="Mixed / indeterminate" mixed />
        <Checkbox label="Read-only checked" checked readOnly />
      </SpecimenGroup>

      <SpecimenGroup label="Custom selected color">
        <Checkbox label="Billable feature" checked selectedColor="#22c55e" />
        <Checkbox label="Requires moderation" checked selectedColor="#f59e0b" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

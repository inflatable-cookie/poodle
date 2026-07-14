import { useState } from "react";
import { Switch } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function SwitchSpecimen() {
  const [darkMode, setDarkMode] = useState(true);
  const [autoSave, setAutoSave] = useState(false);
  const [compactView, setCompactView] = useState(true);

  return (
    <SpecimenLayout
      sizes={(size) => <Switch label="Enabled" size={size} />}
      densities={(density) => <Switch id={`density-${density}`} label="Toggle" density={density} />}
    >
      <SpecimenGroup label="Default">
        <Switch label="Dark mode" checked={darkMode} onCheckedChange={setDarkMode} />
        <Switch label="Auto-save drafts" checked={autoSave} onCheckedChange={setAutoSave} />
        <Switch label="Compact view" checked={compactView} onCheckedChange={setCompactView} />
      </SpecimenGroup>

      <SpecimenGroup label="States">
        <Switch label="Disabled off" disabled />
        <Switch label="Disabled on" checked disabled />
        <Switch label="Read-only on" checked readOnly />
      </SpecimenGroup>

      <SpecimenGroup label="Custom colors">
        <Switch label="Billing alerts" checked onColor="#22c55e" offColor="#cbd5e1" />
        <Switch label="Quiet mode" checked={false} onColor="#f59e0b" offColor="#94a3b8" />
      </SpecimenGroup>

      <SpecimenGroup label="Dual labels and tones">
        <Switch
          defaultChecked
          leftLabel="Draft"
          rightLabel="Live"
          leftTone="danger"
          rightTone="success"
          ariaLabel="Publication status"
        />
        <Switch
          defaultChecked={false}
          leftLabel="Restricted"
          rightLabel="Free"
          leftTone="warning"
          rightTone="success"
          ariaLabel="Access status"
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

import { useState } from "react";
import { TriStateSwitch, type TriStateValue } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const itemStyle = { display: "flex", alignItems: "center", gap: "0.75rem" } as const;
const valueStyle = { fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" } as const;

export function TriStateSwitchSpecimen() {
  const [filter, setFilter] = useState<TriStateValue>("default");

  return (
    <SpecimenLayout
      sizes={(size) => <TriStateSwitch value="default" size={size} ariaLabel={"Switch at " + size} />}
      densities={(density) => <TriStateSwitch value="default" density={density} ariaLabel={"Switch at " + density} />}
    >
      <SpecimenGroup label="Default">
        <div style={itemStyle}>
          <TriStateSwitch value={filter} ariaLabel="Filter mode" onValueChange={(value) => setFilter(value)} />
          <span style={valueStyle}>{filter}</span>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Custom labels">
        <TriStateSwitch options={{ excluded: "Hide", default: "All", included: "Show" }} ariaLabel="Visibility filter" />
      </SpecimenGroup>

      <SpecimenGroup label="Custom colors">
        <TriStateSwitch
          value={filter}
          excludedColor="#ef4444"
          defaultColor="#64748b"
          includedColor="#22c55e"
          ariaLabel="Custom colors"
          onValueChange={(value) => setFilter(value)}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <TriStateSwitch value="included" disabled ariaLabel="Disabled" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

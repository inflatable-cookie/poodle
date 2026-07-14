import { useState } from "react";
import { TriStateSwitch, Eyebrow, Surface, type TriStateValue } from "@poodle/react";
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
      <Surface tone="panel" border="subtle" padding="md">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
          <div style={itemStyle}>
            <Eyebrow>Default</Eyebrow>
            <TriStateSwitch value={filter} ariaLabel="Filter mode" onValueChange={(value) => setFilter(value)} />
            <span style={valueStyle}>{filter}</span>
          </div>

          <div style={itemStyle}>
            <Eyebrow>Custom labels</Eyebrow>
            <TriStateSwitch options={{ excluded: "Hide", default: "All", included: "Show" }} ariaLabel="Visibility filter" />
          </div>

          <div style={itemStyle}>
            <Eyebrow>Custom colors</Eyebrow>
            <TriStateSwitch value={filter} excludedColor="#ef4444" defaultColor="#64748b" includedColor="#22c55e" ariaLabel="Custom colors" onValueChange={(value) => setFilter(value)} />
          </div>

          <div style={itemStyle}>
            <Eyebrow>Disabled</Eyebrow>
            <TriStateSwitch value="included" disabled ariaLabel="Disabled" />
          </div>
        </div>
      </Surface>
    </SpecimenLayout>
  );
}

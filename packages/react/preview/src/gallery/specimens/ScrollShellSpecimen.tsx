import { ScrollShell, Surface } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function ScrollShellSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Vertical scroll">
        <div
          style={{
            height: "10rem",
            border: "1px solid var(--poodle-color-border-default)",
            borderRadius: "4px",
          }}
        >
          <ScrollShell direction="vertical" label="Scrollable content">
            {Array.from({ length: 12 }).map((_, i) => (
              <Surface key={i} padding="sm" border="subtle">
                <p style={{ margin: 0, fontSize: "0.875rem" }}>
                  Item {i + 1} — Scroll to see more content below
                </p>
              </Surface>
            ))}
          </ScrollShell>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Horizontal scroll">
        <ScrollShell direction="horizontal" label="Horizontal items">
          <div style={{ display: "flex", gap: "0.5rem" }}>
            {Array.from({ length: 10 }).map((_, i) => (
              <Surface key={i} padding="sm" border="subtle">
                <p style={{ margin: 0, fontSize: "0.875rem", whiteSpace: "nowrap" }}>Column {i + 1}</p>
              </Surface>
            ))}
          </div>
        </ScrollShell>
      </SpecimenGroup>
    </div>
  );
}

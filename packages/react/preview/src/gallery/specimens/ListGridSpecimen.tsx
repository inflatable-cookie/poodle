import type { CSSProperties } from "react";
import { Button, IconButton, ListGrid, Surface } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const tileTitle: CSSProperties = { margin: "0 0 0.25rem", fontSize: "0.875rem", fontWeight: 600 };
const tileBody: CSSProperties = {
  margin: 0,
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary, #888)",
};

export function ListGridSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Default — auto-fill up to three columns">
        <ListGrid minItemWidth={14}>
          {["Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot"].map((title) => (
            <Surface key={title} padding="md" border="subtle">
              <p style={tileTitle}>{title}</p>
              <p style={tileBody}>
                Responsive columns from <code style={{ fontSize: "0.75em" }}>minItemWidth</code>.
              </p>
            </Surface>
          ))}
        </ListGrid>
      </SpecimenGroup>

      <SpecimenGroup label="With header actions">
        <ListGrid
          minItemWidth={16}
          actions={
            <>
              <Button variant="secondary">Export</Button>
              <IconButton icon="refresh-cw" ariaLabel="Refresh list" variant="secondary" />
            </>
          }
        >
          {["Project A", "Project B", "Project C"].map((title) => (
            <Surface key={title} padding="md" border="subtle">
              <p style={tileTitle}>{title}</p>
              <p style={tileBody}>Actions row uses contract header anatomy.</p>
            </Surface>
          ))}
        </ListGrid>
      </SpecimenGroup>

      <SpecimenGroup label="Compact — single column stack">
        <ListGrid variant="compact">
          {["One", "Two", "Three"].map((title) => (
            <Surface key={title} padding="sm" border="subtle">
              <p style={tileTitle}>{title}</p>
              <p style={tileBody}>Compact uses one column and tighter default gap.</p>
            </Surface>
          ))}
        </ListGrid>
      </SpecimenGroup>
    </div>
  );
}

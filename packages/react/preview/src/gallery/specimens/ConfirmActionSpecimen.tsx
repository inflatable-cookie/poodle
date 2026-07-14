import { useState } from "react";
import { Button, ConfirmAction } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function ConfirmActionSpecimen() {
  const [lastAction, setLastAction] = useState("");

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
      <SpecimenGroup label="Default trigger (danger)">
        <ConfirmAction
          title="Delete this record?"
          description="This record will be permanently removed."
          triggerLabel="Delete record"
          confirmLabel="Delete"
          onConfirm={() => setLastAction("Record deleted")}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Warning tone">
        <ConfirmAction
          title="Archive this project?"
          description="The project will be moved to the archive and can be restored later."
          tone="warning"
          triggerLabel="Archive project"
          confirmLabel="Archive"
          onConfirm={() => setLastAction("Project archived")}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Custom trigger slot">
        <ConfirmAction
          title="Remove all filters?"
          description="This will clear all active filters and show all items."
          tone="warning"
          confirmLabel="Clear all"
          onConfirm={() => setLastAction("Filters cleared")}
          trigger={<Button variant="ghost">Clear filters</Button>}
        />
      </SpecimenGroup>

      <SpecimenGroup label="With body content">
        <ConfirmAction
          title="Revoke API key?"
          description="This key will immediately stop working."
          confirmLabel="Revoke"
          onConfirm={() => setLastAction("Key revoked")}
        >
          <div
            style={{
              padding: "0.5rem 0.75rem",
              borderRadius: "0.375rem",
              background: "var(--poodle-color-background-panel, #1a1a1a)",
            }}
          >
            <code style={{ fontFamily: "var(--poodle-typography-mono-family, monospace)", fontSize: "0.8125rem" }}>
              pk_live_abc123...xyz789
            </code>
          </div>
        </ConfirmAction>
      </SpecimenGroup>

      {lastAction ? (
        <SpecimenGroup label="Last action">
          <p style={{ margin: 0 }}>{lastAction}</p>
        </SpecimenGroup>
      ) : null}
    </div>
  );
}

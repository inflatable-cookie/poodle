import { SpecimenGroup } from "../SpecimenGroup";
import { useState, type CSSProperties } from "react";
import { AlertDialog, Button } from "@inflatable-cookie/poodle-react";
import { SpecimenLayout } from "../SpecimenLayout";

const hintStyle: CSSProperties = { margin: 0, fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" };
const cardStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.125rem",
  padding: "0.5rem 0.75rem",
  borderRadius: "0.375rem",
  background: "var(--poodle-color-background-panel)",
};
const cardSpanStyle: CSSProperties = { color: "var(--poodle-color-text-secondary)", fontSize: "0.8125rem" };

export function AlertDialogSpecimen() {
  const [dangerOpen, setDangerOpen] = useState(false);
  const [warningOpen, setWarningOpen] = useState(false);
  const [asyncOpen, setAsyncOpen] = useState(false);
  const [lastAction, setLastAction] = useState("");

  async function simulateAsync(): Promise<void> {
    await new Promise((resolve) => setTimeout(resolve, 1500));
    setLastAction("Async confirm completed");
  }

  return (
    <SpecimenLayout
      sizes={(size) => (
        <AlertDialog
          open={true}
          title="Delete this item?"
          description="This action cannot be undone. The item and all associated data will be permanently removed."
          confirmLabel="Delete"
          cancelLabel="Keep it"
          size={size}
          onOpenChange={() => {}}
        />
      )}
      densities={(density) => (
        <AlertDialog
          open={true}
          title="Delete this item?"
          description="This action cannot be undone. The item and all associated data will be permanently removed."
          confirmLabel="Delete"
          cancelLabel="Keep it"
          density={density}
          onOpenChange={() => {}}
        />
      )}
    >
      <SpecimenGroup label="Danger tone">
        <Button tone="danger" onClick={() => setDangerOpen(true)}>
                    Delete item
                  </Button>
                  <AlertDialog
                    open={dangerOpen}
                    title="Delete this item?"
                    description="This action cannot be undone. The item and all associated data will be permanently removed."
                    confirmLabel="Delete"
                    cancelLabel="Keep it"
                    onConfirm={() => {
                      setLastAction("Item deleted");
                      setDangerOpen(false);
                    }}
                    onCancel={() => setDangerOpen(false)}
                    onOpenChange={(open) => setDangerOpen(open)}
                  />
      </SpecimenGroup>

              <SpecimenGroup label="Warning tone">
        <Button variant="secondary" onClick={() => setWarningOpen(true)}>
                    Reset settings
                  </Button>
                  <AlertDialog
                    open={warningOpen}
                    title="Reset all settings?"
                    description="Your customized settings will be restored to their default values."
                    tone="warning"
                    confirmLabel="Reset"
                    cancelLabel="Cancel"
                    onConfirm={() => {
                      setLastAction("Settings reset");
                      setWarningOpen(false);
                    }}
                    onCancel={() => setWarningOpen(false)}
                    onOpenChange={(open) => setWarningOpen(open)}
                  />
      </SpecimenGroup>

              <SpecimenGroup label="Async confirm">
        <Button tone="danger" onClick={() => setAsyncOpen(true)}>
                    Archive project
                  </Button>
                  <AlertDialog
                    open={asyncOpen}
                    title="Archive this project?"
                    description="The project will be hidden from active lists but can still be restored later."
                    confirmLabel="Archive"
                    workingLabel="Archiving…"
                    onConfirm={async () => {
                      await simulateAsync();
                      setAsyncOpen(false);
                    }}
                    onCancel={() => setAsyncOpen(false)}
                    onOpenChange={(open) => setAsyncOpen(open)}
                  >
                    <div style={cardStyle}>
                      <strong>Roadmap Cleanup</strong>
                      <span style={cardSpanStyle}>14 linked tasks will move to the archived view.</span>
                    </div>
                  </AlertDialog>
      </SpecimenGroup>

      {lastAction ? (
        <p style={hintStyle}>
          Last action: <strong>{lastAction}</strong>
        </p>
      ) : null}
    </SpecimenLayout>
  );
}

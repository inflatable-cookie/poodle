import { SpecimenGroup } from "../SpecimenGroup";
import { useState, type CSSProperties } from "react";
import { Button, Checkbox, Dialog, Field, Pill, Popover, Select, TextInput } from "@inflatable-cookie/poodle-react";
import { SpecimenLayout } from "../SpecimenLayout";

const WIDTHS = ["sm", "md", "lg", "xl"] as const;
type DialogWidth = (typeof WIDTHS)[number];

const textSecondary = "var(--poodle-color-text-secondary)";
const codeFamily = "var(--poodle-typography-code-family)";

const kbdStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  minWidth: "4.5rem",
  padding: "0.25rem 0.5rem",
  border: "0.0625rem solid var(--poodle-color-border-default)",
  borderRadius: "0.25rem",
  background: "color-mix(in srgb, var(--poodle-color-background-panel) 80%, var(--poodle-color-background-elevated))",
  fontFamily: codeFamily,
  fontSize: "0.75rem",
  fontWeight: 500,
};

const shortcuts: Array<[string, string]> = [
  ["⌘ K", "Command palette"],
  ["⌘ S", "Save"],
  ["⌘ /", "Toggle comment"],
  ["⌘ ⇧ P", "Quick actions"],
  ["Esc", "Close dialog"],
];

const logMessages = [
  "User signed in",
  "Project created",
  "File uploaded",
  "Settings updated",
  "Comment added",
  "Build completed",
  "Deploy started",
  "Review requested",
];

export function DialogSpecimen() {
  const [basicOpen, setBasicOpen] = useState(false);
  const [formOpen, setFormOpen] = useState(false);
  const [contentOnlyOpen, setContentOnlyOpen] = useState(false);
  const [customFooterOpen, setCustomFooterOpen] = useState(false);
  const [bareOpen, setBareOpen] = useState(false);
  const [wideOpen, setWideOpen] = useState(false);
  const [scrollableOpen, setScrollableOpen] = useState(false);
  const [widthOpenMap, setWidthOpenMap] = useState<Record<string, boolean>>({});
  const [overlayInDialogOpen, setOverlayInDialogOpen] = useState(false);

  const setWidthOpen = (w: string, open: boolean) => setWidthOpenMap((prev) => ({ ...prev, [w]: open }));

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Dialog defaultOpen title="Keyboard shortcuts" showCloseButton size={size}>
          <p>Command palette, save, and toggle comment live here.</p>
        </Dialog>
      )}
      densities={(density) => (
        <Dialog defaultOpen title="Keyboard shortcuts" showCloseButton density={density}>
          <p>Command palette, save, and toggle comment live here.</p>
        </Dialog>
      )}
    >
      <SpecimenGroup label="Popover inside a dialog">
        <Button variant="secondary" onClick={() => setOverlayInDialogOpen(true)}>
                      Open dialog
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Informational">
        <Button variant="secondary" onClick={() => setBasicOpen(true)}>
                      View details
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Form">
        <Button variant="secondary" onClick={() => setFormOpen(true)}>
                      Create project
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Custom header">
        <Button variant="secondary" onClick={() => setContentOnlyOpen(true)}>
                      View changelog
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Custom footer">
        <Button variant="secondary" onClick={() => setCustomFooterOpen(true)}>
                      Terms &amp; conditions
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Bare mode">
        <Button variant="secondary" onClick={() => setBareOpen(true)}>
                      Preview image
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Scrollable">
        <Button variant="secondary" onClick={() => setScrollableOpen(true)}>
                      View log
                    </Button>
      </SpecimenGroup>

                <SpecimenGroup label="Width presets">
        {WIDTHS.map((w) => (
                      <Button key={w} variant="secondary" onClick={() => setWidthOpen(w, true)}>
                        {w}
                      </Button>
                    ))}
      </SpecimenGroup>

                <SpecimenGroup label="Non-dismissible">
        <Button variant="secondary" onClick={() => setWideOpen(true)}>
                      Open persistent
                    </Button>
      </SpecimenGroup>

      {/* Dialogs (rendered outside the Surface, portaled to [data-theme]) */}

      <Dialog open={basicOpen} title="Keyboard shortcuts" showCloseButton onOpenChange={setBasicOpen}>
        <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
          {shortcuts.map(([keys, label]) => (
            <div key={label} style={{ display: "flex", alignItems: "center", gap: "0.75rem" }}>
              <kbd style={kbdStyle}>{keys}</kbd>
              <span style={{ color: textSecondary, fontSize: "0.8125rem" }}>{label}</span>
            </div>
          ))}
        </div>
      </Dialog>

      <Dialog
        open={formOpen}
        title="New project"
        description="Set up a new project workspace."
        width="lg"
        showCloseButton
        onOpenChange={setFormOpen}
        actions={
          <>
            <Button variant="ghost" onClick={() => setFormOpen(false)}>
              Cancel
            </Button>
            <Button onClick={() => setFormOpen(false)}>Create project</Button>
          </>
        }
      >
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1rem" }}>
          <Field label="Project name" id="dialog-project-name">
            <TextInput id="dialog-proj-name" placeholder="My project" />
          </Field>
          <Field label="Template" id="dialog-template">
            <Select
              id="dialog-template"
              placeholder="Choose a template"
              options={[
                { value: "blank", label: "Blank" },
                { value: "starter", label: "Starter kit" },
                { value: "advanced", label: "Advanced" },
              ]}
            />
          </Field>
          <div style={{ gridColumn: "1 / -1" }}>
            <Field label="Description" id="dialog-description">
              <TextInput id="dialog-desc" placeholder="What is this project for?" rows={3} />
            </Field>
          </div>
          <div style={{ gridColumn: "1 / -1" }}>
            <Checkbox id="dialog-private" label="Make this project private" />
          </div>
        </div>
      </Dialog>

      <Dialog
        open={contentOnlyOpen}
        showCloseButton
        onOpenChange={setContentOnlyOpen}
        header={
          <div style={{ display: "flex", alignItems: "center", gap: "0.625rem" }}>
            <h2
              style={{
                margin: 0,
                fontFamily: "var(--poodle-typography-heading-family)",
                fontSize: "1rem",
                fontWeight: 600,
                lineHeight: 1.2,
              }}
            >
              What's new
            </h2>
            <Pill tone="info" appearance="badge">
              v2.4.0
            </Pill>
          </div>
        }
      >
        <div style={{ display: "flex", flexDirection: "column", gap: "0.875rem" }}>
          <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
            <strong style={{ fontSize: "0.875rem" }}>Dialog flexibility improvements</strong>
            <p style={{ margin: 0, color: textSecondary, fontSize: "0.8125rem", lineHeight: 1.5 }}>
              Dialogs now support custom headers, footers, width presets, and bare mode.
            </p>
          </div>
          <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
            <strong style={{ fontSize: "0.875rem" }}>Size propagation fixes</strong>
            <p style={{ margin: 0, color: textSecondary, fontSize: "0.8125rem", lineHeight: 1.5 }}>
              All parent components now correctly forward size and density to embedded children.
            </p>
          </div>
        </div>
      </Dialog>

      <Dialog
        open={customFooterOpen}
        title="Terms of service"
        showCloseButton
        onOpenChange={setCustomFooterOpen}
        footer={
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <a
              href="#terms"
              style={{ color: "var(--poodle-color-accent-base)", fontSize: "0.8125rem", textDecoration: "none" }}
            >
              Read full terms
            </a>
            <div style={{ display: "flex", gap: "0.5rem" }}>
              <Button variant="ghost" onClick={() => setCustomFooterOpen(false)}>
                Decline
              </Button>
              <Button onClick={() => setCustomFooterOpen(false)}>Accept</Button>
            </div>
          </div>
        }
      >
        <div>
          <p style={{ margin: "0 0 0.5rem", color: textSecondary, fontSize: "0.8125rem", lineHeight: 1.6 }}>
            By using this service, you agree to our terms and conditions.
          </p>
        </div>
      </Dialog>

      <Dialog open={bareOpen} bare width="lg" ariaLabel="Image preview" onOpenChange={setBareOpen}>
        <div style={{ display: "flex", flexDirection: "column" }}>
          <div
            style={{
              display: "grid",
              placeItems: "center",
              minHeight: "20rem",
              background: "color-mix(in srgb, var(--poodle-color-background-canvas) 90%, black)",
              borderRadius: "var(--poodle-radius-surface) var(--poodle-radius-surface) 0 0",
            }}
          >
            <span style={{ color: textSecondary, fontFamily: codeFamily, fontSize: "0.875rem", opacity: 0.5 }}>
              2400 × 1600
            </span>
          </div>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              gap: "1rem",
              padding: "0.75rem 1rem",
              borderTop: "0.0625rem solid var(--poodle-color-border-subtle)",
            }}
          >
            <div style={{ display: "flex", flexDirection: "column", gap: "0.125rem" }}>
              <strong style={{ fontSize: "0.8125rem" }}>landscape-hero.png</strong>
              <span style={{ color: textSecondary, fontSize: "0.75rem" }}>2.4 MB · Uploaded today</span>
            </div>
            <div style={{ display: "flex", gap: "0.5rem" }}>
              <Button variant="ghost" onClick={() => setBareOpen(false)}>
                Close
              </Button>
              <Button leadingIcon="download" onClick={() => setBareOpen(false)}>
                Download
              </Button>
            </div>
          </div>
        </div>
      </Dialog>

      <Dialog
        open={scrollableOpen}
        title="Activity log"
        description="Recent activity across all projects."
        showCloseButton
        onOpenChange={setScrollableOpen}
        actions={
          <>
            <Button variant="ghost" onClick={() => setScrollableOpen(false)}>
              Close
            </Button>
            <Button onClick={() => setScrollableOpen(false)}>Export log</Button>
          </>
        }
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: "0.25rem",
            maxHeight: "18rem",
            overflowY: "auto",
          }}
        >
          {Array.from({ length: 20 }, (_, i) => (
            <div
              key={i}
              style={{
                display: "flex",
                gap: "0.75rem",
                padding: "0.375rem 0",
                borderBottom: "0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 50%, transparent)",
              }}
            >
              <span style={{ color: textSecondary, fontFamily: codeFamily, fontSize: "0.75rem", minWidth: "3rem" }}>
                {String(9 + Math.floor(i / 3)).padStart(2, "0")}:{String((i * 17) % 60).padStart(2, "0")}
              </span>
              <span style={{ fontSize: "0.8125rem" }}>{logMessages[i % 8]}</span>
            </div>
          ))}
        </div>
      </Dialog>

      {WIDTHS.map((w) => (
        <Dialog
          key={w}
          open={widthOpenMap[w] ?? false}
          width={w as DialogWidth}
          title={`Width: ${w}`}
          showCloseButton
          onOpenChange={(open) => setWidthOpen(w, open)}
          actions={<Button onClick={() => setWidthOpen(w, false)}>Close</Button>}
        >
          <p>
            This dialog uses <code>{`width="${w}"`}</code>.
          </p>
        </Dialog>
      ))}

      <Dialog
        open={wideOpen}
        title="Processing"
        dismissOnBackdrop={false}
        dismissOnEscape={false}
        onOpenChange={setWideOpen}
        actions={<Button onClick={() => setWideOpen(false)}>Done</Button>}
      >
        <p>This dialog cannot be dismissed by clicking the backdrop or pressing Escape.</p>
      </Dialog>

      <Dialog open={overlayInDialogOpen} title="Settings" showCloseButton onOpenChange={setOverlayInDialogOpen}>
        <Field id="dialog-model" label="Model">
          <Popover
            ariaLabel="Model settings"
            trigger={<Button variant="secondary">Opus 5 · Medium</Button>}
          >
            <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem", minWidth: "14rem" }}>
              <strong>Opus 5</strong>
              <span>Reasoning depth. Higher costs more time and tokens.</span>
            </div>
          </Popover>
        </Field>
      </Dialog>
    </SpecimenLayout>
  );
}

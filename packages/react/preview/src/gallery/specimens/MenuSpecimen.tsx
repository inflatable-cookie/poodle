import { useState } from "react";
import { Menu, Button, Eyebrow, Surface, type MenuItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const fileItems: MenuItem[] = [
  { value: "new", label: "New file", shortcutLabel: "⌘N" },
  { value: "open", label: "Open…", shortcutLabel: "⌘O" },
  { value: "save", label: "Save", shortcutLabel: "⌘S" },
  { value: "sep1", label: "", kind: "separator" },
  { value: "export", label: "Export as PDF" },
  { value: "print", label: "Print…", shortcutLabel: "⌘P", disabled: true },
];

const settingsItems: MenuItem[] = [
  { value: "theme", label: "Dark mode", kind: "checkbox", checked: true },
  { value: "notifications", label: "Notifications", kind: "checkbox" },
  { value: "sep1", label: "", kind: "separator" },
  { value: "settings", label: "Settings…" },
];

const destructiveItems: MenuItem[] = [
  { value: "rename", label: "Rename" },
  { value: "archive", label: "Archive" },
  { value: "sep1", label: "", kind: "separator" },
  { value: "delete", label: "Delete", tone: "danger" },
];

export function MenuSpecimen() {
  const [lastAction, setLastAction] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Menu
          items={fileItems}
          size={size}
          ariaLabel={size + " menu"}
          trigger={
            <Button variant="secondary" size={size}>
              {size.toUpperCase()}
            </Button>
          }
        />
      )}
      densities={(density) => (
        <Menu
          items={fileItems}
          density={density}
          ariaLabel={density + " menu"}
          trigger={
            <Button variant="secondary" density={density}>
              {density}
            </Button>
          }
        />
      )}
    >
            <SpecimenGroup label="With shortcuts">
        <Menu
                      items={fileItems}
                      ariaLabel="File menu"
                      onAction={(value) => setLastAction(value)}
                      trigger={<Button variant="secondary">File</Button>}
                    />
                    {lastAction ? (
                      <span style={{ fontSize: "0.75rem", color: "var(--poodle-color-text-secondary)" }}>
                        Last: {lastAction}
                      </span>
                    ) : null}
      </SpecimenGroup>

                <SpecimenGroup label="With checkboxes">
        <Menu
                      items={settingsItems}
                      ariaLabel="Settings menu"
                      trigger={<Button variant="secondary">Settings</Button>}
                    />
      </SpecimenGroup>

      <SpecimenGroup label="Destructive action">
        <Menu
          items={destructiveItems}
          ariaLabel="Item actions"
          trigger={<Button variant="secondary">Actions</Button>}
        />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

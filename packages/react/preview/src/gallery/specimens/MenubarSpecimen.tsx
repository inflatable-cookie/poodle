import { useState } from "react";
import { Menubar, type MenubarItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const items: MenubarItem[] = [
  {
    value: "file",
    label: "File",
    items: [
      { value: "new", label: "New", shortcutLabel: "⌘N" },
      { value: "open", label: "Open…", shortcutLabel: "⌘O" },
      { value: "save", label: "Save", shortcutLabel: "⌘S" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "quit", label: "Quit", shortcutLabel: "⌘Q" },
    ],
  },
  {
    value: "edit",
    label: "Edit",
    items: [
      { value: "undo", label: "Undo", shortcutLabel: "⌘Z" },
      { value: "redo", label: "Redo", shortcutLabel: "⇧⌘Z" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
      { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
      { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
    ],
  },
  {
    value: "view",
    label: "View",
    items: [
      { value: "zoom-in", label: "Zoom in", shortcutLabel: "⌘+" },
      { value: "zoom-out", label: "Zoom out", shortcutLabel: "⌘-" },
      { value: "sep1", label: "", kind: "separator" },
      { value: "fullscreen", label: "Full screen", shortcutLabel: "⌃⌘F" },
    ],
  },
];

export function MenubarSpecimen() {
  const [lastAction, setLastAction] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => <Menubar items={items} size={size} ariaLabel={size + " menu bar"} />}
      densities={(density) => <Menubar items={items} density={density} ariaLabel={density + " menu bar"} />}
    >
      <SpecimenGroup label="Application menu bar">
        <Menubar items={items} ariaLabel="Application menu" onAction={(value) => setLastAction(value)} />
        {lastAction ? (
          <p>
            Last action: <strong>{lastAction}</strong>
          </p>
        ) : null}
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

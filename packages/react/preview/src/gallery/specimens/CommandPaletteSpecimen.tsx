import { useState, type CSSProperties } from "react";
import { Button, CommandPalette, UiPresentationProvider } from "@poodle/react";
import type { CommandActionItem } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const items: CommandActionItem[] = [
  { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
  { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
  { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
  { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
  { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
  { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
  { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
];

const hintStyle: CSSProperties = {
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
  margin: 0,
};

export function CommandPaletteSpecimen() {
  const [open, setOpen] = useState(false);
  const [compactOpen, setCompactOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [compactQuery, setCompactQuery] = useState("");
  const [sizeOpenMap, setSizeOpenMap] = useState<Record<string, boolean>>({});
  const [densityOpenMap, setDensityOpenMap] = useState<Record<string, boolean>>({});
  const [sizeQueryMap, setSizeQueryMap] = useState<Record<string, string>>({});
  const [densityQueryMap, setDensityQueryMap] = useState<Record<string, string>>({});

  return (
    <SpecimenLayout
      sizes={(size) => (
        <SpecimenGroup label={size.toUpperCase()}>
          <Button size={size} onClick={() => setSizeOpenMap((m) => ({ ...m, [size]: true }))}>
            Open {size.toUpperCase()} palette
          </Button>
          <CommandPalette
            open={sizeOpenMap[size] ?? false}
            query={sizeQueryMap[size] ?? ""}
            items={items}
            size={size}
            invocationHint="Cmd+K"
            title={`${size.toUpperCase()} command palette`}
            onOpenChange={(nextOpen) => setSizeOpenMap((m) => ({ ...m, [size]: nextOpen }))}
            onQueryChange={(nextQuery) => setSizeQueryMap((m) => ({ ...m, [size]: nextQuery }))}
          />
        </SpecimenGroup>
      )}
      densities={(density) => (
        <SpecimenGroup label={density}>
          <Button onClick={() => setDensityOpenMap((m) => ({ ...m, [density]: true }))}>
            Open {density} palette
          </Button>
          <CommandPalette
            open={densityOpenMap[density] ?? false}
            query={densityQueryMap[density] ?? ""}
            items={items}
            density={density}
            invocationHint="Cmd+K"
            title={`${density} command palette`}
            onOpenChange={(nextOpen) => setDensityOpenMap((m) => ({ ...m, [density]: nextOpen }))}
            onQueryChange={(nextQuery) => setDensityQueryMap((m) => ({ ...m, [density]: nextQuery }))}
          />
        </SpecimenGroup>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Command Palette">
          <p style={hintStyle}>Click below to open the palette. Close with Escape, click outside, or the X button.</p>
          <div>
            <Button onClick={() => setOpen(true)}>Open Command Palette</Button>
          </div>
          <CommandPalette
            open={open}
            query={query}
            onOpenChange={setOpen}
            onQueryChange={setQuery}
            items={items}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Semantic presentation">
          <UiPresentationProvider density="compact" sizeScale="sm">
            <div className="poodle-specimen__stack">
              <Button onClick={() => setCompactOpen(true)}>Open compact palette</Button>
              <CommandPalette
                open={compactOpen}
                query={compactQuery}
                onOpenChange={setCompactOpen}
                onQueryChange={setCompactQuery}
                items={[
                  { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
                  { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
                ]}
                invocationHint="Cmd+K"
              />
              <CommandPalette
                open={false}
                items={[{ id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" }]}
                sizeRole="prominent"
              />
            </div>
          </UiPresentationProvider>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

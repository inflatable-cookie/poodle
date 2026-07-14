import { useMemo, useState } from "react";
import {
  Button,
  CommandPalette,
  EmptyState,
  Menubar,
  type CommandActionItem,
  type MenubarItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const menubarItems: MenubarItem[] = [
  {
    value: "file",
    label: "File",
    items: [
      { value: "new", label: "New file", shortcutLabel: "⌘N" },
      { value: "open", label: "Open…", shortcutLabel: "⌘O" },
      { kind: "separator", label: "", value: "sep-1" },
      { value: "save", label: "Save", shortcutLabel: "⌘S" },
    ],
  },
  {
    value: "edit",
    label: "Edit",
    items: [
      { value: "undo", label: "Undo", shortcutLabel: "⌘Z" },
      { value: "redo", label: "Redo", shortcutLabel: "⇧⌘Z", disabled: true },
      { kind: "separator", label: "", value: "sep-2" },
      { value: "wrap", label: "Word wrap", kind: "checkbox", checked: true },
    ],
  },
  { value: "help", label: "Help", items: [{ value: "docs", label: "Documentation" }] },
];

const commands: CommandActionItem[] = [
  { id: "new-doc", title: "New document", description: "Create a blank document", group: "Create", shortcut: "⌘N" },
  { id: "new-project", title: "New project", group: "Create", badge: "Beta" },
  { id: "open-settings", title: "Open settings", description: "Workspace preferences", group: "Navigate" },
  { id: "toggle-theme", title: "Toggle theme", group: "Navigate", shortcut: "⌘⇧T" },
  { id: "archive-all", title: "Archive all", group: "Navigate", disabled: true },
];

function CommandChromeDemo() {
  const [lastAction, setLastAction] = useState("");
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [query, setQuery] = useState("");

  const filtered = useMemo(() => {
    if (!query.trim()) return commands;
    const needle = query.trim().toLowerCase();
    return commands.filter(
      (item) =>
        item.title.toLowerCase().includes(needle) ||
        (item.description ?? "").toLowerCase().includes(needle) ||
        (item.keywords ?? []).some((keyword) => keyword.toLowerCase().includes(needle)),
    );
  }, [query]);

  const paletteState = filtered.length === 0 ? "no-results" : "ready";

  return (
    <>
      <SpecimenSection title="Menubar">
        <Menubar items={menubarItems} ariaLabel="App menu" onAction={(value) => setLastAction(`menubar:${value}`)} />
      </SpecimenSection>

      <SpecimenSection title="Command palette">
        <Button onClick={() => setPaletteOpen(true)}>Open palette</Button>
        <CommandPalette
          open={paletteOpen}
          query={query}
          items={filtered}
          state={paletteState}
          invocationHint="⌘K"
          onQueryChange={setQuery}
          onOpenChange={(open) => {
            setPaletteOpen(open);
            if (!open) setQuery("");
          }}
          onCommandSelect={(id) => {
            setLastAction(`palette:${id}`);
            setPaletteOpen(false);
            setQuery("");
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="EmptyState">
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(14rem, 1fr))", gap: "0.75rem" }}>
          <EmptyState title="Nothing here yet" message="Items you add will show up in this list." />
          <EmptyState title="No matches" message="Try a different search." variant="search" size="compact" />
          <EmptyState
            title="Start your first project"
            message="Create a project to begin."
            variant="firstRun"
            actions={<Button size="sm">Create project</Button>}
          />
        </div>
      </SpecimenSection>

      {lastAction ? (
        <SpecimenSection title="Last action">
          <p data-testid="last-action">{lastAction}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "command-chrome",
  title: "Menubar / CommandPalette",
  render: () => <CommandChromeDemo />,
});

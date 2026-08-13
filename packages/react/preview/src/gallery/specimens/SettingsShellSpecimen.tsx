import { useState } from "react";
import { Button, DetailItem, DetailSection, SettingsShell, Surface } from "@inflatable-cookie/poodle-react";

export function SettingsShellSpecimen() {
  const [normalOpen, setNormalOpen] = useState(false);
  const [searchingOpen, setSearchingOpen] = useState(false);
  const [noGroupsOpen, setNoGroupsOpen] = useState(false);
  const [noResultsOpen, setNoResultsOpen] = useState(false);
  const [refusedOpen, setRefusedOpen] = useState(false);

  const [searchingQuery, setSearchingQuery] = useState("storage");

  // Long labels on purpose: the rail renders group titles on one line,
  // truncated with a native title tooltip (R1.2) — never wrapped.
  const groups = [
    { id: "general", label: "General", items: [{ value: "general", label: "General" }] },
    {
      id: "storage",
      label: "Storage & Backups",
      items: [
        { value: "storage", label: "Storage" },
        { value: "backup", label: "Backup" },
        { value: "restore", label: "Restore" },
      ],
    },
    {
      id: "input",
      label: "Keyboard Shortcuts & Input",
      items: [
        { value: "keymap", label: "Keymap" },
        { value: "editing", label: "Editing" },
      ],
    },
  ];

  const results = [
    { pageId: "storage", pageLabel: "Storage", anchorId: "disks", anchorLabel: "Disks" },
    { pageId: "backup", pageLabel: "Backup" },
    { pageId: "restore", pageLabel: "Restore", anchorId: "recovery", anchorLabel: "Recovery" },
  ];

  return (
    <>
      <Surface tone="panel" border="subtle" padding="md">
        <div className="poodle-settings-shell-specimen__triggers" style={{ display: "flex", flexWrap: "wrap", gap: "0.5rem" }}>
          <Button onClick={() => setNormalOpen(true)}>Settings</Button>
          <Button variant="secondary" onClick={() => setSearchingOpen(true)}>
            Searching
          </Button>
          <Button variant="secondary" onClick={() => setNoGroupsOpen(true)}>
            No groups
          </Button>
          <Button variant="secondary" onClick={() => setNoResultsOpen(true)}>
            No results
          </Button>
          <Button variant="secondary" onClick={() => setRefusedOpen(true)}>
            Refused close
          </Button>
        </div>
        <p
          className="poodle-settings-shell-specimen__hint"
          style={{ margin: "0.75rem 0 0", color: "var(--poodle-color-text-secondary)", fontSize: "0.8125rem", lineHeight: 1.5 }}
        >
          Search sits in the dialog header bar; the rail has its own surface and scroll,
          and the page body scrolls independently. The shell draws no page heading — the
          page owns its own. A query replaces the page with a flat result list. A refused
          close is a warning callout, not an error.
        </p>
      </Surface>

      <SettingsShell
        open={normalOpen}
        onOpenChange={setNormalOpen}
        groups={groups}
        activePageId="general"
        pageTitle="General"
        page={
          <>
            <DetailSection title="Appearance">
              <DetailItem label="Theme" value="Dark" />
              <DetailItem label="Density" value="Compact" />
              <DetailItem label="Default size" value="Medium" />
            </DetailSection>
            <DetailSection title="Editor">
              <DetailItem label="Tab size" value="2" />
              <DetailItem label="Word wrap" value="On" />
              <DetailItem label="Minimap" value="Off" />
              <DetailItem label="Format on save" value="On" />
            </DetailSection>
            <DetailSection title="Search">
              <DetailItem label="Search in files" value="On" />
              <DetailItem label="Follow symlinks" value="Off" />
            </DetailSection>
            <DetailSection title="Privacy">
              <DetailItem label="Telemetry" value="Disabled" />
              <DetailItem label="Crash reports" value="Disabled" />
            </DetailSection>
          </>
        }
      />

      <SettingsShell
        open={searchingOpen}
        onOpenChange={setSearchingOpen}
        groups={groups}
        activePageId="storage"
        pageTitle="Storage"
        searchQuery={searchingQuery}
        onSearchQueryChange={setSearchingQuery}
        searchResults={results}
        page={
          <DetailSection title="Storage">
            <DetailItem label="Location" value="~/Library/Application Support" />
          </DetailSection>
        }
      />

      <SettingsShell
        open={noGroupsOpen}
        onOpenChange={setNoGroupsOpen}
        groups={[]}
        pageTitle="General"
        page={
          <DetailSection title="General">
            <DetailItem label="Theme" value="Dark" />
          </DetailSection>
        }
      />

      <SettingsShell
        open={noResultsOpen}
        onOpenChange={setNoResultsOpen}
        groups={groups}
        activePageId="general"
        pageTitle="General"
        searchQuery="xyzzy"
        searchResults={[]}
        page={
          <DetailSection title="General">
            <DetailItem label="Theme" value="Dark" />
          </DetailSection>
        }
      />

      <SettingsShell
        open={refusedOpen}
        onOpenChange={setRefusedOpen}
        groups={groups}
        activePageId="keymap"
        pageTitle="Keymap"
        closeRefusedReason="Apply or discard this page before leaving."
        page={
          <DetailSection title="Keymap">
            <DetailItem label="Save" value="Cmd+S" />
            <DetailItem label="Search" value="Cmd+Shift+F" />
          </DetailSection>
        }
      />
    </>
  );
}

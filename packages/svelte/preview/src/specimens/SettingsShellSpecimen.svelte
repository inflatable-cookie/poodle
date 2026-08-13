<script lang="ts">
  import {
    Button,
    DetailItem,
    DetailSection,
    SettingsShell,
    Surface,
  } from "@inflatable-cookie/poodle-svelte";

  // One $state rune puts the whole file in runes mode, where plain `let`
  // stops being reactive — every flag here must be $state or its shell
  // never opens.
  let normalOpen = $state(false);
  let searchingOpen = $state(false);
  let noGroupsOpen = $state(false);
  let noResultsOpen = $state(false);
  let refusedOpen = $state(false);

  let searchingQuery = $state("storage");

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
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="poodle-settings-shell-specimen__triggers">
    <Button onClick={() => (normalOpen = true)}>Settings</Button>
    <Button variant="secondary" onClick={() => (searchingOpen = true)}>Searching</Button>
    <Button variant="secondary" onClick={() => (noGroupsOpen = true)}>No groups</Button>
    <Button variant="secondary" onClick={() => (noResultsOpen = true)}>No results</Button>
    <Button variant="secondary" onClick={() => (refusedOpen = true)}>Refused close</Button>
  </div>
  <p class="poodle-settings-shell-specimen__hint">
    Search sits in the dialog header bar; the rail has its own surface and scroll,
    and the page body scrolls independently. The shell draws no page heading — the
    page owns its own. A query replaces the page with a flat result list. A refused
    close is a warning callout, not an error.
  </p>
</Surface>

<SettingsShell
  bind:open={normalOpen}
  {groups}
  activePageId="general"
  pageTitle="General"
>
  {#snippet page()}
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
  {/snippet}
</SettingsShell>

<SettingsShell
  bind:open={searchingOpen}
  {groups}
  activePageId="storage"
  pageTitle="Storage"
  bind:searchQuery={searchingQuery}
  searchResults={results}
>
  {#snippet page()}
    <DetailSection title="Storage">
      <DetailItem label="Location" value="~/Library/Application Support" />
    </DetailSection>
  {/snippet}
</SettingsShell>

<SettingsShell
  bind:open={noGroupsOpen}
  groups={[]}
  pageTitle="General"
>
  {#snippet page()}
    <DetailSection title="General">
      <DetailItem label="Theme" value="Dark" />
    </DetailSection>
  {/snippet}
</SettingsShell>

<SettingsShell
  bind:open={noResultsOpen}
  {groups}
  activePageId="general"
  pageTitle="General"
  searchQuery="xyzzy"
  searchResults={[]}
>
  {#snippet page()}
    <DetailSection title="General">
      <DetailItem label="Theme" value="Dark" />
    </DetailSection>
  {/snippet}
</SettingsShell>

<SettingsShell
  bind:open={refusedOpen}
  {groups}
  activePageId="keymap"
  pageTitle="Keymap"
  closeRefusedReason="Apply or discard this page before leaving."
  onRequestClose={() => {
    /* The host decides: the reason above keeps the shell open. */
  }}
>
  {#snippet page()}
    <DetailSection title="Keymap">
      <DetailItem label="Save" value="Cmd+S" />
      <DetailItem label="Search" value="Cmd+Shift+F" />
    </DetailSection>
  {/snippet}
</SettingsShell>

<style>
  .poodle-settings-shell-specimen__triggers {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .poodle-settings-shell-specimen__hint {
    margin: 0.75rem 0 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }
</style>

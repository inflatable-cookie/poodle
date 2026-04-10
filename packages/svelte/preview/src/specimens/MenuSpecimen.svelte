<script lang="ts">
  import { Menu, Button, type MenuItem } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let lastAction = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="With shortcuts">
    <Menu items={fileItems} ariaLabel="File menu" on:action={(e) => (lastAction = e.detail.value)}>
      <Button variant="secondary" slot="trigger">File</Button>
    </Menu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="With checkboxes">
    <Menu items={settingsItems} ariaLabel="Settings menu">
      <Button variant="secondary" slot="trigger">Settings</Button>
    </Menu>
  </SpecimenGroup>

  <SpecimenGroup label="Destructive Action">
    <Menu items={destructiveItems} ariaLabel="Item actions" triggerAriaLabel="Item actions">
      <Button variant="secondary" slot="trigger">Actions</Button>
    </Menu>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Menu items={fileItems} {size} ariaLabel={size + " menu"}>
      <Button variant="secondary" {size} slot="trigger">{size.toUpperCase()}</Button>
    </Menu>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Menu items={fileItems} {density} ariaLabel="{density} menu">
      <Button variant="secondary" {density} slot="trigger">{density}</Button>
    </Menu>
  </svelte:fragment>
</SpecimenLayout>

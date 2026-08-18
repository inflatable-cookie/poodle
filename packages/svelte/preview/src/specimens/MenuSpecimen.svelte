<script lang="ts">
  import { Menu, Button, type MenuItem } from "@inflatable-cookie/poodle-svelte";
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

  let lastAction = $state("");
</script>

<SpecimenLayout>
    <SpecimenGroup label="With shortcuts">
    <Menu items={fileItems} ariaLabel="File menu" onAction={(value) => (lastAction = value)}>
          {#snippet trigger()}
            <Button variant="secondary">File</Button>
          {/snippet}
        </Menu>
        {#if lastAction}
          <span class="poodle-specimen__hint">Last: {lastAction}</span>
        {/if}
  </SpecimenGroup>

        <SpecimenGroup label="With checkboxes">
    <Menu items={settingsItems} ariaLabel="Settings menu">
          {#snippet trigger()}
            <Button variant="secondary">Settings</Button>
          {/snippet}
        </Menu>
  </SpecimenGroup>

        <SpecimenGroup label="Destructive action">
    <Menu items={destructiveItems} ariaLabel="Item actions">
          {#snippet trigger()}
            <Button variant="secondary">Actions</Button>
          {/snippet}
        </Menu>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Menu items={fileItems} {size} ariaLabel={size + " menu"}>
      {#snippet trigger()}
        <Button variant="secondary" {size}>{size.toUpperCase()}</Button>
      {/snippet}
    </Menu>
  {/snippet}

  {#snippet densities(density)}
    <Menu items={fileItems} {density} ariaLabel="{density} menu">
      {#snippet trigger()}
        <Button variant="secondary" {density}>{density}</Button>
      {/snippet}
    </Menu>
  {/snippet}
</SpecimenLayout>

<style>
    .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .poodle-specimen__hint {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

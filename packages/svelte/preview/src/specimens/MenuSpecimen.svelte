<script lang="ts">
  import { Menu, Button, type MenuItem, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

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

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastAction = "";
</script>

<div class="specimen">
  <SpecimenGroup label="With shortcuts">
    <Menu items={fileItems} ariaLabel="File menu" on:action={(e) => (lastAction = e.detail.value)}>
      <Button variant="secondary" slot="trigger">File</Button>
    </Menu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <Menu items={fileItems} ariaLabel={size + " menu"} {size}>
          <Button variant="secondary" slot="trigger">{size.toUpperCase()}</Button>
        </Menu>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Menu items={fileItems} ariaLabel="{density} menu" {density}>
            <Button variant="secondary" slot="trigger">{density}</Button>
          </Menu>
        </div>
      {/each}
    </div>
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
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>

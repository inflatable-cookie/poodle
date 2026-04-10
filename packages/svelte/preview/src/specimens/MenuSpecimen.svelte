<script lang="ts">
  import { Menu, Button, Eyebrow, Surface, type MenuItem } from "@poodle/svelte-primitives";
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
  <Surface tone="panel" border="subtle" padding="md">
    <div class="specimen">
      <div class="specimen__row">
        <Eyebrow>With shortcuts</Eyebrow>
        <Menu items={fileItems} ariaLabel="File menu" on:action={(e) => (lastAction = e.detail.value)}>
          <Button variant="secondary" slot="trigger">File</Button>
        </Menu>
        {#if lastAction}
          <span class="specimen__hint">Last: {lastAction}</span>
        {/if}
      </div>

      <div class="specimen__row">
        <Eyebrow>With checkboxes</Eyebrow>
        <Menu items={settingsItems} ariaLabel="Settings menu">
          <Button variant="secondary" slot="trigger">Settings</Button>
        </Menu>
      </div>

      <div class="specimen__row">
        <Eyebrow>Destructive action</Eyebrow>
        <Menu items={destructiveItems} ariaLabel="Item actions">
          <Button variant="secondary" slot="trigger">Actions</Button>
        </Menu>
      </div>
    </div>
  </Surface>

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

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .specimen__hint {
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

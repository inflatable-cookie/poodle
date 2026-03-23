<script lang="ts">
  import { Menu, Button, Eyebrow, type MenuItem } from "@poodle/svelte-primitives";

  const fileItems: MenuItem[] = [
    { value: "new", label: "New file", shortcutLabel: "⌘N" },
    { value: "open", label: "Open…", shortcutLabel: "⌘O" },
    { value: "save", label: "Save", shortcutLabel: "⌘S" },
    { value: "sep1", label: "", kind: "separator" },
    { value: "export", label: "Export as PDF" },
    { value: "print", label: "Print…", shortcutLabel: "⌘P", isDisabled: true },
  ];

  const settingsItems: MenuItem[] = [
    { value: "theme", label: "Dark mode", kind: "checkbox", isChecked: true },
    { value: "notifications", label: "Notifications", kind: "checkbox" },
    { value: "sep1", label: "", kind: "separator" },
    { value: "settings", label: "Settings…" },
  ];

  let lastAction = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With shortcuts</Eyebrow>
    <Menu items={fileItems} ariaLabel="File menu" on:action={(e) => (lastAction = e.detail.value)}>
      <Button variant="secondary" slot="trigger">File</Button>
    </Menu>
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>With checkboxes</Eyebrow>
    <Menu items={settingsItems} ariaLabel="Settings menu">
      <Button variant="secondary" slot="trigger">Settings</Button>
    </Menu>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>

<script lang="ts">
  import { Eyebrow, Button } from "@poodle/svelte-primitives";
  import { CommandPalette } from "@poodle/svelte-composites";

  let open = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Command Palette</Eyebrow>
    <p class="specimen__hint">Click below to open the palette. Close with Escape, click outside, or the X button.</p>
    <div>
      <Button on:click={() => (open = true)}>Open Command Palette</Button>
    </div>
    <CommandPalette
      bind:open
      items={[
        { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
        { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
        { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
        { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
        { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
        { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
        { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
      ]}
    />
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

  .specimen__hint {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    margin: 0;
  }
</style>

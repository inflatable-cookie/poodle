<script lang="ts">
  import { Menubar, Eyebrow, type MenubarItem } from "@pug/svelte-primitives";

  const items: MenubarItem[] = [
    {
      value: "file",
      label: "File",
      items: [
        { value: "new", label: "New", shortcutLabel: "⌘N" },
        { value: "open", label: "Open…", shortcutLabel: "⌘O" },
        { value: "save", label: "Save", shortcutLabel: "⌘S" },
        { value: "sep1", label: "", kind: "separator" },
        { value: "quit", label: "Quit", shortcutLabel: "⌘Q" },
      ],
    },
    {
      value: "edit",
      label: "Edit",
      items: [
        { value: "undo", label: "Undo", shortcutLabel: "⌘Z" },
        { value: "redo", label: "Redo", shortcutLabel: "⇧⌘Z" },
        { value: "sep1", label: "", kind: "separator" },
        { value: "cut", label: "Cut", shortcutLabel: "⌘X" },
        { value: "copy", label: "Copy", shortcutLabel: "⌘C" },
        { value: "paste", label: "Paste", shortcutLabel: "⌘V" },
      ],
    },
    {
      value: "view",
      label: "View",
      items: [
        { value: "zoom-in", label: "Zoom in", shortcutLabel: "⌘+" },
        { value: "zoom-out", label: "Zoom out", shortcutLabel: "⌘-" },
        { value: "sep1", label: "", kind: "separator" },
        { value: "fullscreen", label: "Full screen", shortcutLabel: "⌃⌘F" },
      ],
    },
  ];

  let lastAction = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Application menu bar</Eyebrow>
    <Menubar {items} ariaLabel="Application menu" on:action={(e) => (lastAction = e.detail.value)} />
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
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
    color: var(--pug-color-text-secondary);
  }
</style>

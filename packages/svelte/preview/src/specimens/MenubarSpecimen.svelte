<script lang="ts">
  import { Menubar, type MenubarItem, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

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

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let lastAction = "";
</script>

<div class="specimen">
  <SpecimenGroup label="Application menu bar">
    <Menubar {items} ariaLabel="Application menu" on:action={(e) => (lastAction = e.detail.value)} />
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <Menubar {items} ariaLabel={size + " menu bar"} {size} />
      {/each}
    </div>
  </SpecimenGroup>
  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Menubar {items} ariaLabel="{density} menu bar" {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

</style>

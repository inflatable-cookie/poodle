<script lang="ts">
  import { Menubar, type MenubarItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

<SpecimenLayout>
  <SpecimenGroup label="Application menu bar">
    <Menubar {items} ariaLabel="Application menu" onAction={(value) => (lastAction = value)} />
    {#if lastAction}
      <p>Last action: <strong>{lastAction}</strong></p>
    {/if}
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Menubar {items} {size} ariaLabel={size + " menu bar"} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Menubar {items} {density} ariaLabel={density + " menu bar"} />
  </svelte:fragment>
</SpecimenLayout>

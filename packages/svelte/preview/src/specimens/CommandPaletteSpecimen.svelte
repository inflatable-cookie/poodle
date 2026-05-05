<script lang="ts">
  import { Button, UiPresentationProvider } from "@poodle/svelte";
  import { CommandPalette } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let open = false;
  let compactOpen = false;
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Command Palette">
    <p class="poodle-specimen__hint">Click below to open the palette. Close with Escape, click outside, or the X button.</p>
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
  </SpecimenGroup>

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="poodle-specimen__stack">
        <Button on:click={() => (compactOpen = true)}>Open compact palette</Button>
        <CommandPalette
          open={compactOpen}
          on:openChange={(e) => (compactOpen = e.detail.open)}
          items={[
            { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
            { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
          ]}
          invocationHint="Cmd+K"
        />
        <CommandPalette
          open={false}
          items={[
            { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
          ]}
          sizeRole="prominent"
        />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__hint {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    margin: 0;
  }

  .poodle-specimen__stack {
    display: grid;
    gap: 0.75rem;
  }
</style>

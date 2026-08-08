<script lang="ts">
  import { Button, UiPresentationProvider } from "@inflatable-cookie/poodle-svelte";
  import { CommandPalette } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let open = $state(false);
  let compactOpen = $state(false);
  let query = $state("");
  let compactQuery = $state("");
  let sizeOpenMap: Record<string, boolean> = {};
  let densityOpenMap: Record<string, boolean> = {};
  let sizeQueryMap: Record<string, string> = {};
  let densityQueryMap: Record<string, string> = {};

  const items = [
    { id: "save", title: "Save", shortcut: "Ctrl+S", group: "File" },
    { id: "open", title: "Open File", shortcut: "Ctrl+O", group: "File" },
    { id: "close", title: "Close Tab", shortcut: "Ctrl+W", group: "File" },
    { id: "find", title: "Find in Files", shortcut: "Ctrl+Shift+F", group: "Edit" },
    { id: "replace", title: "Find and Replace", shortcut: "Ctrl+H", group: "Edit" },
    { id: "terminal", title: "Toggle Terminal", shortcut: "Ctrl+`", group: "View" },
    { id: "sidebar", title: "Toggle Sidebar", shortcut: "Ctrl+B", group: "View" },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Command Palette">
      <p class="poodle-specimen__hint">Click below to open the palette. Close with Escape, click outside, or the X button.</p>
      <div>
        <Button onClick={() => (open = true)}>Open Command Palette</Button>
      </div>
      <CommandPalette
        open={open}
        {query}
        onOpenChange={(nextOpen) => (open = nextOpen)}
        onQueryChange={(nextQuery) => (query = nextQuery)}
        {items}
      />
    </SpecimenGroup>

    <SpecimenGroup label="Semantic presentation">
      <UiPresentationProvider density="compact" sizeScale="sm">
        <div class="poodle-specimen__stack">
          <Button onClick={() => (compactOpen = true)}>Open compact palette</Button>
          <CommandPalette
            open={compactOpen}
            query={compactQuery}
            onOpenChange={(nextOpen) => (compactOpen = nextOpen)}
            onQueryChange={(nextQuery) => (compactQuery = nextQuery)}
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

  {#snippet sizes(size)}
    <SpecimenGroup label={size.toUpperCase()}>
      <Button size={size} onClick={() => (sizeOpenMap[size] = true)}>
        Open {size.toUpperCase()} palette
      </Button>
      <CommandPalette
        open={sizeOpenMap[size] ?? false}
        query={sizeQueryMap[size] ?? ""}
        {items}
        {size}
        invocationHint="Cmd+K"
        title={`${size.toUpperCase()} command palette`}
        onOpenChange={(nextOpen) => (sizeOpenMap[size] = nextOpen)}
        onQueryChange={(nextQuery) => (sizeQueryMap[size] = nextQuery)}
      />
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density}>
      <Button onClick={() => (densityOpenMap[density] = true)}>
        Open {density} palette
      </Button>
      <CommandPalette
        open={densityOpenMap[density] ?? false}
        query={densityQueryMap[density] ?? ""}
        {items}
        {density}
        invocationHint="Cmd+K"
        title={`${density} command palette`}
        onOpenChange={(nextOpen) => (densityOpenMap[density] = nextOpen)}
        onQueryChange={(nextQuery) => (densityQueryMap[density] = nextQuery)}
      />
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

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

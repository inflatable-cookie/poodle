<script lang="ts">
  import { ReorderableList } from "@poodle/svelte-composites";
  import { Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
  import type { ReorderableItem } from "@poodle/svelte-composites";

  let items: ReorderableItem[] = [
    { id: "1", label: "First item" },
    { id: "2", label: "Second item" },
    { id: "3", label: "Third item" },
    { id: "4", label: "Fourth item" },
    { id: "5", label: "Fifth item" },
  ];
  let lastReorder = "";
  let compactItems: ReorderableItem[] = [
    { id: "c1", label: "Kick" },
    { id: "c2", label: "Snare" },
    { id: "c3", label: "Bass" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Drag to reorder (or Alt+Arrow keys)</Eyebrow>
    <ReorderableList
      bind:items
      ariaLabel="Reorderable items"
      on:reorder={(e) => (lastReorder = e.detail.items.map(i => i.label).join(", "))}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <ReorderableList
      items={[{ id: "a", label: "Locked item A" }, { id: "b", label: "Locked item B" }]}
      disabled
    />
  </div>

  {#if lastReorder}
    <div class="specimen__group">
      <Eyebrow>Current order</Eyebrow>
      <p>{lastReorder}</p>
    </div>
  {/if}

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <ReorderableList bind:items={compactItems} ariaLabel="Compact reorderable items" />
        <ReorderableList items={compactItems} ariaLabel="Prominent reorderable items" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__stack { display: grid; gap: 0.75rem; }
  p { margin: 0; font-size: 0.8125rem; }
</style>

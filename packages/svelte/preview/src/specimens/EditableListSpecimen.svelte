<script lang="ts">
  import { EditableList } from "@poodle/svelte-composites";
  import { Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
  import type { ReorderableItem } from "@poodle/svelte-composites";

  let tags: ReorderableItem[] = [
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ];
  let compactItems: ReorderableItem[] = [
    { id: "c1", label: "low-latency" },
    { id: "c2", label: "offline-first" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Reorderable with add/remove</Eyebrow>
    <EditableList
      bind:items={tags}
      ariaLabel="Tags"
      placeholder="Add a tag…"
      addLabel="Add"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With max items (5)</Eyebrow>
    <EditableList
      items={[{ id: "a", label: "Item A" }, { id: "b", label: "Item B" }]}
      maxItems={5}
      ariaLabel="Limited list"
      placeholder="Add item…"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Non-reorderable</Eyebrow>
    <EditableList
      items={[{ id: "x", label: "Static item" }]}
      reorderable={false}
      ariaLabel="Static list"
      placeholder="Add item…"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <EditableList bind:items={compactItems} ariaLabel="Compact list" addLabel="Add" />
        <EditableList items={compactItems} ariaLabel="Prominent list" addLabel="Add" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__stack { display: grid; gap: 0.75rem; }
</style>

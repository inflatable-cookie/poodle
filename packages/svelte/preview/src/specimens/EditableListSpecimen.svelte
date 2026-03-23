<script lang="ts">
  import { EditableList } from "@flint/svelte-composites";
  import { Eyebrow } from "@flint/svelte-primitives";
  import type { ReorderableItem } from "@flint/svelte-composites";

  let tags: ReorderableItem[] = [
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
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
      isReorderable={false}
      ariaLabel="Static list"
      placeholder="Add item…"
    />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
</style>

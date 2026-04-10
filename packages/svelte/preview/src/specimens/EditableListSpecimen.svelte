<script lang="ts">
  import { ReorderableList } from "@poodle/svelte-composites";
  import type { ReorderableItem } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let tags: ReorderableItem[] = [
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ];
</script>

<div class="specimen">
  <SpecimenGroup label="Editable + reorderable">
    <ReorderableList
      bind:items={tags}
      editable
      ariaLabel="Tags"
      addPlaceholder="Add a tag…"
      addLabel="Add"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With max items (5)">
    <ReorderableList
      items={[{ id: "a", label: "Item A" }, { id: "b", label: "Item B" }]}
      editable
      maxItems={5}
      ariaLabel="Limited list"
      addPlaceholder="Add item…"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Removable only (no reorder, no add)">
    <ReorderableList
      items={[{ id: "x", label: "First item" }, { id: "y", label: "Second item" }]}
      reorderable={false}
      removable
      ariaLabel="Static list"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ReorderableList
      items={tags}
      editable
      disabled
      ariaLabel="Disabled list"
    />
  </SpecimenGroup>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
</style>

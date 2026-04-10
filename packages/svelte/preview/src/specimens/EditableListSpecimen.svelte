<script lang="ts">
  import { EditableList } from "@poodle/svelte-composites";
  import type { EditableListItem } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let tags: EditableListItem[] = [
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ];
</script>

<div class="specimen">
  <SpecimenGroup label="Editable + reorderable">
    <EditableList
      bind:items={tags}
      editable
      ariaLabel="Tags"
      addPlaceholder="Add a tag…"
      addLabel="Add"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With max items (5)">
    <EditableList
      items={[{ id: "a", label: "Item A" }, { id: "b", label: "Item B" }]}
      editable
      maxItems={5}
      ariaLabel="Limited list"
      addPlaceholder="Add item…"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Removable only (no reorder, no add)">
    <EditableList
      items={[{ id: "x", label: "First item" }, { id: "y", label: "Second item" }]}
      reorderable={false}
      removable
      ariaLabel="Static list"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <EditableList
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

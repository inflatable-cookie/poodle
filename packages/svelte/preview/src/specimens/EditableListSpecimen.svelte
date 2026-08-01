<script lang="ts">
  import { EditableList } from "@poodle/svelte";
  import type { EditableListItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let tags: EditableListItem[] = $state([
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ]);

  const specimenItems: EditableListItem[] = [
    { id: "1", label: "svelte" },
    { id: "2", label: "typescript" },
    { id: "3", label: "design-system" },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
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

  {#snippet sizes(size)}
    <div class="poodle-editable-list-specimen__variant">
      <EditableList
        items={specimenItems}
        editable
        ariaLabel={`Editable list at ${size}`}
        addPlaceholder="Add a tag…"
        addLabel="Add"
        {size}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-editable-list-specimen__variant">
      <EditableList
        items={specimenItems}
        editable
        ariaLabel={`Editable list at ${density} density`}
        addPlaceholder="Add a tag…"
        addLabel="Add"
        {density}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 1rem; }

  .poodle-editable-list-specimen__variant {
    width: min(100%, 26rem);
  }
</style>

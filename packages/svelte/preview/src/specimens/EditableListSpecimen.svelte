<script lang="ts">
  import { EditableList } from "@poodle/svelte-composites";
  import { UiPresentationProvider } from "@poodle/svelte-primitives";
  import type { ReorderableItem } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  <SpecimenGroup label="Reorderable with add/remove">
    <EditableList
      bind:items={tags}
      ariaLabel="Tags"
      placeholder="Add a tag…"
      addLabel="Add"
    />
  </SpecimenGroup>

  <SpecimenGroup label="With max items (5)">
    <EditableList
      items={[{ id: "a", label: "Item A" }, { id: "b", label: "Item B" }]}
      maxItems={5}
      ariaLabel="Limited list"
      placeholder="Add item…"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Non-reorderable">
    <EditableList
      items={[{ id: "x", label: "Static item" }]}
      reorderable={false}
      ariaLabel="Static list"
      placeholder="Add item…"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <EditableList bind:items={compactItems} ariaLabel="Compact list" addLabel="Add" />
        <EditableList items={compactItems} ariaLabel="Prominent list" addLabel="Add" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
  .specimen__stack { display: grid; gap: 0.75rem; }
</style>

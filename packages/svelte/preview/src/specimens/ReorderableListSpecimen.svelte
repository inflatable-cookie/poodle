<script lang="ts">
  import { ReorderableList } from "@poodle/svelte-composites";
  import { UiPresentationProvider } from "@poodle/svelte-primitives";
  import type { ReorderableItem } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  let sessionItems: ReorderableItem[] = [
    { id: "s1", label: "Intro" },
    { id: "s2", label: "Body" },
    { id: "s3", label: "Summary" },
  ];
  let baselineItems = sessionItems.map((item) => item.id);
  let submitError = "";
  let windowedItems: ReorderableItem[] = Array.from({ length: 12 }, (_, index) => ({
    id: `w${index + 1}`,
    label: `Window item ${index + 1}`,
  }));

  $: isDirty = sessionItems.some((item, index) => item.id !== baselineItems[index]);

  async function saveSession() {
    submitError = "";
    await new Promise((resolve) => setTimeout(resolve, 150));
    baselineItems = sessionItems.map((item) => item.id);
  }

  function cancelSession() {
    sessionItems = baselineItems
      .map((id) => sessionItems.find((item) => item.id === id))
      .filter(Boolean) as ReorderableItem[];
    submitError = "";
  }
</script>

<div class="specimen">
  <SpecimenGroup label="Drag to reorder (or Alt+Arrow keys)">
    <ReorderableList
      bind:items
      ariaLabel="Reorderable items"
      on:reorder={(e) => (lastReorder = e.detail.items.map(i => i.label).join(", "))}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Workflow shell">
    <ReorderableList
      bind:items={sessionItems}
      ariaLabel="Reorder page sections"
      dirty={isDirty}
      errorMessage={submitError || null}
      onsubmit={saveSession}
      oncancel={cancelSession}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Large-list session">
    <ReorderableList
      bind:items={windowedItems}
      ariaLabel="Windowed reorder list"
      longListThreshold={8}
      windowSize={5}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ReorderableList
      items={[{ id: "a", label: "Locked item A" }, { id: "b", label: "Locked item B" }]}
      disabled
    />
  </SpecimenGroup>

  {#if lastReorder}
    <SpecimenGroup label="Current order">
      <p>{lastReorder}</p>
    </SpecimenGroup>
  {/if}

  <SpecimenGroup label="Semantic presentation">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <ReorderableList bind:items={compactItems} ariaLabel="Compact reorderable items" />
        <ReorderableList items={compactItems} ariaLabel="Prominent reorderable items" sizeRole="prominent" />
      </div>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
  .specimen__stack { display: grid; gap: 0.75rem; }
  p { margin: 0; font-size: 0.8125rem; }
</style>

<script lang="ts">
  import { SelectionSummary } from "@poodle/svelte-composites";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let items = [
    { id: "1", label: "Button" },
    { id: "2", label: "Card" },
    { id: "3", label: "Dialog" },
    { id: "4", label: "Table" },
    { id: "5", label: "Tabs" },
  ];
</script>

<div class="specimen">
  <SpecimenGroup label="Multiple items selected">
    <SelectionSummary
      {items}
      selectionMode="multiple"
      on:remove={(e) => (items = items.filter((i) => i.id !== e.detail.id))}
      on:clear={() => (items = [])}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Single item">
    <SelectionSummary items={[{ id: "1", label: "Primary button" }]} selectionMode="single" />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <SelectionSummary
          items={[{ id: "1", label: "Button" }, { id: "2", label: "Card" }, { id: "3", label: "Dialog" }]}
          {size}
        />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Truncated (max 3 visible)">
    <SelectionSummary
      items={[
        { id: "a", label: "Alpha" },
        { id: "b", label: "Beta" },
        { id: "c", label: "Gamma" },
        { id: "d", label: "Delta" },
        { id: "e", label: "Epsilon" },
        { id: "f", label: "Zeta" },
      ]}
      maxVisibleItems={3}
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>

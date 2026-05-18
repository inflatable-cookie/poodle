<script lang="ts">
  import { SelectionSummary } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let items = $state([
    { id: "1", label: "Button" },
    { id: "2", label: "Card" },
    { id: "3", label: "Dialog" },
    { id: "4", label: "Table" },
    { id: "5", label: "Tabs" },
  ]);

  const variantItems = [
    { id: "1", label: "Button" },
    { id: "2", label: "Card" },
    { id: "3", label: "Dialog" },
  ];
</script>

<SpecimenLayout>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="Multiple items selected">
        <SelectionSummary
          {items}
          onRemove={(id) => (items = items.filter((item) => item.id !== id))}
          onClear={() => (items = [])}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Single item">
        <SelectionSummary items={[{ id: "1", label: "Primary button" }]} />
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
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-selection-summary-specimen__variant">
      <SelectionSummary items={variantItems} {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-selection-summary-specimen__variant">
      <SelectionSummary items={variantItems} {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-selection-summary-specimen__variant {
    width: min(100%, 28rem);
  }
</style>

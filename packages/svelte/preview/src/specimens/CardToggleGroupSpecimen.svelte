<script lang="ts">
  import { CardToggleGroup } from "@inflatable-cookie/poodle-svelte";
  import type { CardToggleItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let statusValue: string | null = $state("pending");
  let optionalValue: string | null = $state("marked");

  const statusItems: CardToggleItem[] = [
    { value: "pending", label: "Pending", description: "Waiting for review.", count: 42 },
    { value: "marked", label: "Marked", description: "Reviewed and complete.", count: 128 },
    { value: "void", label: "Void", description: "Removed from the active queue.", count: 6 },
    { value: "all", label: "All", description: "Every item in the collection.", count: 176 },
  ];

  const compactItems: CardToggleItem[] = [
    { value: "draft", label: "Draft", count: 8 },
    { value: "live", label: "Live", count: 21 },
    { value: "archived", label: "Archived", count: 4, disabled: true },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Query variants">
      <CardToggleGroup
        items={statusItems}
        bind:value={statusValue}
        columns={4}
        ariaLabel="Select answer status"
        onValueChange={(value) => (statusValue = value)}
      />
      <p>Selected: <strong>{statusValue ?? "none"}</strong></p>
    </SpecimenGroup>

    <SpecimenGroup label="Deactivation allowed">
      <CardToggleGroup
        items={compactItems}
        bind:value={optionalValue}
        columns={3}
        allowDeactivation
        ariaLabel="Select optional status"
        onValueChange={(value) => (optionalValue = value)}
      />
      <p>Selected: <strong>{optionalValue ?? "none"}</strong></p>
    </SpecimenGroup>

    <SpecimenGroup label="Disabled group">
      <CardToggleGroup
        items={compactItems}
        value="live"
        columns={3}
        disabled
        ariaLabel="Disabled card toggle group"
      />
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-card-toggle-group-specimen__variant">
      <CardToggleGroup
        items={compactItems}
        value="live"
        columns={3}
        {size}
        ariaLabel={`Card toggle group at ${size}`}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-card-toggle-group-specimen__variant">
      <CardToggleGroup
        items={compactItems}
        value="live"
        columns={3}
        {density}
        ariaLabel={`Card toggle group at ${density} density`}
      />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-card-toggle-group-specimen__variant {
    width: min(100%, 42rem);
  }

  p {
    margin: 0;
  }
</style>

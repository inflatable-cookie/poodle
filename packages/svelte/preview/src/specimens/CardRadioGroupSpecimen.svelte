<script lang="ts">
  import { CardRadioGroup } from "@inflatable-cookie/poodle-svelte";
  import type { CardRadioItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let planValue: string | null = $state("pro");
  let sizeValue: string | null = $state(null);

  const planItems: CardRadioItem[] = [
    { value: "free", label: "Free", description: "Basic features for personal use. Up to 3 projects." },
    { value: "pro", label: "Pro", description: "Advanced features for professionals. Unlimited projects." },
    { value: "team", label: "Team", description: "Collaboration tools for teams. Shared workspace included." },
    { value: "enterprise", label: "Enterprise", description: "Custom solutions for large organizations.", disabled: true },
  ];

  const sizeItems: CardRadioItem[] = [
    { value: "sm", label: "Small", description: "1 CPU, 512 MB RAM" },
    { value: "md", label: "Medium", description: "2 CPU, 2 GB RAM" },
    { value: "lg", label: "Large", description: "4 CPU, 8 GB RAM" },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Plan selection (2 columns)">
      <CardRadioGroup
        items={planItems}
        bind:value={planValue}
        columns={2}
        ariaLabel="Select a plan"
        onValueChange={(value) => (planValue = value)}
      />
      {#if planValue}
        <p>Selected: <strong>{planValue}</strong></p>
      {/if}
    </SpecimenGroup>

    <SpecimenGroup label="Instance size (3 columns)">
      <CardRadioGroup
        items={sizeItems}
        bind:value={sizeValue}
        columns={3}
        ariaLabel="Select an instance size"
        onValueChange={(value) => (sizeValue = value)}
      />
      {#if sizeValue}
        <p>Selected: <strong>{sizeValue}</strong></p>
      {/if}
    </SpecimenGroup>

    <SpecimenGroup label="Disabled group">
      <CardRadioGroup
        items={sizeItems}
        value="md"
        columns={3}
        disabled
        ariaLabel="Disabled selection"
      />
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-card-radio-group-specimen__variant">
      <CardRadioGroup
        items={sizeItems}
        value="md"
        columns={3}
        {size}
        ariaLabel={`Card radio group at ${size}`}
      />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-card-radio-group-specimen__variant">
      <CardRadioGroup
        items={sizeItems}
        value="md"
        columns={3}
        {density}
        ariaLabel={`Card radio group at ${density} density`}
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

  .poodle-card-radio-group-specimen__variant {
    width: min(100%, 40rem);
  }

  p { margin: 0; }
</style>

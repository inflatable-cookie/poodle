<script lang="ts">
  import { CardRadioGroup } from "@poodle/svelte";
  import type { CardRadioItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let planValue: string | null = "pro";
  let sizeValue: string | null = null;

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

<div class="specimen">
  <SpecimenGroup label="Plan selection (2 columns)">
    <CardRadioGroup
      items={planItems}
      bind:value={planValue}
      columns={2}
      ariaLabel="Select a plan"
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
    />
    {#if sizeValue}
      <p>Selected: <strong>{sizeValue}</strong></p>
    {/if}
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <CardRadioGroup
          items={sizeItems}
          value="md"
          columns={3}
          {size}
          ariaLabel="Card radio group at {size}"
        />
      {/each}
    </div>
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

  p { margin: 0; }
</style>

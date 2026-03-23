<script lang="ts">
  import { CardRadioGroup } from "@poodle/svelte-composites";
  import { Eyebrow } from "@poodle/svelte-primitives";
  import type { CardRadioItem } from "@poodle/svelte-composites";

  let planValue: string | null = "pro";
  let sizeValue: string | null = null;

  const planItems: CardRadioItem[] = [
    { value: "free", label: "Free", description: "Basic features for personal use. Up to 3 projects." },
    { value: "pro", label: "Pro", description: "Advanced features for professionals. Unlimited projects." },
    { value: "team", label: "Team", description: "Collaboration tools for teams. Shared workspace included." },
    { value: "enterprise", label: "Enterprise", description: "Custom solutions for large organizations.", isDisabled: true },
  ];

  const sizeItems: CardRadioItem[] = [
    { value: "sm", label: "Small", description: "1 CPU, 512 MB RAM" },
    { value: "md", label: "Medium", description: "2 CPU, 2 GB RAM" },
    { value: "lg", label: "Large", description: "4 CPU, 8 GB RAM" },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Plan selection (2 columns)</Eyebrow>
    <CardRadioGroup
      items={planItems}
      bind:value={planValue}
      columns={2}
      ariaLabel="Select a plan"
    />
    {#if planValue}
      <p>Selected: <strong>{planValue}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Instance size (3 columns)</Eyebrow>
    <CardRadioGroup
      items={sizeItems}
      bind:value={sizeValue}
      columns={3}
      ariaLabel="Select an instance size"
    />
    {#if sizeValue}
      <p>Selected: <strong>{sizeValue}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled group</Eyebrow>
    <CardRadioGroup
      items={sizeItems}
      value="md"
      columns={3}
      isDisabled
      ariaLabel="Disabled selection"
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  p { margin: 0; }
</style>

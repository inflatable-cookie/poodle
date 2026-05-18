<script lang="ts">
  import { Spinner } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const spinnerSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Ring">
    <div class="poodle-specimen__row">
      {#each spinnerSizes as size}
        <Spinner variant="ring" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="CLI grid">
    <div class="poodle-specimen__row">
      {#each spinnerSizes as size, index}
        <Spinner variant="grid" {size} tone={index === 0 ? "muted" : index === 2 ? "accent" : "current"} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Context tones">
    <div class="poodle-specimen__row poodle-specimen__row--tones">
      <span class="poodle-specimen__chip poodle-specimen__chip--inverse"><Spinner variant="ring" tone="current" /></span>
      <span class="poodle-specimen__chip"><Spinner variant="ring" tone="accent" /></span>
      <span class="poodle-specimen__chip"><Spinner variant="grid" tone="muted" /></span>
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <div class="poodle-specimen__row">
      <Spinner variant="ring" {size} />
      <Spinner variant="grid" {size} />
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-specimen__row">
      <Spinner variant="ring" {density} />
      <Spinner variant="grid" {density} />
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1rem;
  }

  .poodle-specimen__row--tones {
    gap: 0.75rem;
  }

  .poodle-specimen__chip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 2.5rem;
    min-height: 2.5rem;
    padding: 0.5rem;
    border: 1px solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
  }

  .poodle-specimen__chip--inverse {
    background: var(--poodle-color-text-primary);
    color: var(--poodle-color-text-inverse);
  }
</style>

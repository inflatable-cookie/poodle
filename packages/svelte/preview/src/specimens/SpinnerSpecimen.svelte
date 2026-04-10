<script lang="ts">
  import { Spinner } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const spinnerSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Ring">
    <div class="specimen__row">
      {#each spinnerSizes as size}
        <Spinner variant="ring" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="CLI grid">
    <div class="specimen__row">
      {#each spinnerSizes as size, index}
        <Spinner variant="grid" {size} tone={index === 0 ? "muted" : index === 2 ? "accent" : "current"} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Context tones">
    <div class="specimen__row specimen__row--tones">
      <span class="specimen__chip specimen__chip--inverse"><Spinner variant="ring" tone="current" /></span>
      <span class="specimen__chip"><Spinner variant="ring" tone="accent" /></span>
      <span class="specimen__chip"><Spinner variant="grid" tone="muted" /></span>
    </div>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <div class="specimen__row">
      <Spinner variant="ring" {size} />
      <Spinner variant="grid" {size} />
    </div>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <div class="specimen__row">
      <Spinner variant="ring" {density} />
      <Spinner variant="grid" {density} />
    </div>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1rem;
  }

  .specimen__row--tones {
    gap: 0.75rem;
  }

  .specimen__chip {
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

  .specimen__chip--inverse {
    background: var(--poodle-color-text-primary);
    color: var(--poodle-color-text-inverse);
  }
</style>

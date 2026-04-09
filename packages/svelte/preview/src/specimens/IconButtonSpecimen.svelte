<script lang="ts">
  import { IconButton } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import { plus, settings, x, trash2, star, mapPin, ban, refreshCw } from "@poodle/icons-lucide";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <SpecimenGroup label="Variants">
    <div class="button-row">
      <IconButton icon={plus} ariaLabel="Add" variant="primary" />
      <IconButton icon={settings} ariaLabel="Settings" variant="secondary" />
      <IconButton icon={x} ariaLabel="Close" variant="ghost" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Danger tone">
    <div class="button-row">
      <IconButton icon={trash2} ariaLabel="Delete" variant="primary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="secondary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="ghost" tone="danger" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="button-row">
      {#each controlSizes as size}
        <IconButton icon={star} ariaLabel={`Favorite (${size})`} {size} variant="secondary" />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="button-row">
          <span class="specimen__label">{density}</span>
          <IconButton icon={settings} ariaLabel="Settings" {density} variant="secondary" />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="States">
    <div class="button-row">
      <IconButton icon={mapPin} ariaLabel="Pin" pressed={true} variant="secondary" />
      <IconButton icon={ban} ariaLabel="Disabled" disabled variant="secondary" />
      <IconButton icon={refreshCw} ariaLabel="Loading" loading variant="secondary" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="String name (built-in internals)">
    <div class="button-row">
      <IconButton icon="plus" ariaLabel="Add" variant="secondary" />
      <IconButton icon="search" ariaLabel="Search" variant="secondary" />
      <IconButton icon="x" ariaLabel="Close" variant="ghost" />
    </div>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .button-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>

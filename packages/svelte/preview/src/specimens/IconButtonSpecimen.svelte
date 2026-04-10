<script lang="ts">
  import { IconButton } from "@poodle/svelte-primitives";
  import { plus, settings, x, trash2, star, mapPin, ban, refreshCw } from "@poodle/icons-lucide";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
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

  <svelte:fragment slot="sizes" let:size>
    <IconButton icon={star} ariaLabel={`Favorite (${size})`} variant="secondary" />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <IconButton icon={settings} ariaLabel="Settings" variant="secondary" />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .button-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>

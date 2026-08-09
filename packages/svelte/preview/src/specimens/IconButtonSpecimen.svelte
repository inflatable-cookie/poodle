<script lang="ts">
  import { IconButton } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const [plus, settings, x, trash2, star, mapPin, ban, refreshCw] =
    ["plus", "settings", "x", "trash-2", "star", "map-pin", "ban", "refresh-cw"] as const;

  let bold = $state(false);
  let italic = $state(false);
  let underline = $state(true);
  let pinned = $state(false);
  let starred = $state(false);
</script>

<SpecimenLayout showDensities={false}>
  <SpecimenGroup label="Variants">
    <div class="poodle-button-row">
      <IconButton icon={plus} ariaLabel="Add" variant="primary" />
      <IconButton icon={settings} ariaLabel="Settings" variant="secondary" />
      <IconButton icon={x} ariaLabel="Close" variant="ghost" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Danger tone">
    <div class="poodle-button-row">
      <IconButton icon={trash2} ariaLabel="Delete" variant="primary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="secondary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="ghost" tone="danger" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Toggle (text editor toolbar)">
    <div class="poodle-button-row">
      <IconButton icon="bold" ariaLabel="Bold" variant="ghost" bind:pressed={bold} />
      <IconButton icon="italic" ariaLabel="Italic" variant="ghost" bind:pressed={italic} />
      <IconButton icon="underline" ariaLabel="Underline" variant="ghost" bind:pressed={underline} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Toggle (secondary variant)">
    <div class="poodle-button-row">
      <IconButton icon={mapPin} ariaLabel="Pin" variant="secondary" bind:pressed={pinned} />
      <IconButton icon={star} ariaLabel="Favorite" variant="secondary" bind:pressed={starred} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled and loading">
    <div class="poodle-button-row">
      <IconButton icon={ban} ariaLabel="Disabled" disabled variant="secondary" />
      <IconButton icon={refreshCw} ariaLabel="Loading" loading variant="secondary" />
    </div>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <IconButton icon={star} ariaLabel={`Favorite (${size})`} variant="secondary" {size} />
  {/snippet}

</SpecimenLayout>

<style>
  .poodle-button-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>

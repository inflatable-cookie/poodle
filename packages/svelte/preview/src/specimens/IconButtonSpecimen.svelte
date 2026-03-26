<script lang="ts">
  import { IconButton, Eyebrow } from "@poodle/svelte-primitives";
  import { plus, settings, x, trash2, star, mapPin, ban, refreshCw } from "@poodle/icons-lucide";

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Variants</Eyebrow>
    <div class="button-row">
      <IconButton icon={plus} ariaLabel="Add" variant="primary" />
      <IconButton icon={settings} ariaLabel="Settings" variant="secondary" />
      <IconButton icon={x} ariaLabel="Close" variant="ghost" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Danger tone</Eyebrow>
    <div class="button-row">
      <IconButton icon={trash2} ariaLabel="Delete" variant="primary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="secondary" tone="danger" />
      <IconButton icon={trash2} ariaLabel="Delete" variant="ghost" tone="danger" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="button-row">
      {#each controlSizes as size}
        <IconButton icon={star} ariaLabel={`Favorite (${size})`} {size} variant="secondary" />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <div class="button-row">
      <IconButton icon={mapPin} ariaLabel="Pin" pressed={true} variant="secondary" />
      <IconButton icon={ban} ariaLabel="Disabled" disabled variant="secondary" />
      <IconButton icon={refreshCw} ariaLabel="Loading" loading variant="secondary" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>String name (built-in internals)</Eyebrow>
    <div class="button-row">
      <IconButton icon="plus" ariaLabel="Add" variant="secondary" />
      <IconButton icon="search" ariaLabel="Search" variant="secondary" />
      <IconButton icon="x" ariaLabel="Close" variant="ghost" />
    </div>
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

  .button-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>

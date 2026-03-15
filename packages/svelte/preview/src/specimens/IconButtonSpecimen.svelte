<script lang="ts">
  import { IconButton, Eyebrow } from "@pug/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Variants</Eyebrow>
    <div class="button-row">
      <IconButton icon="x" ariaLabel="Close" variant="ghost" />
      <IconButton icon="plus" ariaLabel="Add" variant="primary" />
      <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
      <IconButton icon="trash-2" ariaLabel="Delete" variant="danger" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="button-row">
      <IconButton icon="star" ariaLabel="Favorite (small)" size="sm" variant="secondary" />
      <IconButton icon="star" ariaLabel="Favorite (medium)" size="md" variant="secondary" />
      <IconButton icon="star" ariaLabel="Favorite (large)" size="lg" variant="secondary" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <div class="button-row">
      <IconButton icon="map-pin" ariaLabel="Pin" isPressed={true} variant="secondary" />
      <IconButton icon="ban" ariaLabel="Disabled" isDisabled variant="secondary" />
      <IconButton icon="refresh-cw" ariaLabel="Loading" isLoading variant="secondary" />
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

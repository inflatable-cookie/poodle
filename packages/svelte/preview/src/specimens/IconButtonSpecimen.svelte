<script lang="ts">
  import { IconButton, Eyebrow } from "@flint/svelte-primitives";
  import { plus, settings, x, trash2, star, mapPin, ban, refreshCw } from "@flint/icons-lucide";
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
      <IconButton icon={star} ariaLabel="Favorite (small)" size="sm" variant="secondary" />
      <IconButton icon={star} ariaLabel="Favorite (medium)" size="md" variant="secondary" />
      <IconButton icon={star} ariaLabel="Favorite (large)" size="lg" variant="secondary" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <div class="button-row">
      <IconButton icon={mapPin} ariaLabel="Pin" isPressed={true} variant="secondary" />
      <IconButton icon={ban} ariaLabel="Disabled" isDisabled variant="secondary" />
      <IconButton icon={refreshCw} ariaLabel="Loading" isLoading variant="secondary" />
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

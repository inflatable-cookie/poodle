<script lang="ts">
  import { Button, Eyebrow } from "@flint/svelte-primitives";

  let clickLog = "No button clicked yet.";

  function log(label: string): void {
    clickLog = `Clicked: ${label}`;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Variants</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" on:click={() => log("Primary")}>Primary</Button>
      <Button variant="secondary" on:click={() => log("Secondary")}>Secondary</Button>
      <Button variant="ghost" on:click={() => log("Ghost")}>Ghost</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Danger tone</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" tone="danger" on:click={() => log("Danger primary")}>Danger primary</Button>
      <Button variant="secondary" tone="danger" on:click={() => log("Danger secondary")}>Danger secondary</Button>
      <Button variant="ghost" tone="danger" on:click={() => log("Danger ghost")}>Danger ghost</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With icons</Eyebrow>
    <div class="specimen__row">
      <Button leadingIcon="plus" on:click={() => log("Leading icon")}>Create</Button>
      <Button trailingIcon="external-link" on:click={() => log("Trailing icon")}>Open</Button>
      <Button leadingIcon="save" trailingIcon="check" on:click={() => log("Both icons")}>Save</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With chevron</Eyebrow>
    <div class="specimen__row">
      <Button chevron on:click={() => log("Chevron")}>Options</Button>
      <Button variant="primary" chevron on:click={() => log("Primary chevron")}>Actions</Button>
      <Button leadingIcon="filter" chevron on:click={() => log("Icon + chevron")}>Filter</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" size="sm" on:click={() => log("Small")}>Small</Button>
      <Button variant="primary" size="md" on:click={() => log("Medium")}>Medium</Button>
      <Button variant="primary" size="lg" on:click={() => log("Large")}>Large</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" isDisabled>Disabled</Button>
      <Button variant="primary" isLoading>Loading</Button>
      <Button variant="secondary" isDisabled>Disabled secondary</Button>
    </div>
  </div>

  <p class="specimen__log">{clickLog}</p>
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__log {
    font-size: 0.8125rem;
    color: var(--flint-color-text-secondary);
    margin: 0;
    padding: 0.5rem 0.75rem;
    background: color-mix(in srgb, var(--flint-color-background-canvas) 80%, transparent);
    border-radius: var(--flint-radius-control);
  }
</style>

<script lang="ts">
  import { Button, Eyebrow } from "@pug/svelte-primitives";

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
      <Button variant="danger" on:click={() => log("Danger")}>Danger</Button>
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
    color: var(--pug-color-text-secondary);
    margin: 0;
    padding: 0.5rem 0.75rem;
    background: color-mix(in srgb, var(--pug-color-background-canvas) 80%, transparent);
    border-radius: var(--pug-radius-control);
  }
</style>

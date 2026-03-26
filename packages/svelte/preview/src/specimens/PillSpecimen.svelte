<script lang="ts">
  import { Pill, Eyebrow } from "@poodle/svelte-primitives";

  const pillSizes = ["xs", "sm", "md", "lg", "xl"] as const;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Tones</Eyebrow>
    <div class="specimen__row">
      <Pill tone="neutral">Neutral</Pill>
      <Pill tone="info">Info</Pill>
      <Pill tone="success">Success</Pill>
      <Pill tone="warning">Warning</Pill>
      <Pill tone="danger">Danger</Pill>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each pillSizes as size}
        <Pill {size}>{size.toUpperCase()}</Pill>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Pill {density}>Label</Pill>
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Code font</Eyebrow>
    <div class="specimen__row">
      <Pill font="code">v2.4.1</Pill>
      <Pill font="code" tone="success">stable</Pill>
      <Pill font="code" tone="warning">beta</Pill>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Muted</Eyebrow>
    <div class="specimen__row">
      <Pill muted>Muted neutral</Pill>
      <Pill muted tone="success">Muted success</Pill>
      <Pill muted tone="danger">Muted danger</Pill>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Badge</Eyebrow>
    <div class="specimen__row">
      <Pill appearance="badge">3</Pill>
      <Pill appearance="badge">12</Pill>
      <Pill appearance="badge">99+</Pill>
      <Pill appearance="badge">New</Pill>
      <Pill appearance="badge" tone="neutral">Draft</Pill>
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

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
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

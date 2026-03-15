<script lang="ts">
  import { Pill, Eyebrow } from "@pug/svelte-primitives";
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
      <Pill size="sm">Small</Pill>
      <Pill size="md">Medium</Pill>
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
      <Pill isMuted>Muted neutral</Pill>
      <Pill isMuted tone="success">Muted success</Pill>
      <Pill isMuted tone="danger">Muted danger</Pill>
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
</style>

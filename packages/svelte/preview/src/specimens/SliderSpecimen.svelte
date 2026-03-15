<script lang="ts">
  import { Slider, Eyebrow } from "@pug/svelte-primitives";

  let volume = 65;
  let opacity = 100;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default</Eyebrow>
    <Slider
      value={volume}
      min={0}
      max={100}
      ariaLabel="Volume"
      on:valueChange={(e) => (volume = e.detail.value)}
    />
    <p>Volume: <strong>{volume}%</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>With step</Eyebrow>
    <Slider
      value={opacity}
      min={0}
      max={100}
      step={10}
      ariaLabel="Opacity"
      on:valueChange={(e) => (opacity = e.detail.value)}
    />
    <p>Opacity: <strong>{opacity}%</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" isDisabled />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 20rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>

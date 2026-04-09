<script lang="ts">
  import { ColorPicker, Eyebrow } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let color = "#6366f1";
  let alphaColor = "#3b82f6";

  const brandSwatches = [
    "#ef4444", "#f97316", "#eab308", "#22c55e",
    "#3b82f6", "#6366f1", "#8b5cf6", "#ec4899",
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic picker</Eyebrow>
    <ColorPicker bind:value={color} />
    <p>Selected: <strong>{color}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <ColorPicker value="#6366f1" {size} />
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <ColorPicker {density} />
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With swatches</Eyebrow>
    <ColorPicker
      bind:value={color}
      swatches={brandSwatches}
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With alpha</Eyebrow>
    <ColorPicker bind:value={alphaColor} showAlpha />
    <p>Selected: <strong>{alphaColor}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Default open, RGB mode</Eyebrow>
    <ColorPicker value="#22c55e" defaultOpen defaultMode="rgb" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Preview only (no input)</Eyebrow>
    <ColorPicker value={color} showInput={false} />
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <ColorPicker value="#22c55e" disabled />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__row { display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  p { margin: 0; }
</style>

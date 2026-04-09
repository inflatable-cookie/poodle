<script lang="ts">
  import { ColorPicker } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  <SpecimenGroup label="Basic picker">
    <ColorPicker bind:value={color} />
    <p>Selected: <strong>{color}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <ColorPicker value="#6366f1" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <ColorPicker {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With swatches">
    <ColorPicker
      bind:value={color}
      swatches={brandSwatches}
    />
  </SpecimenGroup>

  <SpecimenGroup label="With alpha">
    <ColorPicker bind:value={alphaColor} showAlpha />
    <p>Selected: <strong>{alphaColor}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Default open, RGB mode">
    <ColorPicker value="#22c55e" defaultOpen defaultMode="rgb" />
  </SpecimenGroup>

  <SpecimenGroup label="Preview only (no input)">
    <ColorPicker value={color} showInput={false} />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <ColorPicker value="#22c55e" disabled />
  </SpecimenGroup>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1rem; }
  .specimen__row { display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; }
  .specimen__stack { display: flex; flex-direction: column; gap: 0.5rem; }
  .specimen__label { font-size: 0.75rem; font-family: var(--poodle-typography-code-family); color: var(--poodle-color-text-muted); min-width: 6rem; }
  p { margin: 0; }
</style>

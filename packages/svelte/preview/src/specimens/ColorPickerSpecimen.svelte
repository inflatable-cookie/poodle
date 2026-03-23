<script lang="ts">
  import { ColorPicker, Eyebrow } from "@poodle/svelte-primitives";

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
    <ColorPicker value="#22c55e" isDisabled />
  </div>
</div>

<style>
  .specimen { display: flex; flex-direction: column; gap: 1.5rem; }
  .specimen__group { display: flex; flex-direction: column; gap: 0.5rem; }
  p { margin: 0; }
</style>

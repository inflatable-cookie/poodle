<script lang="ts">
  import { ColorPicker } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let color = "#6366f1";
  let alphaColor = "#3b82f6";

  const brandSwatches = [
    "#ef4444", "#f97316", "#eab308", "#22c55e",
    "#3b82f6", "#6366f1", "#8b5cf6", "#ec4899",
  ];
</script>

<SpecimenLayout>
  <SpecimenGroup label="Basic picker">
    <ColorPicker bind:value={color} />
    <p>Selected: <strong>{color}</strong></p>
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

  <svelte:fragment slot="sizes" let:size>
    <ColorPicker value="#6366f1" {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <ColorPicker {density} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  p { margin: 0; }
</style>

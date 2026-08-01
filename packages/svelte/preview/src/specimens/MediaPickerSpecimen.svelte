<script lang="ts">
  import { MediaPicker } from "@poodle/svelte";
  import { Button } from "@poodle/svelte";
  import type { MediaPickerItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let open = $state(false);
  let selected = $state("");
  let sizeOpenMap: Record<string, boolean> = {};
  let densityOpenMap: Record<string, boolean> = {};
  let sizeSelectedMap: Record<string, string> = {};
  let densitySelectedMap: Record<string, string> = {};

  const sampleItems: MediaPickerItem[] = [
    { id: "1", label: "hero-banner.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "2", label: "logo-dark.png", thumbnailUrl: undefined, kind: "image" },
    { id: "3", label: "product-shot.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "4", label: "report-q4.pdf", thumbnailUrl: undefined, kind: "document" },
    { id: "5", label: "team-photo.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "6", label: "presentation.pdf", thumbnailUrl: undefined, kind: "document" },
  ];
</script>

<SpecimenLayout>
  <div class="poodle-specimen">
    <SpecimenGroup label="Media picker dialog">
      <Button variant="secondary" onClick={() => (open = true)}>Select media</Button>
      <MediaPicker
        {open}
        items={sampleItems}
        title="Select an asset"
        onSelect={(item) => { selected = item.label; open = false; }}
        onOpenChange={(nextOpen) => (open = nextOpen)}
      />
      {#if selected}
        <p>Selected: <strong>{selected}</strong></p>
      {/if}
    </SpecimenGroup>
  </div>

  {#snippet sizes(size)}
    <div class="poodle-media-picker-specimen__variant">
      <p class="poodle-media-picker-specimen__label">{size.toUpperCase()}</p>
      <Button variant="secondary" size={size} onClick={() => (sizeOpenMap[size] = true)}>
        Open {size.toUpperCase()} picker
      </Button>
      <MediaPicker
        open={sizeOpenMap[size] ?? false}
        items={sampleItems}
        title={`${size.toUpperCase()} asset picker`}
        {size}
        onSelect={(item) => {
          sizeSelectedMap[size] = item.label;
          sizeOpenMap[size] = false;
        }}
        onOpenChange={(nextOpen) => (sizeOpenMap[size] = nextOpen)}
      />
      {#if sizeSelectedMap[size]}
        <p>Selected: <strong>{sizeSelectedMap[size]}</strong></p>
      {/if}
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-media-picker-specimen__variant">
      <p class="poodle-media-picker-specimen__label">{density}</p>
      <Button variant="secondary" onClick={() => (densityOpenMap[density] = true)}>
        Open {density} picker
      </Button>
      <MediaPicker
        open={densityOpenMap[density] ?? false}
        items={sampleItems}
        title={`${density} asset picker`}
        {density}
        onSelect={(item) => {
          densitySelectedMap[density] = item.label;
          densityOpenMap[density] = false;
        }}
        onOpenChange={(nextOpen) => (densityOpenMap[density] = nextOpen)}
      />
      {#if densitySelectedMap[density]}
        <p>Selected: <strong>{densitySelectedMap[density]}</strong></p>
      {/if}
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-media-picker-specimen__variant {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .poodle-media-picker-specimen__label {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  p { margin: 0; }
</style>

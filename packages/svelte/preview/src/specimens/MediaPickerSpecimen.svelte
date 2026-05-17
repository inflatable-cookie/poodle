<script lang="ts">
  import { MediaPicker } from "@poodle/svelte";
  import { Button, UiPresentationProvider } from "@poodle/svelte";
  import type { MediaPickerItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let open = false;
  let compactOpen = false;
  let selected = "";
  let compactSelected = "";
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

    <SpecimenGroup label="Semantic presentation">
      <UiPresentationProvider density="compact" sizeScale="sm">
        <div class="poodle-specimen__stack">
          <Button variant="secondary" onClick={() => (compactOpen = true)}>Open compact picker</Button>
          <MediaPicker
            open={compactOpen}
            items={sampleItems}
            title="Compact asset picker"
            onSelect={(item) => { compactSelected = item.label; compactOpen = false; }}
            onOpenChange={(nextOpen) => (compactOpen = nextOpen)}
          />
          <MediaPicker
            open={false}
            items={sampleItems}
            title="Prominent asset picker"
            sizeRole="prominent"
          />
          {#if compactSelected}
            <p>Selected: <strong>{compactSelected}</strong></p>
          {/if}
        </div>
      </UiPresentationProvider>
    </SpecimenGroup>
  </div>

  <svelte:fragment slot="sizes" let:size>
    <SpecimenGroup label={size.toUpperCase()}>
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
    </SpecimenGroup>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <SpecimenGroup label={density}>
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
    </SpecimenGroup>
  </svelte:fragment>
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__stack {
    display: grid;
    gap: 0.75rem;
  }

  p { margin: 0; }
</style>

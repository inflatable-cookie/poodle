<script lang="ts">
  import { MediaPicker } from "@poodle/svelte";
  import { Button, UiPresentationProvider } from "@poodle/svelte";
  import type { MediaPickerItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let open = false;
  let compactOpen = false;
  let selected = "";
  let compactSelected = "";

  const sampleItems: MediaPickerItem[] = [
    { id: "1", label: "hero-banner.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "2", label: "logo-dark.png", thumbnailUrl: undefined, kind: "image" },
    { id: "3", label: "product-shot.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "4", label: "report-q4.pdf", thumbnailUrl: undefined, kind: "document" },
    { id: "5", label: "team-photo.jpg", thumbnailUrl: undefined, kind: "image" },
    { id: "6", label: "presentation.pdf", thumbnailUrl: undefined, kind: "document" },
  ];
</script>

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

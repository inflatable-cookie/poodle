<script lang="ts">
  import { MediaPicker } from "@poodle/svelte-composites";
  import { Eyebrow, Button, UiPresentationProvider } from "@poodle/svelte-primitives";
  import type { MediaPickerItem } from "@poodle/svelte-composites";

  let open: boolean | null = null;
  let compactOpen: boolean | null = null;
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

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Media picker dialog</Eyebrow>
    <Button variant="secondary" on:click={() => (open = true)}>Select media</Button>
    <MediaPicker
      {open}
      items={sampleItems}
      title="Select an asset"
      on:select={(e) => { selected = e.detail.item.label; open = false; }}
      on:openChange={(e) => (open = e.detail.open ? true : null)}
    />
    {#if selected}
      <p>Selected: <strong>{selected}</strong></p>
    {/if}
  </div>

  <div class="specimen__group">
    <Eyebrow>Semantic presentation</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <div class="specimen__stack">
        <Button variant="secondary" on:click={() => (compactOpen = true)}>Open compact picker</Button>
        <MediaPicker
          open={compactOpen}
          items={sampleItems}
          title="Compact asset picker"
          on:select={(e) => { compactSelected = e.detail.item.label; compactOpen = false; }}
          on:openChange={(e) => (compactOpen = e.detail.open ? true : null)}
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

  .specimen__stack {
    display: grid;
    gap: 0.75rem;
  }

  p { margin: 0; }
</style>

<script lang="ts">
  import { MediaPicker } from "@pug/svelte-composites";
  import { Eyebrow, Button } from "@pug/svelte-primitives";
  import type { MediaPickerItem } from "@pug/svelte-composites";

  let open: boolean | null = null;
  let selected = "";

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

  p { margin: 0; }
</style>

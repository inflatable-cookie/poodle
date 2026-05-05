<script lang="ts">
  import { EmbedPreview } from "@poodle/svelte";
  import type { ParsedEmbed } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const youtubeParsed: ParsedEmbed = {
    provider: "youtube",
    id: "dQw4w9WgXcQ",
    originalUrl: "https://youtube.com/watch?v=dQw4w9WgXcQ",
  };

  const vimeoParsed: ParsedEmbed = {
    provider: "vimeo",
    id: "76979871",
    originalUrl: "https://vimeo.com/76979871",
  };
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="YouTube embed">
    <EmbedPreview parsed={youtubeParsed} />
  </SpecimenGroup>

  <SpecimenGroup label="Vimeo embed">
    <EmbedPreview parsed={vimeoParsed} />
  </SpecimenGroup>

  <SpecimenGroup label="Loading state">
    <EmbedPreview loading />
  </SpecimenGroup>

  <SpecimenGroup label="Error state">
    <EmbedPreview error="Failed to load embed. The URL may be invalid or the provider is unavailable." />
  </SpecimenGroup>

  <SpecimenGroup label="Empty state">
    <EmbedPreview emptyMessage="Paste a URL above to see a preview" />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>

<script lang="ts">
  import {
    RICH_TEXT_STANDARD_FEATURES,
    RichTextEditor,
    type ProseMirrorDocumentJSON,
    type RichTextFeature,
    type RichTextImageInput,
  } from "@inflatable-cookie/poodle-svelte/rich-text";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import {
    RICH_TEXT_IMAGE_ALT,
    RICH_TEXT_IMAGE_DOCUMENT,
    RICH_TEXT_IMAGE_FEATURES,
    RICH_TEXT_IMAGE_SRC,
    RICH_TEXT_STANDARD_DOCUMENT,
  } from "./web-editor-documents";

  let document = $state<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  let imagesOn = $state(false);
  let imageDocument = $state<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);

  const imageFeatures: readonly RichTextFeature[] = RICH_TEXT_IMAGE_FEATURES;
  const standardFeatures: readonly RichTextFeature[] = RICH_TEXT_STANDARD_FEATURES;

  async function requestImage(): Promise<RichTextImageInput | null> {
    return { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT };
  }

  function toggleImages(): void {
    imagesOn = !imagesOn;
  }
</script>

<SpecimenLayout>
  {#snippet children()}
    <SpecimenGroup
      label="Formatted document"
      description="Ordinary formatting and a table. Host JSON updates on each user transaction."
    >
      <div class="editor-frame" data-part="live-editor">
        <RichTextEditor
          value={document}
          features={standardFeatures}
          ariaLabel="Formatted rich text"
          onChange={(next) => (document = next)}
        />
      </div>
      <pre class="readout" data-part="host-document">{JSON.stringify(document)}</pre>
    </SpecimenGroup>

    <SpecimenGroup
      label="Image policy"
      description="Images are an explicit project choice. Embeds are not a v1 feature."
    >
      <button type="button" data-part="images-toggle" aria-pressed={imagesOn} onclick={toggleImages}>
        {imagesOn ? "Images on" : "Images off"}
      </button>
      <div class="editor-frame" data-part="image-policy-editor">
        {#if imagesOn}
          <RichTextEditor
            value={imageDocument}
            features={imageFeatures}
            requestImage={requestImage}
            ariaLabel="Rich text with images"
            onChange={(next) => (imageDocument = next)}
          />
        {:else}
          <RichTextEditor
            value={RICH_TEXT_STANDARD_DOCUMENT}
            features={standardFeatures}
            ariaLabel="Rich text without images"
          />
        {/if}
      </div>
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density}>
      <div class="editor-frame">
        <RichTextEditor
          value={RICH_TEXT_STANDARD_DOCUMENT}
          {density}
          ariaLabel="Density sample"
        />
      </div>
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

<style>
  .editor-frame {
    height: 20rem;
    margin-top: 0.75rem;
  }
  .readout {
    margin: 0.75rem 0 0;
    padding: 0.5rem;
    border-radius: 0.25rem;
    background: var(--poodle-color-background-surface);
    font-size: 0.75rem;
    white-space: pre-wrap;
    max-height: 8rem;
    overflow: auto;
  }
</style>

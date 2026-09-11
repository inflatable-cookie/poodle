<script lang="ts">
  import {
    RICH_TEXT_STANDARD_FEATURES,
    RichTextEditor,
    type ProseMirrorDocumentJSON,
    type RichTextCommand,
    type RichTextFeature,
    type RichTextImageInput,
  } from "@inflatable-cookie/poodle-svelte/rich-text";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import {
    RICH_TEXT_IMAGE_DOCUMENT,
    RICH_TEXT_IMAGE_FEATURES,
    RICH_TEXT_IMAGE_REQUEST_DELAY_MS,
    RICH_TEXT_PICKED_IMAGE_ALT,
    RICH_TEXT_PICKED_IMAGE_SRC,
    RICH_TEXT_STANDARD_DOCUMENT,
    countRichTextImages,
  } from "./web-editor-documents";

  let document = $state<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  let imagesOn = $state(false);
  let imageDocument = $state<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);
  let imageRequests = $state(0);
  let imageChanges = $state(0);
  const imageCount = $derived(countRichTextImages(imageDocument));

  const imageFeatures: readonly RichTextFeature[] = RICH_TEXT_IMAGE_FEATURES;
  const standardFeatures: readonly RichTextFeature[] = RICH_TEXT_STANDARD_FEATURES;
  /** Consumers choose commands, not their icons or grouping. */
  const subsetToolbar: readonly RichTextCommand[] = ["bold", "italic", "link"];

  /**
   * Host-owned asset choice, standing in for a consumer media picker. The
   * fixture is a self-contained raster data URL: no network, no DNS, no
   * mutable remote content.
   */
  async function requestImage(): Promise<RichTextImageInput | null> {
    imageRequests += 1;
    await new Promise((resolve) => setTimeout(resolve, RICH_TEXT_IMAGE_REQUEST_DELAY_MS));
    return { src: RICH_TEXT_PICKED_IMAGE_SRC, alt: RICH_TEXT_PICKED_IMAGE_ALT };
  }

  function onImageDocumentChange(next: ProseMirrorDocumentJSON): void {
    imageChanges += 1;
    imageDocument = next;
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
      label="Command postures"
      description="Explicit toolbar subset and a disabled editor. Selection drives active and table-context states: click into the bold text and the Bold control lights up; enter the table to enable the row, column, and delete actions."
    >
      <div class="editor-frame" data-part="subset-editor">
        <RichTextEditor
          value={document}
          features={standardFeatures}
          toolbar={subsetToolbar}
          ariaLabel="Subset rich text"
        />
      </div>
      <div class="editor-frame" data-part="disabled-editor">
        <RichTextEditor
          value={RICH_TEXT_STANDARD_DOCUMENT}
          features={standardFeatures}
          disabled
          ariaLabel="Disabled rich text"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup
      label="Image policy"
      description="Images are an explicit project choice. Embeds are not a v1 feature. The seeded document loads offline; Insert image asks the host for an asset and inserts exactly one image at the retained selection. The host document below is retained while images are off."
    >
      <button
        type="button"
        class="images-toggle"
        data-part="images-toggle"
        aria-pressed={imagesOn}
        onclick={toggleImages}
      >
        {imagesOn ? "Images on" : "Images off"}
      </button>
      <div class="editor-frame" data-part="image-policy-editor">
        {#if imagesOn}
          <RichTextEditor
            value={imageDocument}
            features={imageFeatures}
            requestImage={requestImage}
            ariaLabel="Rich text with images"
            onChange={onImageDocumentChange}
          />
        {:else}
          <RichTextEditor
            value={RICH_TEXT_STANDARD_DOCUMENT}
            features={standardFeatures}
            ariaLabel="Rich text without images"
          />
        {/if}
      </div>
      <div class="image-policy-feedback" data-part="image-policy-feedback">
        <p class="image-policy-metric" data-part="image-count" data-count={String(imageCount)}>
          Host document images: {imageCount}
        </p>
        <p
          class="image-policy-metric"
          data-part="image-request-count"
          data-count={String(imageRequests)}
        >
          Host image requests: {imageRequests}
        </p>
        <p
          class="image-policy-metric"
          data-part="image-change-count"
          data-count={String(imageChanges)}
        >
          Host document changes: {imageChanges}
        </p>
        <pre class="readout" data-part="image-host-document">{JSON.stringify(imageDocument)}</pre>
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
  /* Explicit paired chrome: the raw toggle must not inherit either
     gallery's page-level button layout, or the visual gate reads the
     specimen pair as divergent (g18.013). */
  .images-toggle {
    appearance: none;
    display: inline-flex;
    align-items: center;
    width: fit-content;
    gap: 0.375rem;
    padding: 0.3125rem 0.625rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font: inherit;
    font-size: 0.8125rem;
    line-height: 1.2;
    cursor: pointer;
  }
  .images-toggle[aria-pressed="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
  }
  .editor-frame {
    height: 20rem;
    margin-top: 0.75rem;
  }
  .image-policy-feedback {
    margin-top: 0.75rem;
  }
  .image-policy-metric {
    margin: 0;
    font-size: 0.8125rem;
    line-height: 1.4;
    color: var(--poodle-color-text-secondary);
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

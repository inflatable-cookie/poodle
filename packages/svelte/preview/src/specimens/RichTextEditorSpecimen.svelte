<script lang="ts">
  import { Button, Dialog, TextInput } from "@inflatable-cookie/poodle-svelte";
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
    RICH_TEXT_HEADING_DOCUMENT,
    RICH_TEXT_IMAGE_DOCUMENT,
    RICH_TEXT_IMAGE_FEATURES,
    RICH_TEXT_IMAGE_ALT,
    RICH_TEXT_IMAGE_SRC,
    RICH_TEXT_PICKED_IMAGE_ALT,
    RICH_TEXT_PICKED_IMAGE_SRC,
    RICH_TEXT_SPARSE_HEADING_TOOLBAR,
    RICH_TEXT_STANDARD_DOCUMENT,
    countRichTextImages,
  } from "./web-editor-documents";

  let document = $state<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  let imagesOn = $state(false);
  let imageDocument = $state<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);
  let imageRequests = $state(0);
  let imageChanges = $state(0);
  let imagePickerOpen = $state(false);
  let selectedImage = $state<"seeded" | "picked">("picked");
  let customImageSource = $state("");
  let imageAlt = $state(RICH_TEXT_PICKED_IMAGE_ALT);
  let imageTitle = $state("");
  let settleImageRequest: ((image: RichTextImageInput | null) => void) | null = null;
  const imageCount = $derived(countRichTextImages(imageDocument));

  const imageFeatures: readonly RichTextFeature[] = RICH_TEXT_IMAGE_FEATURES;
  const standardFeatures: readonly RichTextFeature[] = RICH_TEXT_STANDARD_FEATURES;
  /** Consumers choose commands, not their icons or grouping. */
  const subsetToolbar: readonly RichTextCommand[] = ["bold", "italic", "link"];

  async function requestImage(): Promise<RichTextImageInput | null> {
    imageRequests += 1;
    imagePickerOpen = true;
    return new Promise((resolve) => {
      settleImageRequest = resolve;
    });
  }

  function chooseImage(choice: "seeded" | "picked"): void {
    selectedImage = choice;
    customImageSource = "";
    imageAlt = choice === "picked" ? RICH_TEXT_PICKED_IMAGE_ALT : RICH_TEXT_IMAGE_ALT;
  }

  function cancelImageRequest(): void {
    settleImageRequest?.(null);
    settleImageRequest = null;
    imagePickerOpen = false;
  }

  function insertImage(): void {
    const presetSource = selectedImage === "picked" ? RICH_TEXT_PICKED_IMAGE_SRC : RICH_TEXT_IMAGE_SRC;
    const title = imageTitle.trim();
    settleImageRequest?.({
      src: customImageSource.trim() || presetSource,
      alt: imageAlt,
      ...(title ? { title } : {}),
    });
    settleImageRequest = null;
    imagePickerOpen = false;
  }

  function onImageDocumentChange(next: ProseMirrorDocumentJSON): void {
    imageChanges += 1;
    imageDocument = next;
  }

  function toggleImages(): void {
    if (imagePickerOpen) cancelImageRequest();
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
      label="Text modes"
      description="One text-mode selector replaces the separate heading buttons. Consumers choose the admitted levels: the full toolbar offers Normal text plus H1–H6, the sparse toolbar offers Normal text plus only H2 and H4, and a selection spanning different blocks reads Mixed. Choosing the active level keeps it; choosing Normal text converts back."
    >
      <div class="editor-frame" data-part="heading-mode-editor">
        <RichTextEditor
          value={RICH_TEXT_HEADING_DOCUMENT}
          features={standardFeatures}
          ariaLabel="Heading modes"
        />
      </div>
      <div class="editor-frame editor-frame--constrained" data-part="sparse-heading-editor">
        <RichTextEditor
          value={RICH_TEXT_HEADING_DOCUMENT}
          features={standardFeatures}
          toolbar={RICH_TEXT_SPARSE_HEADING_TOOLBAR}
          ariaLabel="Sparse heading modes"
        />
      </div>
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
      description="Images are an explicit project choice. Insert image opens this host-owned asset picker, which can choose a fixture or accept a custom source and accessible description. Poodle inserts the result at the retained selection."
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
      <Dialog
        open={imagePickerOpen}
        title="Choose image"
        description="Select an asset or paste a source, then describe the image for readers."
        width="lg"
        showCloseButton
        onOpenChange={(open) => {
          if (!open && imagePickerOpen) cancelImageRequest();
        }}
      >
        <div class="image-picker" data-part="image-picker">
          <div class="image-picker__assets" role="group" aria-label="Example assets">
            <button
              type="button"
              class="image-picker__asset"
              aria-pressed={selectedImage === "picked" && !customImageSource}
              onclick={() => chooseImage("picked")}
            >
              <img src={RICH_TEXT_PICKED_IMAGE_SRC} alt="" />
              <span>Revenue bars</span>
            </button>
            <button
              type="button"
              class="image-picker__asset"
              aria-pressed={selectedImage === "seeded" && !customImageSource}
              onclick={() => chooseImage("seeded")}
            >
              <img src={RICH_TEXT_IMAGE_SRC} alt="" />
              <span>Revenue line</span>
            </button>
          </div>
          <div class="image-picker__fields">
            <label for="rich-text-image-source">Custom image source</label>
            <TextInput
              id="rich-text-image-source"
              value={customImageSource}
              placeholder="https://example.com/image.png"
              inputMode="url"
              onValueChange={(value) => (customImageSource = value)}
            />
            <label for="rich-text-image-alt">Alt text</label>
            <TextInput
              id="rich-text-image-alt"
              value={imageAlt}
              placeholder="Describe the image, or leave empty if decorative"
              onValueChange={(value) => (imageAlt = value)}
            />
            <label for="rich-text-image-title">Title <span>(optional)</span></label>
            <TextInput
              id="rich-text-image-title"
              value={imageTitle}
              placeholder="Shown as supplementary information"
              onValueChange={(value) => (imageTitle = value)}
            />
          </div>
        </div>
        {#snippet actions()}
          <div class="image-picker__actions">
            <Button variant="ghost" onClick={cancelImageRequest}>Cancel</Button>
            <span data-part="image-picker-insert"><Button variant="primary" onClick={insertImage}>Insert image</Button></span>
          </div>
        {/snippet}
      </Dialog>
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
  .image-picker {
    display: grid;
    grid-template-columns: minmax(12rem, 0.8fr) minmax(16rem, 1.2fr);
    gap: 1rem 1.25rem;
  }
  .image-picker__assets { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.5rem; }
  .image-picker__asset {
    display: grid;
    gap: 0.5rem;
    align-content: start;
    padding: 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-panel);
    color: var(--poodle-color-text-primary);
    font: inherit;
    cursor: pointer;
  }
  .image-picker__asset[aria-pressed="true"] { border-color: var(--poodle-color-accent-base); }
  .image-picker__asset img { width: 100%; aspect-ratio: 2 / 1; object-fit: cover; border-radius: calc(var(--poodle-radius-control) * 0.6); image-rendering: pixelated; }
  .image-picker__asset span { font-size: 0.75rem; }
  .image-picker__fields { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.75rem; align-items: center; }
  .image-picker__fields label { color: var(--poodle-color-text-secondary); font-size: 0.75rem; }
  .image-picker__fields label span { color: var(--poodle-color-text-tertiary); }
  .image-picker__actions { display: flex; justify-content: flex-end; gap: 0.5rem; }
  @media (max-width: 42rem) {
    .image-picker { grid-template-columns: 1fr; }
    .image-picker__fields { grid-template-columns: 1fr; gap: 0.25rem; }
    .image-picker__fields :global(.poodle-text-input) { margin-bottom: 0.5rem; }
  }
  .editor-frame {
    height: 20rem;
    margin-top: 0.75rem;
  }
  /* Constrained posture: a narrow pane where the selector must stay one stop
     and the toolbar wraps by cluster instead of overflowing the page. */
  .editor-frame--constrained {
    max-width: 22rem;
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

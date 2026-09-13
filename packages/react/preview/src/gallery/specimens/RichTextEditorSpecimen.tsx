import { useRef, useState } from "react";
import { Button, Dialog, TextInput, type ControlDensity } from "@inflatable-cookie/poodle-react";
import {
  RICH_TEXT_STANDARD_FEATURES,
  RichTextEditor,
  type ProseMirrorDocumentJSON,
  type RichTextCommand,
  type RichTextImageInput,
} from "@inflatable-cookie/poodle-react/rich-text";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
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
} from "../../../../../svelte/preview/src/specimens/web-editor-documents";

/** Consumers choose commands, not their icons or grouping. */
const SUBSET_TOOLBAR: readonly RichTextCommand[] = ["bold", "italic", "link"];

export function RichTextEditorSpecimen() {
  const [document, setDocument] = useState<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  const [imagesOn, setImagesOn] = useState(false);
  const [imageDocument, setImageDocument] =
    useState<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);
  const [imageRequests, setImageRequests] = useState(0);
  const [imageChanges, setImageChanges] = useState(0);
  const [imagePickerOpen, setImagePickerOpen] = useState(false);
  const [selectedImage, setSelectedImage] = useState<"seeded" | "picked">("picked");
  const [customImageSource, setCustomImageSource] = useState("");
  const [imageAlt, setImageAlt] = useState(RICH_TEXT_PICKED_IMAGE_ALT);
  const [imageTitle, setImageTitle] = useState("");
  const settleImageRequest = useRef<((image: RichTextImageInput | null) => void) | null>(null);
  const imageCount = countRichTextImages(imageDocument);

  const onImageDocumentChange = (next: ProseMirrorDocumentJSON): void => {
    setImageChanges((count) => count + 1);
    setImageDocument(next);
  };

  const hostRequestImage = async (): Promise<RichTextImageInput | null> => {
    setImageRequests((count) => count + 1);
    setImagePickerOpen(true);
    return new Promise((resolve) => {
      settleImageRequest.current = resolve;
    });
  };

  const chooseImage = (choice: "seeded" | "picked"): void => {
    setSelectedImage(choice);
    setCustomImageSource("");
    setImageAlt(choice === "picked" ? RICH_TEXT_PICKED_IMAGE_ALT : RICH_TEXT_IMAGE_ALT);
  };

  const cancelImageRequest = (): void => {
    settleImageRequest.current?.(null);
    settleImageRequest.current = null;
    setImagePickerOpen(false);
  };

  const insertImage = (): void => {
    const presetSource = selectedImage === "picked" ? RICH_TEXT_PICKED_IMAGE_SRC : RICH_TEXT_IMAGE_SRC;
    const title = imageTitle.trim();
    settleImageRequest.current?.({
      src: customImageSource.trim() || presetSource,
      alt: imageAlt,
      ...(title ? { title } : {}),
    });
    settleImageRequest.current = null;
    setImagePickerOpen(false);
  };

  const toggleImages = (): void => {
    if (imagePickerOpen) cancelImageRequest();
    setImagesOn((enabled) => !enabled);
  };

  return (
    <SpecimenLayout
      densities={(density) => (
        <SpecimenGroup label={density}>
          <div className="rich-text-editor-frame">
            <RichTextEditor
              value={RICH_TEXT_STANDARD_DOCUMENT}
              density={density as ControlDensity}
              ariaLabel="Density sample"
            />
          </div>
        </SpecimenGroup>
      )}
    >
      <SpecimenGroup
        label="Formatted document"
        description="Ordinary formatting and a table. Host JSON updates on each user transaction."
      >
        <div className="rich-text-editor-frame" data-part="live-editor">
          <RichTextEditor
            value={document}
            features={RICH_TEXT_STANDARD_FEATURES}
            ariaLabel="Formatted rich text"
            onChange={setDocument}
          />
        </div>
        <pre className="rich-text-editor-readout" data-part="host-document">
          {JSON.stringify(document)}
        </pre>
      </SpecimenGroup>

      <SpecimenGroup
        label="Text modes"
        description="One text-mode selector replaces the separate heading buttons. Consumers choose the admitted levels: the full toolbar offers Normal text plus H1–H6, the sparse toolbar offers Normal text plus only H2 and H4, and a selection spanning different blocks reads Mixed. Choosing the active level keeps it; choosing Normal text converts back."
      >
        <div className="rich-text-editor-frame" data-part="heading-mode-editor">
          <RichTextEditor
            value={RICH_TEXT_HEADING_DOCUMENT}
            features={RICH_TEXT_STANDARD_FEATURES}
            ariaLabel="Heading modes"
          />
        </div>
        <div
          className="rich-text-editor-frame rich-text-editor-frame--constrained"
          data-part="sparse-heading-editor"
        >
          <RichTextEditor
            value={RICH_TEXT_HEADING_DOCUMENT}
            features={RICH_TEXT_STANDARD_FEATURES}
            toolbar={RICH_TEXT_SPARSE_HEADING_TOOLBAR}
            ariaLabel="Sparse heading modes"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Command postures"
        description="Explicit toolbar subset and a disabled editor. Selection drives active and table-context states: click into the bold text and the Bold control lights up; enter the table to enable the row, column, and delete actions."
      >
        <div className="rich-text-editor-frame" data-part="subset-editor">
          <RichTextEditor
            value={document}
            features={RICH_TEXT_STANDARD_FEATURES}
            toolbar={SUBSET_TOOLBAR}
            ariaLabel="Subset rich text"
          />
        </div>
        <div className="rich-text-editor-frame" data-part="disabled-editor">
          <RichTextEditor
            value={RICH_TEXT_STANDARD_DOCUMENT}
            features={RICH_TEXT_STANDARD_FEATURES}
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
          className="images-toggle"
          data-part="images-toggle"
          aria-pressed={imagesOn}
          onClick={toggleImages}
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
          actions={
            <div className="image-picker__actions">
              <Button variant="ghost" onClick={cancelImageRequest}>Cancel</Button>
              <span data-part="image-picker-insert"><Button variant="primary" onClick={insertImage}>Insert image</Button></span>
            </div>
          }
        >
          <div className="image-picker" data-part="image-picker">
            <div className="image-picker__assets" role="group" aria-label="Example assets">
              <button
                type="button"
                className="image-picker__asset"
                aria-pressed={selectedImage === "picked" && !customImageSource}
                onClick={() => chooseImage("picked")}
              >
                <img src={RICH_TEXT_PICKED_IMAGE_SRC} alt="" />
                <span>Revenue bars</span>
              </button>
              <button
                type="button"
                className="image-picker__asset"
                aria-pressed={selectedImage === "seeded" && !customImageSource}
                onClick={() => chooseImage("seeded")}
              >
                <img src={RICH_TEXT_IMAGE_SRC} alt="" />
                <span>Revenue line</span>
              </button>
            </div>
            <div className="image-picker__fields">
              <label htmlFor="rich-text-image-source">Custom image source</label>
              <TextInput
                id="rich-text-image-source"
                value={customImageSource}
                placeholder="https://example.com/image.png"
                inputMode="url"
                onValueChange={setCustomImageSource}
              />
              <label htmlFor="rich-text-image-alt">Alt text</label>
              <TextInput
                id="rich-text-image-alt"
                value={imageAlt}
                placeholder="Describe the image, or leave empty if decorative"
                onValueChange={setImageAlt}
              />
              <label htmlFor="rich-text-image-title">Title <span>(optional)</span></label>
              <TextInput
                id="rich-text-image-title"
                value={imageTitle}
                placeholder="Shown as supplementary information"
                onValueChange={setImageTitle}
              />
            </div>
          </div>
        </Dialog>
        <div className="rich-text-editor-frame" data-part="image-policy-editor">
          {imagesOn ? (
            <RichTextEditor
              value={imageDocument}
              features={RICH_TEXT_IMAGE_FEATURES}
              requestImage={hostRequestImage}
              ariaLabel="Rich text with images"
              onChange={onImageDocumentChange}
            />
          ) : (
            <RichTextEditor
              value={RICH_TEXT_STANDARD_DOCUMENT}
              features={RICH_TEXT_STANDARD_FEATURES}
              ariaLabel="Rich text without images"
            />
          )}
        </div>
        <div className="image-policy-feedback" data-part="image-policy-feedback">
          <p className="image-policy-metric" data-part="image-count" data-count={String(imageCount)}>
            Host document images: {imageCount}
          </p>
          <p
            className="image-policy-metric"
            data-part="image-request-count"
            data-count={String(imageRequests)}
          >
            Host image requests: {imageRequests}
          </p>
          <p
            className="image-policy-metric"
            data-part="image-change-count"
            data-count={String(imageChanges)}
          >
            Host document changes: {imageChanges}
          </p>
          <pre className="rich-text-editor-readout" data-part="image-host-document">
            {JSON.stringify(imageDocument)}
          </pre>
        </div>
      </SpecimenGroup>
      <style>{`
        /* Explicit paired chrome: the raw toggle must not inherit either
           gallery's page-level button layout, or the visual gate reads the
           specimen pair as divergent (g18.013). */
        .images-toggle { appearance: none; display: inline-flex; align-items: center; width: fit-content; gap: 0.375rem; padding: 0.3125rem 0.625rem; border: 0.0625rem solid var(--poodle-color-border-default); border-radius: var(--poodle-radius-control); background: var(--poodle-color-background-surface); color: var(--poodle-color-text-primary); font: inherit; font-size: 0.8125rem; line-height: 1.2; cursor: pointer; }
        .images-toggle[aria-pressed="true"] { background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent); }
        .image-picker { display: grid; grid-template-columns: minmax(12rem, 0.8fr) minmax(16rem, 1.2fr); gap: 1rem 1.25rem; }
        .image-picker__assets { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0.5rem; }
        .image-picker__asset { display: grid; gap: 0.5rem; align-content: start; padding: 0.5rem; border: 0.0625rem solid var(--poodle-color-border-default); border-radius: var(--poodle-radius-control); background: var(--poodle-color-background-panel); color: var(--poodle-color-text-primary); font: inherit; cursor: pointer; }
        .image-picker__asset[aria-pressed="true"] { border-color: var(--poodle-color-accent-base); }
        .image-picker__asset img { width: 100%; aspect-ratio: 2 / 1; object-fit: cover; border-radius: calc(var(--poodle-radius-control) * 0.6); image-rendering: pixelated; }
        .image-picker__asset span { font-size: 0.75rem; }
        .image-picker__fields { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 0.5rem 0.75rem; align-items: center; }
        .image-picker__fields label { color: var(--poodle-color-text-secondary); font-size: 0.75rem; }
        .image-picker__fields label span { color: var(--poodle-color-text-tertiary); }
        .image-picker__actions { display: flex; justify-content: flex-end; gap: 0.5rem; }
        @media (max-width: 42rem) { .image-picker { grid-template-columns: 1fr; } .image-picker__fields { grid-template-columns: 1fr; gap: 0.25rem; } .image-picker__fields .poodle-text-input { margin-bottom: 0.5rem; } }
        .rich-text-editor-frame { height: 20rem; margin-top: 0.75rem; }
        .rich-text-editor-frame--constrained { max-width: 22rem; }
        .image-policy-feedback { margin-top: 0.75rem; }
        .image-policy-metric { margin: 0; font-size: 0.8125rem; line-height: 1.4; color: var(--poodle-color-text-secondary); }
        .rich-text-editor-readout { margin: 0.75rem 0 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.75rem; white-space: pre-wrap; max-height: 8rem; overflow: auto; }
      `}</style>
    </SpecimenLayout>
  );
}

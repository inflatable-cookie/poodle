import { useState } from "react";
import type { ControlDensity } from "@inflatable-cookie/poodle-react";
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
  RICH_TEXT_IMAGE_REQUEST_DELAY_MS,
  RICH_TEXT_PICKED_IMAGE_ALT,
  RICH_TEXT_PICKED_IMAGE_SRC,
  RICH_TEXT_SPARSE_HEADING_TOOLBAR,
  RICH_TEXT_STANDARD_DOCUMENT,
  countRichTextImages,
} from "../../../../../svelte/preview/src/specimens/web-editor-documents";

/**
 * Host-owned asset choice, standing in for a consumer media picker. The
 * fixture is a self-contained raster data URL: no network, no DNS, no mutable
 * remote content.
 */
async function requestImage(): Promise<RichTextImageInput | null> {
  await new Promise((resolve) => setTimeout(resolve, RICH_TEXT_IMAGE_REQUEST_DELAY_MS));
  return { src: RICH_TEXT_PICKED_IMAGE_SRC, alt: RICH_TEXT_PICKED_IMAGE_ALT };
}

/** Consumers choose commands, not their icons or grouping. */
const SUBSET_TOOLBAR: readonly RichTextCommand[] = ["bold", "italic", "link"];

export function RichTextEditorSpecimen() {
  const [document, setDocument] = useState<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  const [imagesOn, setImagesOn] = useState(false);
  const [imageDocument, setImageDocument] =
    useState<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);
  const [imageRequests, setImageRequests] = useState(0);
  const [imageChanges, setImageChanges] = useState(0);
  const imageCount = countRichTextImages(imageDocument);

  const onImageDocumentChange = (next: ProseMirrorDocumentJSON): void => {
    setImageChanges((count) => count + 1);
    setImageDocument(next);
  };

  const hostRequestImage = async (): Promise<RichTextImageInput | null> => {
    setImageRequests((count) => count + 1);
    return requestImage();
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
        description="Images are an explicit project choice. Embeds are not a v1 feature. The seeded document loads offline; Insert image asks the host for an asset and inserts exactly one image at the retained selection. The host document below is retained while images are off."
      >
        <button
          type="button"
          className="images-toggle"
          data-part="images-toggle"
          aria-pressed={imagesOn}
          onClick={() => setImagesOn(!imagesOn)}
        >
          {imagesOn ? "Images on" : "Images off"}
        </button>
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
        .rich-text-editor-frame { height: 20rem; margin-top: 0.75rem; }
        .rich-text-editor-frame--constrained { max-width: 22rem; }
        .image-policy-feedback { margin-top: 0.75rem; }
        .image-policy-metric { margin: 0; font-size: 0.8125rem; line-height: 1.4; color: var(--poodle-color-text-secondary); }
        .rich-text-editor-readout { margin: 0.75rem 0 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.75rem; white-space: pre-wrap; max-height: 8rem; overflow: auto; }
      `}</style>
    </SpecimenLayout>
  );
}

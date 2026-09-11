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
  RICH_TEXT_IMAGE_ALT,
  RICH_TEXT_IMAGE_DOCUMENT,
  RICH_TEXT_IMAGE_FEATURES,
  RICH_TEXT_IMAGE_SRC,
  RICH_TEXT_STANDARD_DOCUMENT,
} from "../../../../../svelte/preview/src/specimens/web-editor-documents";

async function requestImage(): Promise<RichTextImageInput | null> {
  return { src: RICH_TEXT_IMAGE_SRC, alt: RICH_TEXT_IMAGE_ALT };
}

/** Consumers choose commands, not their icons or grouping. */
const SUBSET_TOOLBAR: readonly RichTextCommand[] = ["bold", "italic", "link"];

export function RichTextEditorSpecimen() {
  const [document, setDocument] = useState<ProseMirrorDocumentJSON>(RICH_TEXT_STANDARD_DOCUMENT);
  const [imagesOn, setImagesOn] = useState(false);
  const [imageDocument, setImageDocument] =
    useState<ProseMirrorDocumentJSON>(RICH_TEXT_IMAGE_DOCUMENT);

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
        description="Images are an explicit project choice. Embeds are not a v1 feature."
      >
        <button
          type="button"
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
              requestImage={requestImage}
              ariaLabel="Rich text with images"
              onChange={setImageDocument}
            />
          ) : (
            <RichTextEditor
              value={RICH_TEXT_STANDARD_DOCUMENT}
              features={RICH_TEXT_STANDARD_FEATURES}
              ariaLabel="Rich text without images"
            />
          )}
        </div>
      </SpecimenGroup>
      <style>{`
        .rich-text-editor-frame { height: 20rem; margin-top: 0.75rem; }
        .rich-text-editor-readout { margin: 0.75rem 0 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.75rem; white-space: pre-wrap; max-height: 8rem; overflow: auto; }
      `}</style>
    </SpecimenLayout>
  );
}

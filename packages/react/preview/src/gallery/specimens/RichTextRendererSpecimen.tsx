import {
  RICH_TEXT_STANDARD_FEATURES,
  RichTextRenderer,
} from "@inflatable-cookie/poodle-react/rich-text";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
import {
  RICH_TEXT_HEADING_DOCUMENT,
  RICH_TEXT_IMAGE_DOCUMENT,
  RICH_TEXT_IMAGE_FEATURES,
  RICH_TEXT_STANDARD_DOCUMENT,
} from "../../../../../svelte/preview/src/specimens/web-editor-documents";

export function RichTextRendererSpecimen() {
  return (
    <SpecimenLayout>
      <SpecimenGroup
        label="Formatted document"
        description="Same representative ProseMirror JSON and standard features as the editor. Not editable."
      >
        <div className="rich-text-renderer-frame" data-part="standard-renderer">
          <RichTextRenderer
            value={RICH_TEXT_STANDARD_DOCUMENT}
            features={RICH_TEXT_STANDARD_FEATURES}
            ariaLabel="Formatted document"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Heading levels"
        description="H1–H6 are real document structure, not toolbar labels. The renderer keeps the same heading scale the editor admits."
      >
        <div className="rich-text-renderer-frame" data-part="heading-renderer">
          <RichTextRenderer
            value={RICH_TEXT_HEADING_DOCUMENT}
            features={RICH_TEXT_STANDARD_FEATURES}
            ariaLabel="Heading levels document"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup
        label="Images on"
        description="The optional image feature renders the admitted example. Images stay off unless the project opts in."
      >
        <div className="rich-text-renderer-frame" data-part="image-renderer">
          <RichTextRenderer
            value={RICH_TEXT_IMAGE_DOCUMENT}
            features={RICH_TEXT_IMAGE_FEATURES}
            ariaLabel="Document with an image"
          />
        </div>
      </SpecimenGroup>
      <style>{`
        .rich-text-renderer-frame { min-height: 8rem; }
      `}</style>
    </SpecimenLayout>
  );
}

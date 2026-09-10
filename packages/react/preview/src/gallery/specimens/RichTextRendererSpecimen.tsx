import {
  RICH_TEXT_STANDARD_FEATURES,
  RichTextRenderer,
} from "@inflatable-cookie/poodle-react/rich-text";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";
import {
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

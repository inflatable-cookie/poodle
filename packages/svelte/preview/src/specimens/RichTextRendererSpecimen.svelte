<script lang="ts">
  import {
    RICH_TEXT_STANDARD_FEATURES,
    RichTextRenderer,
  } from "@inflatable-cookie/poodle-svelte/rich-text";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import {
    RICH_TEXT_IMAGE_DOCUMENT,
    RICH_TEXT_IMAGE_FEATURES,
    RICH_TEXT_STANDARD_DOCUMENT,
  } from "./web-editor-documents";
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Formatted document"
    description="Same representative ProseMirror JSON and standard features as the editor. Not editable."
  >
    <div class="renderer-frame" data-part="standard-renderer">
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
    <div class="renderer-frame" data-part="image-renderer">
      <RichTextRenderer
        value={RICH_TEXT_IMAGE_DOCUMENT}
        features={RICH_TEXT_IMAGE_FEATURES}
        ariaLabel="Document with an image"
      />
    </div>
  </SpecimenGroup>
</SpecimenLayout>

<style>
  .renderer-frame {
    min-height: 8rem;
  }
</style>

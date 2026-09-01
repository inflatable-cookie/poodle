<script lang="ts">
  import { default as MarkdownEditor } from "../../packages/svelte/components/src/MarkdownEditor.svelte";
  import { LONG_MARKDOWN, SHORT_MARKDOWN } from "./long-markdown";
</script>

<section data-framework="svelte">
  <div data-case="constrained-preview" style="height: 16rem; display: flex; flex-direction: column;">
    <MarkdownEditor mode="preview" value={LONG_MARKDOWN} />
  </div>
  <p data-sibling="constrained-preview">svelte sibling after constrained preview</p>

  <div data-case="constrained-split" style="height: 16rem; display: flex; flex-direction: column;">
    <MarkdownEditor mode="split" value={"short source\n\n" + LONG_MARKDOWN} minHeight="8rem" />
  </div>
  <p data-sibling="constrained-split">svelte sibling after constrained split</p>

  <div data-case="natural-short">
    <MarkdownEditor mode="preview" value={SHORT_MARKDOWN} />
  </div>
</section>

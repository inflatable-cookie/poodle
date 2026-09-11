<script lang="ts">
  import { default as CodeEditor } from "../../packages/svelte/components/src/CodeEditor.svelte";
</script>

<section data-framework="svelte">
  <button type="button" data-before>before svelte</button>
  <CodeEditor value="const answer = 42;" ariaLabel="Svelte code editor" />
</section>

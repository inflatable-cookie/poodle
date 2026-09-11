<script lang="ts">
  import { default as CodeEditor } from "../../packages/svelte/components/src/CodeEditor.svelte";
  import type { CodeEditorDiagnostic } from "../../packages/core/src/index.ts";

  const VALUE = "const answer = 42;\nconst unused = 0;\n";
  const DIAGNOSTICS: CodeEditorDiagnostic[] = [
    { id: "d1", severity: "warning", message: "unused binding", range: { from: 25, to: 31 } },
  ];

  let lineNumbers = $state(true);
</script>

<section data-framework="svelte">
  <button type="button" data-before>before svelte</button>
  <div class="controls">
    <button
      type="button"
      data-part="line-numbers-toggle"
      aria-pressed={lineNumbers}
      onclick={() => (lineNumbers = !lineNumbers)}
    >
      {lineNumbers ? "Line numbers on" : "Line numbers off"}
    </button>
  </div>
  <div class="editor-frame" data-part="config-editor">
    <CodeEditor value={VALUE} {lineNumbers} diagnostics={DIAGNOSTICS} ariaLabel="Svelte configured editor" />
  </div>
</section>

<script lang="ts">
  import { CodeEditor } from "@inflatable-cookie/poodle-svelte/editor";
  import type { CodeEditorLanguageId } from "@inflatable-cookie/poodle-svelte/editor";
  import { createCodeEditorLanguageRegistry } from "@inflatable-cookie/poodle-svelte/editor/codemirror";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import {
    CODE_DIAGNOSTIC_SOURCE,
    CODE_DIAGNOSTICS,
    CODE_JSON_SOURCE,
    CODE_TYPESCRIPT_SOURCE,
  } from "./web-editor-documents";

  // Consumer-owned language registry (g18.012): this preview app installs
  // exactly the grammar packages it names here. Poodle carries no grammar
  // dependencies; ids are plain serializable strings, and "plain-text" needs
  // no registry entry at all.
  const languageRegistry = createCodeEditorLanguageRegistry({
    typescript: async () =>
      (await import("@codemirror/lang-javascript")).javascript({ typescript: true }),
    json: async () => (await import("@codemirror/lang-json")).json(),
  });

  let value = $state(CODE_TYPESCRIPT_SOURCE);
  let language = $state<CodeEditorLanguageId>("typescript");
  let lineNumbers = $state(true);
  const axisSource = "export const ready = true;\n";
</script>

<SpecimenLayout>
  {#snippet children()}
    <SpecimenGroup
      label="TypeScript editing"
      description="Host-controlled value. Typing updates the readout below."
    >
      <div class="editor-frame" data-part="live-editor">
        <CodeEditor
          {value}
          language="typescript"
          {languageRegistry}
          ariaLabel="TypeScript editor"
          onChange={(change) => (value = change.value)}
        />
      </div>
      <pre class="readout" data-part="host-value">{value}</pre>
    </SpecimenGroup>

    <SpecimenGroup label="Diagnostics" description="Host-authored messages, not a language service.">
      <div class="editor-frame" data-part="diagnostics-editor">
        <CodeEditor
          value={CODE_DIAGNOSTIC_SOURCE}
          language="typescript"
          {languageRegistry}
          diagnostics={CODE_DIAGNOSTICS}
          ariaLabel="Diagnostics editor"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Read-only">
      <div class="editor-frame" data-part="read-only-editor">
        <CodeEditor
          value={CODE_TYPESCRIPT_SOURCE}
          language="typescript"
          {languageRegistry}
          readOnly
          ariaLabel="Read-only TypeScript"
        />
      </div>
    </SpecimenGroup>

    <SpecimenGroup label="Configuration">
      <div class="controls">
        <button
          type="button"
          data-part="language-plain-text"
          aria-pressed={language === "plain-text"}
          onclick={() => (language = "plain-text")}
        >
          Plain text
        </button>
        <button
          type="button"
          data-part="language-typescript"
          aria-pressed={language === "typescript"}
          onclick={() => (language = "typescript")}
        >
          TypeScript
        </button>
        <button
          type="button"
          data-part="language-json"
          aria-pressed={language === "json"}
          onclick={() => (language = "json")}
        >
          JSON
        </button>
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
        <CodeEditor
          value={language === "json" ? CODE_JSON_SOURCE : CODE_TYPESCRIPT_SOURCE}
          {language}
          {languageRegistry}
          {lineNumbers}
          ariaLabel="Configured editor"
        />
      </div>
    </SpecimenGroup>
  {/snippet}

  {#snippet densities(density)}
    <SpecimenGroup label={density}>
      <div class="editor-frame">
        <CodeEditor
          value={axisSource}
          language="typescript"
          {languageRegistry}
          {density}
          ariaLabel="Density sample"
        />
      </div>
    </SpecimenGroup>
  {/snippet}
</SpecimenLayout>

<style>
  .editor-frame {
    height: 16rem;
  }
  .controls {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
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

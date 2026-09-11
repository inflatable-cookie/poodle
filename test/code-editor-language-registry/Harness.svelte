<script lang="ts">
  import { default as CodeEditor } from "../../packages/svelte/components/src/CodeEditor.svelte";
  import { createCodeEditorLanguageRegistry } from "../../packages/svelte/components/src/editor-codemirror";

  const VALUE = "const answer = 42;\n";

  // g18.012: the fixture consumer owns its language set. Each loader counts its
  // invocations on window so the probe can prove laziness, memoization, and
  // that only the active language is loaded.
  const counters = { typescript: 0, json: 0, broken: 0 };
  const exposed = (globalThis as Record<string, unknown>);
  exposed.__registryCounters = exposed.__registryCounters ?? {};
  (exposed.__registryCounters as Record<string, unknown>).svelte = counters;

  const languageRegistry = createCodeEditorLanguageRegistry({
    typescript: async () => {
      counters.typescript += 1;
      const { javascript } = await import("@codemirror/lang-javascript");
      return javascript({ typescript: true });
    },
    json: async () => {
      counters.json += 1;
      const { json } = await import("@codemirror/lang-json");
      return json();
    },
    broken: async () => {
      counters.broken += 1;
      throw new Error("grammar exploded");
    },
  });

  let language = $state("typescript");
  // Controlled value tracking: the host owns the exact text, so edits must
  // flow back through onChange or every prop sync would revert the document.
  let value = $state(VALUE);
  let refusalAttempt = $state(false);
  let refusalMessage = $state("");
</script>

<section data-framework="svelte">
  <button type="button" data-before>before svelte</button>
  <div class="controls">
    <button type="button" data-part="language-typescript" onclick={() => (language = "typescript")}>
      TypeScript
    </button>
    <button type="button" data-part="language-json" onclick={() => (language = "json")}>
      JSON
    </button>
    <button type="button" data-part="language-plain-text" onclick={() => (language = "plain-text")}>
      Plain text
    </button>
    <button type="button" data-part="language-broken" onclick={() => (language = "broken")}>
      Broken
    </button>
    <button
      type="button"
      data-part="mount-invalid"
      onclick={() => {
        refusalMessage = "";
        refusalAttempt = true;
      }}
    >
      Mount invalid
    </button>
  </div>
  <div class="editor-frame" data-part="main-editor">
    <CodeEditor {value} {language} {languageRegistry} ariaLabel="Svelte registry editor" onChange={(change) => (value = change.value)} />
  </div>
  <svelte:boundary
    onerror={(error: unknown) => {
      refusalMessage = error instanceof Error ? error.message : String(error);
    }}
  >
    {#if refusalAttempt}
      <div class="editor-frame" data-part="refusal-editor">
        <CodeEditor
          value={VALUE}
          language="cobol"
          {languageRegistry}
          ariaLabel="Svelte invalid language"
        />
      </div>
    {/if}
  </svelte:boundary>
  <p data-part="mount-refusal">{refusalMessage}</p>
</section>

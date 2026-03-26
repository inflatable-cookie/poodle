<script lang="ts">
  import { EmbedInput, resolveEmbedParseState } from "@poodle/svelte-composites";
  import { Eyebrow, Field } from "@poodle/svelte-primitives";
  import type { ParsedEmbed } from "@poodle/svelte-composites";

  let parsed: ParsedEmbed | null = null;
  let value = "";

  const detectionSamples = [
    {
      label: "YouTube short link",
      input: "https://youtu.be/dQw4w9WgXcQ",
      providers: [],
    },
    {
      label: "Vimeo link",
      input: "https://vimeo.com/123456",
      providers: [],
    },
    {
      label: "Iframe embed",
      input: '<iframe src="https://example.com/embed/1" width="640" height="480"></iframe>',
      providers: [],
    },
    {
      label: "Restricted generic URL",
      input: "https://example.com/file.zip",
      providers: ["youtube", "vimeo"],
    },
  ].map((sample) => ({
    ...sample,
    result: resolveEmbedParseState(sample.input, sample.providers),
  }));
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Supported providers</Eyebrow>
    <table class="providers">
      <thead>
        <tr><th>Provider</th><th>Detected patterns</th></tr>
      </thead>
      <tbody>
        <tr><td><code>youtube</code></td><td><code>youtube.com/watch?v=</code>, <code>youtube.com/embed/</code>, <code>youtu.be/</code></td></tr>
        <tr><td><code>vimeo</code></td><td><code>vimeo.com/{'{'}id{'}'}</code></td></tr>
        <tr><td><code>generic</code></td><td>Any valid URL, or <code>&lt;iframe&gt;</code> embed code</td></tr>
      </tbody>
    </table>
  </div>

  <div class="specimen__group">
    <Eyebrow>Detection matrix</Eyebrow>
    <table class="providers">
      <thead>
        <tr><th>Input</th><th>Resolved state</th></tr>
      </thead>
      <tbody>
        {#each detectionSamples as sample}
          <tr>
            <td>
              <strong>{sample.label}</strong>
              <div class="providers__detail"><code>{sample.input}</code></div>
            </td>
            <td>
              {#if sample.result.error}
                <span class="providers__error">{sample.result.error}</span>
              {:else if sample.result.parsed}
                <code>{JSON.stringify(sample.result.parsed)}</code>
              {:else}
                <span>empty</span>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="specimen__group">
    <Eyebrow>URL or embed code input</Eyebrow>
    <EmbedInput
      bind:value
      bind:parsed
      placeholder="Paste a YouTube URL, Vimeo link, or embed code..."
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With Field wrapper</Eyebrow>
    <Field label="Video embed">
      <EmbedInput
        placeholder="https://youtube.com/watch?v=..."
      />
    </Field>
  </div>

  <div class="specimen__group">
    <Eyebrow>Restricted providers</Eyebrow>
    <EmbedInput
      providers={["youtube", "vimeo"]}
      placeholder="Only YouTube and Vimeo allowed..."
    />
  </div>

  {#if parsed}
    <div class="specimen__group">
      <Eyebrow>Parsed result</Eyebrow>
      <pre class="parsed">{JSON.stringify(parsed, null, 2)}</pre>
    </div>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .providers {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--poodle-typography-label-size, 0.75rem);
  }

  .providers th,
  .providers td {
    padding: 0.375rem 0.625rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    text-align: left;
  }

  .providers th {
    color: var(--poodle-color-text-secondary);
    font-weight: 600;
  }

  .providers code {
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
    font-family: var(--poodle-typography-mono-family, monospace);
    font-size: 0.6875rem;
  }

  .providers__detail {
    margin-top: 0.25rem;
  }

  .providers__error {
    color: var(--poodle-color-text-danger, #ef4444);
  }

  .parsed {
    margin: 0;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--poodle-color-background-panel, #1a1a1a);
    font-family: var(--poodle-typography-mono-family, monospace);
    font-size: 0.75rem;
    white-space: pre-wrap;
  }
</style>

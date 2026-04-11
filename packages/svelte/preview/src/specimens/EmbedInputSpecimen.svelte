<script lang="ts">
  import { EmbedInput, resolveEmbedParseState } from "@poodle/svelte";
  import { Field } from "@poodle/svelte";
  import type { ParsedEmbed } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

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
  <SpecimenGroup label="Supported providers">
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
  </SpecimenGroup>

  <SpecimenGroup label="Detection matrix">
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
  </SpecimenGroup>

  <SpecimenGroup label="URL or embed code input">
    <EmbedInput
      bind:value
      bind:parsed
      placeholder="Paste a YouTube URL, Vimeo link, or embed code..."
    />
  </SpecimenGroup>

  <SpecimenGroup label="With Field wrapper">
    <Field label="Video embed" id="embed-input-video">
      <EmbedInput
        placeholder="https://youtube.com/watch?v=..."
      />
    </Field>
  </SpecimenGroup>

  <SpecimenGroup label="Restricted providers">
    <EmbedInput
      providers={["youtube", "vimeo"]}
      placeholder="Only YouTube and Vimeo allowed..."
    />
  </SpecimenGroup>

  {#if parsed}
    <SpecimenGroup label="Parsed result">
      <pre class="parsed">{JSON.stringify(parsed, null, 2)}</pre>
    </SpecimenGroup>
  {/if}
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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

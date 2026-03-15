<script lang="ts">
  import { EmbedInput } from "@pug/svelte-composites";
  import { Eyebrow } from "@pug/svelte-primitives";
  import type { ParsedEmbed } from "@pug/svelte-composites";

  let parsed: ParsedEmbed | null = null;
  let value = "";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>URL or embed code input</Eyebrow>
    <EmbedInput
      bind:value
      bind:parsed
      placeholder="Paste a YouTube URL, Vimeo link, or embed code..."
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>With label</Eyebrow>
    <EmbedInput
      label="Video embed"
      placeholder="https://youtube.com/watch?v=..."
    />
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

  .parsed {
    margin: 0;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    background: var(--pug-color-background-panel, #1a1a1a);
    font-family: var(--pug-typography-mono-family, monospace);
    font-size: 0.75rem;
    white-space: pre-wrap;
  }
</style>

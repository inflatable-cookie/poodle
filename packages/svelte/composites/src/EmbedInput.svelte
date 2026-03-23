<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { TextArea, Pill } from "@flint/svelte-primitives";

  import type { ParsedEmbed } from "./types";

  export let id = "embed-input";
  export let value = "";
  export let parsed: ParsedEmbed | null = null;
  export let placeholder = "Paste a URL or embed code...";
  export let parseDebounce = 300;
  export let providers: string[] = [];
  export let disabled = false;
  export let error: string | null = null;

  const dispatch = createEventDispatcher<{
    parse: { parsed: ParsedEmbed | null; error: string | null };
    change: { value: string };
  }>();

  let parseTimer: ReturnType<typeof setTimeout> | null = null;

  function detectProvider(input: string): ParsedEmbed | null {
    const trimmed = input.trim();

    if (!trimmed) {
      return null;
    }

    const youtubeMatch = trimmed.match(
      /(?:youtube\.com\/(?:watch\?v=|embed\/)|youtu\.be\/)([\w-]+)/,
    );

    if (youtubeMatch) {
      return {
        provider: "youtube",
        id: youtubeMatch[1],
        originalUrl: trimmed,
      };
    }

    const vimeoMatch = trimmed.match(
      /(?:vimeo\.com\/)(\d+)/,
    );

    if (vimeoMatch) {
      return {
        provider: "vimeo",
        id: vimeoMatch[1],
        originalUrl: trimmed,
      };
    }

    try {
      const url = new URL(trimmed);

      return {
        provider: "generic",
        id: url.href,
        originalUrl: url.href,
      };
    } catch {
      /* not a URL */
    }

    if (trimmed.startsWith("<") && trimmed.includes("iframe")) {
      const srcMatch = trimmed.match(/src=["']([^"']+)["']/);

      return {
        provider: "generic",
        id: srcMatch?.[1] ?? trimmed,
        originalUrl: srcMatch?.[1] ?? undefined,
        originalEmbed: trimmed,
      };
    }

    return null;
  }

  function doParse(): void {
    const result = detectProvider(value);

    if (result && providers.length > 0 && !providers.includes(result.provider)) {
      error = `Provider "${result.provider}" is not allowed`;
      parsed = null;
    } else {
      error = null;
      parsed = result;
    }

    dispatch("parse", { parsed, error });
  }

  function handleValueChange(event: CustomEvent<{ value: string }>): void {
    value = event.detail.value;
    dispatch("change", { value });

    if (parseTimer) {
      clearTimeout(parseTimer);
    }

    parseTimer = setTimeout(doParse, parseDebounce);
  }
</script>

<div class="embed-input">
  <TextArea
    {id}
    {value}
    {placeholder}
    isDisabled={disabled}
    rows={3}
    on:valueChange={handleValueChange}
  />

  <div class="embed-input__status">
    {#if error}
      <span class="embed-input__error">{error}</span>
    {:else if parsed}
      <Pill tone="success" size="xs">{parsed.provider}</Pill>
      <span class="embed-input__success">Embed detected</span>
    {/if}
  </div>
</div>

<style>
  .embed-input {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .embed-input__status {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    min-height: 1.25rem;
    font-size: 0.75rem;
  }

  .embed-input__error {
    color: var(--flint-color-text-danger, #ef4444);
  }

  .embed-input__success {
    color: var(--flint-color-text-success, #22c55e);
  }
</style>

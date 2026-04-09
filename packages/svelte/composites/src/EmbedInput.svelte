<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { TextInput, Pill } from "@poodle/svelte-primitives";

  import { resolveEmbedParseState } from "./embed-input";
  import type { EmbedParseState } from "./embed-input";
  import type { ParsedEmbed } from "./types";

  export let id = "embed-input";
  export let value = "";
  export let parsed: ParsedEmbed | null = null;
  export let placeholder = "Paste a URL or embed code...";
  export let parseDebounce = 300;
  export let providers: string[] = [];
  export let disabled = false;
  export let error: string | null = null;
  export let resolveParseState:
    | ((value: string, providers: string[]) => EmbedParseState)
    | undefined = undefined;

  const dispatch = createEventDispatcher<{
    parse: { parsed: ParsedEmbed | null; error: string | null };
    change: { value: string };
  }>();

  let parseTimer: ReturnType<typeof setTimeout> | null = null;

  function doParse(): void {
    const nextState = (resolveParseState ?? resolveEmbedParseState)(value, providers);
    parsed = nextState.parsed;
    error = nextState.error;

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
  <TextInput
    {id}
    {value}
    {placeholder}
    disabled={disabled}
    rows={3}
    on:valueChange={handleValueChange}
  />

  <div class="embed-input__status">
    {#if error}
      <span class="embed-input__error">{error}</span>
    {:else if parsed}
      <Pill tone="success" sizeRole="chrome">{parsed.provider}</Pill>
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
    color: var(--poodle-color-text-danger, #ef4444);
  }

  .embed-input__success {
    color: var(--poodle-color-text-success, #22c55e);
  }
</style>

<script lang="ts">
  import { onDestroy } from "svelte";

  import TextInput from "./TextInput.svelte";
  import Pill from "./Pill.svelte";

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
  export let onParse: ((parsed: ParsedEmbed | null, error: string | null) => void) | null = null;
  export let onValueChange: ((value: string) => void) | null = null;
  export let resolveParseState:
    | ((value: string, providers: string[]) => EmbedParseState)
    | undefined = undefined;

  let parseTimer: ReturnType<typeof setTimeout> | null = null;
  let uncontrolledValue = value;

  $: hasControlledValue = $$props.value !== undefined;
  $: currentValue = hasControlledValue ? value : uncontrolledValue;

  function doParse(): void {
    const nextState = (resolveParseState ?? resolveEmbedParseState)(currentValue, providers);
    parsed = nextState.parsed;
    error = nextState.error;

    onParse?.(parsed, error);
  }

  function handleValueChange(nextValue: string): void {
    if (!hasControlledValue) {
      uncontrolledValue = nextValue;
    }

    onValueChange?.(nextValue);

    if (parseTimer) {
      clearTimeout(parseTimer);
    }

    parseTimer = setTimeout(doParse, parseDebounce);
  }

  onDestroy(() => {
    if (parseTimer) {
      clearTimeout(parseTimer);
    }
  });
</script>

<div class="poodle-embed-input">
  <TextInput
    {id}
    value={currentValue}
    {placeholder}
    disabled={disabled}
    rows={3}
    onValueChange={handleValueChange}
  />

  <div class="poodle-embed-input__status">
    {#if error}
      <span class="poodle-embed-input__error">{error}</span>
    {:else if parsed}
      <Pill tone="success" sizeRole="chrome">{parsed.provider}</Pill>
      <span class="poodle-embed-input__success">Embed detected</span>
    {/if}
  </div>
</div>

<style>
  .poodle-embed-input {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .poodle-embed-input__status {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    min-height: 1.25rem;
    font-size: 0.75rem;
  }

  .poodle-embed-input__error {
    color: var(--poodle-color-text-danger, #ef4444);
  }

  .poodle-embed-input__success {
    color: var(--poodle-color-text-success, #22c55e);
  }
</style>

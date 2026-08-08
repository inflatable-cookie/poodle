<script lang="ts">
  import "@inflatable-cookie/poodle-styles/embed-input.css";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as Pill } from "./Pill.svelte";

  import { resolveEmbedParseState } from "./embed-input";
  import type { EmbedParseState } from "./embed-input";
  import type { ControlDensity, ControlSize, ParsedEmbed, SemanticControlSizeRole } from "./types";

  let {
    id = "embed-input",
    value = $bindable(""),
    parsed = $bindable<ParsedEmbed | null>(null),
    placeholder = "Paste a URL or embed code...",
    parseDebounce = 300,
    providers = [],
    disabled = false,
    error = $bindable<string | null>(null),
    size = null,
    sizeRole = "control",
    density = null,
    onParse = null,
    onValueChange = null,
    resolveParseState = undefined,
  }: {
    id?: string;
    value?: string;
    parsed?: ParsedEmbed | null;
    placeholder?: string;
    parseDebounce?: number;
    providers?: string[];
    disabled?: boolean;
    error?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onParse?: ((parsed: ParsedEmbed | null, error: string | null) => void) | null;
    onValueChange?: ((value: string) => void) | null;
    resolveParseState?: ((value: string, providers: string[]) => EmbedParseState) | undefined;
  } = $props();

  let parseTimer: ReturnType<typeof setTimeout> | null = null;

  function doParse(): void {
    const nextState = (resolveParseState ?? resolveEmbedParseState)(value, providers);
    parsed = nextState.parsed;
    error = nextState.error;

    onParse?.(parsed, error);
  }

  function handleValueChange(nextValue: string): void {
    value = nextValue;

    onValueChange?.(nextValue);

    if (parseTimer) {
      clearTimeout(parseTimer);
    }

    parseTimer = setTimeout(doParse, parseDebounce);
  }

  $effect(() => {
    return () => {
      if (parseTimer) {
        clearTimeout(parseTimer);
      }
    };
  });
</script>

<div class="poodle-embed-input">
  <TextInput
    {id}
    type="multiline"
    {value}
    {placeholder}
    disabled={disabled}
    {size}
    {sizeRole}
    {density}
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


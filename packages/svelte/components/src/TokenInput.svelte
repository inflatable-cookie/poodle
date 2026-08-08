<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/token-input.css";
  import { mergeTokens, splitTokenInput, tokenBackspaceRemoves } from "@inflatable-cookie/poodle-core";
  import type { HTMLInputAttributes } from "svelte/elements";

  import { default as Icon } from "./Icon.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    id?: string;
    values?: string[];
    name?: string | undefined;
    placeholder?: string | null;
    disabled?: boolean;
    readOnly?: boolean;
    required?: boolean;
    spellcheck?: HTMLInputAttributes["spellcheck"];
    autocapitalize?: HTMLInputAttributes["autocapitalize"];
    autocomplete?: HTMLInputAttributes["autocomplete"];
    ariaLabel?: string | null;
    describedBy?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    separators?: string[];
    dedupe?: boolean;
    commitOnBlur?: boolean;
    maxLength?: number | null;
    resolveToken?: ((value: string, values: string[]) => string | null | undefined) | undefined;
    onValuesChange?: ((values: string[]) => void) | undefined;
    onTokenReject?: ((value: string) => void) | undefined;
  }

  let {
    id = "",
    values = $bindable<string[]>([]),
    name = undefined,
    placeholder = null,
    disabled = false,
    readOnly = false,
    required = false,
    spellcheck = false,
    autocapitalize = "none",
    autocomplete = "off",
    ariaLabel = null,
    describedBy = null,
    size = null,
    sizeRole = "control",
    density = null,
    separators = [","],
    dedupe = true,
    commitOnBlur = true,
    maxLength = null,
    resolveToken = undefined,
    onValuesChange = undefined,
    onTokenReject = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let inputValue = $state("");
  let inputElement = $state<HTMLInputElement | null>(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const canEdit = $derived(!disabled && !readOnly);
  const separatorChars = $derived(
    Array.from(new Set(separators.filter((separator) => separator.length > 0))).join("")
  );
  const splitPattern = $derived(
    separatorChars.length > 0
      ? new RegExp(`[${escapeRegExp(separatorChars)}]+`)
      : null
  );

  function escapeRegExp(value: string): string {
    return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  function normalizeToken(value: string, currentValues: string[]): string | null {
    const trimmed = value.trim();
    if (!trimmed) return null;

    const resolved = resolveToken ? resolveToken(trimmed, currentValues) : trimmed;
    if (typeof resolved !== "string") {
      onTokenReject?.(trimmed);
      return null;
    }

    const normalized = resolved.trim();
    if (!normalized) {
      onTokenReject?.(trimmed);
      return null;
    }

    return normalized;
  }

  function applyValues(nextValues: string[]): void {
    values = nextValues;
    onValuesChange?.(nextValues);
  }

  function addTokens(rawTokens: string[]): void {
    const current = values ?? [];
    const nextTokens = rawTokens
      .map((token) => normalizeToken(token, current))
      .filter((token): token is string => Boolean(token));

    if (nextTokens.length === 0) {
      return;
    }

    applyValues(mergeTokens(current, nextTokens, dedupe));
  }

  function commitInput(): void {
    const trimmed = normalizeToken(inputValue, values ?? []);
    if (!trimmed) {
      inputValue = "";
      return;
    }

    addTokens([trimmed]);
    inputValue = "";
  }

  function removeToken(index: number): void {
    if (!canEdit) return;
    applyValues(values.filter((_, currentIndex) => currentIndex !== index));
  }

  function handleInput(nextValue: string): void {
    const split = splitTokenInput(nextValue, splitPattern, separatorChars);

    if (!split) {
      inputValue = nextValue;
      return;
    }

    addTokens(split.committed);
    inputValue = split.remainder;
  }

  function handleKeyDown(event: KeyboardEvent): void {
    if (!canEdit) return;

    if (event.key === "Enter" || event.key === "Tab") {
      if (inputValue.trim().length > 0) {
        event.preventDefault();
        commitInput();
      }
      return;
    }

    if (event.key === "Backspace" && tokenBackspaceRemoves(inputValue, values.length)) {
      event.preventDefault();
      applyValues(values.slice(0, -1));
    }
  }

  function handleBlur(): void {
    if (commitOnBlur) {
      commitInput();
    }
  }

  function focusInput(): void {
    if (!canEdit) return;
    inputElement?.focus();
  }

  function handlePointerDown(event: PointerEvent): void {
    if (!canEdit || event.target === inputElement) return;
    if (event.target instanceof Element && event.target.closest("button")) return;
    event.preventDefault();
    focusInput();
  }

  function focusInputOnPointerDown(node: HTMLDivElement): { destroy: () => void } {
    node.addEventListener("pointerdown", handlePointerDown);
    return {
      destroy: () => node.removeEventListener("pointerdown", handlePointerDown),
    };
  }
</script>

<div
  class="poodle-token-input"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-disabled={disabled || undefined}
  data-read-only={readOnly || undefined}
  use:focusInputOnPointerDown
>
  {#if name}
    {#each values as token, index (`${token}:${index}`)}
      <input type="hidden" {name} value={token} />
    {/each}
  {/if}

  <div class="poodle-token-input__tokens">
    {#each values as token, index (`${token}:${index}`)}
      <span class="poodle-token-input__token">
        <Pill tone="neutral" appearance="subtle" size={resolvedSize} adaptiveWidth>
          <span class="poodle-token-input__token-label">{token}</span>
          {#if canEdit}
            <button
              type="button"
              class="poodle-token-input__remove"
              aria-label={`Remove ${token}`}
              onclick={() => removeToken(index)}
            >
              <Icon name="x" size="xs" />
            </button>
          {/if}
        </Pill>
      </span>
    {/each}

    <input
      {id}
      bind:this={inputElement}
      class="poodle-token-input__control"
      type="text"
      bind:value={inputValue}
      {disabled}
      readonly={readOnly}
      {required}
      {spellcheck}
      {autocapitalize}
      {autocomplete}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      placeholder={values.length === 0 ? (placeholder ?? undefined) : undefined}
      maxlength={maxLength ?? undefined}
      oninput={(event) => handleInput((event.currentTarget as HTMLInputElement).value)}
      onkeydown={handleKeyDown}
      onblur={handleBlur}
    />
  </div>
</div>


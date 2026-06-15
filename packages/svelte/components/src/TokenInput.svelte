<script lang="ts">
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
    onValuesChange?: ((values: string[]) => void) | undefined;
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
    onValuesChange = undefined,
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

  function normalizeToken(value: string): string {
    return value.trim();
  }

  function applyValues(nextValues: string[]): void {
    values = nextValues;
    onValuesChange?.(nextValues);
  }

  function addTokens(rawTokens: string[]): void {
    const nextTokens = rawTokens
      .map(normalizeToken)
      .filter((token) => token.length > 0);

    if (nextTokens.length === 0) {
      return;
    }

    const current = values ?? [];
    const merged = dedupe
      ? [...current, ...nextTokens.filter((token) => !current.includes(token))]
      : [...current, ...nextTokens];

    applyValues(merged);
  }

  function commitInput(): void {
    const trimmed = normalizeToken(inputValue);
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
    if (!splitPattern) {
      inputValue = nextValue;
      return;
    }

    const rawParts = nextValue.split(splitPattern);
    const endsWithSeparator = separatorChars.length > 0
      && separatorChars.split("").some((separator) => nextValue.endsWith(separator));

    if (rawParts.length <= 1 && !endsWithSeparator) {
      inputValue = nextValue;
      return;
    }

    const committed = endsWithSeparator ? rawParts : rawParts.slice(0, -1);
    addTokens(committed);
    inputValue = endsWithSeparator ? "" : rawParts.at(-1) ?? "";
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

    if (event.key === "Backspace" && inputValue.length === 0 && values.length > 0) {
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
      {placeholder}
      maxlength={maxLength ?? undefined}
      oninput={(event) => handleInput((event.currentTarget as HTMLInputElement).value)}
      onkeydown={handleKeyDown}
      onblur={handleBlur}
    />
  </div>
</div>

<style>
  .poodle-token-input {
    --poodle-token-input-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    --poodle-token-input-fill: var(
      --poodle-treatment-interactive-subtle-fill,
      color-mix(in srgb, var(--poodle-color-background-surface) 96%, transparent)
    );
    --poodle-token-input-fill-focus: var(
      --poodle-treatment-interactive-subtle-fill-hover,
      var(--poodle-token-input-fill)
    );
    --poodle-token-input-border: var(
      --poodle-treatment-interactive-subtle-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 76%, transparent)
    );
    --poodle-token-input-border-focus: var(
      --poodle-treatment-interactive-subtle-border-focus,
      color-mix(in srgb, var(--poodle-color-accent-base) 58%, var(--poodle-color-border-subtle))
    );
    --poodle-token-input-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    --poodle-token-input-shadow-focus: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)
    );
    --poodle-token-input-padding-inline: var(--poodle-space-control-x);
    --poodle-token-input-padding-block: var(--poodle-space-control-y);
    --poodle-token-input-gap: 0.375rem;
    --poodle-token-input-font-size: var(--poodle-typography-body-size);
    --poodle-token-input-line-height: var(--poodle-typography-body-lineHeight);
    display: flex;
    width: 100%;
    min-width: 0;
    border: 0.0625rem solid var(--poodle-token-input-border);
    border-radius: var(--poodle-token-input-radius);
    background: var(--poodle-token-input-fill);
    box-shadow: var(--poodle-token-input-shadow);
    cursor: text;
    transition:
      border-color 120ms ease,
      background-color 120ms ease,
      box-shadow 120ms ease;
  }

  .poodle-token-input:focus-within {
    border-color: var(--poodle-token-input-border-focus);
    background: var(--poodle-token-input-fill-focus);
    box-shadow: var(--poodle-token-input-shadow-focus);
  }

  .poodle-token-input[data-size="xs"] {
    --poodle-token-input-padding-inline: calc(var(--poodle-space-control-x) - 0.25rem);
    --poodle-token-input-padding-block: calc(var(--poodle-space-control-y) - 0.125rem);
    --poodle-token-input-font-size: 0.75rem;
  }

  .poodle-token-input[data-size="sm"] {
    --poodle-token-input-padding-inline: calc(var(--poodle-space-control-x) - 0.125rem);
    --poodle-token-input-padding-block: calc(var(--poodle-space-control-y) - 0.0625rem);
    --poodle-token-input-font-size: 0.8125rem;
  }

  .poodle-token-input[data-size="lg"] {
    --poodle-token-input-padding-inline: calc(var(--poodle-space-control-x) + 0.125rem);
    --poodle-token-input-padding-block: calc(var(--poodle-space-control-y) + 0.0625rem);
    --poodle-token-input-font-size: 0.9375rem;
  }

  .poodle-token-input[data-size="xl"] {
    --poodle-token-input-padding-inline: calc(var(--poodle-space-control-x) + 0.1875rem);
    --poodle-token-input-padding-block: calc(var(--poodle-space-control-y) + 0.125rem);
    --poodle-token-input-font-size: 1rem;
  }

  .poodle-token-input[data-density="compact"] {
    --poodle-token-input-gap: 0.25rem;
  }

  .poodle-token-input[data-density="comfortable"] {
    --poodle-token-input-gap: 0.5rem;
  }

  .poodle-token-input[data-disabled="true"] {
    opacity: 0.65;
    cursor: not-allowed;
  }

  .poodle-token-input[data-read-only="true"] {
    cursor: default;
  }

  .poodle-token-input__tokens {
    display: flex;
    flex: 1 1 auto;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-token-input-gap);
    min-width: 0;
    padding:
      var(--poodle-token-input-padding-block)
      var(--poodle-token-input-padding-inline);
  }

  .poodle-token-input__token {
    display: inline-flex;
    min-width: 0;
    max-width: 100%;
  }

  .poodle-token-input__token :global(.poodle-pill) {
    min-width: 0;
    max-width: 100%;
    align-items: flex-start;
    white-space: normal;
  }

  .poodle-token-input__token-label {
    min-width: 0;
    overflow-wrap: anywhere;
    word-break: break-word;
  }

  .poodle-token-input__remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    opacity: 0.75;
  }

  .poodle-token-input__remove:hover {
    opacity: 1;
  }

  .poodle-token-input__remove:focus-visible {
    outline: 0.0625rem solid currentColor;
    outline-offset: 0.125rem;
    border-radius: 999px;
  }

  .poodle-token-input__control {
    flex: 1 1 8rem;
    min-width: 8rem;
    border: 0;
    outline: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font: inherit;
    font-size: var(--poodle-token-input-font-size);
    line-height: var(--poodle-token-input-line-height);
    padding: 0;
  }

  .poodle-token-input__control::placeholder {
    color: var(--poodle-color-text-tertiary);
  }

  .poodle-token-input__control:disabled,
  .poodle-token-input[data-read-only="true"] .poodle-token-input__control {
    cursor: not-allowed;
  }
</style>

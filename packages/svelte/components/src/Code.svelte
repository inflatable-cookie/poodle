<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  let {
    source = "",
    language = null,
    showLineNumbers = false,
    highlightLines = [],
    showCopyButton = true,
    maxHeight = null,
    inline = false,
    ariaLabel = null,
    inlineVariant = "default",
    typography = "body",
    sizeRole = "chrome",
    size = null,
    density = null,
  }: {
    source?: string;
    language?: string | null;
    showLineNumbers?: boolean;
    highlightLines?: number[];
    showCopyButton?: boolean;
    maxHeight?: string | null;
    inline?: boolean;
    ariaLabel?: string | null;
    inlineVariant?: "default" | "plain";
    typography?: "body" | "inline";
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
  } = $props();

  const uiPresentation = getUiPresentation();

  let copied = $state(false);
  let copyResetTimer: ReturnType<typeof setTimeout> | null = null;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const lines = $derived(source.split("\n"));
  const highlightSet = $derived(new Set(highlightLines));

  async function copyToClipboard(): Promise<void> {
    try {
      await navigator.clipboard.writeText(source);
      if (copyResetTimer !== null) {
        clearTimeout(copyResetTimer);
      }
      copied = true;
      copyResetTimer = setTimeout(() => {
        copied = false;
        copyResetTimer = null;
      }, 2000);
    } catch {
      // Fallback for browsers without clipboard API
    }
  }

  $effect(() => {
    return () => {
      if (copyResetTimer !== null) {
        clearTimeout(copyResetTimer);
      }
    };
  });
</script>

{#if inline}
  <span
    class="poodle-code poodle-code--inline-wrap"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-inline-variant={inlineVariant}
    data-typography={typography}
  >
    <code
      class="poodle-code poodle-code--inline"
      aria-label={ariaLabel ?? undefined}
      data-language={language}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-inline-variant={inlineVariant}
      data-typography={typography}
    >
      {source}
    </code>
    {#if showCopyButton}
      <button
        type="button"
        class="poodle-code__copy poodle-code__copy--inline"
        aria-label={copied ? "Copied" : "Copy to clipboard"}
        onclick={copyToClipboard}
      >
        {#if copied}
          <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
            <path d="M3 8.5l3 3 7-7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        {:else}
          <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
            <rect x="5" y="5" width="8" height="8" rx="1" stroke="currentColor" stroke-width="1.25" />
            <path d="M3 11V3a1 1 0 011-1h8" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" />
          </svg>
        {/if}
      </button>
    {/if}
  </span>
{:else}
  <div
    class="poodle-code poodle-code--block"
    aria-label={ariaLabel ?? `Code block${language ? ` (${language})` : ""}`}
    data-language={language}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    style={maxHeight ? `max-height: ${maxHeight}` : undefined}
  >
    {#if language || showCopyButton}
      <div class="poodle-code__toolbar">
        {#if language}
          <span class="poodle-code__language">{language}</span>
        {/if}
        <div class="poodle-code__toolbar-actions">
          {#if showCopyButton}
            <button
              type="button"
              class="poodle-code__copy"
              aria-label={copied ? "Copied" : "Copy to clipboard"}
              onclick={copyToClipboard}
            >
              {#if copied}
                <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                  <path d="M3 8.5l3 3 7-7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
              {:else}
                <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                  <rect x="5" y="5" width="8" height="8" rx="1" stroke="currentColor" stroke-width="1.25" />
                  <path d="M3 11V3a1 1 0 011-1h8" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" />
                </svg>
              {/if}
            </button>
          {/if}
        </div>
      </div>
    {/if}

    <div class="poodle-code__scroll">
      <pre class="poodle-code__pre"><code class="poodle-code__source">{#each lines as line, i}<span
          class="poodle-code__line"
          class:poodle-code__line--highlighted={highlightSet.has(i + 1)}
        >{#if showLineNumbers}<span class="poodle-code__line-number" aria-hidden="true">{i + 1}</span>{/if}<span class="poodle-code__line-content">{line}</span></span>{/each}</code></pre>
    </div>
  </div>
{/if}

<style>
  .poodle-code--inline-wrap {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
  }

  .poodle-code--inline {
    display: inline;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated));
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: calc(0.8125em * var(--poodle-typography-code-adjustmentRatio));
    line-height: 1.5;
  }

  .poodle-code--inline[data-inline-variant="plain"] {
    padding: 0;
    border-radius: 0;
    background: transparent;
  }

  .poodle-code--inline[data-typography="inline"] {
    font-size: calc(1em * var(--poodle-typography-code-adjustmentRatio));
    line-height: inherit;
  }

  .poodle-code--block {
    display: flex;
    flex-direction: column;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .poodle-code__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.375rem 0.625rem;
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 60%, var(--poodle-color-background-panel));
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-code__language {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .poodle-code__toolbar-actions {
    display: flex;
    gap: 0.25rem;
  }

  .poodle-code__copy {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: 0;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    transition: color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-code__copy:hover {
    color: var(--poodle-color-text-primary);
  }

  .poodle-code__copy:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
  }

  .poodle-code__copy svg {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-code__copy--inline {
    width: 1.25rem;
    height: 1.25rem;
  }

  .poodle-code__copy--inline svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  .poodle-code__scroll {
    overflow-x: auto;
  }

  .poodle-code__pre {
    margin: 0;
    padding: 0.75rem 1rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 92%, black);
  }

  .poodle-code__source {
    display: block;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    line-height: 1.4;
    color: var(--poodle-color-text-primary);
    tab-size: 2;
    white-space: pre;
  }

  .poodle-code__line {
    display: block;
  }

  .poodle-code__line--highlighted {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    margin: 0 -1rem;
    padding: 0 1rem;
  }

  .poodle-code__line-number {
    display: inline-block;
    width: 2.5rem;
    padding-right: 1rem;
    color: var(--poodle-color-text-secondary);
    text-align: right;
    user-select: none;
    font-variant-numeric: tabular-nums;
  }

  /* Size variants */
  .poodle-code[data-size="xs"] .poodle-code__source {
    font-size: 0.6875rem;
  }

  .poodle-code--inline[data-size="xs"] {
    font-size: calc(0.6875em * var(--poodle-typography-code-adjustmentRatio));
  }

  .poodle-code[data-size="sm"] .poodle-code__source {
    font-size: 0.75rem;
  }

  .poodle-code--inline[data-size="sm"] {
    font-size: calc(0.75em * var(--poodle-typography-code-adjustmentRatio));
  }

  .poodle-code[data-size="lg"] .poodle-code__source {
    font-size: 0.875rem;
  }

  .poodle-code--inline[data-size="lg"] {
    font-size: calc(0.875em * var(--poodle-typography-code-adjustmentRatio));
  }

  .poodle-code[data-size="xl"] .poodle-code__source {
    font-size: 0.9375rem;
  }

  .poodle-code--inline[data-size="xl"] {
    font-size: calc(0.9375em * var(--poodle-typography-code-adjustmentRatio));
  }

  /* Density variants */
  .poodle-code[data-density="compact"] .poodle-code__pre { padding: 0.5rem 0.75rem; }
  .poodle-code[data-density="comfortable"] .poodle-code__pre { padding: 1rem 1.25rem; }
</style>

<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlSize, SemanticControlSizeRole } from "./types";

  export let source = "";
  export let language: string | null = null;
  export let showLineNumbers = false;
  export let highlightLines: number[] = [];
  export let showCopyButton = true;
  export let maxHeight: string | null = null;
  export let inline = false;
  export let ariaLabel: string | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let size: ControlSize | null = null;

  const uiPresentation = getUiPresentation();

  let copied = false;

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: lines = source.split("\n");
  $: highlightSet = new Set(highlightLines);

  async function copyToClipboard(): Promise<void> {
    try {
      await navigator.clipboard.writeText(source);
      copied = true;
      setTimeout(() => { copied = false; }, 2000);
    } catch {
      // Fallback for browsers without clipboard API
    }
  }
</script>

{#if inline}
  <code class="code code--inline" aria-label={ariaLabel ?? undefined} data-language={language} data-size={resolvedSize}>
    {source}
  </code>
{:else}
  <div
    class="code code--block"
    aria-label={ariaLabel ?? `Code block${language ? ` (${language})` : ""}`}
    data-language={language}
    data-size={resolvedSize}
    style={maxHeight ? `max-height: ${maxHeight}` : undefined}
  >
    {#if language || showCopyButton}
      <div class="code__toolbar">
        {#if language}
          <span class="code__language">{language}</span>
        {/if}
        <div class="code__toolbar-actions">
          {#if showCopyButton}
            <button
              type="button"
              class="code__copy"
              aria-label={copied ? "Copied" : "Copy to clipboard"}
              on:click={copyToClipboard}
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

    <div class="code__scroll">
      <pre class="code__pre"><code class="code__source">{#each lines as line, i}<span
          class="code__line"
          class:code__line--highlighted={highlightSet.has(i + 1)}
        >{#if showLineNumbers}<span class="code__line-number" aria-hidden="true">{i + 1}</span>{/if}<span class="code__line-content">{line}</span></span>{/each}</code></pre>
    </div>
  </div>
{/if}

<style>
  .code--inline {
    display: inline;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated));
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125em;
    line-height: 1.5;
  }

  .code--block {
    display: flex;
    flex-direction: column;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, var(--poodle-color-background-elevated));
    overflow: hidden;
  }

  .code__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.375rem 0.625rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent);
  }

  .code__language {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .code__toolbar-actions {
    display: flex;
    gap: 0.25rem;
  }

  .code__copy {
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

  .code__copy:hover {
    color: var(--poodle-color-text-primary);
  }

  .code__copy:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
  }

  .code__copy svg {
    width: 0.875rem;
    height: 0.875rem;
  }

  .code__scroll {
    overflow-x: auto;
  }

  .code__pre {
    margin: 0;
    padding: 0.75rem 1rem;
  }

  .code__source {
    display: block;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    line-height: 1.4;
    color: var(--poodle-color-text-primary);
    tab-size: 2;
    white-space: pre;
  }

  .code__line {
    display: block;
  }

  .code__line--highlighted {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    margin: 0 -1rem;
    padding: 0 1rem;
  }

  .code__line-number {
    display: inline-block;
    width: 2.5rem;
    padding-right: 1rem;
    color: var(--poodle-color-text-secondary);
    text-align: right;
    user-select: none;
    font-variant-numeric: tabular-nums;
  }

  /* Size variants */
  .code[data-size="xs"] .code__source {
    font-size: 0.6875rem;
  }

  .code--inline[data-size="xs"] {
    font-size: 0.6875em;
  }

  .code[data-size="sm"] .code__source {
    font-size: 0.75rem;
  }

  .code--inline[data-size="sm"] {
    font-size: 0.75em;
  }

  .code[data-size="lg"] .code__source {
    font-size: 0.875rem;
  }

  .code--inline[data-size="lg"] {
    font-size: 0.875em;
  }

  .code[data-size="xl"] .code__source {
    font-size: 0.9375rem;
  }

  .code--inline[data-size="xl"] {
    font-size: 0.9375em;
  }
</style>

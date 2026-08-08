<script lang="ts">
  import "@inflatable-cookie/poodle-styles/code.css";
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


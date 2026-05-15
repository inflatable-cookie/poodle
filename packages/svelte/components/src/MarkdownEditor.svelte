<script lang="ts">
  import { tick } from "svelte";
  import { marked } from "marked";

  import Icon from "./Icon.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  export let value = "";
  export let name: string | null = null;
  export let placeholder = "Write markdown...";
  export let disabled = false;
  export let required = false;
  export let ariaLabel = "Markdown editor";
  export let minHeight = "12rem";
  export let mode: "edit" | "preview" | "split" = "edit";
  /** Custom markdown-to-HTML renderer. When provided, replaces the built-in
   *  fallback. Use this to plug in a real parser (marked, remark, etc.). */
  export let renderHtml: ((markdown: string) => string) | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let onValueChange: ((value: string) => void) | null = null;

  let textareaEl: HTMLTextAreaElement | null = null;
  const uiPresentation = getUiPresentation();
  let uncontrolledValue = value;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: hasControlledValue = $$props.value !== undefined;
  $: currentValue = hasControlledValue ? value : uncontrolledValue;

  function setValue(nextValue: string): void {
    if (!hasControlledValue) {
      uncontrolledValue = nextValue;
    }

    onValueChange?.(nextValue);
  }

  function insertMarkdown(before: string, after = ""): void {
    if (!textareaEl || disabled) return;

    const start = textareaEl.selectionStart;
    const end = textareaEl.selectionEnd;
    const selected = currentValue.slice(start, end);
    const replacement = `${before}${selected || "text"}${after}`;

    setValue(currentValue.slice(0, start) + replacement + currentValue.slice(end));

    tick().then(() => {
      if (!textareaEl) return;
      const cursorPos = start + before.length;
      const cursorEnd = cursorPos + (selected.length || 4);
      textareaEl.focus();
      textareaEl.setSelectionRange(cursorPos, cursorEnd);
    });
  }

  function insertLine(prefix: string): void {
    if (!textareaEl || disabled) return;

    const start = textareaEl.selectionStart;
    const lineStart = currentValue.lastIndexOf("\n", start - 1) + 1;
    const lineEnd = currentValue.indexOf("\n", start);
    const end = lineEnd === -1 ? currentValue.length : lineEnd;
    const line = currentValue.slice(lineStart, end);

    setValue(currentValue.slice(0, lineStart) + `${prefix}${line}` + currentValue.slice(end));
  }

  function handleInput(event: Event): void {
    setValue((event.currentTarget as HTMLTextAreaElement).value);
  }

  $: previewHtml = renderHtml ? renderHtml(currentValue) : marked.parse(currentValue, { async: false }) as string;

  const toolbarActions = [
    { label: "Bold", icon: "bold", action: () => insertMarkdown("**", "**") },
    { label: "Italic", icon: "italic", action: () => insertMarkdown("*", "*") },
    { label: "Heading", icon: "heading", action: () => insertLine("## ") },
    { label: "Link", icon: "link", action: () => insertMarkdown("[", "](url)") },
    { label: "Code", icon: "code", action: () => insertMarkdown("`", "`") },
    { label: "Quote", icon: "quote", action: () => insertLine("> ") },
    { label: "List", icon: "list", action: () => insertLine("- ") },
  ];
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <div
    class="poodle-md-editor"
    class:poodle-md-editor--disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <div class="poodle-md-editor__toolbar">
      <div class="poodle-md-editor__tools">
        {#each toolbarActions as tool}
          <button
            type="button"
            class="poodle-md-editor__tool-btn"
            title={tool.label}
            aria-label={tool.label}
            disabled={disabled || mode === "preview"}
            onclick={tool.action}
          >
            <Icon icon={tool.icon} />
          </button>
        {/each}
      </div>

      <div class="poodle-md-editor__modes">
        <button
          type="button"
          class="poodle-md-editor__mode-btn"
          class:poodle-active={mode === "edit"}
          onclick={() => (mode = "edit")}
        >Edit</button>
        <button
          type="button"
          class="poodle-md-editor__mode-btn"
          class:poodle-active={mode === "split"}
          onclick={() => (mode = "split")}
        >Split</button>
        <button
          type="button"
          class="poodle-md-editor__mode-btn"
          class:poodle-active={mode === "preview"}
          onclick={() => (mode = "preview")}
        >Preview</button>
      </div>
    </div>

    <div class="poodle-md-editor__body" data-mode={mode}>
      {#if mode !== "preview"}
        <textarea
          bind:this={textareaEl}
          class="poodle-md-editor__textarea"
          name={name ?? undefined}
          {placeholder}
          disabled={disabled}
          {required}
          aria-label={ariaLabel}
          style="min-height: {minHeight}"
          oninput={handleInput}
          value={currentValue}
        ></textarea>
      {/if}

      {#if mode !== "edit"}
        <div class="poodle-md-editor__preview" aria-label="Preview">
          {#if currentValue.trim()}
            {@html previewHtml}
          {:else}
            <p class="poodle-md-editor__preview-empty">Nothing to preview</p>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</UiPresentationProvider>

<style>
  .poodle-md-editor {
    --poodle-md-editor-toolbar-y: 0.375rem;
    --poodle-md-editor-toolbar-x: 0.5rem;
    --poodle-md-editor-tool-gap: 0.125rem;
    --poodle-md-editor-tool-size: 1.75rem;
    --poodle-md-editor-mode-y: 0.1875rem;
    --poodle-md-editor-mode-x: 0.5rem;
    --poodle-md-editor-pane-x: 0.75rem;
    --poodle-md-editor-pane-y: 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-surface);
    overflow: hidden;
  }

  .poodle-md-editor[data-size="xs"] {
    --poodle-md-editor-tool-size: 1.5rem;
    --poodle-md-editor-mode-x: 0.375rem;
  }

  .poodle-md-editor[data-size="sm"] {
    --poodle-md-editor-tool-size: 1.75rem;
  }

  .poodle-md-editor[data-size="md"] {
    --poodle-md-editor-tool-size: 2rem;
  }

  .poodle-md-editor[data-size="lg"] {
    --poodle-md-editor-tool-size: 2.25rem;
    --poodle-md-editor-mode-x: 0.625rem;
  }

  .poodle-md-editor[data-size="xl"] {
    --poodle-md-editor-tool-size: 2.5rem;
    --poodle-md-editor-mode-x: 0.75rem;
  }

  .poodle-md-editor[data-density="compact"] {
    --poodle-md-editor-toolbar-y: 0.25rem;
    --poodle-md-editor-toolbar-x: 0.375rem;
    --poodle-md-editor-tool-gap: 0.0625rem;
    --poodle-md-editor-mode-y: 0.125rem;
    --poodle-md-editor-pane-x: 0.625rem;
    --poodle-md-editor-pane-y: 0.625rem;
  }

  .poodle-md-editor[data-density="comfortable"] {
    --poodle-md-editor-toolbar-y: 0.5rem;
    --poodle-md-editor-toolbar-x: 0.625rem;
    --poodle-md-editor-tool-gap: 0.1875rem;
    --poodle-md-editor-mode-y: 0.25rem;
    --poodle-md-editor-pane-x: 0.875rem;
    --poodle-md-editor-pane-y: 0.875rem;
  }

  .poodle-md-editor--disabled {
    opacity: var(--poodle-state-opacity-disabled);
    pointer-events: none;
  }

  .poodle-md-editor__toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: var(--poodle-md-editor-toolbar-y) var(--poodle-md-editor-toolbar-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
    flex-wrap: wrap;
  }

  .poodle-md-editor__tools {
    display: flex;
    gap: var(--poodle-md-editor-tool-gap);
  }

  .poodle-md-editor__tool-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-md-editor-tool-size);
    height: var(--poodle-md-editor-tool-size);
    padding: 0;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-md-editor__tool-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-md-editor__tool-btn:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-md-editor__tool-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .poodle-md-editor__modes {
    display: flex;
    gap: var(--poodle-md-editor-tool-gap);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    overflow: hidden;
  }

  .poodle-md-editor__mode-btn {
    min-height: calc(var(--poodle-md-editor-tool-size) - (var(--poodle-md-editor-toolbar-y) * 0.5));
    padding: var(--poodle-md-editor-mode-y) var(--poodle-md-editor-mode-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-md-editor__mode-btn:hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
  }

  .poodle-md-editor__mode-btn.poodle-active {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-md-editor__body {
    display: flex;
  }

  .poodle-md-editor__body[data-mode="split"] {
    gap: 0;
  }

  .poodle-md-editor__body[data-mode="split"] .poodle-md-editor__textarea {
    flex: 1;
    border-right: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-md-editor__body[data-mode="split"] .poodle-md-editor__preview {
    flex: 1;
  }

  .poodle-md-editor__textarea {
    flex: 1;
    width: 100%;
    padding: var(--poodle-md-editor-pane-y) var(--poodle-md-editor-pane-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    line-height: 1.6;
    resize: vertical;
    outline: none;
  }

  .poodle-md-editor__textarea::placeholder {
    color: var(--poodle-color-text-tertiary);
  }

  .poodle-md-editor__preview {
    flex: 1;
    padding: var(--poodle-md-editor-pane-y) var(--poodle-md-editor-pane-x);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.6;
    color: var(--poodle-color-text-primary);
    overflow-y: auto;
  }

  .poodle-md-editor__preview :global(h1) {
    font-size: 1.25rem;
    font-weight: 700;
    margin: 0 0 0.5rem;
  }

  .poodle-md-editor__preview :global(h2) {
    font-size: 1.0625rem;
    font-weight: 600;
    margin: 0 0 0.375rem;
  }

  .poodle-md-editor__preview :global(h3) {
    font-size: 0.9375rem;
    font-weight: 600;
    margin: 0 0 0.25rem;
  }

  .poodle-md-editor__preview :global(p) {
    margin: 0 0 0.5rem;
  }

  .poodle-md-editor__preview :global(strong) {
    font-weight: 700;
  }

  .poodle-md-editor__preview :global(code) {
    padding: 0.125rem 0.25rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
  }

  .poodle-md-editor__preview :global(blockquote) {
    margin: 0 0 0.5rem;
    padding: 0.375rem 0.75rem;
    border-left: 0.1875rem solid var(--poodle-color-border-default);
    color: var(--poodle-color-text-secondary);
  }

  .poodle-md-editor__preview :global(ul),
  .poodle-md-editor__preview :global(ol) {
    margin: 0 0 0.5rem;
    padding-left: 1.25rem;
  }

  .poodle-md-editor__preview :global(li) {
    margin: 0 0 0.125rem;
  }

  .poodle-md-editor__preview :global(hr) {
    border: 0;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    margin: 0.75rem 0;
  }

  .poodle-md-editor__preview :global(a) {
    color: var(--poodle-color-accent-default, #6366f1);
    text-decoration: underline;
  }

  .poodle-md-editor__preview-empty {
    color: var(--poodle-color-text-tertiary);
    font-style: italic;
    margin: 0;
  }
</style>

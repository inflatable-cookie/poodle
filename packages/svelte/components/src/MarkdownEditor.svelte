<script lang="ts">
  import { tick } from "svelte";
  import { marked } from "marked";

  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    value?: string | undefined;
    name?: string | null;
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    ariaLabel?: string;
    minHeight?: string;
    mode?: "edit" | "preview" | "split";
    renderHtml?: ((markdown: string) => string) | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | null;
  }

  let {
    value = $bindable<string | undefined>(undefined),
    name = null,
    placeholder = "Write markdown...",
    disabled = false,
    required = false,
    ariaLabel = "Markdown editor",
    minHeight = "12rem",
    mode = "edit",
    renderHtml = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = null,
  }: Props = $props();

  /** Custom markdown-to-HTML renderer. When provided, replaces the built-in
   *  fallback. Use this to plug in a real parser (marked, remark, etc.). */
  let textareaEl = $state<HTMLTextAreaElement | null>(null);
  const uiPresentation = getUiPresentation();
  let uncontrolledValue = $state("");
  let currentMode = $state<"edit" | "preview" | "split">("edit");
  let seededState = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const currentValue = $derived(hasControlledValue ? (value ?? "") : uncontrolledValue);

  $effect.pre(() => {
    if (seededState) {
      return;
    }

    uncontrolledValue = value ?? "";
    currentMode = mode;
    seededState = true;
  });

  $effect(() => {
    currentMode = mode;
  });

  function setValue(nextValue: string): void {
    value = nextValue;

    if (!hasControlledValue) {
      uncontrolledValue = nextValue;
    }

    onValueChange?.(nextValue);
  }

  function syncTextareaValue(): void {
    if (!textareaEl) return;
    setValue(textareaEl.value);
  }

  function applyNativeTextEdit(start: number, end: number, replacement: string): void {
    if (!textareaEl) return;

    textareaEl.focus();
    textareaEl.setSelectionRange(start, end);

    const usedNativeCommand =
      typeof document !== "undefined" &&
      typeof document.execCommand === "function" &&
      document.execCommand("insertText", false, replacement);

    if (!usedNativeCommand) {
      textareaEl.setRangeText(replacement, start, end, "end");
    }

    syncTextareaValue();
  }

  function insertMarkdown(before: string, after = ""): void {
    if (!textareaEl || disabled) return;

    const start = textareaEl.selectionStart;
    const end = textareaEl.selectionEnd;
    const selected = currentValue.slice(start, end);
    const replacement = `${before}${selected || "text"}${after}`;
    applyNativeTextEdit(start, end, replacement);

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
    applyNativeTextEdit(lineStart, end, `${prefix}${line}`);

    tick().then(() => {
      if (!textareaEl) return;
      const nextCursor = start + prefix.length;
      textareaEl.setSelectionRange(nextCursor, nextCursor);
    });
  }

  function handleInput(event: Event): void {
    setValue((event.currentTarget as HTMLTextAreaElement).value);
  }

  const previewHtml = $derived(
    renderHtml ? renderHtml(currentValue) : (marked.parse(currentValue, { async: false }) as string)
  );

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
            disabled={disabled || currentMode === "preview"}
            onclick={tool.action}
          >
            <Icon icon={tool.icon} />
          </button>
        {/each}
      </div>

      <div class="poodle-md-editor__modes">
        <IconButton
          icon="pencil"
          ariaLabel="Edit"
          tooltip="Edit"
          variant={currentMode === "edit" ? "secondary" : "ghost"}
          sizeRole="chrome"
          onClick={() => (currentMode = "edit")}
        />
        <IconButton
          icon="columns-2"
          ariaLabel="Split"
          tooltip="Split"
          variant={currentMode === "split" ? "secondary" : "ghost"}
          sizeRole="chrome"
          onClick={() => (currentMode = "split")}
        />
        <IconButton
          icon="eye"
          ariaLabel="Preview"
          tooltip="Preview"
          variant={currentMode === "preview" ? "secondary" : "ghost"}
          sizeRole="chrome"
          onClick={() => (currentMode = "preview")}
        />
      </div>
    </div>

    <div class="poodle-md-editor__body" data-mode={currentMode}>
      {#if currentMode !== "preview"}
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

      {#if currentMode !== "edit"}
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
    background: var(--poodle-recipe-markdown-editor-fill, var(--poodle-color-background-surface));
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
    background: var(--poodle-recipe-markdown-editor-toolbar-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent));
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
    background: var(--poodle-recipe-markdown-editor-tool-btn-fill, transparent);
    color: var(--poodle-recipe-markdown-editor-tool-btn-text, var(--poodle-color-text-secondary));
    cursor: pointer;
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-md-editor__tool-btn:hover:not(:disabled) {
    background: var(--poodle-recipe-markdown-editor-hover-tool-btn-fill, color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent));
    color: var(--poodle-recipe-markdown-editor-hover-tool-btn-text, var(--poodle-color-text-primary));
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
    padding: 0;
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
    background: var(--poodle-recipe-markdown-editor-textarea-fill, transparent);
    color: var(--poodle-recipe-markdown-editor-textarea-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
    line-height: 1.6;
    resize: vertical;
    outline: none;
  }

  .poodle-md-editor__textarea::placeholder {
    color: var(--poodle-recipe-markdown-editor-placeholder-textarea-text, var(--poodle-color-text-tertiary));
  }

  .poodle-md-editor__preview {
    flex: 1;
    padding: var(--poodle-md-editor-pane-y) var(--poodle-md-editor-pane-x);
    font-family: var(--poodle-typography-body-family);
    font-size: 0.875rem;
    line-height: 1.6;
    color: var(--poodle-recipe-markdown-editor-preview-text, var(--poodle-color-text-primary));
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
    background: var(--poodle-recipe-markdown-editor-preview-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
  }

  .poodle-md-editor__preview :global(blockquote) {
    margin: 0 0 0.5rem;
    padding: 0.375rem 0.75rem;
    border-left: 0.1875rem solid var(--poodle-color-border-default);
    color: var(--poodle-recipe-markdown-editor-preview-text, var(--poodle-color-text-secondary));
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
    color: var(--poodle-recipe-markdown-editor-preview-link-text, var(--poodle-color-accent-base));
    text-decoration: underline;
  }

  .poodle-md-editor__preview-empty {
    color: var(--poodle-recipe-markdown-editor-preview-empty-text, var(--poodle-color-text-tertiary));
    font-style: italic;
    margin: 0;
  }
</style>

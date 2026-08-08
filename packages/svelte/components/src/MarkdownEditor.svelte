<script lang="ts">
  import "@inflatable-cookie/poodle-styles/markdown-editor.css";
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


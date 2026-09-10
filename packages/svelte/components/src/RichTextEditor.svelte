<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/rich-text.css";
  import {
    RICH_TEXT_COMMAND_LABELS,
    RICH_TEXT_STANDARD_FEATURES,
    RICH_TEXT_TOGGLE_COMMANDS,
    installInputModality,
  } from "@inflatable-cookie/poodle-core";
  import type {
    ProseMirrorDocumentJSON,
    RichTextCommand,
    RichTextFeature,
  } from "@inflatable-cookie/poodle-core";
  import type { ControlDensity } from "./types";
  import { onDestroy, onMount } from "svelte";

  import {
    assertAdmittedFeatures,
    assertAdmittedToolbar,
    assertValidRichTextDocument,
    createRichTextEngine,
    createRichTextSchema,
  } from "./rich-text-engine";
  import type {
    RichTextEngine,
    RichTextImageInput,
    RichTextToolbarSnapshot,
  } from "./rich-text-engine";
  import { getUiPresentation } from "./presentation";

  /**
   * Web-admitted controlled rich-text editing surface over TipTap 3 and
   * ProseMirror. ProseMirror document JSON and schema semantics are the
   * authority; the engine stays private and the component is reached only
   * through `./rich-text`.
   */
  interface Props {
    value: ProseMirrorDocumentJSON;
    features?: readonly RichTextFeature[];
    toolbar?: "auto" | readonly RichTextCommand[];
    readOnly?: boolean;
    disabled?: boolean;
    placeholder?: string;
    ariaLabel?: string;
    requestImage?: (() => Promise<RichTextImageInput | null>) | null;
    density?: ControlDensity | null;
    onChange?: ((document: ProseMirrorDocumentJSON) => void) | null;
  }

  let {
    value,
    features = RICH_TEXT_STANDARD_FEATURES,
    toolbar = "auto",
    readOnly = false,
    disabled = false,
    placeholder = "",
    ariaLabel = "Rich text editor",
    requestImage = null,
    density = null,
    onChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  /** Imperative escape hatch: focus the editing surface. Documented as a method, not a prop. */
  export function focus(): void {
    (hostElement?.querySelector(".ProseMirror") as HTMLElement | null)?.focus();
  }

  let hostElement: HTMLDivElement | null = $state(null);
  let engine: RichTextEngine | null = null;
  let snapshot: RichTextToolbarSnapshot | null = $state(null);
  let linkEditorOpen = $state(false);
  let linkValue = $state("");
  const surfaceId = `poodle-rich-text-editor-${Math.random().toString(36).slice(2)}`;

  function currentOptions() {
    return {
      value,
      features,
      toolbar,
      readOnly,
      disabled,
      placeholder,
      ariaLabel,
      requestImage,
    };
  }

  onMount(() => {
    // Fail closed before any mount output: unknown or duplicate features,
    // unsupported explicit toolbar commands, and invalid documents refuse
    // with zero callbacks and zero partial output.
    assertAdmittedFeatures(features);
    assertAdmittedToolbar(toolbar, features, requestImage);
    assertValidRichTextDocument(createRichTextSchema(features), value);
    installInputModality();
    engine = createRichTextEngine(hostElement as HTMLDivElement, currentOptions(), {
      onChange: (document) => onChange?.(document),
      onToolbar: (next) => {
        snapshot = next;
      },
    });
    return () => {
      engine?.destroy();
      engine = null;
    };
  });

  onDestroy(() => {
    engine?.destroy();
    engine = null;
  });

  $effect(() => {
    // Read every prop unconditionally before touching `engine`.
    const next = currentOptions();
    engine?.update(next);
  });

  function runToolbarCommand(command: RichTextCommand): void {
    if (command === "link") {
      linkValue = engine?.linkHref() ?? "";
      linkEditorOpen = true;
      return;
    }
    engine?.runCommand(command);
  }

  function submitLink(): void {
    if (!engine) return;
    engine.applyLink(linkValue);
    linkEditorOpen = false;
    focus();
  }

  function removeLink(): void {
    engine?.removeLink();
    linkEditorOpen = false;
    focus();
  }

  function closeLinkEditor(): void {
    linkEditorOpen = false;
    focus();
  }

  function handleLinkInputKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      event.preventDefault();
      submitLink();
      return;
    }
    if (event.key === "Escape") {
      event.stopPropagation();
      closeLinkEditor();
    }
  }

  function handleToolbarKeydown(event: KeyboardEvent): void {
    // Rove between visible toolbar controls with the arrow keys.
    if (event.key !== "ArrowRight" && event.key !== "ArrowLeft") return;
    const root = event.currentTarget as HTMLElement;
    const buttons = [...root.querySelectorAll<HTMLButtonElement>("button:not(:disabled)")];
    if (buttons.length === 0) return;
    event.preventDefault();
    const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
    const offset = event.key === "ArrowRight" ? 1 : -1;
    const next = buttons[(index + offset + buttons.length) % buttons.length];
    if (next) next.focus();
  }

  const isToggle = (command: RichTextCommand): boolean =>
    (RICH_TEXT_TOGGLE_COMMANDS as readonly string[]).includes(command);
</script>

<div
  class="poodle-rich-text-editor"
  data-density={resolvedDensity}
  data-disabled={disabled || undefined}
  data-readonly={readOnly || undefined}
  inert={disabled || undefined}
>
  {#if snapshot && snapshot.commands.length > 0}
    <div
      class="poodle-rich-text-editor__toolbar"
      role="toolbar"
      tabindex="-1"
      aria-label={ariaLabel}
      aria-controls={surfaceId}
      onkeydown={handleToolbarKeydown}
    >
      {#each snapshot.commands as command (command)}
        <button
          type="button"
          class="poodle-rich-text-editor__toolbar-button"
          data-command={command}
          data-pressed={snapshot.states[command].active || undefined}
          aria-pressed={isToggle(command)
            ? (snapshot.states[command].active ? "true" : "false")
            : undefined}
          disabled={!snapshot.states[command].available || disabled}
          onclick={() => runToolbarCommand(command)}
        >
          {RICH_TEXT_COMMAND_LABELS[command]}
        </button>
      {/each}
    </div>
  {/if}
  {#if linkEditorOpen}
    <div class="poodle-rich-text-editor__link-editor" role="group" aria-label="Edit link">
      <input
        class="poodle-rich-text-editor__link-input"
        type="url"
        aria-label="Link URL"
        bind:value={linkValue}
        onkeydown={handleLinkInputKeydown}
      />
      <button
        type="button"
        class="poodle-rich-text-editor__toolbar-button"
        data-command="apply-link"
        onclick={submitLink}
      >
        Apply
      </button>
      {#if snapshot?.states.link.active}
        <button
          type="button"
          class="poodle-rich-text-editor__toolbar-button"
          data-command="remove-link"
          onclick={removeLink}
        >
          Remove link
        </button>
      {/if}
    </div>
  {/if}
  <div bind:this={hostElement} id={surfaceId} class="poodle-rich-text-editor__viewport"></div>
</div>

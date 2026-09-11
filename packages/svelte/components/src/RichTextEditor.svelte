<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/rich-text.css";
  import {
    RICH_TEXT_COMMAND_GROUP_LABELS,
    RICH_TEXT_COMMAND_PRESENTATION,
    RICH_TEXT_STANDARD_FEATURES,
    installInputModality,
  } from "@inflatable-cookie/poodle-core";
  import type {
    ProseMirrorDocumentJSON,
    RichTextCommand,
    RichTextFeature,
    RichTextCommandGroup,
  } from "@inflatable-cookie/poodle-core";
  import type { ControlDensity } from "./types";
  import { onDestroy, onMount } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as IconButton } from "./IconButton.svelte";
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
  // The last host value object pushed to the engine: only a genuinely new
  // value object is a controlled push; re-renders with the previous value
  // never count as a host revert.
  let sentValue: ProseMirrorDocumentJSON | undefined;
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
    // Push the value only when the host actually sent a new value object:
    // effects that re-run for unrelated prop changes (label, placeholder)
    // carry the previous host value, which must never count as a revert.
    const valueChanged = next.value !== sentValue;
    if (valueChanged) {
      engine?.update(next);
      sentValue = next.value;
    } else if (engine) {
      const { value: _sentValue, ...rest } = next;
      engine.update(rest);
    }
  });

  function runToolbarCommand(command: RichTextCommand): void {
    if (command === "link") {
      linkValue = engine?.linkHref() ?? "";
      linkEditorOpen = true;
      return;
    }
    engine?.runCommand(command);
  }

  /** Shared control props: one command-presentation map drives chrome, name,
   *  tooltip, icon, tone, and availability for every admitted command. */
  function controlProps(command: RichTextCommand) {
    const presentation = RICH_TEXT_COMMAND_PRESENTATION[command];
    const state = snapshot?.states[command];
    return {
      variant: "ghost" as const,
      tone: presentation.destructive ? ("danger" as const) : ("default" as const),
      sizeRole: "chrome" as const,
      density: resolvedDensity,
      icon: presentation.icon,
      ariaLabel: presentation.label,
      tooltip: presentation.label,
      disabled: !state?.available || disabled,
      onClick: () => runToolbarCommand(command),
    };
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

  interface RichTextToolbarCluster {
    group: RichTextCommandGroup;
    commands: RichTextCommand[];
  }

  /** Feature-derived order is preserved; consecutive same-group commands form
   *  one intact cluster that wraps as a unit at constrained widths. */
  const clusters = $derived.by<RichTextToolbarCluster[]>(() => {
    const result: RichTextToolbarCluster[] = [];
    for (const command of snapshot?.commands ?? []) {
      const group = RICH_TEXT_COMMAND_PRESENTATION[command].group;
      const last = result[result.length - 1];
      if (last && last.group === group) last.commands.push(command);
      else result.push({ group, commands: [command] });
    }
    return result;
  });
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
      aria-label={`${ariaLabel} toolbar`}
      aria-controls={surfaceId}
      onkeydown={handleToolbarKeydown}
    >
      {#each clusters as cluster (cluster.commands[0])}
        <div
          class="poodle-rich-text-editor__group"
          role="group"
          aria-label={RICH_TEXT_COMMAND_GROUP_LABELS[cluster.group]}
        >
          {#each cluster.commands as command (command)}
            <span class="poodle-rich-text-editor__command" data-command={command}>
              {#if RICH_TEXT_COMMAND_PRESENTATION[command].glyph}
                <IconButton
                  {...controlProps(command)}
                  pressed={snapshot.states[command].active}
                >
                  <span class="poodle-rich-text-editor__glyph" aria-hidden="true">
                    {RICH_TEXT_COMMAND_PRESENTATION[command].glyph}
                  </span>
                </IconButton>
              {:else}
                <IconButton
                  {...controlProps(command)}
                  pressed={RICH_TEXT_COMMAND_PRESENTATION[command].toggle
                    ? snapshot.states[command].active
                    : null}
                />
              {/if}
            </span>
          {/each}
        </div>
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
      <Button variant="secondary" sizeRole="chrome" density={resolvedDensity} onclick={submitLink}>
        Apply
      </Button>
      {#if snapshot?.states.link.active}
        <Button
          variant="ghost"
          tone="danger"
          sizeRole="chrome"
          density={resolvedDensity}
          onclick={removeLink}
        >
          Remove link
        </Button>
      {/if}
    </div>
  {/if}
  <div bind:this={hostElement} id={surfaceId} class="poodle-rich-text-editor__viewport"></div>
</div>

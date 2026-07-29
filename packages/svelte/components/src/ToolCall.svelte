<script lang="ts">
  import "@poodle/styles/tool-call.css";

  import Code from "./Code.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    ToolCallStatus,
  } from "./types";

  interface Props {
    id: string;
    label: string;
    detail?: string | null;
    status?: ToolCallStatus;
    icon?: string | null;
    output?: string | null;
    outputLanguage?: string | null;
    expanded?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((id: string) => void) | undefined;
  }

  let {
    id,
    label,
    detail = null,
    status = "success",
    icon = null,
    output = null,
    outputLanguage = null,
    expanded = $bindable<boolean>(false),
    size = null,
    sizeRole = "control",
    density = null,
    onToggle = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSupportingVisualSize(resolvedSize));

  const hasOutput = $derived(typeof output === "string" && output.length > 0);

  /**
   * The icon for a kind of work.
   *
   * Shared with the natives through the spec so every target agrees. `icon`
   * exists so a host with its own vocabulary is never stuck with the fallback.
   */
  function iconForLabel(value: string): string {
    const key = value.trim().toLowerCase();
    if (key.startsWith("ran command") || key.startsWith("command")) return "terminal";
    if (key.startsWith("file change") || key.startsWith("edited")) return "file-pen";
    if (key.startsWith("search")) return "search";
    if (key.startsWith("read")) return "file-text";
    return "dot";
  }

  const resolvedIcon = $derived(icon ?? iconForLabel(label));

  /**
   * Status reaches assistive technology through the name; colour and glyph do
   * not. `success` is omitted as the unremarkable case, and the detail is
   * carried in full — the truncation is visual, and the whole command is
   * exactly what a truncated row is hiding.
   */
  const accessibleName = $derived(
    [label, detail ? `: ${detail}` : "", status === "success" ? "" : `, ${status}`].join(""),
  );

  function toggle(): void {
    if (!hasOutput) return;
    expanded = !expanded;
    onToggle?.(id);
  }
</script>

<div
  class="poodle-tool-call"
  data-status={status}
  data-expanded={hasOutput ? String(expanded) : undefined}
  data-interactive={String(hasOutput)}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <!-- A control that cannot do anything should not be in the tab order at all,
       so a row with no output is a div rather than a disabled button. -->
  <svelte:element
    this={hasOutput ? "button" : "div"}
    class="poodle-tool-call__trigger"
    type={hasOutput ? "button" : undefined}
    role={hasOutput ? undefined : "presentation"}
    aria-expanded={hasOutput ? expanded : undefined}
    aria-controls={hasOutput ? `${id}-output` : undefined}
    aria-label={hasOutput ? accessibleName : undefined}
    onclick={hasOutput ? toggle : undefined}
  >
    <span class="poodle-tool-call__icon"><Icon name={resolvedIcon} size={glyphSize} /></span>
    <span class="poodle-tool-call__label">{label}</span>
    {#if detail}
      <code class="poodle-tool-call__detail">{detail}</code>
    {/if}
    {#if hasOutput}
      <span class="poodle-tool-call__disclosure"><Icon name="chevron-down" size={glyphSize} /></span>
    {/if}
    <span class="poodle-tool-call__status" aria-hidden="true">
      {#if status === "running"}
        <Spinner variant="ring" size={glyphSize} tone="current" />
      {:else if status === "error"}
        <Icon name="x" size={glyphSize} />
      {:else}
        <Icon name="check" size={glyphSize} />
      {/if}
    </span>
  </svelte:element>

  <!-- Lazily constructed: a transcript of a thousand rows must not build a
       thousand code blocks for output nobody opened. -->
  {#if hasOutput && expanded}
    <div class="poodle-tool-call__output" id={`${id}-output`}>
      <Code source={output ?? ""} language={outputLanguage} size={resolvedSize} />
    </div>
  {/if}
</div>

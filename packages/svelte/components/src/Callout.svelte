<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/callout.css";
  import type { Snippet } from "svelte";
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { default as Spinner } from "./Spinner.svelte";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, SpinnerTone, StatusTone, ToneFill } from "./types";

  type CalloutAnnounceMode = "none" | "polite" | "assertive";

  interface Props {
    tone?: StatusTone;
    fill?: ToneFill;
    title?: string | null;
    message?: string | null;
    ariaLabel?: string | null;
    announceMode?: CalloutAnnounceMode;
    dismissible?: boolean;
    dismissLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onDismiss?: (() => void) | undefined;
    icon?: Snippet;
    actions?: Snippet;
    children?: Snippet;
  }

  const uiPresentation = getUiPresentation();

  const toneIcon: Record<string, string> = {
    success: "check",
    warning: "triangle-alert",
    danger: "circle-x",
    info: "info",
    neutral: "info",
  };

  let {
    tone = "neutral",
    fill = "tint",
    title = null,
    message = null,
    ariaLabel = null,
    announceMode = "none",
    dismissible = false,
    dismissLabel = "Dismiss message",
    size = null,
    sizeRole = "control",
    density = null,
    onDismiss = undefined,
    icon,
    actions,
    children,
  }: Props = $props();

  let resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  let resolvedDensity = $derived(density ?? $uiPresentation.density);
  let role = $derived(
    announceMode === "assertive"
      ? "alert"
      : announceMode === "polite"
        ? "status"
        : undefined,
  );
  let ariaLive = $derived(
    announceMode === "assertive"
      ? "assertive" as const
      : announceMode === "polite"
        ? "polite" as const
        : undefined,
  );
  let spinnerTone: SpinnerTone = $derived(fill === "solid" ? "current" : "accent");
</script>

<section
  class="poodle-callout"
  data-tone={tone}
  data-fill={fill}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  aria-label={ariaLabel ?? undefined}
  role={role}
  aria-live={ariaLive}
>
  <div class="poodle-callout__body">
    <span class="poodle-callout__icon" aria-hidden="true">
      {#if icon}
        {@render icon()}
      {:else if tone === "pending"}
        <Spinner variant="ring" size={resolvedSize} sizeRole="chrome" tone={spinnerTone} />
      {:else}
        <Icon name={toneIcon[tone] ?? "info"} size={resolvedSize} />
      {/if}
    </span>

    <div class="poodle-callout__content">
      {#if title}
        <strong>{title}</strong>
      {/if}
      {#if message}
        <p>{message}</p>
      {/if}
      {@render children?.()}
    </div>
  </div>

  {#if actions}
    <div class="poodle-callout__actions">
      {@render actions()}
    </div>
  {/if}

  {#if dismissible}
    <button
      type="button"
      class="poodle-callout__dismiss"
      aria-label={dismissLabel}
      onclick={() => onDismiss?.()}
    >
      <Icon name="x" />
    </button>
  {/if}
</section>

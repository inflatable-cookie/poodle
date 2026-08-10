<script module lang="ts">
  let nextRemediationBannerId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/remediation-banner.css";
  import Button from "./Button.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import type { AnnouncementMode, RemediationAction, StatusTone } from "./types";

  let {
    tone = "warning", title, message, announceMode = "polite", primaryAction = null,
    secondaryAction = null, isDismissible = false, dismissLabel = "Dismiss",
    onAction = undefined, onDismiss = undefined,
  }: {
    tone?: StatusTone;
    title: string;
    message: string;
    announceMode?: AnnouncementMode;
    primaryAction?: RemediationAction | null;
    secondaryAction?: RemediationAction | null;
    isDismissible?: boolean;
    dismissLabel?: string;
    onAction?: ((id: string) => void) | undefined;
    onDismiss?: (() => void) | undefined;
  } = $props();

  const titleId = `poodle-remediation-banner-title-${++nextRemediationBannerId}`;
  const toneIcon: Record<StatusTone, string> = {
    neutral: "info", info: "info", success: "check", warning: "triangle-alert",
    danger: "circle-x", pending: "loader-circle",
  };
  const role = $derived(announceMode === "assertive" ? "alert" : announceMode === "polite" ? "status" : undefined);
  const ariaLive = $derived(announceMode === "none" ? undefined : announceMode);
</script>

<section class="poodle-remediation-banner" data-tone={tone} aria-labelledby={titleId} {role} aria-live={ariaLive}>
  <span class="poodle-remediation-banner__icon" aria-hidden="true">
    {#if tone === "pending"}<Spinner variant="ring" tone="accent" />{:else}<Icon name={toneIcon[tone]} />{/if}
  </span>
  <div class="poodle-remediation-banner__content">
    <strong id={titleId}>{title}</strong>
    <p>{message}</p>
  </div>
  {#if primaryAction || secondaryAction}
    <div class="poodle-remediation-banner__actions">
      {#if primaryAction}
        <Button variant={primaryAction.variant} disabled={primaryAction.isDisabled} onClick={() => onAction?.(primaryAction.id)}>{primaryAction.label}</Button>
      {/if}
      {#if secondaryAction}
        <Button variant={secondaryAction.variant} disabled={secondaryAction.isDisabled} onClick={() => onAction?.(secondaryAction.id)}>{secondaryAction.label}</Button>
      {/if}
    </div>
  {/if}
  {#if isDismissible}
    <button class="poodle-remediation-banner__dismiss" type="button" aria-label={dismissLabel} onclick={() => onDismiss?.()}><Icon name="x" /></button>
  {/if}
</section>

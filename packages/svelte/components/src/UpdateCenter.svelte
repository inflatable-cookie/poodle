<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/update-center.css";
  import {
    updateDownloadLabel,
    type Channel,
    type UpdateAheadOfChannel,
    type UpdateAvailabilityProjection,
    type UpdateControllerStatus,
    type UpdateDeferral,
    type UpdatePresence,
    type UpdateProgressProjection,
    type UpdateRejectionCode,
  } from "@inflatable-cookie/poodle-core";

  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Popover } from "./Popover.svelte";
  import { default as UpdateStatus } from "./UpdateStatus.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole } from "./types";

  interface Props {
    presence: UpdatePresence;
    status?: UpdateControllerStatus;
    availability?: UpdateAvailabilityProjection;
    progress?: UpdateProgressProjection;
    channel?: Channel;
    installedVersion?: string;
    deferral?: UpdateDeferral;
    lastRejection?: UpdateRejectionCode;
    aheadOfChannel?: UpdateAheadOfChannel;
    pending?: boolean;
    observe?: ((observer: () => void) => () => void) | null;
    open?: boolean | null;
    defaultOpen?: boolean;
    placement?: OverlayPlacement;
    title?: string;
    ariaLabel?: string | null;
    triggerLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    installLabel?: string;
    deferLabel?: string;
    checkLabel?: string;
    retryLabel?: string;
    confirmInstall?: boolean;
    onCheck?: (() => void) | null;
    onInstall?: (() => void) | null;
    onDefer?: (() => void) | null;
    onOpenChange?: ((open: boolean) => void) | null;
  }

  let {
    presence,
    status = { kind: "idle" },
    availability = undefined,
    progress = undefined,
    channel = undefined,
    installedVersion = undefined,
    deferral = undefined,
    lastRejection = undefined,
    aheadOfChannel = undefined,
    pending = false,
    observe = null,
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    placement = "bottom-end",
    title = "Updates",
    ariaLabel = null,
    triggerLabel = null,
    size = null,
    sizeRole = "chrome",
    density = null,
    installLabel = "Install and restart",
    deferLabel = "Later",
    checkLabel = "Check for updates",
    retryLabel = "Try again",
    confirmInstall = true,
    onCheck = null,
    onInstall = null,
    onDefer = null,
    onOpenChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let notify = $state(0);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  $effect(() => {
    const unsubscribe = observe?.(() => {
      notify += 1;
    });
    return () => unsubscribe?.();
  });

  const isOpen = $derived(open === null ? uncontrolledOpen : open);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedTriggerLabel = $derived(triggerLabel ?? title);

  const isDownloading = $derived(progress?.state === "downloading");
  const downloadFraction = $derived(progress?.state === "downloading" ? progress.fraction : null);
  const downloadingLabel = $derived(isDownloading ? updateDownloadLabel(downloadFraction) : resolvedTriggerLabel);

  function handleOpenChange(next: boolean): void {
    if (open === null) uncontrolledOpen = next;
    else open = next;
    onOpenChange?.(next);
  }
</script>

{#if presence !== "hidden"}
  <div class="poodle-update-center">
    <Popover
      open={isOpen}
      {placement}
      initialFocus="content"
      triggerIsInteractive
      ariaLabel={ariaLabel ?? title}
      surfaceMinWidth="min(16rem, calc(100vw - 2rem))"
      surfaceMaxWidth="min(24rem, calc(100vw - 2rem))"
      onOpenChange={handleOpenChange}
    >
      {#snippet trigger()}
        <span class="poodle-update-center__trigger" data-presence={presence}>
          <IconButton
            icon="download"
            ariaLabel={downloadingLabel}
            tooltip={title}
            variant="ghost"
            size={resolvedSize}
            density={resolvedDensity}
            expanded={isOpen}
          >
            {#snippet children()}
              {#if isDownloading}
                {@const circumference = 2 * Math.PI * 9}
                <span
                  class="poodle-update-center__ring"
                  data-indeterminate={downloadFraction === null}
                >
                  <svg viewBox="0 0 24 24" width="24" height="24" fill="none" aria-hidden="true">
                    <circle class="poodle-update-center__ring-track" cx="12" cy="12" r="9"></circle>
                    {#if downloadFraction === null}
                      <circle class="poodle-update-center__ring-fill" cx="12" cy="12" r="9"></circle>
                    {:else}
                      <circle
                        class="poodle-update-center__ring-fill"
                        cx="12"
                        cy="12"
                        r="9"
                        stroke-dasharray={circumference}
                        stroke-dashoffset={circumference * (1 - downloadFraction)}
                      ></circle>
                    {/if}
                  </svg>
                </span>
              {:else}
                <Icon icon="download" size={resolvedSize} />
              {/if}
            {/snippet}
          </IconButton>
          {#if presence === "attention"}
            <span class="poodle-update-center__indicator" aria-hidden="true"></span>
          {/if}
        </span>
      {/snippet}

      <section class="poodle-update-center__surface" aria-label={ariaLabel ?? title}>
        <header class="poodle-update-center__header">
          <h2>{title}</h2>
        </header>

        <div class="poodle-update-center__body">
          <UpdateStatus
            {status}
            {availability}
            {progress}
            {channel}
            {installedVersion}
            {deferral}
            {lastRejection}
            {aheadOfChannel}
            {pending}
            {observe}
            {installLabel}
            {deferLabel}
            {checkLabel}
            {retryLabel}
            {confirmInstall}
            {onCheck}
            {onInstall}
            {onDefer}
          />
        </div>
      </section>
    </Popover>
  </div>
{/if}

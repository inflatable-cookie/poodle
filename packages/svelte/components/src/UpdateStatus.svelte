<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/update-center.css";
  import {
    updateStatusView,
    type Channel,
    type UpdateAheadOfChannel,
    type UpdateAvailabilityProjection,
    type UpdateControllerStatus,
    type UpdateDeferral,
    type UpdateProgressProjection,
    type UpdateRejectionCode,
    type UpdateStatusAction,
  } from "@inflatable-cookie/poodle-core";

  import { default as AlertDialog } from "./AlertDialog.svelte";
  import { default as Button } from "./Button.svelte";
  import { default as Progress } from "./Progress.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
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
  }

  let {
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
    size = null,
    sizeRole = "control",
    density = null,
    installLabel = "Install and restart",
    deferLabel = "Later",
    checkLabel = "Check for updates",
    retryLabel = "Try again",
    confirmInstall = true,
    onCheck = null,
    onInstall = null,
    onDefer = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let confirmOpen = $state(false);
  // Bumped by `observe` so the view re-derives when the authority notifies.
  let notify = $state(0);

  $effect(() => {
    const unsubscribe = observe?.(() => {
      notify += 1;
    });
    return () => unsubscribe?.();
  });

  const view = $derived.by(() => {
    void notify;
    return updateStatusView({
      status,
      availability,
      progress,
      deferral,
      lastRejection,
      aheadOfChannel,
      channel,
      installedVersion,
    });
  });

  const actionLabels = $derived<Record<UpdateStatusAction["type"], string>>({
    install: installLabel,
    defer: deferLabel,
    check: checkLabel,
  });

  function dispatch(action: UpdateStatusAction): void {
    if (action.type === "install") {
      if (confirmInstall) {
        confirmOpen = true;
        return;
      }
      onInstall?.();
    } else if (action.type === "check") {
      onCheck?.();
    } else {
      onDefer?.();
    }
  }

  function confirmAction(): void {
    confirmOpen = false;
    onInstall?.();
  }
</script>

<div class="poodle-update-status" data-state={view.state} data-tone={view.tone}>
  <div class="poodle-update-status__head">
    {#if view.busy}
      <Spinner variant="ring" size="sm" tone="muted" />
    {/if}
    <span class="poodle-update-status__title">{view.title}</span>
  </div>

  {#if view.body}
    <p class="poodle-update-status__body">{view.body}</p>
  {/if}

  {#if view.progress}
    <Progress
      value={view.progress.fraction === null ? null : Math.round(view.progress.fraction * 100)}
      indeterminate={view.progress.fraction === null}
      ariaLabel="Download progress"
      size="sm"
    />
  {/if}

  {#if view.notice}
    {@const retry = view.notice.retry}
    <div class="poodle-update-status__notice" data-tone={view.notice.tone} role="status">
      <span>{view.notice.message}</span>
      {#if retry}
        <Button
          variant="ghost"
          size="xs"
          density={resolvedDensity}
          disabled={pending}
          onClick={() => dispatch(retry)}
        >
          {retryLabel}
        </Button>
      {/if}
    </div>
  {/if}

  {#if view.actions.length > 0}
    <div class="poodle-update-status__actions">
      {#each view.actions as action}
        <Button
          variant={action.type === "install" ? "primary" : "secondary"}
          size="sm"
          density={resolvedDensity}
          disabled={pending}
          onClick={() => dispatch(action)}
        >
          {actionLabels[action.type]}
        </Button>
      {/each}
    </div>
  {/if}
</div>

<AlertDialog
  open={confirmOpen}
  tone="warning"
  title="Install and restart?"
  description="The application will close and restart to finish the update."
  confirmLabel={installLabel}
  cancelLabel="Cancel"
  onConfirm={confirmAction}
  onCancel={() => (confirmOpen = false)}
  onOpenChange={(next) => {
    if (!next) {
      confirmOpen = false;
    }
  }}
  size={resolvedSize}
  density={resolvedDensity}
/>

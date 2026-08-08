<script lang="ts">
  import "@inflatable-cookie/poodle-styles/toast-host.css";
  import { normalizeToast, reconcileToastTimers } from "@inflatable-cookie/poodle-headless";
  import type { Readable } from "svelte/store";

  import { default as ToastStack } from "./ToastStack.svelte";
  import type {
    ToastHostPlacement,
    ToastHostStore,
    ToastHostStoreItem,
    ToastItem,
  } from "./types";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  let {
    store,
    autoDismissMs = 6000,
    stickyTones = ["danger"],
    placement = "bottom-end",
    ariaLabel = "Notifications",
    size = null,
    sizeRole = "chrome",
    density = null,
    onAction = null,
    onDismiss = null,
  }: {
    store: ToastHostStore;
    autoDismissMs?: number;
    stickyTones?: ToastItem["tone"][];
    placement?: ToastHostPlacement;
    ariaLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onAction?: ((id: string) => void) | null;
    onDismiss?: ((id: string) => void) | null;
  } = $props();

  let items = $state<ToastItem[]>([]);

  const timers = new Map<string, ReturnType<typeof setTimeout>>();

  function clearTimer(id: string) {
    const timer = timers.get(id);
    if (!timer) return;
    clearTimeout(timer);
    timers.delete(id);
  }

  function reconcileTimers(next: ToastHostStoreItem[]) {
    const plan = reconcileToastTimers([...timers.keys()], next, {
      autoDismissMs,
      // ToastItem["tone"] is optional; undefined entries can never match a tone
      stickyTones: stickyTones.filter((tone): tone is NonNullable<typeof tone> => tone != null),
    });

    for (const id of plan.clear) {
      clearTimer(id);
    }

    for (const id of plan.start) {
      const timer = setTimeout(() => {
        store.dismiss(id);
        timers.delete(id);
      }, autoDismissMs);

      timers.set(id, timer);
    }
  }

  function handleDismiss(id: string) {
    clearTimer(id);
    store.dismiss(id);
    onDismiss?.(id);
  }

  function handleAction(id: string) {
    onAction?.(id);
  }

  $effect(() => {
    const unsubscribe = (store.toasts as Readable<ToastHostStoreItem[]>).subscribe((next) => {
      items = next.map(normalizeToast);
      reconcileTimers(next);
    });

    return () => {
      unsubscribe();
    };
  });

  $effect(() => {
    return () => {
      for (const timer of timers.values()) clearTimeout(timer);
      timers.clear();
    };
  });
</script>

{#if items.length > 0}
  <div class="poodle-toast-host" data-placement={placement}>
    <ToastStack
      {items}
      {ariaLabel}
      {size}
      {sizeRole}
      {density}
      onDismiss={handleDismiss}
      onAction={handleAction}
    />
  </div>
{/if}


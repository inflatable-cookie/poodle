<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/toast-host.css";
  import { normalizeToast, reconcileToastTimers, uniqueToastInputs } from "@inflatable-cookie/poodle-core";
  import type { Readable } from "svelte/store";
  import { untrack } from "svelte";

  import { default as ToastStack } from "./ToastStack.svelte";
  import type {
    ToastHostPlacement,
    ToastHostStore,
    ToastHostStoreItem,
    ToastItem,
  } from "./types";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  const DEFAULT_STICKY_TONES: NonNullable<ToastItem["tone"]>[] = ["danger"];

  let {
    store,
    autoDismissMs = 6000,
    stickyTones = DEFAULT_STICKY_TONES,
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
  let rawItems = $state<ToastHostStoreItem[]>([]);

  const timers = new Map<string, ReturnType<typeof setTimeout>>();

  function clearTimer(id: string) {
    const timer = timers.get(id);
    if (!timer) return;
    clearTimeout(timer);
    timers.delete(id);
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
      rawItems = next;
    });

    return () => {
      unsubscribe();
    };
  });

  $effect(() => {
    const delay = autoDismissMs;
    const tones = stickyTones;
    const snapshot = rawItems;
    untrack(() => {
      const unique = uniqueToastInputs(snapshot);
      items = unique.map(normalizeToast);
      const plan = reconcileToastTimers([...timers.keys()], unique, {
        autoDismissMs: delay,
        stickyTones: tones.filter((tone): tone is NonNullable<typeof tone> => tone != null),
      });

      for (const id of plan.clear) {
        clearTimer(id);
      }

      for (const id of plan.start) {
        const timer = setTimeout(() => {
          store.dismiss(id);
          timers.delete(id);
        }, plan.delayMs);

        timers.set(id, timer);
      }
    });
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


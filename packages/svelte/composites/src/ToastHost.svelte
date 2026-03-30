<script lang="ts">
  import { onDestroy, onMount, createEventDispatcher } from "svelte";
  import type { Readable } from "svelte/store";

  import ToastStack from "./ToastStack.svelte";
  import type {
    ToastHostPlacement,
    ToastHostStore,
    ToastHostStoreItem,
    ToastItem,
  } from "./types";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  export let store: ToastHostStore;
  export let autoDismissMs = 6000;
  export let stickyTones: ToastItem["tone"][] = ["danger"];
  export let placement: ToastHostPlacement = "bottom-end";
  export let ariaLabel = "Notifications";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let onAction: ((id: string) => void) | null = null;

  const dispatch = createEventDispatcher<{
    dismiss: { id: string };
    action: { id: string };
  }>();

  let items: ToastItem[] = [];

  const timers = new Map<string, ReturnType<typeof setTimeout>>();

  function resolveTone(toast: ToastHostStoreItem): ToastItem["tone"] {
    if (toast.tone) return toast.tone;
    if (toast.variant === "error") return "danger";
    if (toast.variant === "warning") return "warning";
    if (toast.variant === "success") return "success";
    return "info";
  }

  function normalizeToast(toast: ToastHostStoreItem): ToastItem {
    const title = toast.title?.trim() || toast.message || "Notification";
    const message = toast.title?.trim() ? toast.message ?? null : null;

    return {
      id: toast.id,
      title,
      message,
      tone: resolveTone(toast),
      actionLabel: toast.actionLabel ?? null,
    };
  }

  function isSticky(toast: ToastHostStoreItem): boolean {
    if (toast.sticky === true) return true;
    return stickyTones.includes(resolveTone(toast));
  }

  function clearTimer(id: string) {
    const timer = timers.get(id);
    if (!timer) return;
    clearTimeout(timer);
    timers.delete(id);
  }

  function reconcileTimers(next: ToastHostStoreItem[]) {
    const ids = new Set(next.map((toast) => toast.id));

    for (const id of timers.keys()) {
      if (!ids.has(id)) clearTimer(id);
    }

    for (const toast of next) {
      if (isSticky(toast)) continue;
      if (autoDismissMs <= 0) continue;
      if (timers.has(toast.id)) continue;

      const timer = setTimeout(() => {
        store.dismiss(toast.id);
        timers.delete(toast.id);
      }, autoDismissMs);

      timers.set(toast.id, timer);
    }
  }

  function handleDismiss(id: string) {
    clearTimer(id);
    store.dismiss(id);
    dispatch("dismiss", { id });
  }

  function handleAction(id: string) {
    onAction?.(id);
    dispatch("action", { id });
  }

  onMount(() => {
    const unsubscribe = (store.toasts as Readable<ToastHostStoreItem[]>).subscribe((next) => {
      items = next.map(normalizeToast);
      reconcileTimers(next);
    });

    return () => {
      unsubscribe();
    };
  });

  onDestroy(() => {
    for (const timer of timers.values()) clearTimeout(timer);
    timers.clear();
  });
</script>

{#if items.length > 0}
  <div class="toast-host" data-placement={placement}>
    <ToastStack
      {items}
      {ariaLabel}
      {size}
      {sizeRole}
      {density}
      on:dismiss={(event) => handleDismiss(event.detail.id)}
      on:action={(event) => handleAction(event.detail.id)}
    />
  </div>
{/if}

<style>
  .toast-host {
    position: fixed;
    z-index: 80;
    width: min(28rem, calc(100vw - 2rem));
  }

  .toast-host[data-placement="bottom-end"] {
    right: 1rem;
    bottom: 1rem;
  }

  .toast-host[data-placement="bottom-start"] {
    left: 1rem;
    bottom: 1rem;
  }

  .toast-host[data-placement="top-end"] {
    right: 1rem;
    top: 1rem;
  }

  .toast-host[data-placement="top-start"] {
    left: 1rem;
    top: 1rem;
  }

  @media (max-width: 40rem) {
    .toast-host {
      width: calc(100vw - 1rem);
    }

    .toast-host[data-placement="bottom-end"],
    .toast-host[data-placement="bottom-start"] {
      left: 0.5rem;
      right: 0.5rem;
      bottom: 0.5rem;
      width: auto;
    }

    .toast-host[data-placement="top-end"],
    .toast-host[data-placement="top-start"] {
      left: 0.5rem;
      right: 0.5rem;
      top: 0.5rem;
      width: auto;
    }
  }
</style>

<script lang="ts">
  import type { Readable } from "svelte/store";

  import ToastStack from "./ToastStack.svelte";
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

<style>
  .poodle-toast-host {
    position: fixed;
    z-index: 80;
    width: min(28rem, calc(100vw - 2rem));
  }

  .poodle-toast-host[data-placement="bottom-end"] {
    right: 1rem;
    bottom: 1rem;
  }

  .poodle-toast-host[data-placement="bottom-start"] {
    left: 1rem;
    bottom: 1rem;
  }

  .poodle-toast-host[data-placement="top-end"] {
    right: 1rem;
    top: 1rem;
  }

  .poodle-toast-host[data-placement="top-start"] {
    left: 1rem;
    top: 1rem;
  }

  @media (max-width: 40rem) {
    .poodle-toast-host {
      width: calc(100vw - 1rem);
    }

    .poodle-toast-host[data-placement="bottom-end"],
    .poodle-toast-host[data-placement="bottom-start"] {
      left: 0.5rem;
      right: 0.5rem;
      bottom: 0.5rem;
      width: auto;
    }

    .poodle-toast-host[data-placement="top-end"],
    .poodle-toast-host[data-placement="top-start"] {
      left: 0.5rem;
      right: 0.5rem;
      top: 0.5rem;
      width: auto;
    }
  }
</style>

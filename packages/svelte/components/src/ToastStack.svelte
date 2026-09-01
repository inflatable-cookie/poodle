<script module lang="ts">
  let nextToastStackId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/toast-stack.css";
  import {
    applyToastExitInert,
    cancelToastPresence,
    dropToastVisual,
    moveToastFocus,
    nextToastVisuals,
    playToastPresence,
    settleToastVisual,
    type ToastVisual,
  } from "@inflatable-cookie/poodle-core";
  import { default as Button } from "./Button.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { onDestroy, untrack } from "svelte";
  import { getMotionPolicy } from "./motion-policy";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { MotionPolicy } from "@inflatable-cookie/poodle-core";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ToastItem } from "./types";

  const EMPTY_TOAST_ITEMS: ToastItem[] = [];

  let {
    items = EMPTY_TOAST_ITEMS,
    ariaLabel = "Notifications",
    size = null,
    sizeRole = "chrome",
    density = null,
    onDismiss = undefined,
    onAction = undefined,
  }: {
    items?: ToastItem[];
    ariaLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onDismiss?: ((id: string) => void) | undefined;
    onAction?: ((id: string) => void) | undefined;
  } = $props();

  const uiPresentation = getUiPresentation();
  const motionPolicy = getMotionPolicy();
  const stackId = `toast-stack-${++nextToastStackId}`;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  let visuals = $state<ToastVisual[]>([]);
  let initialPass = true;
  let mounted = true;
  let stackElement: HTMLUListElement | null = null;
  let enteredFrom: Element | null = null;
  let retainedItems = $state(new Map<string, ToastItem>());
  const rows = $derived(
    visuals.flatMap((visual) => {
      const item = retainedItems.get(visual.id);
      return item ? [{ visual, item }] : [];
    }),
  );

  $effect(() => {
    const liveItems = items;
    const liveIds = liveItems.map((item) => item.id);
    untrack(() => {
      const currentVisuals = visuals;
      const currentRetained = retainedItems;
      const nextRetained = new Map(currentRetained);
      for (const item of liveItems) {
        nextRetained.set(item.id, item);
      }
      const nextVisuals = nextToastVisuals(currentVisuals, liveIds, initialPass);
      initialPass = false;
      let retainedChanged = nextRetained.size !== currentRetained.size;
      if (!retainedChanged) {
        for (const [id, item] of nextRetained) {
          if (currentRetained.get(id) !== item) {
            retainedChanged = true;
            break;
          }
        }
      }
      const visualsUnchanged =
        nextVisuals.length === currentVisuals.length &&
        nextVisuals.every(
          (visual, index) =>
            visual.id === currentVisuals[index]?.id && visual.phase === currentVisuals[index]?.phase,
        );
      if (retainedChanged) {
        retainedItems = nextRetained;
      }
      if (!visualsUnchanged) {
        visuals = nextVisuals;
      }
    });
  });

  onDestroy(() => {
    mounted = false;
    for (const visual of visuals) {
      cancelToastPresence(`${stackId}:${visual.id}`);
    }
  });

  function prune(id: string) {
    if (!mounted) {
      return;
    }
    visuals = dropToastVisual(visuals, id);
    const nextRetained = new Map(retainedItems);
    nextRetained.delete(id);
    retainedItems = nextRetained;
  }

  type ToastActionState = { visual: ToastVisual; policy: MotionPolicy };

  function toastElement(node: HTMLElement, initial: ToastActionState) {
    const ownerFor = (id: string) => `${stackId}:${id}`;
    let current = initial;

    function play(next: ToastActionState) {
      const ownerChanged = current.visual.id !== next.visual.id;
      const policyChanged = current.policy !== next.policy;
      if (ownerChanged || policyChanged) {
        cancelToastPresence(ownerFor(current.visual.id));
      }
      current = next;

      const { visual } = next;
      const exiting = visual.phase === "exit";
      applyToastExitInert(node, exiting);
      if (visual.phase === "settled") {
        return;
      }
      playToastPresence(node, {
        owner: ownerFor(visual.id),
        phase: exiting ? "exit" : "enter",
        policy: next.policy,
        initial: false,
        onComplete: (status) => {
          if (!mounted || status !== "finish") {
            return;
          }
          if (exiting) {
            prune(visual.id);
          } else {
            visuals = settleToastVisual(visuals, visual.id);
          }
        },
      });
    }
    play(initial);
    return {
      update(next: ToastActionState) {
        play(next);
      },
      destroy() {
        cancelToastPresence(ownerFor(current.visual.id));
      },
    };
  }

  function handleDismiss(id: string, toastEl: HTMLElement | null, activator: EventTarget | null) {
    if (stackElement && toastEl) {
      moveToastFocus(stackElement, toastEl, enteredFrom, activator);
      applyToastExitInert(toastEl, true);
    }
    onDismiss?.(id);
  }
</script>

<ul
  bind:this={stackElement}
  class="poodle-toast-stack"
  aria-label={ariaLabel}
  aria-live="polite"
  aria-atomic="false"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onfocusin={(event) => {
    if (!stackElement?.contains(event.relatedTarget as Node | null)) {
      enteredFrom = event.relatedTarget as Element | null;
    }
  }}
>
  {#each rows as { visual, item } (visual.id)}
    <li
      class="poodle-toast"
      data-tone={item.tone ?? "info"}
      data-motion={visual.phase}
      data-motion-inert={visual.phase === "exit" ? "true" : undefined}
      inert={visual.phase === "exit" ? true : undefined}
      aria-live={visual.phase === "exit" ? undefined : item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
      aria-hidden={visual.phase === "exit" ? "true" : undefined}
      use:toastElement={{ visual, policy: $motionPolicy }}
    >
      <button
        type="button"
        class="poodle-toast__dismiss"
        aria-label={`Dismiss ${item.title}`}
        tabindex={visual.phase === "exit" ? -1 : undefined}
        onclick={(event) => handleDismiss(item.id, event.currentTarget.closest(".poodle-toast"), event.currentTarget)}
      >
        <Icon name="x" />
      </button>

      <div class="poodle-toast__copy">
        <strong>{item.title}</strong>
        {#if item.message}
          <p>{item.message}</p>
        {/if}
      </div>

      {#if item.actionLabel}
        <div class="poodle-toast__actions">
          <Button variant="secondary" size={resolvedSize} density={resolvedDensity} onClick={() => onAction?.(item.id)}>
            {item.actionLabel}
          </Button>
        </div>
      {/if}
    </li>
  {/each}
</ul>

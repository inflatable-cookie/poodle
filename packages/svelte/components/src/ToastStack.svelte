<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/toast-stack.css";
  import {
    dropToastVisual,
    moveToastFocus,
    nextToastVisuals,
    playToastPresence,
    settleToastVisual,
    type ToastVisual,
  } from "@inflatable-cookie/poodle-core";
  import { default as Button } from "./Button.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { untrack } from "svelte";
  import { getMotionPolicy } from "./motion-policy";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  import type { ToastItem } from "./types";

  let {
    items = [],
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

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  let visuals = $state<ToastVisual[]>([]);
  let initialPass = true;
  let stackElement: HTMLUListElement | null = null;
  let enteredFrom: Element | null = null;
  let retainedItems = $state(new Map<string, ToastItem>());

  $effect(() => {
    const liveIds = items.map((item) => item.id);
    const currentVisuals = untrack(() => visuals);
    const currentRetained = untrack(() => retainedItems);
    const nextRetained = new Map(currentRetained);
    for (const item of items) {
      nextRetained.set(item.id, item);
    }
    retainedItems = nextRetained;
    visuals = nextToastVisuals(currentVisuals, liveIds, initialPass);
    initialPass = false;
  });

  function toastElement(node: HTMLElement, visual: ToastVisual) {
    function play(next: ToastVisual) {
      playToastPresence(node, {
        owner: next.id,
        phase: next.phase === "exit" ? "exit" : "enter",
        policy: $motionPolicy,
        initial: next.phase === "settled",
      });
    }
    play(visual);
    return {
      update(next: ToastVisual) {
        play(next);
      },
      destroy() {},
    };
  }

  function handleDismiss(id: string, toastEl: HTMLElement | null) {
    if (stackElement && toastEl) {
      moveToastFocus(stackElement, toastEl, enteredFrom);
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
  {#each visuals as visual (visual.id)}
    {@const item = retainedItems.get(visual.id)}
    {#if item}
    <li
      class="poodle-toast"
      data-tone={item.tone ?? "info"}
      data-motion={visual.phase}
      data-motion-inert={visual.phase === "exit" ? "true" : undefined}
      aria-live={visual.phase === "exit" ? undefined : item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
      aria-hidden={visual.phase === "exit" ? "true" : undefined}
      use:toastElement={visual}
      onanimationend={() => {
        if (visual.phase === "exit") {
          visuals = dropToastVisual(visuals, visual.id);
        } else if (visual.phase === "enter") {
          visuals = settleToastVisual(visuals, visual.id);
        }
      }}
    >
      <button type="button" class="poodle-toast__dismiss" aria-label={`Dismiss ${item.title}`} onclick={(event) => handleDismiss(item.id, event.currentTarget.closest(".poodle-toast"))}>
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
    {/if}
  {/each}
</ul>


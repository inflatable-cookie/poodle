<script module lang="ts">
  let nextCollapsibleId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/collapsible.css";
  import { disclosureTransition } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { useMotionReady } from "./motion-ready.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface TriggerSnippetProps {
    isOpen: boolean;
  }

  interface Props {
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    title?: string | null;
    description?: string | null;
    disabled?: boolean;
    highlighted?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | undefined;
    trigger?: Snippet<[TriggerSnippetProps]>;
    children?: Snippet;
  }

  let {
    open = undefined,
    defaultOpen = false,
    title = null,
    description = null,
    disabled = false,
    highlighted = false,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onOpenChange = undefined,
    trigger,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const motionReady = useMotionReady();
  const collapsibleId = ++nextCollapsibleId;
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);
  let closing = $state(false);
  let previousOpen = false;

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== undefined);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  $effect.pre(() => {
    if (seededDefaultOpen) {
      return;
    }

    uncontrolledOpen = defaultOpen;
    seededDefaultOpen = true;
    previousOpen = defaultOpen;
  });

  $effect.pre(() => {
    const open = isOpen;
    if (open) {
      closing = false;
    } else if (previousOpen && motionReady.ready) {
      closing = true;
    } else {
      closing = false;
    }
    previousOpen = open;
  });

  function toggle(): void {
    const result = disclosureTransition({ open: isOpen, disabled }, { type: "TOGGLE" });

    for (const effect of result.effects) {
      if (effect.type === "emitOpenChange") {
        if (!isControlled) {
          uncontrolledOpen = effect.open;
        }

        onOpenChange?.(effect.open);
      }
    }
  }
</script>

<section
  class="poodle-collapsible"
  data-open={isOpen}
  data-disabled={disabled}
  data-highlighted={highlighted}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-motion-ready={motionReady.ready}
>
  <button
    type="button"
    class="poodle-collapsible__trigger"
    id={`poodle-collapsible-trigger-${collapsibleId}`}
    disabled={disabled}
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={`poodle-collapsible-content-${collapsibleId}`}
    aria-label={title ? undefined : ariaLabel ?? undefined}
    onclick={toggle}
  >
    <span class="poodle-collapsible__heading">
      {#if trigger}
        {@render trigger({ isOpen })}
      {:else}
        {#if title}
          <span class="poodle-collapsible__title">{title}</span>
        {/if}

        {#if description}
          <span class="poodle-collapsible__description">{description}</span>
        {/if}
      {/if}
    </span>
    <span class="poodle-collapsible__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
  </button>

  <div
    class="poodle-collapsible__content-clip"
    ontransitionend={() => {
      if (!isOpen) closing = false;
    }}
  >
    <div
      class="poodle-collapsible__content"
      id={`poodle-collapsible-content-${collapsibleId}`}
      role="region"
      aria-labelledby={`poodle-collapsible-trigger-${collapsibleId}`}
      hidden={!isOpen && !closing}
      inert={!isOpen}
      aria-hidden={!isOpen ? "true" : undefined}
    >
      {@render children?.()}
    </div>
  </div>
</section>


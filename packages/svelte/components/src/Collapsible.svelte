<script module lang="ts">
  let nextCollapsibleId = 0;
</script>

<script lang="ts">
  import { disclosureTransition } from "@poodle/headless";
  import type { Snippet } from "svelte";
  import { slide } from "svelte/transition";

  import { default as Icon } from "./Icon.svelte";
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
  const collapsibleId = ++nextCollapsibleId;
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);

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

  {#if isOpen}
    <div
      class="poodle-collapsible__content"
      id={`poodle-collapsible-content-${collapsibleId}`}
      role="region"
      aria-labelledby={`poodle-collapsible-trigger-${collapsibleId}`}
      transition:slide={{ duration: 180 }}
    >
      {@render children?.()}
    </div>
  {/if}
</section>

<style>
  .poodle-collapsible {
    --poodle-collapsible-fill: var(--poodle-recipe-collapsible-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 40%, var(--poodle-color-background-panel)));
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
    padding: 0.625rem var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 36%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-collapsible-fill);
    --poodle-surface: var(--poodle-collapsible-fill);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent);
  }

  .poodle-collapsible[data-open="false"] {
    gap: 0;
  }

  .poodle-collapsible[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-collapsible[data-highlighted="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 55%, transparent);
    box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .poodle-collapsible__trigger {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    text-align: left;
    font: inherit;
  }

  .poodle-collapsible__trigger:disabled {
    cursor: not-allowed;
  }

  .poodle-collapsible__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
  }

  .poodle-collapsible__heading {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .poodle-collapsible__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .poodle-collapsible__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .poodle-collapsible__indicator {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    transition: transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-collapsible[data-open="true"] .poodle-collapsible__indicator {
    transform: rotate(180deg);
  }

  .poodle-collapsible__content {
    min-width: 0;
    padding-top: 0.125rem;
  }

  .poodle-collapsible[data-size="xs"] .poodle-collapsible__title {
    font-size: 0.8125rem;
  }

  .poodle-collapsible[data-size="xs"] .poodle-collapsible__description {
    font-size: 0.6875rem;
  }

  .poodle-collapsible[data-size="sm"] .poodle-collapsible__title {
    font-size: 0.875rem;
  }

  .poodle-collapsible[data-size="sm"] .poodle-collapsible__description {
    font-size: 0.75rem;
  }

  .poodle-collapsible[data-size="lg"] .poodle-collapsible__title {
    font-size: 1.0625rem;
  }

  .poodle-collapsible[data-size="lg"] .poodle-collapsible__description {
    font-size: 0.875rem;
  }

  .poodle-collapsible[data-size="xl"] .poodle-collapsible__title {
    font-size: 1.125rem;
  }

  .poodle-collapsible[data-size="xl"] .poodle-collapsible__description {
    font-size: 0.9375rem;
  }

  .poodle-collapsible[data-density="compact"] { padding-inline: 0.5rem; }
  .poodle-collapsible[data-density="comfortable"] { padding-inline: 1rem; }
</style>

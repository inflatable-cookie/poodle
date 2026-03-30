<script context="module" lang="ts">
  let nextCollapsibleId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { slide } from "svelte/transition";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let title: string | null = null;
  export let description: string | null = null;
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const uiPresentation = getUiPresentation();
  const collapsibleId = ++nextCollapsibleId;
  let uncontrolledOpen = defaultOpen;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }
</script>

<section class="collapsible" data-open={isOpen} data-disabled={disabled} data-size={resolvedSize} data-density={resolvedDensity}>
  <button
    type="button"
    class="collapsible__trigger"
    id={`poodle-collapsible-trigger-${collapsibleId}`}
    disabled={disabled}
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={`poodle-collapsible-content-${collapsibleId}`}
    aria-label={title ? undefined : ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span class="collapsible__heading">
      {#if $$slots.trigger}
        <slot name="trigger" {isOpen} />
      {:else}
        {#if title}
          <span class="collapsible__title">{title}</span>
        {/if}

        {#if description}
          <span class="collapsible__description">{description}</span>
        {/if}
      {/if}
    </span>
    <span class="collapsible__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
  </button>

  {#if isOpen}
    <div
      class="collapsible__content"
      id={`poodle-collapsible-content-${collapsibleId}`}
      role="region"
      aria-labelledby={`poodle-collapsible-trigger-${collapsibleId}`}
      transition:slide={{ duration: 180 }}
    >
      <slot />
    </div>
  {/if}
</section>

<style>
  .collapsible {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary));
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent);
  }

  .collapsible[data-open="false"] {
    gap: 0;
  }

  .collapsible[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .collapsible__trigger {
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

  .collapsible__trigger:disabled {
    cursor: not-allowed;
  }

  .collapsible__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
  }

  .collapsible__heading {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .collapsible__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .collapsible__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .collapsible__indicator {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    transition: transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .collapsible[data-open="true"] .collapsible__indicator {
    transform: rotate(180deg);
  }

  .collapsible__content {
    min-width: 0;
    padding-top: 0.125rem;
  }

  /* Size variants */
  .collapsible[data-size="xs"] .collapsible__title {
    font-size: 0.8125rem;
  }

  .collapsible[data-size="xs"] .collapsible__description {
    font-size: 0.6875rem;
  }

  .collapsible[data-size="sm"] .collapsible__title {
    font-size: 0.875rem;
  }

  .collapsible[data-size="sm"] .collapsible__description {
    font-size: 0.75rem;
  }

  .collapsible[data-size="lg"] .collapsible__title {
    font-size: 1.0625rem;
  }

  .collapsible[data-size="lg"] .collapsible__description {
    font-size: 0.875rem;
  }

  .collapsible[data-size="xl"] .collapsible__title {
    font-size: 1.125rem;
  }

  .collapsible[data-size="xl"] .collapsible__description {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .collapsible[data-density="compact"] { padding: 0.375rem 0.5rem; }
  .collapsible[data-density="comfortable"] { padding: 0.75rem 1rem; }
</style>

<script context="module" lang="ts">
  let nextAccordionId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { slide } from "svelte/transition";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { AccordionItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let items: AccordionItem[] = [];
  export let value: string | string[] | null = null;
  export let defaultValue: string | string[] | null = null;
  export let selectionMode: "single" | "multiple" = "single";
  export let collapsible = true;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | string[] | null };
  }>();

  const uiPresentation = getUiPresentation();
  const accordionId = ++nextAccordionId;
  let uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;
  $: openValues = Array.isArray(currentValue)
    ? currentValue
    : currentValue
      ? [currentValue]
      : [];

  function toggle(itemValue: string): void {
    const currentlyOpen = openValues.includes(itemValue);
    let nextValue: string | string[] | null;

    if (selectionMode === "multiple") {
      nextValue = currentlyOpen
        ? openValues.filter((valueItem) => valueItem !== itemValue)
        : [...openValues, itemValue];
    } else if (currentlyOpen) {
      nextValue = collapsible ? null : itemValue;
    } else {
      nextValue = itemValue;
    }

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<div
  class="poodle-accordion"
  role={selectionMode === "multiple" ? "group" : undefined}
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#each items as item (item.value)}
    <section class="poodle-accordion__item" data-open={openValues.includes(item.value)}>
      <h3 class="poodle-accordion__heading">
        <button
          type="button"
          class="poodle-accordion__trigger"
          id={`poodle-accordion-trigger-${accordionId}-${item.value}`}
          disabled={item.disabled === true}
          aria-expanded={openValues.includes(item.value) ? "true" : "false"}
          aria-controls={`poodle-accordion-panel-${accordionId}-${item.value}`}
          on:click={() => toggle(item.value)}
        >
          <span class="poodle-accordion__summary">
            <span class="poodle-accordion__title">{item.label}</span>
            {#if item.description}
              <span class="poodle-accordion__description">{item.description}</span>
            {/if}
          </span>
          <span class="poodle-accordion__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
        </button>
      </h3>

      {#if openValues.includes(item.value)}
        <div
          class="poodle-accordion__panel"
          id={`poodle-accordion-panel-${accordionId}-${item.value}`}
          role="region"
          aria-labelledby={`poodle-accordion-trigger-${accordionId}-${item.value}`}
          transition:slide={{ duration: 180 }}
        >
          <slot item={item} isOpen={true} />
        </div>
      {/if}
    </section>
  {/each}
</div>

<style>
  .poodle-accordion {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
  }

  .poodle-accordion__item {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
    padding: 0.625rem var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 36%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary));
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent);
  }

  .poodle-accordion__heading {
    margin: 0;
  }

  .poodle-accordion__trigger {
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

  .poodle-accordion__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-accordion__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
  }

  .poodle-accordion__summary {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .poodle-accordion__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .poodle-accordion__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .poodle-accordion__indicator {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    transition: transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-accordion__item[data-open="true"] .poodle-accordion__indicator {
    transform: rotate(180deg);
  }

  .poodle-accordion__panel {
    min-width: 0;
  }

  /* Size variants */
  .poodle-accordion[data-size="xs"] .poodle-accordion__title {
    font-size: 0.8125rem;
  }

  .poodle-accordion[data-size="xs"] .poodle-accordion__description {
    font-size: 0.6875rem;
  }

  .poodle-accordion[data-size="sm"] .poodle-accordion__title {
    font-size: 0.875rem;
  }

  .poodle-accordion[data-size="sm"] .poodle-accordion__description {
    font-size: 0.75rem;
  }

  .poodle-accordion[data-size="lg"] .poodle-accordion__title {
    font-size: 1.0625rem;
  }

  .poodle-accordion[data-size="lg"] .poodle-accordion__description {
    font-size: 0.875rem;
  }

  .poodle-accordion[data-size="xl"] .poodle-accordion__title {
    font-size: 1.125rem;
  }

  .poodle-accordion[data-size="xl"] .poodle-accordion__description {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .poodle-accordion[data-density="compact"] .poodle-accordion__item { padding-inline: 0.5rem; }
  .poodle-accordion[data-density="comfortable"] .poodle-accordion__item { padding-inline: 1rem; }
</style>

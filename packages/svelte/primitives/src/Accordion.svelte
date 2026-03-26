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

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
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
  class="accordion"
  role={selectionMode === "multiple" ? "group" : undefined}
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#each items as item (item.value)}
    <section class="accordion__item" data-open={openValues.includes(item.value)}>
      <h3 class="accordion__heading">
        <button
          type="button"
          class="accordion__trigger"
          id={`poodle-accordion-trigger-${accordionId}-${item.value}`}
          disabled={item.disabled === true}
          aria-expanded={openValues.includes(item.value) ? "true" : "false"}
          aria-controls={`poodle-accordion-panel-${accordionId}-${item.value}`}
          on:click={() => toggle(item.value)}
        >
          <span class="accordion__summary">
            <span class="accordion__title">{item.label}</span>
            {#if item.description}
              <span class="accordion__description">{item.description}</span>
            {/if}
          </span>
          <span class="accordion__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
        </button>
      </h3>

      {#if openValues.includes(item.value)}
        <div
          class="accordion__panel"
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
  .accordion {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
  }

  .accordion__item {
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 36%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary));
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 8%, transparent);
  }

  .accordion__heading {
    margin: 0;
  }

  .accordion__trigger {
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

  .accordion__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .accordion__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
  }

  .accordion__summary {
    display: grid;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .accordion__title {
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .accordion__description {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .accordion__indicator {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    transition: transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .accordion__item[data-open="true"] .accordion__indicator {
    transform: rotate(180deg);
  }

  .accordion__panel {
    min-width: 0;
  }

  /* Size variants */
  .accordion[data-size="xs"] .accordion__title {
    font-size: 0.8125rem;
  }

  .accordion[data-size="xs"] .accordion__description {
    font-size: 0.6875rem;
  }

  .accordion[data-size="sm"] .accordion__title {
    font-size: 0.875rem;
  }

  .accordion[data-size="sm"] .accordion__description {
    font-size: 0.75rem;
  }

  .accordion[data-size="lg"] .accordion__title {
    font-size: 1.0625rem;
  }

  .accordion[data-size="lg"] .accordion__description {
    font-size: 0.875rem;
  }

  .accordion[data-size="xl"] .accordion__title {
    font-size: 1.125rem;
  }

  .accordion[data-size="xl"] .accordion__description {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .accordion[data-density="compact"] .accordion__item { padding: 0.375rem 0.5rem; }
  .accordion[data-density="comfortable"] .accordion__item { padding: 0.75rem 1rem; }
</style>

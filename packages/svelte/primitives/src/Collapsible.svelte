<script context="module" lang="ts">
  let nextCollapsibleId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";

  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let title: string | null = null;
  export let description: string | null = null;
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    openChange: { open: boolean };
  }>();

  const collapsibleId = ++nextCollapsibleId;
  let uncontrolledOpen = defaultOpen;

  $: isControlled = open !== null;
  $: isOpen = isControlled ? open === true : uncontrolledOpen;

  function setOpen(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }
</script>

<section class="collapsible" data-open={isOpen} data-disabled={isDisabled}>
  <button
    type="button"
    class="collapsible__trigger"
    id={`pug-collapsible-trigger-${collapsibleId}`}
    disabled={isDisabled}
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={`pug-collapsible-content-${collapsibleId}`}
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
    <span class="collapsible__indicator" aria-hidden="true"><Icon name="chevron-down" size="sm" /></span>
  </button>

  {#if isOpen}
    <div
      class="collapsible__content"
      id={`pug-collapsible-content-${collapsibleId}`}
      role="region"
      aria-labelledby={`pug-collapsible-trigger-${collapsibleId}`}
    >
      <slot />
    </div>
  {/if}
</section>

<style>
  .collapsible {
    display: grid;
    gap: 0.5rem;
    min-width: 0;
    padding: 0.875rem 1rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 42%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(
      in srgb,
      var(--pug-color-background-elevated) 88%,
      var(--pug-color-background-surface)
    );
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--pug-color-text-inverse) 8%, transparent);
  }

  .collapsible[data-open="false"] {
    gap: 0;
  }

  .collapsible[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
  }

  .collapsible__trigger {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--pug-color-text-primary);
    cursor: pointer;
    text-align: left;
    font: inherit;
  }

  .collapsible__trigger:disabled {
    cursor: not-allowed;
  }

  .collapsible__trigger:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
  }

  .collapsible__heading {
    display: grid;
    gap: 0.3125rem;
    min-width: 0;
  }

  .collapsible__title {
    font-family: var(--pug-typography-heading-family);
    font-size: 1rem;
    font-weight: 700;
    line-height: 1.2;
  }

  .collapsible__description {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .collapsible__indicator {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    transition: transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .collapsible[data-open="true"] .collapsible__indicator {
    transform: rotate(180deg);
  }

  .collapsible__content {
    min-width: 0;
    padding-top: 0.125rem;
  }
</style>

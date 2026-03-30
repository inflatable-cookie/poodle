<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Card, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  import type { CardRadioItem } from "./types";

  export let items: CardRadioItem[] = [];
  export let value: string | null = null;
  export let columns: 1 | 2 | 3 | 4 = 2;
  export let ariaLabel: string | null = null;
  export let disabled = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    change: { value: string };
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  function select(itemValue: string): void {
    if (disabled) return;
    const item = items.find((i) => i.value === itemValue);
    if (item?.disabled) return;
    value = itemValue;
    dispatch("change", { value: itemValue });
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const enabledItems = items.filter((i) => !i.disabled);
    const currentEnabledIndex = enabledItems.findIndex(
      (i) => i.value === items[index].value
    );

    let nextItem: CardRadioItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[(currentEnabledIndex + 1) % enabledItems.length];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem =
        enabledItems[
          (currentEnabledIndex - 1 + enabledItems.length) % enabledItems.length
        ];
    }

    if (nextItem) {
      select(nextItem.value);
      const nextIndex = items.findIndex((i) => i.value === nextItem!.value);
      const el = document.querySelector<HTMLElement>(
        `[data-card-radio-index="${nextIndex}"]`
      );
      el?.focus();
    }
  }
</script>

<div
  class="card-radio-group"
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  style="--columns: {columns}"
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#each items as item, index (item.value)}
    {@const isChecked = value === item.value}
    {@const isItemDisabled = disabled || item.disabled === true}
    <div
      class="card-radio-group__option"
      role="radio"
      tabindex={isItemDisabled ? -1 : isChecked || (value === null && index === 0) ? 0 : -1}
      aria-checked={isChecked ? "true" : "false"}
      aria-disabled={isItemDisabled ? "true" : undefined}
      data-card-radio-index={index}
      on:click={() => !isItemDisabled && select(item.value)}
      on:keydown={(e) => !isItemDisabled && handleKeydown(e, index)}
    >
      <Card
        interactive={!isItemDisabled}
        selected={isChecked}
        ariaLabel={item.label}
      >
        <svelte:fragment slot="header">
          <div class="card-radio-group__header">
            <span
              class="card-radio-group__indicator"
              data-checked={isChecked}
              data-disabled={isItemDisabled}
              aria-hidden="true"
            >
              {#if isChecked}
                <span class="card-radio-group__dot"></span>
              {/if}
            </span>
            <span class="card-radio-group__title" data-disabled={isItemDisabled}>
              {item.label}
            </span>
          </div>
        </svelte:fragment>

        {#if item.description}
          <p class="card-radio-group__description" data-disabled={isItemDisabled}>
            {item.description}
          </p>
        {/if}

        {#if $$slots.card}
          <slot name="card" {item} checked={isChecked} disabled={isItemDisabled} />
        {/if}
      </Card>
    </div>
  {/each}
</div>

<style>
  .card-radio-group {
    display: grid;
    grid-template-columns: repeat(var(--columns, 2), 1fr);
    gap: 0.75rem;
  }

  .card-radio-group__option {
    cursor: pointer;
    outline: none;
  }

  .card-radio-group__option:focus-visible :global(.card) {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .card-radio-group__option[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .card-radio-group__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .card-radio-group__indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.125rem;
    height: 1.125rem;
    flex-shrink: 0;
    border: 0.125rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: transparent;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .card-radio-group__indicator[data-checked="true"] {
    border-color: var(--poodle-color-accent-base);
    background: var(--poodle-color-accent-base);
  }

  .card-radio-group__dot {
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 999px;
    background: var(--poodle-color-text-inverse);
  }

  .card-radio-group__title {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .card-radio-group__description {
    margin: 0;
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .card-radio-group[data-size="xs"] .card-radio-group__indicator {
    width: 0.875rem;
    height: 0.875rem;
  }

  .card-radio-group[data-size="xs"] .card-radio-group__dot {
    width: 0.25rem;
    height: 0.25rem;
  }

  .card-radio-group[data-size="xs"] .card-radio-group__title {
    font-size: 0.75rem;
  }

  .card-radio-group[data-size="xs"] .card-radio-group__description {
    font-size: 0.6875rem;
  }

  .card-radio-group[data-size="sm"] .card-radio-group__indicator {
    width: 1rem;
    height: 1rem;
  }

  .card-radio-group[data-size="sm"] .card-radio-group__title {
    font-size: 0.8125rem;
  }

  .card-radio-group[data-size="sm"] .card-radio-group__description {
    font-size: 0.75rem;
  }

  .card-radio-group[data-size="lg"] .card-radio-group__indicator {
    width: 1.25rem;
    height: 1.25rem;
  }

  .card-radio-group[data-size="lg"] .card-radio-group__dot {
    width: 0.4375rem;
    height: 0.4375rem;
  }

  .card-radio-group[data-size="lg"] .card-radio-group__title {
    font-size: 1.0625rem;
  }

  .card-radio-group[data-size="lg"] .card-radio-group__description {
    font-size: 0.875rem;
  }

  .card-radio-group[data-size="xl"] .card-radio-group__indicator {
    width: 1.375rem;
    height: 1.375rem;
  }

  .card-radio-group[data-size="xl"] .card-radio-group__dot {
    width: 0.5rem;
    height: 0.5rem;
  }

  .card-radio-group[data-size="xl"] .card-radio-group__title {
    font-size: 1.125rem;
  }

  .card-radio-group[data-size="xl"] .card-radio-group__description {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .card-radio-group[data-density="compact"] {
    gap: 0.5rem;
  }
  .card-radio-group[data-density="compact"] .card-radio-group__option :global(.card) { padding: 0.5rem; }

  .card-radio-group[data-density="comfortable"] {
    gap: 1rem;
  }
  .card-radio-group[data-density="comfortable"] .card-radio-group__option :global(.card) { padding: 1rem; }
</style>

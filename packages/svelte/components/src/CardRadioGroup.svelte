<script lang="ts">
  import type { Snippet } from "svelte";

  import Card from "./Card.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  import type { CardRadioItem } from "./types";

  interface CardSnippetProps {
    item: CardRadioItem;
    checked: boolean;
    disabled: boolean;
  }

  interface Props {
    items?: CardRadioItem[];
    value?: string | null | undefined;
    columns?: 1 | 2 | 3 | 4;
    ariaLabel?: string | null;
    disabled?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | undefined;
    card?: Snippet<[CardSnippetProps]>;
  }

  let {
    items = [],
    value = $bindable<string | null | undefined>(undefined),
    columns = 2,
    ariaLabel = null,
    disabled = false,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    card,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledValue = $state<string | null>(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? value ?? null : uncontrolledValue);

  function select(itemValue: string): void {
    if (disabled) {
      return;
    }

    const item = items.find((candidate) => candidate.value === itemValue);
    if (item?.disabled) {
      return;
    }

    if (isControlled) {
      value = itemValue;
    } else {
      uncontrolledValue = itemValue;
    }

    onValueChange?.(itemValue);
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const enabledItems = items.filter((item) => !item.disabled);
    const currentEnabledIndex = enabledItems.findIndex((item) => item.value === items[index].value);

    let nextItem: CardRadioItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[(currentEnabledIndex + 1) % enabledItems.length];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[(currentEnabledIndex - 1 + enabledItems.length) % enabledItems.length];
    }

    if (!nextItem) {
      return;
    }

    select(nextItem.value);
    const nextIndex = items.findIndex((item) => item.value === nextItem.value);
    const element = document.querySelector<HTMLElement>(`[data-card-radio-index="${nextIndex}"]`);
    element?.focus();
  }
</script>

<div
  class="poodle-card-radio-group"
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  style={`--columns: ${columns}`}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#each items as item, index (item.value)}
    {@const isChecked = currentValue === item.value}
    {@const isItemDisabled = disabled || item.disabled === true}
    <div
      class="poodle-card-radio-group__option"
      role="radio"
      tabindex={isItemDisabled ? -1 : isChecked || (currentValue === null && index === 0) ? 0 : -1}
      aria-checked={isChecked ? "true" : "false"}
      aria-disabled={isItemDisabled ? "true" : undefined}
      data-card-radio-index={index}
      onclick={() => !isItemDisabled && select(item.value)}
      onkeydown={(event) => !isItemDisabled && handleKeydown(event, index)}
    >
      <Card interactive={!isItemDisabled} selected={isChecked} ariaLabel={item.label}>
        {#snippet header()}
          <div class="poodle-card-radio-group__header">
            <span
              class="poodle-card-radio-group__indicator"
              data-checked={isChecked}
              data-disabled={isItemDisabled}
              aria-hidden="true"
            >
              {#if isChecked}
                <span class="poodle-card-radio-group__dot"></span>
              {/if}
            </span>
            <span class="poodle-card-radio-group__title" data-disabled={isItemDisabled}>
              {item.label}
            </span>
          </div>
        {/snippet}

        {#if item.description}
          <p class="poodle-card-radio-group__description" data-disabled={isItemDisabled}>
            {item.description}
          </p>
        {/if}

        {@render card?.({ item, checked: isChecked, disabled: isItemDisabled })}
      </Card>
    </div>
  {/each}
</div>

<style>
  .poodle-card-radio-group {
    display: grid;
    grid-template-columns: repeat(var(--columns, 2), 1fr);
    gap: 0.75rem;
  }

  .poodle-card-radio-group__option {
    cursor: pointer;
    outline: none;
  }

  .poodle-card-radio-group__option:focus-visible :global(.poodle-card) {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-card-radio-group__option[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-card-radio-group__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-card-radio-group__indicator {
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

  .poodle-card-radio-group__indicator[data-checked="true"] {
    border-color: var(--poodle-color-accent-base);
    background: var(--poodle-color-accent-base);
  }

  .poodle-card-radio-group__dot {
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 999px;
    background: var(--poodle-color-text-inverse);
  }

  .poodle-card-radio-group__title {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .poodle-card-radio-group__description {
    margin: 0;
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-card-radio-group[data-size="xs"] .poodle-card-radio-group__indicator {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-card-radio-group[data-size="xs"] .poodle-card-radio-group__dot {
    width: 0.25rem;
    height: 0.25rem;
  }

  .poodle-card-radio-group[data-size="xs"] .poodle-card-radio-group__title {
    font-size: 0.75rem;
  }

  .poodle-card-radio-group[data-size="xs"] .poodle-card-radio-group__description {
    font-size: 0.6875rem;
  }

  .poodle-card-radio-group[data-size="sm"] .poodle-card-radio-group__indicator {
    width: 1rem;
    height: 1rem;
  }

  .poodle-card-radio-group[data-size="sm"] .poodle-card-radio-group__title {
    font-size: 0.8125rem;
  }

  .poodle-card-radio-group[data-size="sm"] .poodle-card-radio-group__description {
    font-size: 0.75rem;
  }

  .poodle-card-radio-group[data-size="lg"] .poodle-card-radio-group__indicator {
    width: 1.25rem;
    height: 1.25rem;
  }

  .poodle-card-radio-group[data-size="lg"] .poodle-card-radio-group__dot {
    width: 0.4375rem;
    height: 0.4375rem;
  }

  .poodle-card-radio-group[data-size="lg"] .poodle-card-radio-group__title {
    font-size: 1.0625rem;
  }

  .poodle-card-radio-group[data-size="lg"] .poodle-card-radio-group__description {
    font-size: 0.875rem;
  }

  .poodle-card-radio-group[data-size="xl"] .poodle-card-radio-group__indicator {
    width: 1.375rem;
    height: 1.375rem;
  }

  .poodle-card-radio-group[data-size="xl"] .poodle-card-radio-group__dot {
    width: 0.5rem;
    height: 0.5rem;
  }

  .poodle-card-radio-group[data-size="xl"] .poodle-card-radio-group__title {
    font-size: 1.125rem;
  }

  .poodle-card-radio-group[data-size="xl"] .poodle-card-radio-group__description {
    font-size: 0.9375rem;
  }

  .poodle-card-radio-group[data-density="compact"] {
    gap: 0.5rem;
  }

  .poodle-card-radio-group[data-density="compact"] .poodle-card-radio-group__option :global(.poodle-card) {
    padding-inline: 0.5rem;
  }
</style>

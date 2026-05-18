<script lang="ts">
  import type { Snippet } from "svelte";

  import { default as Card } from "./Card.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    CardToggleItem,
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface CardSnippetProps {
    item: CardToggleItem;
    selected: boolean;
    disabled: boolean;
  }

  interface Props {
    items?: CardToggleItem[];
    value?: string | null | undefined;
    defaultValue?: string | null;
    allowDeactivation?: boolean;
    columns?: 1 | 2 | 3 | 4;
    ariaLabel?: string | null;
    disabled?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | null) => void) | undefined;
    card?: Snippet<[CardSnippetProps]>;
  }

  let {
    items = [],
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    allowDeactivation = false,
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
  let seededDefaultValue = $state(false);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? value ?? null : uncontrolledValue);
  const firstEnabledIndex = $derived(items.findIndex((item) => !item.disabled));

  function select(itemValue: string): void {
    if (disabled) {
      return;
    }

    const item = items.find((candidate) => candidate.value === itemValue);
    if (item?.disabled) {
      return;
    }

    const nextValue = allowDeactivation && currentValue === itemValue ? null : itemValue;

    if (isControlled) {
      value = nextValue;
    } else {
      uncontrolledValue = nextValue;
    }

    onValueChange?.(nextValue);
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const enabledItems = items.filter((item) => !item.disabled);
    const currentEnabledIndex = enabledItems.findIndex((item) => item.value === items[index].value);

    let nextItem: CardToggleItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[(currentEnabledIndex + 1) % enabledItems.length];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[(currentEnabledIndex - 1 + enabledItems.length) % enabledItems.length];
    } else if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      select(items[index].value);
      return;
    }

    if (!nextItem) {
      return;
    }

    select(nextItem.value);
    const nextIndex = items.findIndex((item) => item.value === nextItem.value);
    const element = document.querySelector<HTMLElement>(`[data-card-toggle-index="${nextIndex}"]`);
    element?.focus();
  }
</script>

<div
  class="poodle-card-toggle-group"
  role="group"
  aria-label={ariaLabel ?? undefined}
  style={`--columns: ${columns}`}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#each items as item, index (item.value)}
    {@const isSelected = currentValue === item.value}
    {@const isItemDisabled = disabled || item.disabled === true}
    <div
      class="poodle-card-toggle-group__option"
      role="button"
      tabindex={isItemDisabled ? -1 : isSelected || (currentValue === null && index === firstEnabledIndex) ? 0 : -1}
      aria-pressed={isSelected ? "true" : "false"}
      aria-disabled={isItemDisabled ? "true" : undefined}
      data-card-toggle-index={index}
      onclick={() => !isItemDisabled && select(item.value)}
      onkeydown={(event) => !isItemDisabled && handleKeydown(event, index)}
    >
      <Card interactive={!isItemDisabled} selected={isSelected} density={resolvedDensity} ariaLabel={item.label}>
        {#snippet header()}
          <div class="poodle-card-toggle-group__header">
            <span
              class="poodle-card-toggle-group__indicator"
              data-selected={isSelected}
              data-disabled={isItemDisabled}
              aria-hidden="true"
            >
              {#if isSelected}
                <span class="poodle-card-toggle-group__check"></span>
              {/if}
            </span>
            <span class="poodle-card-toggle-group__title" data-disabled={isItemDisabled}>
              {item.label}
            </span>
            {#if item.count !== null && item.count !== undefined}
              <span class="poodle-card-toggle-group__count" data-disabled={isItemDisabled}>
                {item.count}
              </span>
            {/if}
          </div>
        {/snippet}

        {#if item.description}
          <p class="poodle-card-toggle-group__description" data-disabled={isItemDisabled}>
            {item.description}
          </p>
        {/if}

        {@render card?.({ item, selected: isSelected, disabled: isItemDisabled })}
      </Card>
    </div>
  {/each}
</div>

<style>
  .poodle-card-toggle-group {
    display: grid;
    grid-template-columns: repeat(var(--columns, 2), 1fr);
    gap: 0.75rem;
  }

  .poodle-card-toggle-group[data-density="compact"] {
    gap: 0.5rem;
  }

  .poodle-card-toggle-group[data-density="comfortable"] {
    gap: 1rem;
  }

  .poodle-card-toggle-group__option {
    cursor: pointer;
    outline: none;
  }

  .poodle-card-toggle-group__option:focus-visible :global(.poodle-card) {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-card-toggle-group__option[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-card-toggle-group__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-card-toggle-group__indicator {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.125rem;
    height: 1.125rem;
    flex-shrink: 0;
    border: 0.125rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-xs, 0.25rem);
    background: transparent;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-card-toggle-group__indicator[data-selected="true"] {
    border-color: var(--poodle-color-accent-base);
    background: var(--poodle-color-accent-base);
  }

  .poodle-card-toggle-group__check {
    width: 0.5rem;
    height: 0.3125rem;
    border-bottom: 0.125rem solid var(--poodle-color-text-inverse);
    border-left: 0.125rem solid var(--poodle-color-text-inverse);
    transform: translateY(-0.0625rem) rotate(-45deg);
  }

  .poodle-card-toggle-group__title {
    min-width: 0;
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
  }

  .poodle-card-toggle-group__count {
    margin-left: auto;
    flex-shrink: 0;
    padding: 0.0625rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: 999px;
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-weight: 700;
    line-height: 1.25;
  }

  .poodle-card-toggle-group__description {
    margin: 0;
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-card-toggle-group[data-size="xs"] .poodle-card-toggle-group__indicator {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-card-toggle-group[data-size="xs"] .poodle-card-toggle-group__check {
    width: 0.375rem;
    height: 0.25rem;
  }

  .poodle-card-toggle-group[data-size="xs"] .poodle-card-toggle-group__title {
    font-size: 0.75rem;
  }

  .poodle-card-toggle-group[data-size="xs"] .poodle-card-toggle-group__description {
    font-size: 0.6875rem;
  }

  .poodle-card-toggle-group[data-size="sm"] .poodle-card-toggle-group__indicator {
    width: 1rem;
    height: 1rem;
  }

  .poodle-card-toggle-group[data-size="sm"] .poodle-card-toggle-group__title {
    font-size: 0.8125rem;
  }

  .poodle-card-toggle-group[data-size="sm"] .poodle-card-toggle-group__description {
    font-size: 0.75rem;
  }

  .poodle-card-toggle-group[data-size="lg"] .poodle-card-toggle-group__indicator {
    width: 1.25rem;
    height: 1.25rem;
  }

  .poodle-card-toggle-group[data-size="lg"] .poodle-card-toggle-group__check {
    width: 0.5625rem;
    height: 0.375rem;
  }

  .poodle-card-toggle-group[data-size="lg"] .poodle-card-toggle-group__title {
    font-size: 1.0625rem;
  }

  .poodle-card-toggle-group[data-size="lg"] .poodle-card-toggle-group__description {
    font-size: 0.875rem;
  }

  .poodle-card-toggle-group[data-size="xl"] .poodle-card-toggle-group__indicator {
    width: 1.375rem;
    height: 1.375rem;
  }

  .poodle-card-toggle-group[data-size="xl"] .poodle-card-toggle-group__check {
    width: 0.625rem;
    height: 0.4375rem;
  }

  .poodle-card-toggle-group[data-size="xl"] .poodle-card-toggle-group__title {
    font-size: 1.125rem;
  }

  .poodle-card-toggle-group[data-size="xl"] .poodle-card-toggle-group__description {
    font-size: 0.9375rem;
  }
</style>

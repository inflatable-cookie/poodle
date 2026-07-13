<script lang="ts">
  import "@poodle/styles/card-toggle-group.css";
  import { menuListNavigate, toggleGroupTransition } from "@poodle/headless";
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
    const result = toggleGroupTransition(
      {
        value: currentValue,
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode: "single",
        allowDeactivation: allowDeactivation,
        disabled,
      },
      { type: "TOGGLE", value: itemValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        const nextValue = effect.value as string | null;

        if (isControlled) {
          value = nextValue;
        } else {
          uncontrolledValue = nextValue;
        }

        onValueChange?.(nextValue);
      }
    }
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const enabledItems = items.filter((item) => !item.disabled);
    const currentEnabledIndex = enabledItems.findIndex((item) => item.value === items[index].value);

    let nextItem: CardToggleItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "next")];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "prev")];
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


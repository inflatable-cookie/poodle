<script lang="ts">
  import "@poodle/styles/card-radio-group.css";
  import { menuListNavigate, toggleGroupTransition } from "@poodle/headless";
  import type { Snippet } from "svelte";

  import { default as Card } from "./Card.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types.ts";

  import type { CardRadioItem } from "./types.ts";

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
    const result = toggleGroupTransition(
      {
        value: currentValue,
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode: "single",
        allowDeactivation: false,
        disabled,
      },
      { type: "TOGGLE", value: itemValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (effect.value === null) continue;
        const nextValue = effect.value as string;

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

    let nextItem: CardRadioItem | undefined;

    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "next")];
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      nextItem = enabledItems[menuListNavigate(enabledItems, currentEnabledIndex, "prev")];
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
      <Card interactive={!isItemDisabled} selected={isChecked} density={resolvedDensity} ariaLabel={item.label}>
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


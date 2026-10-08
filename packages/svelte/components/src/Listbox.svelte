<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/listbox.css";
  import { listboxInitialFocus, listboxTransition, type ListboxContext, type ListboxEvent, type ListboxItem, type ListboxOrientation, type ListboxSelectionMode } from "@inflatable-cookie/poodle-core";
  import { onMount, type Snippet } from "svelte";

  const isDevelopment = (import.meta as ImportMeta & { env?: { DEV?: boolean } }).env?.DEV ??
    (typeof process !== "undefined" && process.env.NODE_ENV !== "production");

  interface Props {
    items?: ListboxItem[];
    selectionMode?: ListboxSelectionMode;
    value?: string | null;
    values?: string[];
    defaultValue?: string | null;
    defaultValues?: string[];
    orientation?: ListboxOrientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    ariaLabelledby?: string | null;
    onValueChange?: ((value: string) => void) | undefined;
    onValuesChange?: ((values: string[]) => void) | undefined;
    onActivate?: ((value: string) => void) | undefined;
    children?: Snippet<[item: ListboxItem, selected: boolean, focused: boolean]>;
  }

  let {
    items = [],
    selectionMode = "single",
    value,
    values,
    defaultValue = null,
    defaultValues = [],
    orientation = "vertical",
    disabled = false,
    ariaLabel = null,
    ariaLabelledby = null,
    onValueChange,
    onValuesChange,
    onActivate,
    children,
  }: Props = $props();

  let uncontrolledValue = $state<string | null>(defaultValue);
  let uncontrolledValues = $state<string[]>([...defaultValues]);
  let localFocus = $state<string | null>(null);
  let rangeAnchor = $state<string | null>(null);
  let typeaheadAt = $state(0);
  let typeahead = $state("");
  let root: HTMLDivElement;

  const isControlled = $derived(selectionMode === "single" ? value !== undefined : values !== undefined);
  const selectedValues = $derived(selectionMode === "single"
    ? (isControlled ? (value == null ? [] : [value]) : (uncontrolledValue == null ? [] : [uncontrolledValue]))
    : (isControlled ? (values ?? []) : uncontrolledValues));
  const validFocus = $derived(items.some((item) => item.value === localFocus && !item.disabled) ? localFocus : null);
  const focusedValue = $derived(validFocus ?? listboxInitialFocus(items, selectedValues, disabled));
  const enabledCount = $derived(disabled ? 0 : items.filter((item) => !item.disabled).length);

  function context(): ListboxContext {
    return {
      items,
      selectionMode,
      orientation,
      disabled,
      selectedValues: [...selectedValues],
      focusedValue,
      anchorValue: rangeAnchor,
      typeahead,
      typeaheadAt,
    };
  }

  function focusOption(valueToFocus: string): void {
    localFocus = valueToFocus;
    queueMicrotask(() => {
      const option = [...(root?.querySelectorAll<HTMLElement>("[data-listbox-option]") ?? [])]
        .find((candidate) => candidate.dataset.listboxOption === valueToFocus);
      option?.focus();
    });
  }

  function run(event: ListboxEvent): void {
    const result = listboxTransition(context(), event);
    localFocus = result.context.focusedValue;
    rangeAnchor = result.context.anchorValue;
    typeahead = result.context.typeahead;
    typeaheadAt = result.context.typeaheadAt;
    for (const effect of result.effects) {
      if (effect.type === "focus") focusOption(effect.value);
      else if (effect.type === "activate") onActivate?.(effect.value);
      else if (effect.type === "selectionChanged") {
        if (selectionMode === "single") {
          const next = effect.values[0] ?? null;
          if (!isControlled) uncontrolledValue = next;
          if (next !== null) onValueChange?.(next);
        } else {
          if (!isControlled) uncontrolledValues = effect.values;
          onValuesChange?.(effect.values);
        }
      }
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (disabled) return;
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "a" && selectionMode === "multiple") {
      event.preventDefault();
      run({ type: "SELECT_ALL" });
      return;
    }
    if ((orientation === "vertical" && event.key === "ArrowDown") || (orientation === "horizontal" && event.key === "ArrowRight")) {
      event.preventDefault(); run({ type: "MOVE", direction: 1, extendSelection: event.shiftKey }); return;
    }
    if ((orientation === "vertical" && event.key === "ArrowUp") || (orientation === "horizontal" && event.key === "ArrowLeft")) {
      event.preventDefault(); run({ type: "MOVE", direction: -1, extendSelection: event.shiftKey }); return;
    }
    if (event.key === "Home" || event.key === "End") {
      event.preventDefault(); run({ type: "BOUNDARY", boundary: event.key === "Home" ? "first" : "last" }); return;
    }
    if (event.key === " " || event.key === "Spacebar") {
      event.preventDefault(); run({ type: "SPACE" }); return;
    }
    if (event.key === "Enter") {
      event.preventDefault(); run({ type: "ACTIVATE" }); return;
    }
    if (!event.altKey && !event.ctrlKey && !event.metaKey && event.key.length === 1) {
      run({ type: "TYPEAHEAD", character: event.key, now: performance.now() });
    }
  }

  onMount(() => {
    let warned = false;
    const interactive = 'a[href],button,input:not([type="hidden"]),select,textarea,[tabindex]:not([tabindex="-1"]),[contenteditable]:not([contenteditable="false"]),[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="combobox"],[role="textbox"],[role="slider"],[role="menuitem"],[role="option"]';
    const check = (): void => {
      if (warned || !root) return;
      const hasInteractive = [...root.querySelectorAll<HTMLElement>("[data-listbox-option]")]
        .some((option) => option.querySelector(interactive) !== null);
      if (isDevelopment && hasInteractive) {
        warned = true;
        console.warn("Listbox options must not contain interactive descendants; Listbox owns option keyboard interaction.");
      }
    };
    check();
    const observer = new MutationObserver(check);
    if (root) observer.observe(root, { childList: true, subtree: true, attributes: true, attributeFilter: ["href", "tabindex", "role", "contenteditable", "type"] });
    return () => observer.disconnect();
  });
</script>

<div
  bind:this={root}
  class="poodle-listbox"
  data-scope="listbox"
  data-part="root"
  data-orientation={orientation}
  data-disabled={disabled}
  role="listbox"
  aria-orientation={orientation}
  aria-label={ariaLabel ?? undefined}
  aria-labelledby={ariaLabelledby ?? undefined}
  aria-disabled={disabled ? "true" : undefined}
  aria-multiselectable={selectionMode === "multiple" ? "true" : undefined}
  tabindex={disabled ? -1 : enabledCount === 0 ? 0 : undefined}
  onkeydown={handleKeydown}
  onfocusin={(event) => {
    const option = (event.target as HTMLElement).closest<HTMLElement>("[data-listbox-option]");
    if (option?.dataset.listboxOption) run({ type: "FOCUS", value: option.dataset.listboxOption });
  }}
>
  {#each items as item (item.value)}
    {@const selected = selectedValues.includes(item.value)}
    {@const focused = focusedValue === item.value && !disabled && !item.disabled}
    <div
      class="poodle-listbox__option"
      data-scope="listbox"
      data-part="option"
      data-listbox-option={item.value}
      data-selected={selected}
      data-focused={focused}
      role="option"
      aria-label={item.label}
      aria-selected={selected ? "true" : "false"}
      aria-disabled={disabled || item.disabled ? "true" : undefined}
      tabindex={focused ? 0 : -1}
      onclick={(event) => run({ type: "SELECT", value: item.value, additive: event.ctrlKey || event.metaKey, range: event.shiftKey })}
      ondblclick={() => run({ type: "ACTIVATE", value: item.value })}
    >
      {#if children}
        {@render children(item, selected, focused)}
      {:else}
        {item.label}
      {/if}
    </div>
  {/each}
</div>

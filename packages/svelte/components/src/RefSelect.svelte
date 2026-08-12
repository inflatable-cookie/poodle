<script module lang="ts">
  let nextRefSelectId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/ref-select.css";
  import { layerContains, registerDismissLayer } from "@inflatable-cookie/poodle-core";
  import { tick } from "svelte";

  import { anchored } from "./anchored";
  import { default as Icon } from "./Icon.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { filterRefs, groupHeadingFor, refIcon, refKindIcon, refLabel } from "./ref-select-model";
  import type {
    ControlDensity,
    ControlSize,
    RefOption,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    refs?: RefOption[];
    value?: string;
    /** The checked-out ref, marked in the list. Often equals `value`, but a host
     * browsing another ref keeps the marker where it belongs. */
    currentRef?: string | null;
    currentLabel?: string;
    placeholder?: string;
    searchable?: boolean;
    /** Controlled query. When supplied the component stops filtering — the host
     * owns which refs to pass. */
    searchValue?: string | null;
    searchPlaceholder?: string;
    searchLabel?: string;
    loading?: boolean;
    loadingLabel?: string;
    emptyLabel?: string;
    ariaLabel?: string;
    disabled?: boolean;
    variant?: "bare" | "outlined";
    emphasis?: "default" | "subdued";
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    dismissOnOutsideInteract?: boolean;
    onChange?: ((value: string) => void) | null;
    onSearchChange?: ((query: string) => void) | null;
  }

  let {
    refs = [],
    value = $bindable(""),
    currentRef = null,
    currentLabel = "current",
    placeholder = "Select ref",
    searchable = true,
    searchValue = null,
    searchPlaceholder = "Search refs…",
    searchLabel = "Search refs",
    loading = false,
    loadingLabel = "Loading more refs…",
    emptyLabel = "No refs found",
    ariaLabel = "Ref",
    disabled = false,
    variant = "bare",
    emphasis = "default",
    sizeRole = "control",
    size = null,
    density = null,
    dismissOnOutsideInteract = true,
    onChange = null,
    onSearchChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const panelId = `poodle-ref-select-${++nextRefSelectId}`;

  let open = $state(false);
  let rootElement = $state<HTMLDivElement | null>(null);
  let panelElement = $state<HTMLDivElement | null>(null);
  // Its home is a composer footer pinned to the bottom of a viewport, so it
  // prefers to open upward and flips only when it must.
  let placement = $state<"top" | "bottom">("top");
  let localQuery = $state("");

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hostDrivesSearch = $derived(searchValue !== null);
  const query = $derived(hostDrivesSearch ? searchValue ?? "" : localQuery);
  // A host-supplied query means the passed list is already the answer.
  const visibleRefs = $derived(hostDrivesSearch ? refs : filterRefs(refs, query));
  const selected = $derived(refs.find((option) => option.value === value));
  const triggerLabel = $derived(refLabel(refs, value, placeholder));
  const triggerIcon = $derived(selected ? refIcon(selected) : refKindIcon(undefined));

  $effect(() => {
    if (!open) return;
    tick().then(() => {
      const panel = panelElement;
      if (!panel) return;
      const search = panel.querySelector<HTMLInputElement>("input");
      const first = panel.querySelector<HTMLElement>(".poodle-ref-select__option:not([disabled])");
      (search ?? first)?.focus();
    });
  });

  $effect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, panelElement),
      dismissOnOutsideInteract,
      onDismiss: () => {
        open = false;
      },
    });
  });

  function toggleOpen(): void {
    if (disabled) return;
    open = !open;
  }

  function selectRef(next: string): void {
    if (disabled) return;
    value = next;
    onChange?.(next);
    // Choosing a ref is the terminal action here — unlike ModelPicker, nothing
    // follows it in the panel.
    open = false;
    rootElement?.querySelector<HTMLElement>(".poodle-ref-select__trigger")?.focus();
  }

  function setQuery(next: string): void {
    if (!hostDrivesSearch) localQuery = next;
    onSearchChange?.(next);
  }

  /** Arrow keys move through the filtered rows from anywhere in the panel, so
   * typing and choosing are one gesture. */
  function moveFocus(event: KeyboardEvent, delta: number): void {
    event.preventDefault();
    const options = Array.from(
      panelElement?.querySelectorAll<HTMLButtonElement>(
        ".poodle-ref-select__option:not([disabled])",
      ) ?? [],
    );
    if (options.length === 0) return;
    const current = options.indexOf(document.activeElement as HTMLButtonElement);
    const next = current === -1 ? (delta > 0 ? 0 : options.length - 1) : (current + delta + options.length) % options.length;
    options[next]?.focus();
  }

  function handlePanelKeydown(event: KeyboardEvent): void {
    if (event.key === "ArrowDown") moveFocus(event, 1);
    else if (event.key === "ArrowUp") moveFocus(event, -1);
  }
</script>

<div
  bind:this={rootElement}
  class="poodle-ref-select"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-variant={variant}
  data-emphasis={emphasis}
  data-open={open}
  data-disabled={disabled}
>
  <button
    type="button"
    class="poodle-ref-select__trigger"
    disabled={disabled}
    aria-label={`${ariaLabel}: ${triggerLabel}`}
    aria-haspopup="dialog"
    aria-expanded={open ? "true" : "false"}
    aria-controls={open ? panelId : undefined}
    onclick={toggleOpen}
  >
    <span class="poodle-ref-select__icon">
      <Icon name={triggerIcon} size="xs" />
    </span>
    <span class="poodle-ref-select__label" data-placeholder={!value}>{triggerLabel}</span>
    <span class="poodle-ref-select__chevron" aria-hidden="true">▾</span>
  </button>

  {#if open}
    <div
      bind:this={panelElement}
      use:anchored={{
        anchor: rootElement,
        placement: "top-start",
        offset: 8,
        onPlacement: (next) => (placement = next.startsWith("top") ? "top" : "bottom"),
      }}
      id={panelId}
      class="poodle-ref-select__surface"
      data-placement={placement}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="dialog"
      aria-label={ariaLabel}
      tabindex="-1"
      onkeydown={handlePanelKeydown}
    >
      {#if searchable}
        <div class="poodle-ref-select__search">
          <TextInput
            type="search"
            value={query}
            placeholder={searchPlaceholder}
            ariaLabel={searchLabel}
            size={resolvedSize}
            density={resolvedDensity}
            disabled={disabled}
            onValueChange={setQuery}
          />
        </div>
      {/if}

      <div class="poodle-ref-select__list" role="listbox" aria-label={ariaLabel}>
        {#each visibleRefs as option, index (option.value)}
          {@const heading = groupHeadingFor(visibleRefs, index)}
          {@const isSelected = option.value === value}
          {#if heading}
            <span class="poodle-ref-select__group">{heading}</span>
          {/if}
          <button
            type="button"
            class="poodle-ref-select__option"
            role="option"
            aria-selected={isSelected ? "true" : "false"}
            data-selected={isSelected}
            data-current={option.value === currentRef}
            data-disabled={option.disabled ?? false}
            data-kind={option.kind}
            disabled={disabled || option.disabled}
            onclick={() => selectRef(option.value)}
          >
            <span class="poodle-ref-select__option-icon">
              <Icon name={refIcon(option)} size="xs" />
            </span>
            <span class="poodle-ref-select__option-text">
              <span class="poodle-ref-select__option-label">{option.label}</span>
              {#if option.description}
                <span class="poodle-ref-select__option-description">{option.description}</span>
              {/if}
            </span>
            {#if option.value === currentRef}
              <span class="poodle-ref-select__option-marker">{currentLabel}</span>
            {/if}
          </button>
        {/each}
      </div>

      {#if visibleRefs.length === 0 && !loading}
        <p class="poodle-ref-select__empty">{emptyLabel}</p>
      {/if}

      {#if loading}
        <p class="poodle-ref-select__loading" role="status">{loadingLabel}</p>
      {/if}
    </div>
  {/if}
</div>

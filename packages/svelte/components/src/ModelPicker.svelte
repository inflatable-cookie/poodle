<script module lang="ts">
  let nextModelPickerId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-picker.css";
  import { layerContains, registerDismissLayer } from "@inflatable-cookie/poodle-core";
  import { tick } from "svelte";

  import { anchored } from "./anchored";
  import { default as Icon } from "./Icon.svelte";
  import { default as SegmentedControl } from "./SegmentedControl.svelte";
  import { default as Switch } from "./Switch.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import {
    applicableAxes,
    axisControlKind,
    axisValue,
    groupHeadingFor,
    initialSelection,
    modelLabel,
    resolveSelection,
    summaryText,
  } from "./model-picker-model";
  import type {
    ControlDensity,
    ControlSize,
    ModelAxisValue,
    ModelCapabilityAxis,
    ModelOption,
    ModelSelection,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    models?: ModelOption[];
    axes?: ModelCapabilityAxis[];
    value?: ModelSelection | undefined;
    placeholder?: string;
    ariaLabel?: string;
    disabled?: boolean;
    showAxisSummary?: boolean;
    showModelDescriptions?: boolean;
    /** `bare` is the borderless inline trigger used in composer toolbars;
     * `outlined` draws the standard control border and fill. */
    variant?: "bare" | "outlined";
    /** `default` is full-strength trigger text. `subdued` dims the label and
     * summary so the picker recedes beside a more important control (its home in
     * `AgentChatInput`, where the editor should hold the eye); hover and focus
     * bring it back to full strength. */
    emphasis?: "default" | "subdued";
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    dismissOnOutsideInteract?: boolean;
    onChange?: ((value: ModelSelection) => void) | null;
  }

  let {
    models = [],
    axes = [],
    value = $bindable<ModelSelection | undefined>(undefined),
    placeholder = "Select model",
    ariaLabel = "Model",
    disabled = false,
    showAxisSummary = true,
    showModelDescriptions = true,
    variant = "bare",
    emphasis = "default",
    sizeRole = "control",
    size = null,
    density = null,
    dismissOnOutsideInteract = true,
    onChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const panelId = `poodle-model-picker-${++nextModelPickerId}`;

  let open = $state(false);
  let rootElement = $state<HTMLDivElement | null>(null);
  let panelElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<ModelSelection | null>(null);
  // The picker's home is a composer toolbar pinned to the bottom of a
  // viewport, so it prefers to open upward and flips only when it must.
  let placement = $state<"top" | "bottom">("top");

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasValueProp = $derived(value !== undefined);
  const effectiveValue = $derived<ModelSelection>(
    hasValueProp
      ? value ?? { model: "", axes: {} }
      : uncontrolledValue ?? initialSelection(models, axes),
  );

  const selectedModel = $derived(models.find((model) => model.value === effectiveValue.model));
  const triggerLabel = $derived(modelLabel(models, effectiveValue, placeholder));
  const axisSummary = $derived(showAxisSummary ? summaryText(models, axes, effectiveValue) : "");
  const visibleAxes = $derived(applicableAxes(models, axes, effectiveValue.model));
  // Two columns (models | axes) whenever the selected model has applicable
  // axes; a plain list otherwise.
  const panelLayout = $derived(visibleAxes.length > 0 ? "split" : "single");
  const triggerAriaLabel = $derived(
    axisSummary ? `${ariaLabel}: ${triggerLabel}, ${axisSummary}` : `${ariaLabel}: ${triggerLabel}`,
  );

  $effect(() => {
    if (!open) return;
    tick().then(() => {
      const panel = panelElement;
      if (!panel) return;
      const selected = panel.querySelector<HTMLElement>('[data-selected="true"]:not([disabled])');
      const first = panel.querySelector<HTMLElement>(".poodle-model-picker__option:not([disabled])");
      (selected ?? first)?.focus();
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

  function sync(next: ModelSelection): void {
    // Every emission is normalised, so a scoped-out axis value never leaks.
    const resolved = resolveSelection(models, axes, next);
    if (hasValueProp) {
      value = resolved;
    } else {
      uncontrolledValue = resolved;
    }
    onChange?.(resolved);
  }

  function selectModel(model: string): void {
    if (disabled) return;
    // The popover stays open: the axes belong to the model just chosen, so
    // closing here would force a second trip to adjust them. Escape or an
    // outside interaction dismisses.
    sync({ model, axes: { ...effectiveValue.axes } });
  }

  function setAxis(key: string, next: ModelAxisValue): void {
    if (disabled) return;
    // Changing an axis leaves the popover open — only a model choice closes it.
    sync({ model: effectiveValue.model, axes: { ...effectiveValue.axes, [key]: next } });
  }

  function toggleOpen(): void {
    if (disabled) return;
    open = !open;
  }

  function handleOptionKeydown(event: KeyboardEvent): void {
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    event.preventDefault();
    const options = Array.from(
      panelElement?.querySelectorAll<HTMLButtonElement>(
        ".poodle-model-picker__option:not([disabled])",
      ) ?? [],
    );
    if (options.length === 0) return;
    const current = options.indexOf(event.currentTarget as HTMLButtonElement);
    const delta = event.key === "ArrowDown" ? 1 : -1;
    const next = (current + delta + options.length) % options.length;
    options[next]?.focus();
  }

  function axisOptionItems(axis: ModelCapabilityAxis) {
    return (axis.options ?? []).map((option) => ({
      value: option.value,
      label: option.label,
      disabled: option.disabled,
    }));
  }
</script>

<div
  bind:this={rootElement}
  class="poodle-model-picker"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-variant={variant}
  data-emphasis={emphasis}
  data-open={open}
  data-disabled={disabled}
>
  <button
    type="button"
    class="poodle-model-picker__trigger"
    disabled={disabled}
    aria-label={triggerAriaLabel}
    aria-haspopup="dialog"
    aria-expanded={open ? "true" : "false"}
    aria-controls={open ? panelId : undefined}
    onclick={toggleOpen}
  >
    {#if selectedModel?.image}
      <span class="poodle-model-picker__icon">
        <img
          class="poodle-model-picker__image"
          src={selectedModel.image.src}
          alt={selectedModel.image.alt ?? ""}
        />
      </span>
    {:else if selectedModel?.icon}
      <span class="poodle-model-picker__icon">
        <Icon name={selectedModel.icon} size="xs" />
      </span>
    {/if}
    <span class="poodle-model-picker__label" data-placeholder={!effectiveValue.model}>
      {triggerLabel}
    </span>
    {#if axisSummary}
      <span class="poodle-model-picker__summary" aria-hidden="true">{axisSummary}</span>
    {/if}
    <span class="poodle-model-picker__chevron" aria-hidden="true">▾</span>
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
      class="poodle-model-picker__surface"
      data-layout={panelLayout}
      data-placement={placement}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="dialog"
      aria-label={ariaLabel}
      tabindex="-1"
    >
      <div class="poodle-model-picker__panel">
        <div class="poodle-model-picker__models" role="radiogroup" aria-label="Model">
          {#each models as model, index (model.value)}
            {@const heading = groupHeadingFor(models, index)}
            {@const isSelected = model.value === effectiveValue.model}
            {#if heading}
              <span class="poodle-model-picker__group">{heading}</span>
            {/if}
            <button
              type="button"
              class="poodle-model-picker__option"
              role="radio"
              aria-checked={isSelected ? "true" : "false"}
              data-selected={isSelected}
              data-disabled={model.disabled ?? false}
              disabled={disabled || model.disabled}
              onclick={() => selectModel(model.value)}
              onkeydown={handleOptionKeydown}
            >
              {#if model.image}
                <span class="poodle-model-picker__option-icon">
                  <img
                    class="poodle-model-picker__option-image"
                    src={model.image.src}
                    alt={model.image.alt ?? ""}
                  />
                </span>
              {:else if model.icon}
                <span class="poodle-model-picker__option-icon">
                  <Icon name={model.icon} size="sm" />
                </span>
              {/if}
              <span class="poodle-model-picker__option-text">
                <span class="poodle-model-picker__option-label">{model.label}</span>
                {#if showModelDescriptions && model.description}
                  <span class="poodle-model-picker__option-description">{model.description}</span>
                {/if}
              </span>
              {#if model.badge}
                <span class="poodle-model-picker__option-badge">{model.badge}</span>
              {/if}
              {#if isSelected}
                <span class="poodle-model-picker__option-check">
                  <Icon name="check" size="xs" />
                </span>
              {/if}
            </button>
          {/each}
        </div>

        {#if visibleAxes.length > 0}
          <div class="poodle-model-picker__axes">
            {#each visibleAxes as axis (axis.key)}
              {@const current = axisValue(axis, effectiveValue)}
              <div
                class="poodle-model-picker__axis"
                data-kind={axis.kind}
                data-control={axisControlKind(axis)}
              >
                <span class="poodle-model-picker__axis-label">{axis.label}</span>
                {#if axis.description}
                  <span class="poodle-model-picker__axis-description">{axis.description}</span>
                {/if}
                {#if axis.kind === "select" && axisControlKind(axis) === "list"}
                  <div class="poodle-model-picker__axis-list" role="radiogroup" aria-label={axis.label}>
                    {#each axis.options ?? [] as option (option.value)}
                      <button
                        type="button"
                        class="poodle-model-picker__axis-option"
                        role="radio"
                        aria-checked={current === option.value ? "true" : "false"}
                        data-selected={current === option.value}
                        data-disabled={option.disabled ?? false}
                        disabled={disabled || axis.disabled || option.disabled}
                        onclick={() => setAxis(axis.key, option.value)}
                      >
                        <span class="poodle-model-picker__axis-option-label">{option.label}</span>
                        {#if current === option.value}
                          <span class="poodle-model-picker__axis-option-check">
                            <Icon name="check" size="xs" />
                          </span>
                        {/if}
                      </button>
                    {/each}
                  </div>
                {:else if axis.kind === "select"}
                  <SegmentedControl
                    options={axisOptionItems(axis)}
                    value={typeof current === "string" ? current : ""}
                    ariaLabel={axis.label}
                    size={resolvedSize}
                    density={resolvedDensity}
                    equalWidth
                    disabled={disabled || axis.disabled}
                    onValueChange={(next) => setAxis(axis.key, next)}
                  />
                {:else}
                  <Switch
                    checked={current === true}
                    ariaLabel={axis.label}
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={disabled || axis.disabled}
                    onCheckedChange={(checked) => setAxis(axis.key, checked)}
                  />
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

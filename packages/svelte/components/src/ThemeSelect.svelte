<script module lang="ts">
  import type { ThemeOption } from "./types";

  let nextThemeSelectId = 0;

  function swatchStyle(option: ThemeOption): string {
    return `background:${option.swatch.canvas};border-color:${option.swatch.border}`;
  }
</script>

<script lang="ts">
  import "@poodle/styles/theme-select.css";
  import { layerContains, registerDismissLayer } from "@poodle/headless";
  import { tick } from "svelte";
  import { get } from "svelte/store";

  import { anchored } from "./anchored";
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { getThemeController } from "./theme-controller";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    /** Theme catalogue. Falls back to the theme controller's list, then empty. */
    themes?: ThemeOption[];
    /** Controlled current theme value. */
    value?: string | undefined;
    onChange?: ((value: string) => void) | null;
    ariaLabel?: string;
    disabled?: boolean;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    /** Swatch-tile columns in the popover. */
    columns?: number;
    /** Show the current theme name in the trigger. */
    showLabel?: boolean;
  }

  let {
    themes = undefined,
    value = $bindable<string | undefined>(undefined),
    onChange = null,
    ariaLabel = "Theme",
    disabled = false,
    sizeRole = "control",
    size = null,
    density = null,
    columns = 3,
    showLabel = true,
  }: Props = $props();

  const controller = getThemeController();
  const uiPresentation = getUiPresentation();
  const panelId = `poodle-theme-select-${++nextThemeSelectId}`;

  let open = $state(false);
  let rootElement = $state<HTMLDivElement | null>(null);
  let panelElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<string>("");
  let controllerValue = $state<string>(controller ? get(controller.current) : "");

  $effect(() => {
    if (!controller) return;
    return controller.current.subscribe((next) => {
      controllerValue = next;
    });
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const options = $derived<ThemeOption[]>(themes ?? controller?.themes ?? []);
  const hasValueProp = $derived(value !== undefined);
  const currentValue = $derived(
    hasValueProp ? (value ?? "") : controller ? controllerValue : uncontrolledValue,
  );
  const currentOption = $derived(options.find((option) => option.value === currentValue));
  const triggerLabel = $derived(currentOption?.label ?? "Theme");

  $effect(() => {
    if (!open) return;
    tick().then(() => {
      const selected = panelElement?.querySelector<HTMLElement>('[data-selected="true"]');
      const first = panelElement?.querySelector<HTMLElement>("button:not([disabled])");
      (selected ?? first)?.focus();
    });
  });

  $effect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, panelElement),
      dismissOnOutsideInteract: true,
      onDismiss: () => {
        open = false;
      },
    });
  });

  function toggleOpen(): void {
    if (disabled) return;
    open = !open;
  }

  function select(next: string): void {
    if (disabled) return;
    open = false;
    if (hasValueProp) {
      value = next;
    } else if (controller) {
      controller.setTheme(next);
    } else {
      uncontrolledValue = next;
    }
    onChange?.(next);
  }
</script>

<div
  bind:this={rootElement}
  class="poodle-theme-select"
  role="group"
  aria-label={ariaLabel}
  data-disabled={disabled}
  data-open={open}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <button
    type="button"
    class="poodle-theme-select__trigger"
    disabled={disabled}
    aria-label={`${ariaLabel}: ${triggerLabel}`}
    aria-haspopup="dialog"
    aria-expanded={open ? "true" : "false"}
    aria-controls={open ? panelId : undefined}
    onclick={toggleOpen}
  >
    {#if currentOption}
      <span class="poodle-theme-select__swatch" style={swatchStyle(currentOption)} aria-hidden="true">
        <span class="poodle-theme-select__swatch-surface" style={`background:${currentOption.swatch.surface}`}></span>
        <span class="poodle-theme-select__swatch-accent" style={`background:${currentOption.swatch.accent}`}></span>
        <span class="poodle-theme-select__swatch-text" style={`background:${currentOption.swatch.text}`}></span>
      </span>
    {/if}
    {#if showLabel}
      <span class="poodle-theme-select__label">{triggerLabel}</span>
    {/if}
    <span class="poodle-theme-select__chevron" aria-hidden="true">▾</span>
  </button>

  {#if open}
    <div
      bind:this={panelElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 8 }}
      id={panelId}
      class="poodle-theme-select__surface"
      role="dialog"
      aria-label={ariaLabel}
      tabindex="-1"
    >
      <div
        class="poodle-theme-select__grid"
        role="listbox"
        aria-label={ariaLabel}
        style={`--poodle-theme-select-columns:${columns}`}
      >
        {#each options as option (option.value)}
          <button
            type="button"
            class="poodle-theme-select__tile"
            role="option"
            aria-selected={option.value === currentValue}
            data-selected={option.value === currentValue}
            title={option.description ?? option.label}
            onclick={() => select(option.value)}
          >
            <span class="poodle-theme-select__swatch poodle-theme-select__swatch--tile" style={swatchStyle(option)} aria-hidden="true">
              <span class="poodle-theme-select__swatch-surface" style={`background:${option.swatch.surface}`}></span>
              <span class="poodle-theme-select__swatch-accent" style={`background:${option.swatch.accent}`}></span>
              <span class="poodle-theme-select__swatch-text" style={`background:${option.swatch.text}`}></span>
              {#if option.value === currentValue}
                <span class="poodle-theme-select__check" style={`color:${option.swatch.accent}`}><Icon name="check" size="xs" /></span>
              {/if}
            </span>
            <span class="poodle-theme-select__tile-label">{option.label}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

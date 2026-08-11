<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/app-header.css";
  import type { Snippet } from "svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    title?: string | null;
    subtitle?: string | null;
    dragRegion?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    /** Bindable escape hatch: the rendered `<header>` DOM element, so a host
     * can attach behaviour (for example window dragging) to the root. */
    element?: HTMLElement | null;
    identity?: Snippet;
    /** Optional centre region (g13-b017). Its presence is the signal: it
     * switches the grid to the symmetric side-column layout and groups
     * `actions` + `utility` into the trailing column. */
    center?: Snippet;
    actions?: Snippet;
    utility?: Snippet;
  }

  let {
    title = null,
    subtitle = null,
    dragRegion = false,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    element = $bindable<HTMLElement | null>(null),
    identity,
    center,
    actions,
    utility,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <header
    bind:this={element}
    class="poodle-app-header"
    data-drag-region={dragRegion}
    data-center={center ? "" : undefined}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    aria-label={ariaLabel ?? title ?? undefined}
  >
    <div class="poodle-app-header__identity">
      {#if identity}
        {@render identity()}
      {:else if title}
        <div class="poodle-app-header__title-group">
          <strong>{title}</strong>
          {#if subtitle}
            <span class="poodle-app-header__subtitle">{subtitle}</span>
          {/if}
        </div>
      {/if}
    </div>

    {#if center}
      <div class="poodle-app-header__center">
        {@render center()}
      </div>
    {/if}

    {#if center}
      <div class="poodle-app-header__trailing">
        {#if actions}
          <div class="poodle-app-header__actions">
            {@render actions()}
          </div>
        {/if}
        {#if utility}
          <div class="poodle-app-header__utility">
            {@render utility()}
          </div>
        {/if}
      </div>
    {:else}
      {#if actions}
        <div class="poodle-app-header__actions">
          {@render actions()}
        </div>
      {/if}

      {#if utility}
        <div class="poodle-app-header__utility">
          {@render utility()}
        </div>
      {/if}
    {/if}
  </header>
</UiPresentationProvider>


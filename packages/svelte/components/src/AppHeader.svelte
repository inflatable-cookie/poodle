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
    identity?: Snippet;
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
    identity,
    actions,
    utility,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
  <header
    class="poodle-app-header"
    data-drag-region={dragRegion}
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
  </header>
</UiPresentationProvider>


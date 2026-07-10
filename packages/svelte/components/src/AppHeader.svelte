<script lang="ts">
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

<style>
  .poodle-app-header {
    --poodle-app-header-gap: var(--poodle-space-inline-md);
    --poodle-app-header-region-gap: var(--poodle-space-inline-sm);
    --poodle-app-header-min-height: var(--poodle-size-panel-header);
    --poodle-app-header-padding-block: var(--poodle-space-control-y);
    --poodle-app-header-padding-inline: var(--poodle-space-panel-x);
    --poodle-app-header-title-size: 0.9375rem;
    --poodle-app-header-subtitle-size: 0.75rem;
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    gap: var(--poodle-app-header-gap);
    align-items: center;
    min-height: var(--poodle-app-header-min-height);
    padding: var(--poodle-app-header-padding-block) var(--poodle-app-header-padding-inline);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: var(--poodle-recipe-app-header-fill, color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent));
    overflow: visible;
  }

  .poodle-app-header__identity,
  .poodle-app-header__actions,
  .poodle-app-header__utility {
    display: flex;
    align-items: center;
    gap: var(--poodle-app-header-region-gap);
    min-width: 0;
  }

  .poodle-app-header__title-group {
    display: flex;
    align-items: baseline;
    gap: var(--poodle-app-header-region-gap);
    min-width: 0;
  }

  .poodle-app-header__identity strong {
    font-size: var(--poodle-app-header-title-size);
    line-height: 1.2;
    white-space: nowrap;
  }

  .poodle-app-header__subtitle {
    color: var(--poodle-recipe-app-header-subtitle-text, var(--poodle-color-text-secondary));
    font-size: var(--poodle-app-header-subtitle-size);
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .poodle-app-header[data-size="xs"] {
    --poodle-app-header-min-height: 2.25rem;
    --poodle-app-header-title-size: 0.8125rem;
    --poodle-app-header-subtitle-size: 0.6875rem;
  }

  .poodle-app-header[data-size="sm"] {
    --poodle-app-header-min-height: 2.5rem;
    --poodle-app-header-title-size: 0.875rem;
    --poodle-app-header-subtitle-size: 0.71875rem;
  }

  .poodle-app-header[data-size="md"] {
    --poodle-app-header-min-height: 2.75rem;
    --poodle-app-header-title-size: 0.9375rem;
    --poodle-app-header-subtitle-size: 0.75rem;
  }

  .poodle-app-header[data-size="lg"] {
    --poodle-app-header-min-height: 3rem;
    --poodle-app-header-title-size: 1rem;
    --poodle-app-header-subtitle-size: 0.8125rem;
  }

  .poodle-app-header[data-size="xl"] {
    --poodle-app-header-min-height: 3.25rem;
    --poodle-app-header-title-size: 1.0625rem;
    --poodle-app-header-subtitle-size: 0.875rem;
  }

  .poodle-app-header[data-density="compact"] {
    --poodle-app-header-gap: 0.625rem;
    --poodle-app-header-region-gap: 0.375rem;
    --poodle-app-header-padding-block: 0.25rem;
    --poodle-app-header-padding-inline: calc(var(--poodle-space-panel-x) - 0.125rem);
  }

  .poodle-app-header[data-density="comfortable"] {
    --poodle-app-header-gap: 1rem;
    --poodle-app-header-region-gap: 0.625rem;
    --poodle-app-header-padding-block: 0.5rem;
    --poodle-app-header-padding-inline: calc(var(--poodle-space-panel-x) + 0.125rem);
  }

  .poodle-app-header__utility {
    justify-content: flex-end;
  }

  @media (max-width: 45rem) {
    .poodle-app-header {
      grid-template-columns: 1fr;
    }

    .poodle-app-header__utility {
      justify-content: flex-start;
    }
  }
</style>

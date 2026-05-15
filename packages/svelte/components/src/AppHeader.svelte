<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string | null;
    subtitle?: string | null;
    dragRegion?: boolean;
    ariaLabel?: string | null;
    identity?: Snippet;
    actions?: Snippet;
    utility?: Snippet;
  }

  let {
    title = null,
    subtitle = null,
    dragRegion = false,
    ariaLabel = null,
    identity,
    actions,
    utility,
  }: Props = $props();
</script>

<header
  class="poodle-app-header"
  data-drag-region={dragRegion}
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

<style>
  .poodle-app-header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto auto;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    min-height: var(--poodle-size-panel-header);
    padding: var(--poodle-space-control-y) var(--poodle-space-panel-x);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
    overflow: visible;
  }

  .poodle-app-header__identity,
  .poodle-app-header__actions,
  .poodle-app-header__utility {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .poodle-app-header__title-group {
    display: flex;
    align-items: baseline;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
  }

  .poodle-app-header__identity strong {
    font-size: 0.9375rem;
    line-height: 1.2;
    white-space: nowrap;
  }

  .poodle-app-header__subtitle {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
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

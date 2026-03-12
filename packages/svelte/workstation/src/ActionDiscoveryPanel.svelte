<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { EmptyState } from "@pug/svelte-composites";

  import type { ActionDiscoverySection } from "./types";

  export let title = "Action discovery";
  export let description: string | null = null;
  export let sections: ActionDiscoverySection[] = [];
  export let invocationHint: string | null = null;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    actionSelect: { id: string };
  }>();

  $: hasActions = sections.some((section) => section.actions.length > 0);
</script>

<section class="action-discovery-panel" aria-label={ariaLabel ?? title}>
  <div class="action-discovery-panel__header">
    <div>
      <h3>{title}</h3>
      {#if description}
        <p>{description}</p>
      {/if}
    </div>
    {#if invocationHint}
      <span class="action-discovery-panel__hint">{invocationHint}</span>
    {/if}
  </div>

  {#if hasActions}
    <nav class="action-discovery-panel__sections" aria-label={`${ariaLabel ?? title} sections`}>
      {#each sections as section (section.id)}
        {#if section.actions.length > 0}
          <section class="action-discovery-panel__section" aria-labelledby={`action-section-${section.id}`}>
            <div class="action-discovery-panel__section-header">
              <h4 id={`action-section-${section.id}`}>{section.title}</h4>
              {#if section.description}
                <p>{section.description}</p>
              {/if}
            </div>

            <ul class="action-discovery-panel__list">
              {#each section.actions as action (action.id)}
                <li>
                  <button
                    type="button"
                    class="action-discovery-panel__item"
                    disabled={action.isDisabled}
                    on:click={() => dispatch("actionSelect", { id: action.id })}
                  >
                    <span class="action-discovery-panel__copy">
                      <strong>{action.title}</strong>
                      {#if action.description}
                        <small>{action.description}</small>
                      {/if}
                    </span>
                    <span class="action-discovery-panel__meta">
                      {#if action.badge}
                        <span class="action-discovery-panel__badge">{action.badge}</span>
                      {/if}
                      {#if action.shortcut}
                        <kbd>{action.shortcut}</kbd>
                      {/if}
                    </span>
                  </button>
                </li>
              {/each}
            </ul>
          </section>
        {/if}
      {/each}
    </nav>
  {:else}
    <EmptyState
      title="No actions available"
      message="Keep discovery surfaces visible even when the current scope has no suggested commands."
    />
  {/if}
</section>

<style>
  .action-discovery-panel,
  .action-discovery-panel__header,
  .action-discovery-panel__sections,
  .action-discovery-panel__section,
  .action-discovery-panel__section-header,
  .action-discovery-panel__copy {
    display: grid;
    gap: var(--pug-space-stack-sm);
  }

  .action-discovery-panel {
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
  }

  .action-discovery-panel__header {
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
    gap: var(--pug-space-inline-md);
  }

  .action-discovery-panel__header h3,
  .action-discovery-panel__header p,
  .action-discovery-panel__section-header h4,
  .action-discovery-panel__section-header p,
  .action-discovery-panel__copy strong,
  .action-discovery-panel__copy small {
    margin: 0;
  }

  .action-discovery-panel__header h3 {
    font-size: 1.25rem;
    line-height: 1.2;
  }

  .action-discovery-panel__header p,
  .action-discovery-panel__section-header p,
  .action-discovery-panel__copy small {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .action-discovery-panel__hint {
    padding: 0.375rem 0.625rem;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 74%, transparent);
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-code-family);
    font-size: 0.75rem;
  }

  .action-discovery-panel__list {
    display: grid;
    gap: var(--pug-space-stack-sm);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .action-discovery-panel__item {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--pug-space-inline-md);
    width: 100%;
    padding: 0.875rem;
    border: 0.0625rem solid transparent;
    border-radius: calc(var(--pug-radius-surface) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-surface) 74%, transparent);
    color: var(--pug-color-text-primary);
    text-align: left;
    cursor: pointer;
    font: inherit;
  }

  .action-discovery-panel__item:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .action-discovery-panel__item:hover {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 92%, transparent);
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-border-subtle) 32%, transparent);
  }

  .action-discovery-panel__item:disabled {
    opacity: var(--pug-state-opacity-disabled);
    cursor: not-allowed;
  }

  .action-discovery-panel__meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: center;
    justify-content: flex-end;
  }

  .action-discovery-panel__badge,
  .action-discovery-panel__meta kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 1.5rem;
    padding: 0 0.5rem;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 82%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
  }

  .action-discovery-panel__meta kbd {
    font-family: var(--pug-typography-code-family);
  }

  :global([data-theme="light"]) .action-discovery-panel {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 16%, transparent);
    box-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-border-subtle) 36%, transparent),
      0 0.5rem 1.25rem rgba(49, 66, 85, 0.04);
  }

  :global([data-theme="light"]) .action-discovery-panel__item {
    background: color-mix(in srgb, var(--pug-color-background-elevated) 96%, var(--pug-color-background-panel));
    box-shadow: inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-border-subtle) 30%, transparent);
  }

  @media (max-width: 45rem) {
    .action-discovery-panel__header,
    .action-discovery-panel__item {
      grid-template-columns: 1fr;
    }

    .action-discovery-panel__meta {
      justify-content: flex-start;
    }
  }
</style>

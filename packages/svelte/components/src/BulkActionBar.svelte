<script lang="ts">
  import type { Component } from "svelte";
  import { createEventDispatcher } from "svelte";
  import Icon from "./Icon.svelte";
  import IconButton from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    BulkAction,
    ControlDensity,
    ControlSize,
    IconProp,
    SemanticControlSizeRole
  } from "./types";

  export let selectionCount = 0;
  export let totalCount: number | null = null;
  export let actions: BulkAction[] = [];
  export let loading = false;
  export let disabled = false;
  export let showSelectAll = false;
  export let allSelected = false;
  export let selectAllLabel = "Select all";
  export let sizeRole: SemanticControlSizeRole = "control";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    action: { id: string };
    clear: void;
    selectAll: void;
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isUnavailable = disabled || loading;
  $: actionsDisabled = isUnavailable || selectionCount === 0;

  function isIconComponent(icon: BulkAction["icon"]): icon is Component<any> {
    return icon != null && typeof icon !== "string" && !Array.isArray(icon);
  }

  function isNamedIcon(icon: BulkAction["icon"]): icon is IconProp {
    return icon !== undefined && icon !== null && !isIconComponent(icon);
  }

</script>

<div class="poodle-bulk-action-bar" role="region" aria-label="Bulk actions" data-size={resolvedSize} data-density={resolvedDensity}>
  <div class="poodle-bulk-action-bar__summary">
    <strong>{selectionCount} selected</strong>
    {#if totalCount !== null}
      <span>of {totalCount}</span>
    {/if}
    {#if showSelectAll && !allSelected}
      <IconButton
        icon="check-check"
        ariaLabel={totalCount !== null ? `${selectAllLabel} (${totalCount})` : selectAllLabel}
        tooltip={totalCount !== null ? `${selectAllLabel} (${totalCount})` : selectAllLabel}
        variant="ghost"
        sizeRole="chrome"
        disabled={isUnavailable}
        on:click={() => dispatch("selectAll")}
      />
    {/if}
  </div>

  <div class="poodle-bulk-action-bar__actions">
    {#each actions as action}
      {@const actionTone = action.tone ?? "default"}
      <span
        class="poodle-bulk-action-bar__icon-action"
        data-tone={actionTone !== "default" ? actionTone : undefined}
      >
        {#if action.icon && isIconComponent(action.icon)}
          <IconButton
            icon="circle"
            ariaLabel={action.label}
            tooltip={action.label}
            variant="ghost"
            tone={actionTone === "danger" ? "danger" : "default"}
            size={resolvedSize}
            disabled={actionsDisabled || action.disabled}
            on:click={() => dispatch("action", { id: action.id })}
          >
            <svelte:component this={action.icon} size={16} />
          </IconButton>
        {:else}
          <IconButton
            icon={isNamedIcon(action.icon) ? action.icon : "circle"}
            ariaLabel={action.label}
            tooltip={action.label}
            variant="ghost"
            tone={actionTone === "danger" ? "danger" : "default"}
            size={resolvedSize}
            disabled={actionsDisabled || action.disabled}
            on:click={() => dispatch("action", { id: action.id })}
          />
        {/if}
      </span>
    {/each}
    <IconButton
      icon="x"
      ariaLabel="Clear selection"
      variant="ghost"
      size={resolvedSize}
      disabled={isUnavailable}
      on:click={() => dispatch("clear")}
    />
  </div>
</div>

<style>
  .poodle-bulk-action-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    padding: 0.5rem var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    --poodle-recipe-bulk-fill: color-mix(in srgb, var(--poodle-color-background-panel) 93%, var(--poodle-color-text-primary));
    background: var(--poodle-recipe-bulk-fill);
    --poodle-surface: var(--poodle-recipe-bulk-fill);
  }

  .poodle-bulk-action-bar__summary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: baseline;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .poodle-bulk-action-bar__summary span {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-bulk-action-bar__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }


  .poodle-bulk-action-bar__icon-action {
    display: inline-flex;
  }

  .poodle-bulk-action-bar__icon-action[data-tone="warning"] :global(.poodle-icon-button) {
    color: var(--poodle-color-status-warning);
  }

  .poodle-bulk-action-bar__icon-action[data-tone="warning"] :global(.poodle-icon-button:hover),
  .poodle-bulk-action-bar__icon-action[data-tone="warning"] :global(.poodle-icon-button:focus-visible) {
    color: color-mix(in srgb, var(--poodle-color-status-warning) 82%, var(--poodle-color-text-primary));
  }

  /* Size variants — controls and text only, not container padding */
  .poodle-bulk-action-bar[data-size="xs"] .poodle-bulk-action-bar__summary { font-size: 0.75rem; }

  .poodle-bulk-action-bar[data-size="sm"] .poodle-bulk-action-bar__summary { font-size: 0.8125rem; }

  .poodle-bulk-action-bar[data-size="lg"] .poodle-bulk-action-bar__summary { font-size: 0.9375rem; }

  .poodle-bulk-action-bar[data-size="xl"] .poodle-bulk-action-bar__summary { font-size: 1rem; }

  /* Density variants — horizontal padding and gap only, not vertical padding */
  .poodle-bulk-action-bar[data-density="compact"] { padding-inline: 0.75rem; gap: 0.375rem; }
  .poodle-bulk-action-bar[data-density="compact"] .poodle-bulk-action-bar__actions { gap: 0.125rem; }
  .poodle-bulk-action-bar[data-density="comfortable"] { padding-inline: 1.25rem; gap: 1rem; }
  .poodle-bulk-action-bar[data-density="comfortable"] .poodle-bulk-action-bar__actions { gap: 0.5rem; }
</style>

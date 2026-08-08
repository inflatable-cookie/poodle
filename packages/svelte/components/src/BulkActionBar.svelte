<script lang="ts">
  import "@poodle/styles/bulk-action-bar.css";
  import type { Component } from "svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    BulkAction,
    ControlDensity,
    ControlSize,
    IconProp,
    SemanticControlSizeRole
  } from "./types";

  interface Props {
    selectionCount?: number;
    totalCount?: number | null;
    actions?: BulkAction[];
    loading?: boolean;
    disabled?: boolean;
    showSelectAll?: boolean;
    allSelected?: boolean;
    selectAllLabel?: string;
    onAction?: ((id: string) => void) | null;
    onClear?: (() => void) | null;
    onSelectAll?: (() => void) | null;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
  }

  let {
    selectionCount = 0,
    totalCount = null,
    actions = [],
    loading = false,
    disabled = false,
    showSelectAll = false,
    allSelected = false,
    selectAllLabel = "Select all",
    onAction = null,
    onClear = null,
    onSelectAll = null,
    sizeRole = "control",
    size = null,
    density = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isUnavailable = $derived(disabled || loading);
  const actionsDisabled = $derived(isUnavailable || selectionCount === 0);

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
        onClick={() => onSelectAll?.()}
      />
    {/if}
  </div>

  <div class="poodle-bulk-action-bar__actions">
    {#each actions as action}
      {@const actionTone = action.tone ?? "default"}
      {@const fallbackIcon = actionTone === "danger" ? "trash-2" : "circle"}
      <span
        class="poodle-bulk-action-bar__icon-action"
        data-tone={actionTone !== "default" ? actionTone : undefined}
      >
        {#if action.icon && isIconComponent(action.icon)}
          {@const DynamicIcon = action.icon}
          <IconButton
            icon={fallbackIcon}
            ariaLabel={action.label}
            tooltip={action.label}
            variant="ghost"
            tone={actionTone === "danger" ? "danger" : "default"}
            size={resolvedSize}
            disabled={actionsDisabled || action.disabled}
            onClick={() => onAction?.(action.id)}
          >
            <DynamicIcon size={16} />
          </IconButton>
        {:else}
          <IconButton
            icon={isNamedIcon(action.icon) ? action.icon : fallbackIcon}
            ariaLabel={action.label}
            tooltip={action.label}
            variant="ghost"
            tone={actionTone === "danger" ? "danger" : "default"}
            size={resolvedSize}
            disabled={actionsDisabled || action.disabled}
            onClick={() => onAction?.(action.id)}
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
      onClick={() => onClear?.()}
    />
  </div>
</div>


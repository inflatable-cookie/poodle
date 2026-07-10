<script lang="ts">
  interface Props {
    label?: string;
    color?: string | null;
    minHeight?: string;
  }

  let {
    label = "",
    color = null,
    minHeight = "4rem",
  }: Props = $props();
</script>

<div
  class="poodle-region"
  style:--region-color={color}
  style:min-height={minHeight}
  role="presentation"
>
  {#if label}
    <span class="poodle-region__label">{label}</span>
  {/if}
</div>

<style>
  .poodle-region {
    display: flex;
    align-items: center;
    justify-content: center;
    border: 0.125rem dashed var(--region-color, var(--poodle-color-border-default, #444));
    border-radius: var(--poodle-radius-surface, 0.375rem);
    padding: var(--poodle-space-inline-md, 0.75rem);
  }

  .poodle-region__label {
    color: var(--poodle-recipe-region-label-text, var(--region-color, var(--poodle-color-text-tertiary, #888)));
    font-family: var(--poodle-typography-label-family, sans-serif);
    font-size: var(--poodle-typography-label-size, 0.75rem);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    user-select: none;
  }
</style>

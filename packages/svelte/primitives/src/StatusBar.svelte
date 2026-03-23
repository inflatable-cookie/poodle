<script lang="ts">
  export let summary: string | null = null;
  export let ariaLabel: string | null = null;
</script>

<footer class="status-bar" aria-label={ariaLabel ?? summary ?? "Status"}>
  <div class="status-bar__leading">
    {#if $$slots.leading}
      <slot name="leading" />
    {:else if summary}
      <span>{summary}</span>
    {/if}
  </div>

  {#if $$slots.trailing}
    <div class="status-bar__trailing">
      <slot name="trailing" />
    </div>
  {/if}
</footer>

<style>
  .status-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .status-bar__leading,
  .status-bar__trailing {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }
</style>

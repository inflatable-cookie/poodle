<script lang="ts">
  export let summary: string | null = null;
  export let ariaLabel: string | null = null;
</script>

<footer class="shell-status-bar" aria-label={ariaLabel ?? summary ?? "Shell status"}>
  <div class="shell-status-bar__leading">
    {#if $$slots.leading}
      <slot name="leading" />
    {:else if summary}
      <span>{summary}</span>
    {/if}
  </div>

  {#if $$slots.trailing}
    <div class="shell-status-bar__trailing">
      <slot name="trailing" />
    </div>
  {/if}
</footer>

<style>
  .shell-status-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--pug-space-inline-md);
    padding: 0.625rem var(--pug-space-panel-x);
    border-top: 0.0625rem solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .shell-status-bar__leading,
  .shell-status-bar__trailing {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--pug-space-inline-sm);
  }
</style>

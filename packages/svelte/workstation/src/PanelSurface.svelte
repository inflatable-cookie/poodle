<script lang="ts">
  export let title: string | null = null;
  export let isActive = false;
  export let isElevated = false;
  export let hasHeader = true;
  export let bodyPadding: "none" | "sm" | "md" = "md";
  export let scrollMode: "panel" | "content" = "panel";
  export let ariaLabel: string | null = null;
</script>

<section
  class="panel-surface"
  data-active={isActive}
  data-elevated={isElevated}
  data-body-padding={bodyPadding}
  data-scroll-mode={scrollMode}
  aria-label={ariaLabel ?? title ?? undefined}
>
  {#if hasHeader && ($$slots.header || title)}
    <div class="panel-surface__header">
      {#if $$slots.header}
        <slot name="header" />
      {:else}
        <div class="panel-surface__header-title">
          <strong>{title}</strong>
        </div>
      {/if}
    </div>
  {/if}

  <div class="panel-surface__body">
    <slot />
  </div>
</section>

<style>
  .panel-surface {
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    min-height: 0;
    border: 1px solid var(--pug-color-border-subtle);
    border-radius: calc(var(--pug-radius-surface) - 1px);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
  }

  .panel-surface[data-active="true"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 22%, var(--pug-color-border-subtle));
    box-shadow: inset 0 1px 0 color-mix(in srgb, var(--pug-color-accent-base) 45%, transparent);
  }

  .panel-surface[data-elevated="true"] {
    box-shadow: var(--pug-elevation-surface);
  }

  .panel-surface__body {
    min-height: 0;
    overflow: auto;
  }

  .panel-surface[data-body-padding="sm"] .panel-surface__body {
    padding: 10px;
  }

  .panel-surface[data-body-padding="md"] .panel-surface__body {
    padding: 14px;
  }

  .panel-surface[data-scroll-mode="content"] .panel-surface__body {
    overflow: visible;
  }

  .panel-surface__header-title {
    padding: 10px 12px;
    border-bottom: 1px solid var(--pug-color-border-subtle);
  }

  .panel-surface__header-title strong {
    font-size: 14px;
    line-height: 1.2;
  }
</style>

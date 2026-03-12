<script lang="ts">
  import type { WorkspaceShellState } from "./types";

  export let state: WorkspaceShellState = "ready";
  export let isDisabled = false;
  export let activeSurfaceLabel: string | null = null;
  export let ariaLabel: string | null = null;
  export let stateTitle: string | null = null;
  export let stateMessage: string | null = null;
</script>

<section
  class="workspace-shell"
  data-state={state}
  data-disabled={isDisabled}
  aria-label={ariaLabel ?? activeSurfaceLabel ?? undefined}
>
  {#if $$slots.appHeader}
    <div class="workspace-shell__app-header">
      <slot name="appHeader" />
    </div>
  {/if}

  {#if $$slots.projectHeader}
    <div class="workspace-shell__project-header">
      <slot name="projectHeader" />
    </div>
  {/if}

  {#if $$slots.surfaceTabs}
    <div class="workspace-shell__surface-tabs">
      <slot name="surfaceTabs" />
    </div>
  {/if}

  {#if state === "ready"}
    <div class="workspace-shell__body">
      <slot />
    </div>
  {:else}
    <div class="workspace-shell__state" data-state={state}>
      {#if $$slots.state}
        <slot name="state" />
      {:else}
        <strong>{stateTitle ?? "Workspace state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}

  {#if $$slots.statusBar}
    <div class="workspace-shell__status-bar">
      <slot name="statusBar" />
    </div>
  {/if}

  {#if $$slots.overlay}
    <div class="workspace-shell__overlay">
      <slot name="overlay" />
    </div>
  {/if}
</section>

<style>
  .workspace-shell {
    position: relative;
    display: grid;
    gap: 0;
    min-height: 35rem;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    overflow: clip;
    background:
      radial-gradient(circle at top right, color-mix(in srgb, var(--pug-color-accent-base) 10%, transparent), transparent 32%),
      linear-gradient(180deg, color-mix(in srgb, var(--pug-color-background-panel) 98%, transparent), var(--pug-color-background-canvas));
  }

  .workspace-shell[data-disabled="true"] {
    opacity: 0.82;
  }

  .workspace-shell__body,
  .workspace-shell__state {
    min-height: 22.5rem;
  }

  .workspace-shell__body {
    display: grid;
  }

  .workspace-shell__state {
    display: grid;
    gap: var(--pug-space-stack-sm);
    padding: calc(var(--pug-space-panel-y) * 2) calc(var(--pug-space-panel-x) * 1.5);
    align-content: start;
  }

  .workspace-shell__state strong,
  .workspace-shell__state p {
    margin: 0;
  }

  .workspace-shell__state p {
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .workspace-shell__overlay {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }
</style>

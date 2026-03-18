<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Icon } from "@pug/svelte-primitives";

  import type { HostedSurfaceState } from "./types";

  export let surfaceId: string;
  export let title: string;
  export let state: HostedSurfaceState = "ready";
  export let stateMessage: string | null = null;
  export let hosting: "embedded" | "detached" = "embedded";
  export let isActive = false;
  export let isDetachable = false;
  export let isReloadable = false;
  export let isClosable = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    requestDetach: { surfaceId: string };
    requestAttach: { surfaceId: string };
    requestReload: { surfaceId: string };
    requestClose: { surfaceId: string };
    focusChange: { surfaceId: string; isFocused: boolean };
  }>();

  $: isReady = state === "ready";
  $: isDegraded = state === "degraded";
  $: showOverlay = !isReady && !isDegraded;

  $: stateLabel = ({
    ready: "",
    loading: "Loading…",
    unavailable: stateMessage ?? "Unavailable",
    blocked: stateMessage ?? "Blocked",
    degraded: stateMessage ?? "Degraded",
  })[state];

  function handleFocusIn(): void {
    dispatch("focusChange", { surfaceId, isFocused: true });
  }

  function handleFocusOut(): void {
    dispatch("focusChange", { surfaceId, isFocused: false });
  }
</script>

<section
  class="hosted-surface"
  data-state={state}
  data-hosting={hosting}
  data-active={isActive || undefined}
  aria-label={ariaLabel ?? title}
  role="region"
  on:focusin={handleFocusIn}
  on:focusout={handleFocusOut}
>
  <header class="hosted-surface__header">
    <span class="hosted-surface__title">{title}</span>

    {#if isDegraded}
      <span class="hosted-surface__status" aria-label="Degraded">⚠</span>
    {/if}

    {#if hosting === "detached"}
      <span class="hosted-surface__badge">Detached</span>
    {/if}

    <div class="hosted-surface__actions">
      {#if isReloadable}
        <button
          type="button"
          class="hosted-surface__action"
          aria-label="Reload {title}"
          on:click={() => dispatch("requestReload", { surfaceId })}
        >
          <Icon name="refresh-cw" size="sm" />
        </button>
      {/if}

      {#if isDetachable}
        <button
          type="button"
          class="hosted-surface__action"
          aria-label={hosting === "embedded" ? `Detach ${title}` : `Attach ${title}`}
          on:click={() => {
            if (hosting === "embedded") {
              dispatch("requestDetach", { surfaceId });
            } else {
              dispatch("requestAttach", { surfaceId });
            }
          }}
        >
          <Icon name={hosting === "embedded" ? "external-link" : "corner-down-left"} size="sm" />
        </button>
      {/if}

      {#if isClosable}
        <button
          type="button"
          class="hosted-surface__action"
          aria-label="Close {title}"
          on:click={() => dispatch("requestClose", { surfaceId })}
        >
          <Icon name="x" size="sm" />
        </button>
      {/if}
    </div>
  </header>

  <div class="hosted-surface__viewport">
    <slot />

    {#if showOverlay}
      <div class="hosted-surface__overlay" aria-live="polite">
        {#if state === "loading"}
          <div class="hosted-surface__loading">
            <span>Loading…</span>
          </div>
        {:else}
          <div class="hosted-surface__status-message">
            <strong>{stateLabel}</strong>
            {#if stateMessage && state !== "loading"}
              <p>{stateMessage}</p>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</section>

<style>
  .hosted-surface {
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    min-height: 0;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-surface, 0.5rem);
    background: var(--pug-color-background-panel);
    overflow: hidden;
  }

  .hosted-surface[data-active] {
    border-color: var(--pug-color-accent-base);
  }

  .hosted-surface__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.625rem;
    border-bottom: 0.0625rem solid var(--pug-color-border-subtle);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 36%, transparent);
    font-size: 0.8125rem;
  }

  .hosted-surface__title {
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .hosted-surface__status {
    font-size: 0.75rem;
  }

  .hosted-surface__badge {
    font-size: 0.625rem;
    padding: 0.0625rem 0.375rem;
    border-radius: 999rem;
    background: var(--pug-color-surface-active);
    color: var(--pug-color-text-secondary);
    white-space: nowrap;
  }

  .hosted-surface__actions {
    display: flex;
    align-items: center;
    gap: 0.125rem;
    margin-left: auto;
  }

  .hosted-surface__action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.125rem;
    border: 0;
    border-radius: var(--pug-radius-sm, 0.25rem);
    background: transparent;
    color: var(--pug-color-text-muted);
    cursor: pointer;
  }

  .hosted-surface__action:hover {
    background: var(--pug-color-surface-hover);
    color: var(--pug-color-text-default);
  }

  .hosted-surface__action:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .hosted-surface__viewport {
    position: relative;
    min-height: 0;
    overflow: hidden;
  }

  .hosted-surface__overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--pug-color-background-panel) 90%, transparent);
    z-index: 1;
  }

  .hosted-surface__loading {
    color: var(--pug-color-text-muted);
    font-size: 0.8125rem;
  }

  .hosted-surface__status-message {
    text-align: center;
    color: var(--pug-color-text-secondary);
    font-size: 0.8125rem;
    padding: 1.5rem;
  }

  .hosted-surface__status-message strong {
    display: block;
    margin-bottom: 0.25rem;
  }

  .hosted-surface__status-message p {
    margin: 0;
    color: var(--pug-color-text-muted);
    font-size: 0.75rem;
  }
</style>

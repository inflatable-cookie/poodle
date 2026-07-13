<script lang="ts">
  import "@poodle/styles/detail-shell.css";
  import type { Snippet } from "svelte";
  import { default as Spinner } from "./Spinner.svelte";

  import type { BrowseState } from "./types";

  interface Props {
    title?: string | null;
    scrollMode?: "shell" | "body";
    state?: Exclude<BrowseState, "no-results">;
    ariaLabel?: string | null;
    stateTitle?: string | null;
    stateMessage?: string | null;
    header?: Snippet;
    stateContent?: Snippet;
    children?: Snippet;
  }

  let {
    title = null,
    scrollMode = "body",
    state = "ready",
    ariaLabel = null,
    stateTitle = null,
    stateMessage = null,
    header,
    stateContent,
    children,
  }: Props = $props();
</script>

<section class="poodle-detail-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
  {#if header || title}
    <div class="poodle-detail-shell__header">
      {#if header}
        {@render header()}
      {:else if title}
        <h2>{title}</h2>
      {/if}
    </div>
  {/if}

  {#if state === "ready"}
    <div class="poodle-detail-shell__body">
      {@render children?.()}
    </div>
  {:else}
    <div class="poodle-detail-shell__state" data-state={state}>
      {#if stateContent}
        {@render stateContent()}
      {:else}
        {#if state === "loading"}
          <span class="poodle-detail-shell__spinner" aria-hidden="true">
            <Spinner variant="grid" tone="accent" />
          </span>
        {/if}
        <strong>{stateTitle ?? "Detail state"}</strong>
        {#if stateMessage}
          <p>{stateMessage}</p>
        {/if}
      {/if}
    </div>
  {/if}
</section>


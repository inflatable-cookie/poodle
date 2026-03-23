<script lang="ts">
  import { pxToRem } from "@flint/svelte-tokens";
  import type { SkeletonShape, SkeletonPreset } from "./types";

  export let shape: SkeletonShape = "line";
  export let preset: SkeletonPreset | null = null;
  export let width: string | null = null;
  export let height: string | null = null;
  export let lines: number = 3;
  export let isAnimated = true;

  $: resolvedWidth =
    width ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? "100%" : "100%");
  $: resolvedHeight =
    height ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? pxToRem(96) : pxToRem(14));
</script>

{#if preset === "table-row"}
  <div class="skeleton-preset skeleton-preset--table-row" data-animated={isAnimated} aria-hidden="true">
    {#each { length: 4 } as _, i}
      <span class="skeleton skeleton--cell" style="--flint-skeleton-width: {i === 0 ? '40%' : i === 3 ? '20%' : '60%'}"></span>
    {/each}
  </div>
{:else if preset === "card"}
  <div class="skeleton-preset skeleton-preset--card" data-animated={isAnimated} aria-hidden="true">
    <span class="skeleton skeleton--block-header"></span>
    <div class="skeleton-preset__card-body">
      <span class="skeleton skeleton--line" style="--flint-skeleton-width: 80%"></span>
      <span class="skeleton skeleton--line" style="--flint-skeleton-width: 100%"></span>
      <span class="skeleton skeleton--line" style="--flint-skeleton-width: 60%"></span>
    </div>
    <div class="skeleton-preset__card-footer">
      <span class="skeleton skeleton--pill"></span>
      <span class="skeleton skeleton--pill"></span>
    </div>
  </div>
{:else if preset === "list-item"}
  <div class="skeleton-preset skeleton-preset--list-item" data-animated={isAnimated} aria-hidden="true">
    <span class="skeleton skeleton--avatar"></span>
    <div class="skeleton-preset__list-text">
      <span class="skeleton skeleton--line" style="--flint-skeleton-width: 60%"></span>
      <span class="skeleton skeleton--line-sm" style="--flint-skeleton-width: 40%"></span>
    </div>
  </div>
{:else if preset === "detail-section"}
  <div class="skeleton-preset skeleton-preset--detail" data-animated={isAnimated} aria-hidden="true">
    <span class="skeleton skeleton--heading"></span>
    {#each { length: lines } as _}
      <div class="skeleton-preset__detail-row">
        <span class="skeleton skeleton--label"></span>
        <span class="skeleton skeleton--value"></span>
      </div>
    {/each}
  </div>
{:else if preset === "avatar-line"}
  <div class="skeleton-preset skeleton-preset--avatar-line" data-animated={isAnimated} aria-hidden="true">
    <span class="skeleton skeleton--avatar"></span>
    <span class="skeleton skeleton--line" style="--flint-skeleton-width: 10rem"></span>
  </div>
{:else}
  <span
    class="skeleton"
    data-shape={shape}
    data-animated={isAnimated}
    style={`--flint-skeleton-width: ${resolvedWidth}; --flint-skeleton-height: ${resolvedHeight};`}
    aria-hidden="true"
  ></span>
{/if}

<style>
  .skeleton {
    display: block;
    width: var(--flint-skeleton-width, 100%);
    height: var(--flint-skeleton-height, 0.875rem);
    border-radius: var(--flint-radius-control);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--flint-color-background-elevated) 88%, transparent) 0%,
        color-mix(in srgb, var(--flint-color-background-surface) 92%, white) 48%,
        color-mix(in srgb, var(--flint-color-background-elevated) 88%, transparent) 100%
      );
    background-size: 220% 100%;
  }

  .skeleton[data-shape="circle"] {
    border-radius: 999rem;
  }

  .skeleton[data-shape="block"] {
    border-radius: calc(var(--flint-radius-surface) - 0.25rem);
  }

  .skeleton[data-animated="true"],
  .skeleton-preset[data-animated="true"] .skeleton {
    animation: skeleton-shimmer 1.6s linear infinite;
  }

  @keyframes skeleton-shimmer {
    from {
      background-position: 200% 0;
    }

    to {
      background-position: -20% 0;
    }
  }

  /* ── Preset shared ───────────────────────────────────────── */

  .skeleton-preset {
    display: flex;
    flex-direction: column;
  }

  /* ── table-row ───────────────────────────────────────────── */

  .skeleton-preset--table-row {
    flex-direction: row;
    gap: 0.75rem;
    padding: 0.625rem 0;
    border-bottom: 1px solid color-mix(in srgb, var(--flint-color-border-subtle) 42%, transparent);
  }

  .skeleton--cell {
    height: 0.875rem;
    flex: 1;
  }

  /* ── card ─────────────────────────────────────────────────── */

  .skeleton-preset--card {
    gap: 0.75rem;
    padding: 1rem;
    border: 1px solid color-mix(in srgb, var(--flint-color-border-default) 42%, transparent);
    border-radius: var(--flint-radius-surface);
  }

  .skeleton--block-header {
    height: 6rem;
    border-radius: calc(var(--flint-radius-surface) - 0.375rem);
  }

  .skeleton-preset__card-body {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .skeleton-preset__card-footer {
    display: flex;
    gap: 0.5rem;
    padding-top: 0.25rem;
  }

  .skeleton--pill {
    width: 3.5rem;
    height: 1.25rem;
    border-radius: 999rem;
  }

  /* ── list-item ───────────────────────────────────────────── */

  .skeleton-preset--list-item {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
  }

  .skeleton--avatar {
    width: 2.25rem;
    height: 2.25rem;
    flex-shrink: 0;
    border-radius: 999rem;
  }

  .skeleton-preset__list-text {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    flex: 1;
    min-width: 0;
  }

  .skeleton--line {
    height: 0.875rem;
  }

  .skeleton--line-sm {
    height: 0.6875rem;
  }

  /* ── detail-section ──────────────────────────────────────── */

  .skeleton-preset--detail {
    gap: 0.625rem;
  }

  .skeleton--heading {
    width: 8rem;
    height: 1rem;
    margin-bottom: 0.25rem;
  }

  .skeleton-preset__detail-row {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .skeleton--label {
    width: 6rem;
    height: 0.75rem;
    flex-shrink: 0;
  }

  .skeleton--value {
    height: 0.75rem;
    flex: 1;
    max-width: 14rem;
  }

  /* ── avatar-line ─────────────────────────────────────────── */

  .skeleton-preset--avatar-line {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
  }
</style>

<script lang="ts">
  import { pxToRem } from "@poodle/svelte-tokens";
  import type { SkeletonShape, SkeletonPreset } from "./types";

  export let shape: SkeletonShape = "line";
  export let preset: SkeletonPreset | null = null;
  export let width: string | null = null;
  export let height: string | null = null;
  export let lines: number = 3;
  export let animated = true;

  $: resolvedWidth =
    width ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? "100%" : "100%");
  $: resolvedHeight =
    height ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? pxToRem(96) : pxToRem(14));
</script>

{#if preset === "table-row"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--table-row" data-animated={animated} aria-hidden="true">
    {#each { length: 4 } as _, i}
      <span class="poodle-skeleton poodle-skeleton--cell" style="--poodle-skeleton-width: {i === 0 ? '40%' : i === 3 ? '20%' : '60%'}"></span>
    {/each}
  </div>
{:else if preset === "card"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--card" data-animated={animated} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--block-header"></span>
    <div class="poodle-skeleton-preset__card-body">
      <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 80%"></span>
      <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 100%"></span>
      <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 60%"></span>
    </div>
    <div class="poodle-skeleton-preset__card-footer">
      <span class="poodle-skeleton poodle-skeleton--pill"></span>
      <span class="poodle-skeleton poodle-skeleton--pill"></span>
    </div>
  </div>
{:else if preset === "list-item"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--list-item" data-animated={animated} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--avatar"></span>
    <div class="poodle-skeleton-preset__list-text">
      <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 60%"></span>
      <span class="poodle-skeleton poodle-skeleton--line-sm" style="--poodle-skeleton-width: 40%"></span>
    </div>
  </div>
{:else if preset === "detail-section"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--detail" data-animated={animated} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--heading"></span>
    {#each { length: lines } as _}
      <div class="poodle-skeleton-preset__detail-item">
        <span class="poodle-skeleton poodle-skeleton--label"></span>
        <span class="poodle-skeleton poodle-skeleton--value"></span>
      </div>
    {/each}
  </div>
{:else if preset === "avatar-line"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--avatar-line" data-animated={animated} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--avatar"></span>
    <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 10rem"></span>
  </div>
{:else}
  <span
    class="poodle-skeleton"
    data-shape={shape}
    data-animated={animated}
    style={`--poodle-skeleton-width: ${resolvedWidth}; --poodle-skeleton-height: ${resolvedHeight};`}
    aria-hidden="true"
  ></span>
{/if}

<style>
  .poodle-skeleton {
    display: block;
    width: var(--poodle-skeleton-width, 100%);
    height: var(--poodle-skeleton-height, 0.875rem);
    border-radius: var(--poodle-radius-control);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent) 0%,
        color-mix(in srgb, var(--poodle-color-background-surface) 92%, white) 48%,
        color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent) 100%
      );
    background-size: 220% 100%;
  }

  .poodle-skeleton[data-shape="circle"] {
    border-radius: 999rem;
  }

  .poodle-skeleton[data-shape="block"] {
    border-radius: calc(var(--poodle-radius-surface) - 0.25rem);
  }

  .poodle-skeleton[data-animated="true"],
  .poodle-skeleton-preset[data-animated="true"] .poodle-skeleton {
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

  .poodle-skeleton-preset {
    display: flex;
    flex-direction: column;
  }

  /* ── table-row ───────────────────────────────────────────── */

  .poodle-skeleton-preset--table-row {
    flex-direction: row;
    gap: 0.75rem;
    padding: 0.625rem 0;
    border-bottom: 1px solid color-mix(in srgb, var(--poodle-color-border-subtle) 42%, transparent);
  }

  .poodle-skeleton--cell {
    height: 0.875rem;
    flex: 1;
  }

  /* ── card ─────────────────────────────────────────────────── */

  .poodle-skeleton-preset--card {
    gap: 0.75rem;
    padding: 1rem;
    border: 1px solid color-mix(in srgb, var(--poodle-color-border-default) 42%, transparent);
    border-radius: var(--poodle-radius-surface);
  }

  .poodle-skeleton--block-header {
    height: 6rem;
    border-radius: calc(var(--poodle-radius-surface) - 0.375rem);
  }

  .poodle-skeleton-preset__card-body {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .poodle-skeleton-preset__card-footer {
    display: flex;
    gap: 0.5rem;
    padding-top: 0.25rem;
  }

  .poodle-skeleton--pill {
    width: 3.5rem;
    height: 1.25rem;
    border-radius: 999rem;
  }

  /* ── list-item ───────────────────────────────────────────── */

  .poodle-skeleton-preset--list-item {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
  }

  .poodle-skeleton--avatar {
    width: 2.25rem;
    height: 2.25rem;
    flex-shrink: 0;
    border-radius: 999rem;
  }

  .poodle-skeleton-preset__list-text {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    flex: 1;
    min-width: 0;
  }

  .poodle-skeleton--line {
    height: 0.875rem;
  }

  .poodle-skeleton--line-sm {
    height: 0.6875rem;
  }

  /* ── detail-section ──────────────────────────────────────── */

  .poodle-skeleton-preset--detail {
    gap: 0.625rem;
  }

  .poodle-skeleton--heading {
    width: 8rem;
    height: 1rem;
    margin-bottom: 0.25rem;
  }

  .poodle-skeleton-preset__detail-item {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .poodle-skeleton--label {
    width: 6rem;
    height: 0.75rem;
    flex-shrink: 0;
  }

  .poodle-skeleton--value {
    height: 0.75rem;
    flex: 1;
    max-width: 14rem;
  }

  /* ── avatar-line ─────────────────────────────────────────── */

  .poodle-skeleton-preset--avatar-line {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
  }
</style>

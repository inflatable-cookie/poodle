<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/skeleton.css";
  import { pxToRem } from "@inflatable-cookie/poodle-core/tokens";
  import { useMotionReady } from "./motion-ready.svelte";
  import type { SkeletonShape, SkeletonPreset } from "./types";

  let {
    shape = "line",
    preset = null,
    width = null,
    height = null,
    lines = 3,
    animated = true,
  }: {
    shape?: SkeletonShape;
    preset?: SkeletonPreset | null;
    width?: string | null;
    height?: string | null;
    lines?: number;
    animated?: boolean;
  } = $props();

  const motionReady = useMotionReady(() => animated);

  const resolvedWidth = $derived(
    width ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? "100%" : "100%")
  );
  const resolvedHeight = $derived(
    height ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? pxToRem(96) : pxToRem(14))
  );
</script>

{#if preset === "table-row"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--table-row" data-animated={animated} data-motion-ready={animated && motionReady.ready} aria-hidden="true">
    {#each { length: 4 } as _, i}
      <span class="poodle-skeleton poodle-skeleton--cell" style="--poodle-skeleton-width: {i === 0 ? '40%' : i === 3 ? '20%' : '60%'}"></span>
    {/each}
  </div>
{:else if preset === "card"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--card" data-animated={animated} data-motion-ready={animated && motionReady.ready} aria-hidden="true">
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
  <div class="poodle-skeleton-preset poodle-skeleton-preset--list-item" data-animated={animated} data-motion-ready={animated && motionReady.ready} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--avatar"></span>
    <div class="poodle-skeleton-preset__list-text">
      <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 60%"></span>
      <span class="poodle-skeleton poodle-skeleton--line-sm" style="--poodle-skeleton-width: 40%"></span>
    </div>
  </div>
{:else if preset === "detail-section"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--detail" data-animated={animated} data-motion-ready={animated && motionReady.ready} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--heading"></span>
    {#each { length: lines } as _}
      <div class="poodle-skeleton-preset__detail-item">
        <span class="poodle-skeleton poodle-skeleton--label"></span>
        <span class="poodle-skeleton poodle-skeleton--value"></span>
      </div>
    {/each}
  </div>
{:else if preset === "avatar-line"}
  <div class="poodle-skeleton-preset poodle-skeleton-preset--avatar-line" data-animated={animated} data-motion-ready={animated && motionReady.ready} aria-hidden="true">
    <span class="poodle-skeleton poodle-skeleton--avatar"></span>
    <span class="poodle-skeleton poodle-skeleton--line" style="--poodle-skeleton-width: 10rem"></span>
  </div>
{:else}
  <span
    class="poodle-skeleton"
    data-shape={shape}
    data-animated={animated} data-motion-ready={animated && motionReady.ready}
    style={`--poodle-skeleton-width: ${resolvedWidth}; --poodle-skeleton-height: ${resolvedHeight};`}
    aria-hidden="true"
  ></span>
{/if}


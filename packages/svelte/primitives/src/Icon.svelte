<script lang="ts">
  import type { ControlSize } from "./types";
  import type { IconNodes } from "./icon-registry";

  import { resolveIconNodes, getIconSet } from "./icon-registry";

  /**
   * The icon to display. Accepts:
   * - An `IconNodes` array (e.g. from `lucide-static/icon-nodes.json`)
   * - A string name resolved from the `IconProvider` icon set or built-in internals
   */
  export let icon: IconNodes | string | null = null;
  /** @deprecated Use `icon` instead. Alias kept for internal convenience. */
  export let name: string | null = null;
  export let size: ControlSize = "md";
  export let ariaLabel: string | null = null;

  const iconSet = getIconSet();

  $: resolvedIcon = icon ?? name;
  $: nodes = resolveIconNodes(resolvedIcon, iconSet);
</script>

<svg
  class="pug-icon"
  data-size={size}
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  role={ariaLabel ? "img" : "presentation"}
  aria-label={ariaLabel ?? undefined}
  aria-hidden={ariaLabel ? undefined : "true"}
>
  {#each nodes as [tag, attrs]}
    {#if tag === "path"}
      <path {...attrs} />
    {:else if tag === "circle"}
      <circle {...attrs} />
    {:else if tag === "rect"}
      <rect {...attrs} />
    {:else if tag === "line"}
      <line {...attrs} />
    {:else if tag === "polyline"}
      <polyline {...attrs} />
    {:else if tag === "polygon"}
      <polygon {...attrs} />
    {:else if tag === "ellipse"}
      <ellipse {...attrs} />
    {/if}
  {/each}
</svg>

<style>
  .pug-icon {
    display: inline-block;
    width: var(--pug-size-icon-md);
    height: var(--pug-size-icon-md);
    vertical-align: middle;
    flex-shrink: 0;
  }

  .pug-icon[data-size="sm"] {
    width: var(--pug-size-icon-sm);
    height: var(--pug-size-icon-sm);
  }

  .pug-icon[data-size="lg"] {
    width: var(--pug-size-icon-lg);
    height: var(--pug-size-icon-lg);
  }
</style>

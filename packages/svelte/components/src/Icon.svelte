<script lang="ts">
  import "@inflatable-cookie/poodle-styles/icon.css";
  import { fromStore } from "svelte/store";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { ControlSize } from "./types";
  import type { IconNodes } from "./icon-registry";

  import { resolveIconNodes, getIconSetStore } from "./icon-registry";

  /**
   * The icon to display. Accepts:
   * - An `IconNodes` array (e.g. from `@inflatable-cookie/poodle-icons-lucide` or `lucide-static/icon-nodes.json`)
   * - A string name resolved from the `IconProvider` icon set, or lazily
   *   auto-imported from `@inflatable-cookie/poodle-icons-lucide`
   */
  let {
    icon = null,
    name = null,
    size = null,
    sizeRole = "chrome",
    density = null,
    ariaLabel = null,
  }: {
    icon?: IconNodes | string | null;
    name?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    ariaLabel?: string | null;
  } = $props();

  const iconSet = fromStore(getIconSetStore());
  const uiPresentation = getUiPresentation();

  const resolvedIcon = $derived(icon ?? name);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const nodes = $derived(resolveIconNodes(resolvedIcon, iconSet.current));
</script>

<svg
  class="poodle-icon"
  data-size={resolvedSize}
  data-density={resolvedDensity}
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


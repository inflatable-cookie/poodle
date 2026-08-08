<script lang="ts">
  import "@inflatable-cookie/poodle-styles/spinner.css";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { SpinnerSize, SpinnerTone, SpinnerVariant } from "./types";

  let {
    variant = "ring",
    size = null,
    sizeRole = "control",
    density = null,
    tone = "current",
    ariaLabel = null,
    class: className = "",
    style = null,
    ...restProps
  }: {
    variant?: SpinnerVariant;
    size?: SpinnerSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    tone?: SpinnerTone;
    ariaLabel?: string | null;
    class?: string;
    style?: string | null;
  } & Record<string, unknown> = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(
    (size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole)) as SpinnerSize,
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const gridCells = [
    { id: 0, phase: "tl" },
    { id: 1, phase: "tr" },
    { id: 2, phase: "ml" },
    { id: 3, phase: "mr" },
    { id: 4, phase: "bl" },
    { id: 5, phase: "br" },
  ];
</script>

<span
  {...restProps}
  class={`poodle-spinner ${className}`.trim()}
  data-variant={variant}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-tone={tone}
  style={style ?? undefined}
  role={ariaLabel ? "status" : undefined}
  aria-label={ariaLabel ?? undefined}
  aria-live={ariaLabel ? "polite" : undefined}
  aria-hidden={ariaLabel ? undefined : "true"}
>
  {#if variant === "ring"}
    <span class="poodle-spinner__ring" aria-hidden="true"></span>
  {:else if variant === "dots"}
    <span class="poodle-spinner__dots" aria-hidden="true">
      {#each [0, 1, 2] as phase (phase)}
        <span class="poodle-spinner__dot" data-phase={phase}></span>
      {/each}
    </span>
  {:else}
    <span class="poodle-spinner__grid" aria-hidden="true">
      {#each gridCells as cell}
        <span
          class="poodle-spinner__cell"
          data-phase={cell.phase}
        ></span>
      {/each}
    </span>
  {/if}
</span>


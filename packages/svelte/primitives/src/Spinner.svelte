<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { SpinnerSize, SpinnerTone, SpinnerVariant } from "./types";

  export let variant: SpinnerVariant = "ring";
  export let size: SpinnerSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let tone: SpinnerTone = "current";
  export let ariaLabel: string | null = null;
  export let className = "";
  export let style: string | null = null;
  const uiPresentation = getUiPresentation();
  $: resolvedSize =
    (size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole)) as SpinnerSize;
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";

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
  {...$$restProps}
  class={`spinner ${className}`.trim()}
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
    <span class="spinner__ring" aria-hidden="true"></span>
  {:else}
    <span class="spinner__grid" aria-hidden="true">
      {#each gridCells as cell}
        <span
          class="spinner__cell"
          data-phase={cell.phase}
        ></span>
      {/each}
    </span>
  {/if}
</span>

<style>
  .spinner {
    --poodle-spinner-color: currentColor;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--poodle-spinner-color);
  }

  .spinner[data-tone="accent"] {
    --poodle-spinner-color: var(--poodle-color-accent-base);
  }

  .spinner[data-tone="muted"] {
    --poodle-spinner-color: var(--poodle-color-text-secondary);
  }

  .spinner[data-size="sm"] {
    --poodle-spinner-ring-size: 0.75rem;
    --poodle-spinner-grid-width: 0.4375rem;
    --poodle-spinner-grid-height: 0.6875rem;
    --poodle-spinner-grid-gap: 0.078125rem;
  }

  .spinner[data-size="xs"] {
    --poodle-spinner-ring-size: 0.625rem;
    --poodle-spinner-grid-width: 0.375rem;
    --poodle-spinner-grid-height: 0.5625rem;
    --poodle-spinner-grid-gap: 0.0625rem;
  }

  .spinner[data-size="md"] {
    --poodle-spinner-ring-size: 1rem;
    --poodle-spinner-grid-width: 0.5625rem;
    --poodle-spinner-grid-height: 0.9375rem;
    --poodle-spinner-grid-gap: 0.09375rem;
  }

  .spinner[data-size="lg"] {
    --poodle-spinner-ring-size: 1.5rem;
    --poodle-spinner-grid-width: 0.75rem;
    --poodle-spinner-grid-height: 1.25rem;
    --poodle-spinner-grid-gap: 0.125rem;
  }

  .spinner[data-size="xl"] {
    --poodle-spinner-ring-size: 1.875rem;
    --poodle-spinner-grid-width: 0.9375rem;
    --poodle-spinner-grid-height: 1.5625rem;
    --poodle-spinner-grid-gap: 0.15625rem;
  }

  .spinner__ring {
    width: var(--poodle-spinner-ring-size);
    height: var(--poodle-spinner-ring-size);
    border: 0.125rem solid color-mix(in srgb, currentColor 24%, transparent);
    border-top-color: currentColor;
    border-radius: 999px;
    box-sizing: border-box;
    animation: poodle-spinner-ring 0.8s linear infinite;
  }

  .spinner__grid {
    display: inline-grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(3, 1fr);
    gap: var(--poodle-spinner-grid-gap);
    width: var(--poodle-spinner-grid-width);
    height: var(--poodle-spinner-grid-height);
  }

  .spinner__cell {
    width: 100%;
    height: 100%;
    border-radius: 0.125rem;
    background: currentColor;
    opacity: 0.2;
    animation: poodle-spinner-grid-idle 1.24s linear infinite;
  }

  .spinner__cell[data-phase="tl"] {
    animation-name: poodle-spinner-grid-tl;
  }

  .spinner__cell[data-phase="tr"] {
    animation-name: poodle-spinner-grid-tr;
  }

  .spinner__cell[data-phase="mr"] {
    animation-name: poodle-spinner-grid-mr;
  }

  .spinner__cell[data-phase="ml"] {
    animation-name: poodle-spinner-grid-ml;
  }

  .spinner__cell[data-phase="bl"] {
    animation-name: poodle-spinner-grid-bl;
  }

  .spinner__cell[data-phase="br"] {
    animation-name: poodle-spinner-grid-br;
  }

  @keyframes poodle-spinner-ring {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes poodle-spinner-grid-idle {
    0%,
    100% {
      opacity: 0.2;
    }
  }

  @keyframes poodle-spinner-grid-tl {
    0%,
    100% {
      opacity: 0.76;
    }

    8% {
      opacity: 0.58;
    }

    16% {
      opacity: 0.34;
    }

    24%,
    84% {
      opacity: 0.2;
    }

    92% {
      opacity: 0.34;
    }
  }

  @keyframes poodle-spinner-grid-tr {
    0%,
    4%,
    100% {
      opacity: 0.2;
    }

    10% {
      opacity: 0.34;
    }

    18% {
      opacity: 0.58;
    }

    26% {
      opacity: 0.76;
    }

    34% {
      opacity: 0.58;
    }

    42% {
      opacity: 0.34;
    }
  }

  @keyframes poodle-spinner-grid-mr {
    0%,
    100% {
      opacity: 0.2;
    }

    14%,
    58% {
      opacity: 0.2;
    }

    20%,
    64% {
      opacity: 0.34;
    }

    28%,
    72% {
      opacity: 0.58;
    }

    36%,
    80% {
      opacity: 0.76;
    }

    44%,
    88% {
      opacity: 0.58;
    }

    52%,
    96% {
      opacity: 0.38;
    }
  }

  @keyframes poodle-spinner-grid-ml {
    0%,
    100% {
      opacity: 0.2;
    }

    26%,
    70% {
      opacity: 0.2;
    }

    32%,
    76% {
      opacity: 0.34;
    }

    40%,
    84% {
      opacity: 0.58;
    }

    48%,
    92% {
      opacity: 0.76;
    }

    56% {
      opacity: 0.58;
    }

    64% {
      opacity: 0.34;
    }
  }

  @keyframes poodle-spinner-grid-bl {
    0%,
    100% {
      opacity: 0.2;
    }

    38% {
      opacity: 0.2;
    }

    44% {
      opacity: 0.34;
    }

    52% {
      opacity: 0.58;
    }

    60% {
      opacity: 0.76;
    }

    68% {
      opacity: 0.58;
    }

    76% {
      opacity: 0.34;
    }
  }

  @keyframes poodle-spinner-grid-br {
    0%,
    100% {
      opacity: 0.2;
    }

    50% {
      opacity: 0.2;
    }

    56% {
      opacity: 0.34;
    }

    64% {
      opacity: 0.58;
    }

    72% {
      opacity: 0.76;
    }

    80% {
      opacity: 0.58;
    }

    88% {
      opacity: 0.34;
    }
  }
</style>

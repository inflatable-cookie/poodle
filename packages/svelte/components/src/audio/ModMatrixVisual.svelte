<script lang="ts">
  import type { ModMatrixVisualState } from "@inflatable-cookie/poodle-core";
  let { visualState }: { visualState: ModMatrixVisualState } = $props();
</script>

<span class="poodle-mod-matrix-grid-visual" aria-hidden="true">
  <span></span>
  {#each visualState.destinations as destination (destination.id)}<span class="poodle-mod-matrix-grid-visual__header">{destination.label}</span>{/each}
  {#each visualState.sources as source (source.id)}
    <span class="poodle-mod-matrix-grid-visual__header">{source.label}</span>
    {#each visualState.cells.filter((cell) => cell.sourceId === source.id) as cell (`${cell.sourceId}:${cell.destinationId}`)}
      <span class="poodle-mod-matrix-grid-visual__cell poodle-slider" data-variant="embedded" data-orientation="horizontal" data-polarity={cell.parameters.min < 0 && cell.parameters.max > 0 ? "bipolar" : "unipolar"} data-source-id={cell.sourceId} data-destination-id={cell.destinationId} data-enabled={cell.enabled} data-negative={cell.amountNorm < cell.zeroNorm} data-focused={cell.focused} style={`--poodle-slider-center:${cell.zeroNorm * 100}%;--poodle-slider-fill-start:${cell.fillStartNorm * 100}%;--poodle-slider-fill-span:${cell.fillSpanNorm * 100}%`}>
        <span class="poodle-slider__track"><span class="poodle-slider__fill"></span><span class="poodle-slider__center"></span></span>
      </span>
    {/each}
  {/each}
</span>

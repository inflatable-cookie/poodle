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
      <span class="poodle-mod-matrix-grid-visual__cell" data-source-id={cell.sourceId} data-destination-id={cell.destinationId} data-enabled={cell.enabled} data-negative={cell.amount < 0} data-focused={cell.focused}>
        <span class="poodle-mod-matrix-grid-visual__amount" style={`--poodle-mod-amount:${cell.amount}`}></span>
      </span>
    {/each}
  {/each}
</span>

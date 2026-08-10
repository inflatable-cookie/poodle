<script lang="ts">
  import type { WaveformVisualState } from "@inflatable-cookie/poodle-core";
  let { visualState }: { visualState: WaveformVisualState } = $props();
  const span = $derived(Math.max(visualState.visibleEnd - visualState.visibleStart, 1));
</script>

<span class="poodle-waveform-display-visual" data-focus={visualState.focus} data-enabled={visualState.enabled} aria-hidden="true">
  {#each visualState.columns as column}
    <span class="poodle-waveform-display-visual__column" style={`--poodle-wave-min:${column.min};--poodle-wave-max:${column.max}`}></span>
  {/each}
  {#if visualState.selection}
    <span class="poodle-waveform-display-visual__selection" style={`left:${(visualState.selection.start - visualState.visibleStart) / span * 100}%;width:${(visualState.selection.end - visualState.selection.start + 1) / span * 100}%`}></span>
  {/if}
  {#if visualState.cursorSample !== null}
    <span class="poodle-waveform-display-visual__cursor" style={`left:${(visualState.cursorSample - visualState.visibleStart) / span * 100}%`}></span>
  {/if}
</span>

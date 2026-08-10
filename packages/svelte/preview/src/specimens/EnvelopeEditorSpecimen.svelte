<script lang="ts">
  import type { EnvelopePoint } from "@inflatable-cookie/poodle-core";
  import { EnvelopeEditor } from "@inflatable-cookie/poodle-svelte";
  let points = $state<EnvelopePoint[]>([
    { id: "start", x: 0, y: 0, curve: 0.35 },
    { id: "attack", x: 0.18, y: 1, curve: -0.25 },
    { id: "decay", x: 0.48, y: 0.62, curve: 0.2 },
    { id: "end", x: 1, y: 0, curve: 0 },
  ]);
  const snap = (point: { x: number; y: number }) => ({ x: Math.round(point.x * 20) / 20, y: Math.round(point.y * 20) / 20 });
</script>

<div class="specimen"><EnvelopeEditor bind:points snapPoint={snap} ariaLabel="Amplitude envelope" /></div>
<style>.specimen { width: min(32rem, 80vw); }</style>

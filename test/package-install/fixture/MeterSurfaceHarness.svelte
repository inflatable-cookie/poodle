<script lang="ts">
  import type { MeterBus, MeterSurfacePainter } from "@inflatable-cookie/poodle-core";
  import { AudioMeter, MeterSurface } from "@inflatable-cookie/poodle-svelte";

  let { bus, painter }: { bus: MeterBus; painter: MeterSurfacePainter } = $props();
</script>

<MeterSurface {bus} {painter}>
  <AudioMeter surface={bus} channel="packed-channel" ariaLabel="Packed channel" segments={12} />
</MeterSurface>

<script lang="ts">
  import type { MeterBus, MeterFeedFrame, MeterSurfacePainter } from "@inflatable-cookie/poodle-core";
  import AudioMeter from "../src/AudioMeter.svelte";
  import MeterSurface from "../src/MeterSurface.svelte";

  interface MeterHandle {
    push(frame: MeterFeedFrame, channel?: "left" | "right"): void;
    resetClip(channel?: "left" | "right" | "both"): void;
  }

  let {
    bus,
    meterBus = null,
    painter = null,
    rightChannel = null,
    showSecond = true,
    firstSurface = true,
    firstChannel = "a",
  }: {
    bus: MeterBus;
    meterBus?: MeterBus | null;
    painter?: MeterSurfacePainter | null;
    rightChannel?: string | null;
    showSecond?: boolean;
    /** false renders the first meter in standalone tier inside the surface. */
    firstSurface?: boolean;
    firstChannel?: string;
  } = $props();

  const surfaceBus = $derived(meterBus ?? bus);
  let meterA = $state<MeterHandle | null>(null);

  export function meter(): MeterHandle | null {
    return meterA;
  }
</script>

<MeterSurface {bus} {painter}>
  <AudioMeter
    bind:this={meterA}
    surface={firstSurface ? surfaceBus : null}
    channel={firstSurface ? firstChannel : null}
    rightChannel={firstSurface ? rightChannel : null}
    ariaLabel="Channel A"
    segments={12}
  />
  {#if showSecond}
    <AudioMeter surface={surfaceBus} channel="b" ariaLabel="Channel B" segments={12} />
  {/if}
</MeterSurface>

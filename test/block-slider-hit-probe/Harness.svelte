<script lang="ts">
  import RangeSlider from "../../packages/svelte/components/src/RangeSlider.svelte";
  import Slider from "../../packages/svelte/components/src/Slider.svelte";

  let sliderXs = $state(50);
  let sliderCompact = $state(50);
  let rangeXs = $state<[number, number]>([20, 80]);
  let sliderHits = $state(0);
  let compactHits = $state(0);
  let rangeHits = $state(0);
  let sliderTrace = $state("idle");
  let compactTrace = $state("idle");
  let rangeTrace = $state("idle");
</script>

<section data-framework="svelte">
  <div data-case="slider-xs" style="width: 240px; padding: 24px;">
    <Slider
      appearance="block"
      size="xs"
      min={0}
      max={100}
      step={10}
      ariaLabel="Gain"
      bind:value={sliderXs}
      onValueChange={(next) => {
        sliderXs = next;
        sliderHits += 1;
        sliderTrace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={sliderHits}>{sliderTrace}</p>
  </div>
  <div data-case="slider-xs-compact" style="width: 240px; padding: 24px;">
    <Slider
      appearance="block"
      size="xs"
      density="compact"
      min={0}
      max={100}
      step={10}
      ariaLabel="Compact gain"
      bind:value={sliderCompact}
      onValueChange={(next) => {
        sliderCompact = next;
        compactHits += 1;
        compactTrace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={compactHits}>{compactTrace}</p>
  </div>
  <div data-case="range-xs" style="width: 240px; padding: 24px;">
    <RangeSlider
      appearance="block"
      size="xs"
      min={0}
      max={100}
      step={10}
      ariaLabel="Range"
      bind:value={rangeXs}
      onValueChange={(next) => {
        rangeXs = next;
        rangeHits += 1;
        rangeTrace = `change:${next.join(",")}`;
      }}
    />
    <p data-testid="trace" data-hits={rangeHits}>{rangeTrace}</p>
  </div>
</section>

<script lang="ts">
  import RangeSlider from "../../packages/svelte/components/src/RangeSlider.svelte";
  import Slider from "../../packages/svelte/components/src/Slider.svelte";
  import Button from "../../packages/svelte/components/src/Button.svelte";
  import TextInput from "../../packages/svelte/components/src/TextInput.svelte";

  // g18.024 layout probe: the five shared control heights, density
  // independence, alignment with same-size reference controls, and hit
  // rectangles that never enter the layout box.
  const sizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let sliderXs = $state(50);
  let sliderCompact = $state(50);
  let rangeXs = $state<[number, number]>([20, 80]);
  let sizeValues = $state<Record<string, number>>({ xs: 40, sm: 40, md: 40, lg: 40, xl: 40 });
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
  {#each sizes as size (size)}
    <!-- Mixed row: the Slider must align edge for edge with same-size
         reference controls (Button, TextInput) on the shared ladder. -->
    <div data-case={`slider-${size}-row`} class="g18-row">
      <Button size={size} ariaLabel={`Reference button at ${size}`}>Go</Button>
      <Slider
        {size}
        min={0}
        max={100}
        step={1}
        ariaLabel={`Standard slider at ${size}`}
        bind:value={sizeValues[size]}
      />
      <TextInput size={size} ariaLabel={`Reference input at ${size}`} value="88" />
    </div>
  {/each}
</section>

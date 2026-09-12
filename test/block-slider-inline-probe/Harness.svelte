<script lang="ts">
  import RangeSlider from "../../packages/svelte/components/src/RangeSlider.svelte";
  import Slider from "../../packages/svelte/components/src/Slider.svelte";

  // g18.017 inline-presentation probe. The three journey cases differ only in
  // thumb position; the formatted value text stays the same string so any
  // glyph movement is detectable. The constant formatter is the point: the
  // fill boundary moves across an identical glyph run.
  const sameValue = () => "67";

  let low = $state(10);
  let mid = $state(50);
  let high = $state(90);
  let collision = $state(12);
  let rtl = $state(20);
  let range = $state<[number, number]>([20, 80]);
  let vertical = $state(50);
  let fraction = $state(0.85);
  let rangeFraction = $state<[number, number]>([0.3, 0.85]);
  let rangeVertical = $state<[number, number]>([20, 80]);
  let dispatches = $state(0);
  let trace = $state("idle");
</script>

<section data-framework="svelte">
  <div data-case="slider-low" style="width: 240px; padding: 24px;">
    <Slider
      min={0}
      max={100}
      step={1}
      value={low}
      visibleLabel="Gain"
      ariaLabel="Gain"
      formatVisibleValue={sameValue}
      onValueChange={(next) => {
        low = next;
        dispatches += 1;
        trace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={dispatches}>{trace}</p>
  </div>
  <div data-case="slider-mid" style="width: 240px; padding: 24px;">
    <Slider
      min={0}
      max={100}
      step={1}
      value={mid}
      visibleLabel="Gain"
      ariaLabel="Gain"
      formatVisibleValue={sameValue}
      onValueChange={(next) => {
        mid = next;
        dispatches += 1;
        trace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={dispatches}>{trace}</p>
  </div>
  <div data-case="slider-high" style="width: 240px; padding: 24px;">
    <Slider
      min={0}
      max={100}
      step={1}
      value={high}
      visibleLabel="Gain"
      ariaLabel="Gain"
      formatVisibleValue={sameValue}
      onValueChange={(next) => {
        high = next;
        dispatches += 1;
        trace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={dispatches}>{trace}</p>
  </div>
  <div data-case="slider-collision" style="width: 64px; padding: 24px;">
    <Slider
      min={0}
      max={100}
      step={1}
      value={collision}
      visibleLabel="Compressor makeup gain"
      ariaLabel="Collision gain"
      onValueChange={(next) => {
        collision = next;
        dispatches += 1;
        trace = `change:${next}`;
      }}
    />
    <p data-testid="trace" data-hits={dispatches}>{trace}</p>
  </div>
  <!-- g18.022 vertical block: the value paints at the capsule's physical top
       and the optional label at its center — anchored to the whole capsule,
       not wherever the row happens to lay out. -->
  <div data-case="slider-vertical" style="width: 200px; height: 220px; padding: 24px;">
    <Slider
      orientation="vertical"
      min={0}
      max={100}
      step={1}
      value={vertical}
      visibleLabel="Gain"
      ariaLabel="Vertical gain"
      onValueChange={(next) => {
        vertical = next;
        dispatches += 1;
        trace = `change:${next}`;
      }}
    />
  </div>
  <div data-case="slider-rtl" style="width: 240px; padding: 24px;">
    <Slider
      direction="rtl"
      min={0}
      max={100}
      step={1}
      value={rtl}
      visibleLabel="Opacity"
      ariaLabel="Opacity"
    />
  </div>
  <div data-case="range-block" style="width: 240px; padding: 24px;">
    <RangeSlider
      min={0}
      max={100}
      step={1}
      value={range}
      visibleLabel="Price"
      ariaLabel="Range"
      onValueChange={(next) => {
        range = next;
        dispatches += 1;
        trace = `change:${next.join(",")}`;
      }}
    />
    <p data-testid="trace" data-hits={dispatches}>{trace}</p>
  </div>
  <!-- g18.024: fractional steps render short decimals; a snapped 0.85 never
       leaks a binary tail. -->
  <div data-case="slider-fraction" style="width: 240px; padding: 24px;">
    <Slider min={0} max={1} step={0.01} value={fraction} visibleLabel="Drive" ariaLabel="Drive" />
  </div>
  <div data-case="range-fraction" style="width: 240px; padding: 24px;">
    <RangeSlider
      min={0}
      max={1}
      step={0.01}
      value={rangeFraction}
      visibleLabel="Band"
      ariaLabel="Band"
    />
  </div>
  <!-- g18.024 vertical range repair: upper top, label centered on the exact
       middle, lower bottom; nothing clips, shifts, or collapses. -->
  <div data-case="range-vertical" style="width: 200px; height: 240px; padding: 24px;">
    <RangeSlider
      orientation="vertical"
      min={0}
      max={100}
      step={1}
      value={rangeVertical}
      visibleLabel="Price"
      ariaLabel="Vertical range"
    />
  </div>
</section>

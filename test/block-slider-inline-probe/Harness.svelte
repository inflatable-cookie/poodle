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
  let dispatches = $state(0);
  let trace = $state("idle");
</script>

<section data-framework="svelte">
  <div data-case="slider-low" style="width: 240px; padding: 24px;">
    <Slider
      appearance="block"
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
      appearance="block"
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
      appearance="block"
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
      appearance="block"
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
  <div data-case="slider-rtl" style="width: 240px; padding: 24px;">
    <Slider
      appearance="block"
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
      appearance="block"
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
</section>

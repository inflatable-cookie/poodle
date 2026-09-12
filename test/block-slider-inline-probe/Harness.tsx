import { useState } from "react";

import { RangeSlider } from "../../packages/react/components/src/RangeSlider";
import { Slider } from "../../packages/react/components/src/Slider";

// g18.017 inline-presentation probe. See Harness.svelte: the journey cases
// move only the thumb; the formatted value stays the same string.
const sameValue = () => "67";

export function Harness() {
  const [low, setLow] = useState(10);
  const [mid, setMid] = useState(50);
  const [high, setHigh] = useState(90);
  const [collision, setCollision] = useState(12);
  const [rtl, setRtl] = useState(20);
  const [range, setRange] = useState<[number, number]>([20, 80]);
  const [vertical, setVertical] = useState(50);
  const [fraction, setFraction] = useState(0.85);
  const [rangeFraction, setRangeFraction] = useState<[number, number]>([0.3, 0.85]);
  const [rangeVertical, setRangeVertical] = useState<[number, number]>([20, 80]);
  const [rangeExtrema, setRangeExtrema] = useState<[number, number]>([0, 100]);
  const [rangeEquality, setRangeEquality] = useState<[number, number]>([50, 50]);
  const [dispatches, setDispatches] = useState(0);
  const [trace, setTrace] = useState("idle");

  const bump = (next: string) => {
    setDispatches((count) => count + 1);
    setTrace(next);
  };

  return (
    <section data-framework="react">
      <div data-case="slider-low" style={{ width: 240, padding: 24 }}>
        <Slider
          min={0}
          max={100}
          step={1}
          value={low}
          visibleLabel="Gain"
          ariaLabel="Gain"
          formatVisibleValue={sameValue}
          onValueChange={(next) => {
            setLow(next);
            bump(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={dispatches}>{trace}</p>
      </div>
      <div data-case="slider-mid" style={{ width: 240, padding: 24 }}>
        <Slider
          min={0}
          max={100}
          step={1}
          value={mid}
          visibleLabel="Gain"
          ariaLabel="Gain"
          formatVisibleValue={sameValue}
          onValueChange={(next) => {
            setMid(next);
            bump(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={dispatches}>{trace}</p>
      </div>
      <div data-case="slider-high" style={{ width: 240, padding: 24 }}>
        <Slider
          min={0}
          max={100}
          step={1}
          value={high}
          visibleLabel="Gain"
          ariaLabel="Gain"
          formatVisibleValue={sameValue}
          onValueChange={(next) => {
            setHigh(next);
            bump(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={dispatches}>{trace}</p>
      </div>
      <div data-case="slider-collision" style={{ width: 64, padding: 24 }}>
        <Slider
          min={0}
          max={100}
          step={1}
          value={collision}
          visibleLabel="Compressor makeup gain"
          ariaLabel="Collision gain"
          onValueChange={(next) => {
            setCollision(next);
            bump(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={dispatches}>{trace}</p>
      </div>
      {/* g18.022 vertical block: value paints at the capsule's physical top
          and the optional label at its center — anchored to the whole
          capsule, not wherever the row happens to lay out. */}
      <div data-case="slider-vertical" style={{ width: 200, height: 220, padding: 24 }}>
        <Slider
          orientation="vertical"
          min={0}
          max={100}
          step={1}
          value={vertical}
          visibleLabel="Gain"
          ariaLabel="Vertical gain"
          onValueChange={(next) => {
            setVertical(next);
            bump(`change:${next}`);
          }}
        />
      </div>
      <div data-case="slider-rtl" style={{ width: 240, padding: 24 }}>
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
      <div data-case="range-block" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          min={0}
          max={100}
          step={1}
          value={range}
          visibleLabel="Price"
          ariaLabel="Range"
          onValueChange={(next) => {
            setRange(next);
            bump(`change:${next.join(",")}`);
          }}
        />
        <p data-testid="trace" data-hits={dispatches}>{trace}</p>
      </div>
      {/* g18.024: fractional steps render short decimals; a snapped 0.85
          never leaks a binary tail. */}
      <div data-case="slider-fraction" style={{ width: 240, padding: 24 }}>
        <Slider min={0} max={1} step={0.01} value={fraction} visibleLabel="Drive" ariaLabel="Drive" />
      </div>
      <div data-case="range-fraction" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          min={0}
          max={1}
          step={0.01}
          value={rangeFraction}
          visibleLabel="Band"
          ariaLabel="Band"
        />
      </div>
      {/* g18.024 vertical range repair: upper top, label centered on the
          exact middle, lower bottom; nothing clips, shifts, or collapses. */}
      <div data-case="range-vertical" style={{ width: 200, height: 240, padding: 24 }}>
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
      {/* g18.026 family handle parity: both markers stay inside the capsule
          at the extrema and at equality, inset toward the window interior. */}
      <div data-case="range-extrema" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          min={0}
          max={100}
          step={1}
          value={rangeExtrema}
          visibleLabel="Price"
          ariaLabel="Extrema range"
        />
      </div>
      <div data-case="range-equality" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          min={0}
          max={100}
          step={1}
          value={rangeEquality}
          visibleLabel="Price"
          ariaLabel="Equality range"
        />
      </div>
    </section>
  );
}

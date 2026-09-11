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
          appearance="block"
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
          appearance="block"
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
          appearance="block"
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
          appearance="block"
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
      <div data-case="slider-rtl" style={{ width: 240, padding: 24 }}>
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
      <div data-case="range-block" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          appearance="block"
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
    </section>
  );
}

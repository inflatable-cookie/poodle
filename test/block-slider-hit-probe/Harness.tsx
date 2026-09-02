import { useState } from "react";

import { RangeSlider } from "../../packages/react/components/src/RangeSlider";
import { Slider } from "../../packages/react/components/src/Slider";

export function Harness() {
  const [sliderXs, setSliderXs] = useState(50);
  const [sliderCompact, setSliderCompact] = useState(50);
  const [rangeXs, setRangeXs] = useState<[number, number]>([20, 80]);
  const [sliderHits, setSliderHits] = useState(0);
  const [compactHits, setCompactHits] = useState(0);
  const [rangeHits, setRangeHits] = useState(0);
  const [sliderTrace, setSliderTrace] = useState("idle");
  const [compactTrace, setCompactTrace] = useState("idle");
  const [rangeTrace, setRangeTrace] = useState("idle");

  return (
    <section data-framework="react">
      <div data-case="slider-xs" style={{ width: 240, padding: 24 }}>
        <Slider
          appearance="block"
          size="xs"
          min={0}
          max={100}
          step={10}
          ariaLabel="Gain"
          value={sliderXs}
          onValueChange={(next) => {
            setSliderXs(next);
            setSliderHits((count) => count + 1);
            setSliderTrace(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={sliderHits}>{sliderTrace}</p>
      </div>
      <div data-case="slider-xs-compact" style={{ width: 240, padding: 24 }}>
        <Slider
          appearance="block"
          size="xs"
          density="compact"
          min={0}
          max={100}
          step={10}
          ariaLabel="Compact gain"
          value={sliderCompact}
          onValueChange={(next) => {
            setSliderCompact(next);
            setCompactHits((count) => count + 1);
            setCompactTrace(`change:${next}`);
          }}
        />
        <p data-testid="trace" data-hits={compactHits}>{compactTrace}</p>
      </div>
      <div data-case="range-xs" style={{ width: 240, padding: 24 }}>
        <RangeSlider
          appearance="block"
          size="xs"
          min={0}
          max={100}
          step={10}
          ariaLabel="Range"
          value={rangeXs}
          onValueChange={(next) => {
            setRangeXs(next);
            setRangeHits((count) => count + 1);
            setRangeTrace(`change:${next.join(",")}`);
          }}
        />
        <p data-testid="trace" data-hits={rangeHits}>{rangeTrace}</p>
      </div>
    </section>
  );
}

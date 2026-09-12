import { useState } from "react";

import { RangeSlider } from "../../packages/react/components/src/RangeSlider";
import { Slider } from "../../packages/react/components/src/Slider";
import { Button } from "../../packages/react/components/src/Button";
import { TextInput } from "../../packages/react/components/src/TextInput";

const sizes = ["xs", "sm", "md", "lg", "xl"] as const;

export function Harness() {
  const [sliderXs, setSliderXs] = useState(50);
  const [sliderCompact, setSliderCompact] = useState(50);
  const [rangeXs, setRangeXs] = useState<[number, number]>([20, 80]);
  const [sizeValues, setSizeValues] = useState<Record<string, number>>({ xs: 40, sm: 40, md: 40, lg: 40, xl: 40 });
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
      {sizes.map((size) => (
        // Mixed row: the Slider must align edge for edge with same-size
        // reference controls (Button, TextInput) on the shared ladder.
        <div key={size} data-case={`slider-${size}-row`} className="g18-row">
          <Button size={size} ariaLabel={`Reference button at ${size}`}>Go</Button>
          <Slider
            size={size}
            min={0}
            max={100}
            step={1}
            ariaLabel={`Standard slider at ${size}`}
            value={sizeValues[size]}
            onValueChange={(next) => setSizeValues((values) => ({ ...values, [size]: next }))}
          />
          <TextInput size={size} ariaLabel={`Reference input at ${size}`} value="88" />
        </div>
      ))}
    </section>
  );
}

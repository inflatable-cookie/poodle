import { useState } from "react";
import { RangeSlider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function RangeSliderSpecimen() {
  const [priceRange, setPriceRange] = useState<[number, number]>([20, 80]);
  const [ageRange, setAgeRange] = useState<[number, number]>([23, 43]);
  const [embeddedUnipolarRange, setEmbeddedUnipolarRange] = useState<[number, number]>([0.2, 0.75]);
  const [embeddedBipolarRange, setEmbeddedBipolarRange] = useState<[number, number]>([-0.6, 0.35]);
  const [verticalRange, setVerticalRange] = useState<[number, number]>([30, 70]);
  const [sizeRanges, setSizeRanges] = useState<Record<string, [number, number]>>({ xs: [0.2, 0.75], sm: [0.2, 0.75], md: [0.2, 0.75], lg: [0.2, 0.75], xl: [0.2, 0.75] });

  const axisStyle = { width: "min(100%, 20rem)" };

  return (
    <SpecimenLayout
      /* One control per step. The axis tabs exist so Examples does not have to
         carry a matrix; filling them with a matrix defeats the point. */
      sizes={(size) => (
        <div style={axisStyle}>
          <RangeSlider value={sizeRanges[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Range at ${size}`} onValueChange={(value) => setSizeRanges((current) => ({ ...current, [size]: value }))} />
        </div>
      )}
      densities={(density) => (
        <div style={axisStyle}>
          <RangeSlider value={[20, 80]} min={0} max={100} density={density} ariaLabel={`Range at ${density} density`} />
        </div>
      )}
    >
      <div style={{ maxWidth: "20rem" }}>
        <SpecimenGroup label="A lower and upper bound the reader drags">
          <RangeSlider
            value={priceRange}
            min={0}
            max={100}
            ariaLabel="Price range"
            onValueChange={setPriceRange}
          />
          <p>
            ${priceRange[0]} – ${priceRange[1]}
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="Stepped — the thumbs land on whole increments">
          <RangeSlider
            value={ageRange}
            min={18}
            max={65}
            step={5}
            ariaLabel="Age range"
            onValueChange={setAgeRange}
          />
          <p>
            Ages {ageRange[0]} – {ageRange[1]}
          </p>
        </SpecimenGroup>

        {/* The embedded variant is the dense control used inside audio and
            modulation panels; unipolar fills from the floor, bipolar from centre. */}
        <SpecimenGroup label="Embedded variant — unipolar fills from the floor, bipolar from centre">
          <RangeSlider variant="embedded" polarity="unipolar" value={embeddedUnipolarRange} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation range" onValueChange={setEmbeddedUnipolarRange} />
          <RangeSlider variant="embedded" polarity="bipolar" value={embeddedBipolarRange} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation range" onValueChange={setEmbeddedBipolarRange} />
        </SpecimenGroup>

        <SpecimenGroup label="Disabled">
          <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
        </SpecimenGroup>

        <SpecimenGroup label="Block appearance">
          <RangeSlider appearance="block" value={priceRange} min={0} max={100} visibleLabel="Price" ariaLabel="Price range" onValueChange={setPriceRange} />
          <RangeSlider appearance="block" direction="rtl" value={ageRange} min={18} max={65} step={5} visibleLabel="Age" ariaLabel="Age range" onValueChange={setAgeRange} />
        </SpecimenGroup>
      </div>

      <SpecimenGroup label="Vertical — the same control on the other axis">
        <div style={{ display: "flex", alignItems: "flex-start", gap: "2rem", height: "12rem" }}>
          <RangeSlider
            orientation="vertical"
            value={verticalRange}
            min={0}
            max={100}
            ariaLabel="Vertical range"
            onValueChange={setVerticalRange}
          />
          <RangeSlider
            orientation="vertical"
            variant="embedded"
            polarity="bipolar"
            value={[-0.4, 0.6]}
            min={-1}
            max={1}
            step={0.01}
            ariaLabel="Vertical embedded range"
          />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

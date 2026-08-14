import { useState } from "react";
import { RangeSlider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function RangeSliderSpecimen() {
  const [priceRange, setPriceRange] = useState<[number, number]>([20, 80]);
  const [ageRange, setAgeRange] = useState<[number, number]>([23, 43]);
  const [embeddedUnipolarRange, setEmbeddedUnipolarRange] = useState<[number, number]>([0.2, 0.75]);
  const [embeddedBipolarRange, setEmbeddedBipolarRange] = useState<[number, number]>([-0.6, 0.35]);
  const [sizeUnipolarRanges, setSizeUnipolarRanges] = useState<Record<string, [number, number]>>({ xs: [0.2, 0.75], sm: [0.2, 0.75], md: [0.2, 0.75], lg: [0.2, 0.75], xl: [0.2, 0.75] });
  const [sizeBipolarRanges, setSizeBipolarRanges] = useState<Record<string, [number, number]>>({ xs: [-0.5, 0.5], sm: [-0.5, 0.5], md: [-0.5, 0.5], lg: [-0.5, 0.5], xl: [-0.5, 0.5] });

  const variantStyle = { display: "flex", width: "100%", flexDirection: "column" as const, gap: "0.375rem" };
  const labelStyle = { color: "var(--poodle-color-text-secondary)", fontSize: "var(--poodle-typography-label-size)" };

  return (
    <div style={{ maxWidth: "20rem" }}>
      <SpecimenLayout
        sizes={(size) => (
          <span style={variantStyle}>
            <span style={labelStyle}>{size.toUpperCase()} · standard</span>
            <RangeSlider value={sizeUnipolarRanges[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Standard range at ${size}`} onValueChange={(value) => setSizeUnipolarRanges((current) => ({ ...current, [size]: value }))} />
            <span style={labelStyle}>{size.toUpperCase()} · embedded unipolar</span>
            <RangeSlider variant="embedded" polarity="unipolar" value={sizeUnipolarRanges[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Embedded unipolar range at ${size}`} onValueChange={(value) => setSizeUnipolarRanges((current) => ({ ...current, [size]: value }))} />
            <span style={labelStyle}>{size.toUpperCase()} · embedded bipolar</span>
            <RangeSlider variant="embedded" polarity="bipolar" value={sizeBipolarRanges[size]} min={-1} max={1} step={0.01} size={size} ariaLabel={`Embedded bipolar range at ${size}`} onValueChange={(value) => setSizeBipolarRanges((current) => ({ ...current, [size]: value }))} />
          </span>
        )}
        densities={(density) => (
          <RangeSlider variant="embedded" polarity="bipolar" value={[-0.5, 0.5]} min={-1} max={1} density={density} ariaLabel={`Embedded range at ${density} density`} />
        )}
      >
        <SpecimenGroup label="Default">
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

        <SpecimenGroup label="With step">
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

        <SpecimenGroup label="Disabled">
          <RangeSlider value={[30, 70]} min={0} max={100} disabled ariaLabel="Disabled range" />
        </SpecimenGroup>

        <SpecimenGroup label="Embedded unipolar control">
          <RangeSlider variant="embedded" polarity="unipolar" value={embeddedUnipolarRange} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation range" onValueChange={setEmbeddedUnipolarRange} />
        </SpecimenGroup>

        <SpecimenGroup label="Embedded bipolar control">
          <RangeSlider variant="embedded" polarity="bipolar" value={embeddedBipolarRange} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation range" onValueChange={setEmbeddedBipolarRange} />
        </SpecimenGroup>
      </SpecimenLayout>
    </div>
  );
}

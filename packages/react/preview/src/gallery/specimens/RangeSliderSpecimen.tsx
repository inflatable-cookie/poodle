import { useState } from "react";
import { RangeSlider } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function RangeSliderSpecimen() {
  const [priceRange, setPriceRange] = useState<[number, number]>([20, 80]);
  const [ageRange, setAgeRange] = useState<[number, number]>([23, 43]);

  return (
    <div style={{ maxWidth: "20rem" }}>
      <SpecimenLayout
        showDensities={false}
        sizes={(size) => (
          <RangeSlider value={[25, 75]} min={0} max={100} size={size} ariaLabel={`Range at ${size}`} />
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
      </SpecimenLayout>
    </div>
  );
}

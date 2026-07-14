import { useState } from "react";
import { Slider } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function SliderSpecimen() {
  const [volume, setVolume] = useState(65);
  const [opacity, setOpacity] = useState(100);

  return (
    <div style={{ maxWidth: "20rem" }}>
      <SpecimenLayout
        showDensities={false}
        sizes={(size) => (
          <Slider value={50} min={0} max={100} size={size} ariaLabel={"Slider at " + size} />
        )}
      >
        <SpecimenGroup label="Default">
          <Slider
            value={volume}
            min={0}
            max={100}
            ariaLabel="Volume"
            onValueChange={(value) => setVolume(value)}
          />
          <p>
            Volume: <strong>{volume}%</strong>
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="With step">
          <Slider
            value={opacity}
            min={0}
            max={100}
            step={10}
            ariaLabel="Opacity"
            onValueChange={(value) => setOpacity(value)}
          />
          <p>
            Opacity: <strong>{opacity}%</strong>
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="Disabled">
          <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" disabled />
        </SpecimenGroup>
      </SpecimenLayout>
    </div>
  );
}

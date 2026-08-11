import { useState } from "react";
import { Slider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function SliderSpecimen() {
  const [volume, setVolume] = useState(65);
  const [opacity, setOpacity] = useState(100);
  const [unipolar, setUnipolar] = useState(0.35);
  const [bipolar, setBipolar] = useState(-0.45);

  return (
    <div style={{ maxWidth: "20rem" }}>
      <SpecimenLayout
        sizes={(size) => (
          <Slider value={50} min={0} max={100} size={size} ariaLabel={"Slider at " + size} />
        )}
        densities={(density) => (
          <Slider variant="embedded" polarity="bipolar" value={-0.4} min={-1} max={1} density={density} ariaLabel={`Embedded slider at ${density} density`} />
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

        <SpecimenGroup label="Embedded controls">
          <Slider variant="embedded" polarity="unipolar" value={unipolar} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation" onValueChange={setUnipolar} />
          <Slider variant="embedded" polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation" onValueChange={setBipolar} />
        </SpecimenGroup>
      </SpecimenLayout>
    </div>
  );
}

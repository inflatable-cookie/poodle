import { useState } from "react";
import { Slider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function SliderSpecimen() {
  const [volume, setVolume] = useState(65);
  const [opacity, setOpacity] = useState(100);
  const [unipolar, setUnipolar] = useState(0.35);
  const [bipolar, setBipolar] = useState(-0.45);
  const [sizeValues, setSizeValues] = useState<Record<string, number>>({ xs: 0.4, sm: 0.4, md: 0.4, lg: 0.4, xl: 0.4 });
  const [densityValues, setDensityValues] = useState<Record<string, number>>({ compact: -0.4, default: -0.4, comfortable: -0.4 });

  const variantStyle = { display: "flex", width: "100%", flexDirection: "column" as const, gap: "0.375rem" };
  const labelStyle = { color: "var(--poodle-color-text-secondary)", fontSize: "var(--poodle-typography-label-size)" };

  return (
    <div style={{ maxWidth: "20rem" }}>
      <SpecimenLayout
        sizes={(size) => (
          <span style={variantStyle}>
            <span style={labelStyle}>{size.toUpperCase()} · standard</span>
            <Slider value={sizeValues[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Standard slider at ${size}`} onValueChange={(value) => setSizeValues((current) => ({ ...current, [size]: value }))} />
            <span style={labelStyle}>{size.toUpperCase()} · embedded</span>
            <Slider variant="embedded" polarity="unipolar" value={sizeValues[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Embedded slider at ${size}`} onValueChange={(value) => setSizeValues((current) => ({ ...current, [size]: value }))} />
          </span>
        )}
        densities={(density) => (
          <span style={variantStyle}>
            <span style={labelStyle}>{density}</span>
            <Slider variant="embedded" polarity="bipolar" value={densityValues[density]} min={-1} max={1} step={0.01} density={density} ariaLabel={`Embedded slider at ${density} density`} onValueChange={(value) => setDensityValues((current) => ({ ...current, [density]: value }))} />
          </span>
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

        <SpecimenGroup label="Block appearance">
          <Slider appearance="block" value={volume} min={0} max={100} visibleLabel="Volume" ariaLabel="Volume" onValueChange={(value) => setVolume(value)} />
          <Slider appearance="block" direction="rtl" value={opacity} min={0} max={100} visibleLabel="Opacity" ariaLabel="Opacity" onValueChange={(value) => setOpacity(value)} />
        </SpecimenGroup>
      </SpecimenLayout>
    </div>
  );
}

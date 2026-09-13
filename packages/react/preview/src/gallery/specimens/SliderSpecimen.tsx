import { useState } from "react";
import { RangeSlider, Slider } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const sliderSizes = ["xs", "sm", "md", "lg", "xl"] as const;

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
          <>
            <div style={variantStyle}>
              <span style={labelStyle}>{size.toUpperCase()} · block</span>
              <Slider value={sizeValues[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Block slider at ${size}`} onValueChange={(value) => setSizeValues((current) => ({ ...current, [size]: value }))} />
              <span style={labelStyle}>{size.toUpperCase()} · embedded</span>
              <Slider variant="embedded" polarity="unipolar" value={sizeValues[size]} min={0} max={1} step={0.01} size={size} ariaLabel={`Embedded slider at ${size}`} onValueChange={(value) => setSizeValues((current) => ({ ...current, [size]: value }))} />
            </div>
            {size === "xl" ? (
              <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem", width: "100%", marginTop: "1rem" }}>
                <span style={labelStyle}>Vertical block</span>
                <div style={{ display: "flex", alignItems: "flex-start", justifyContent: "space-between", gap: "0.75rem", width: "100%" }}>
                  {sliderSizes.map((verticalSize) => (
                    <div key={verticalSize} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "0.375rem" }}>
                      <span style={labelStyle}>{verticalSize.toUpperCase()}</span>
                      <div style={{ display: "flex", alignItems: "stretch", height: "12rem" }}>
                        <Slider orientation="vertical" value={sizeValues[verticalSize]} min={0} max={1} step={0.01} size={verticalSize} visibleLabel="Gain" ariaLabel={`Vertical block slider at ${verticalSize}`} onValueChange={(value) => setSizeValues((current) => ({ ...current, [verticalSize]: value }))} />
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            ) : null}
          </>
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
            visibleLabel="Volume"
            formatVisibleValue={(value) => `${value}%`}
            ariaLabel="Volume"
            onValueChange={(value) => setVolume(value)}
          />
        </SpecimenGroup>

        <SpecimenGroup label="With step">
          <Slider
            value={opacity}
            min={0}
            max={100}
            step={10}
            visibleLabel="Opacity"
            formatVisibleValue={(value) => `${value}%`}
            ariaLabel="Opacity"
            onValueChange={(value) => setOpacity(value)}
          />
        </SpecimenGroup>

        <SpecimenGroup label="Disabled">
          <Slider value={40} min={0} max={100} ariaLabel="Disabled slider" disabled />
        </SpecimenGroup>

        <SpecimenGroup label="Embedded unipolar">
          <Slider variant="embedded" polarity="unipolar" value={unipolar} min={0} max={1} step={0.01} ariaLabel="Unipolar modulation" onValueChange={setUnipolar} />
        </SpecimenGroup>

        <SpecimenGroup label="Bipolar — block and embedded fill from center">
          <Slider polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} visibleLabel="Drive" ariaLabel="Bipolar block" onValueChange={setBipolar} />
          <Slider variant="embedded" polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} ariaLabel="Bipolar modulation" onValueChange={setBipolar} />
        </SpecimenGroup>

        {/* g18.022: block is the default variant; vertical and bipolar block are native. */}
        <SpecimenGroup label="Block — the default capsule">
          <Slider value={volume} min={0} max={100} visibleLabel="Volume" ariaLabel="Volume" onValueChange={(value) => setVolume(value)} />
          <Slider direction="rtl" value={opacity} min={0} max={100} visibleLabel="Opacity" ariaLabel="Opacity" onValueChange={(value) => setOpacity(value)} />
        </SpecimenGroup>

        <SpecimenGroup label="Vertical block — rotated label, external upright values">
          <div style={{ display: "flex", alignItems: "flex-start", gap: "2rem", height: "12rem" }}>
            <Slider orientation="vertical" value={volume} min={0} max={100} visibleLabel="Volume" ariaLabel="Vertical volume" onValueChange={(value) => setVolume(value)} />
            <Slider orientation="vertical" polarity="bipolar" value={bipolar} min={-1} max={1} step={0.01} ariaLabel="Vertical bipolar block" onValueChange={setBipolar} />
            <Slider orientation="vertical" value={0.85} min={0} max={1} step={0.01} visibleLabel="Drive" ariaLabel="Vertical fractional block" />
            <RangeSlider orientation="vertical" value={[0.3, 0.85]} min={0} max={1} step={0.01} visibleLabel="Band" ariaLabel="Vertical fractional range" />
          </div>
        </SpecimenGroup>

      </SpecimenLayout>
    </div>
  );
}

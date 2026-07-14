import { useState, type CSSProperties } from "react";
import { ColorPicker } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const brandSwatches = [
  "#ef4444", "#f97316", "#eab308", "#22c55e",
  "#3b82f6", "#6366f1", "#8b5cf6", "#ec4899",
];

const paragraphStyle: CSSProperties = { margin: 0 };

export function ColorPickerSpecimen() {
  const [color, setColor] = useState("#6366f1");
  const [alphaColor, setAlphaColor] = useState("#3b82f6");

  return (
    <SpecimenLayout
      sizes={(size) => <ColorPicker value="#6366f1" size={size} />}
      densities={(density) => <ColorPicker density={density} />}
    >
      <SpecimenGroup label="Basic picker">
        <ColorPicker value={color} onChange={setColor} />
        <p style={paragraphStyle}>Selected: <strong>{color}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="With swatches">
        <ColorPicker value={color} onChange={setColor} swatches={brandSwatches} />
      </SpecimenGroup>

      <SpecimenGroup label="With alpha">
        <ColorPicker value={alphaColor} onChange={setAlphaColor} showAlpha />
        <p style={paragraphStyle}>Selected: <strong>{alphaColor}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Default open, RGB mode">
        <ColorPicker value="#22c55e" defaultOpen defaultMode="rgb" />
      </SpecimenGroup>

      <SpecimenGroup label="Preview only (no input)">
        <ColorPicker value={color} showInput={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <ColorPicker value="#22c55e" disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

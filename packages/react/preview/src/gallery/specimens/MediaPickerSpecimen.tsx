import { useState, type CSSProperties } from "react";
import { Button, MediaPicker } from "@inflatable-cookie/poodle-react";
import type { MediaPickerItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const sampleItems: MediaPickerItem[] = [
  { id: "1", label: "hero-banner.jpg", kind: "image" },
  { id: "2", label: "logo-dark.png", kind: "image" },
  { id: "3", label: "product-shot.jpg", kind: "image" },
  { id: "4", label: "report-q4.pdf", kind: "document" },
  { id: "5", label: "team-photo.jpg", kind: "image" },
  { id: "6", label: "presentation.pdf", kind: "document" },
];

const variantStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "flex-start",
  gap: "0.5rem",
};

const labelStyle: CSSProperties = {
  margin: 0,
  color: "var(--poodle-color-text-secondary)",
  fontFamily: "var(--poodle-typography-label-family)",
  fontSize: "var(--poodle-typography-label-size)",
  fontWeight: "var(--poodle-typography-label-weight)",
  letterSpacing: "0.08em",
  textTransform: "uppercase",
};

export function MediaPickerSpecimen() {
  const [open, setOpen] = useState(false);
  const [selected, setSelected] = useState("");
  const [sizeOpenMap, setSizeOpenMap] = useState<Record<string, boolean>>({});
  const [densityOpenMap, setDensityOpenMap] = useState<Record<string, boolean>>({});
  const [sizeSelectedMap, setSizeSelectedMap] = useState<Record<string, string>>({});
  const [densitySelectedMap, setDensitySelectedMap] = useState<Record<string, string>>({});

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={variantStyle}>
          <p style={labelStyle}>{size.toUpperCase()}</p>
          <Button variant="secondary" size={size} onClick={() => setSizeOpenMap((m) => ({ ...m, [size]: true }))}>
            Open {size.toUpperCase()} picker
          </Button>
          <MediaPicker
            open={sizeOpenMap[size] ?? false}
            items={sampleItems}
            title={`${size.toUpperCase()} asset picker`}
            size={size}
            onSelect={(item) => {
              setSizeSelectedMap((m) => ({ ...m, [size]: item.label }));
              setSizeOpenMap((m) => ({ ...m, [size]: false }));
            }}
            onOpenChange={(nextOpen) => setSizeOpenMap((m) => ({ ...m, [size]: nextOpen }))}
          />
          {sizeSelectedMap[size] ? (
            <p style={{ margin: 0 }}>
              Selected: <strong>{sizeSelectedMap[size]}</strong>
            </p>
          ) : null}
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <p style={labelStyle}>{density}</p>
          <Button variant="secondary" onClick={() => setDensityOpenMap((m) => ({ ...m, [density]: true }))}>
            Open {density} picker
          </Button>
          <MediaPicker
            open={densityOpenMap[density] ?? false}
            items={sampleItems}
            title={`${density} asset picker`}
            density={density}
            onSelect={(item) => {
              setDensitySelectedMap((m) => ({ ...m, [density]: item.label }));
              setDensityOpenMap((m) => ({ ...m, [density]: false }));
            }}
            onOpenChange={(nextOpen) => setDensityOpenMap((m) => ({ ...m, [density]: nextOpen }))}
          />
          {densitySelectedMap[density] ? (
            <p style={{ margin: 0 }}>
              Selected: <strong>{densitySelectedMap[density]}</strong>
            </p>
          ) : null}
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Media picker dialog">
          <Button variant="secondary" onClick={() => setOpen(true)}>
            Select media
          </Button>
          <MediaPicker
            open={open}
            items={sampleItems}
            title="Select an asset"
            onSelect={(item) => {
              setSelected(item.label);
              setOpen(false);
            }}
            onOpenChange={(nextOpen) => setOpen(nextOpen)}
          />
          {selected ? (
            <p style={{ margin: 0 }}>
              Selected: <strong>{selected}</strong>
            </p>
          ) : null}
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

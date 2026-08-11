import { useState, type CSSProperties } from "react";
import { IconButton } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const buttonRow: CSSProperties = { display: "flex", gap: "0.5rem", alignItems: "center" };

export function IconButtonSpecimen() {
  const [bold, setBold] = useState(false);
  const [italic, setItalic] = useState(false);
  const [underline, setUnderline] = useState(true);
  const [pinned, setPinned] = useState(false);
  const [starred, setStarred] = useState(false);

  return (
    <SpecimenLayout
      showDensities={false}
      sizes={(size) => (
        <IconButton icon="star" ariaLabel={`Favorite (${size})`} variant="secondary" size={size} />
      )}
    >
      <SpecimenGroup label="Variants">
        <div style={buttonRow}>
          <IconButton icon="plus" ariaLabel="Add" variant="primary" />
          <IconButton icon="settings" ariaLabel="Settings" variant="secondary" />
          <IconButton icon="x" ariaLabel="Close" variant="ghost" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Danger tone">
        <div style={buttonRow}>
          <IconButton icon="trash-2" ariaLabel="Delete" variant="primary" tone="danger" />
          <IconButton icon="trash-2" ariaLabel="Delete" variant="secondary" tone="danger" />
          <IconButton icon="trash-2" ariaLabel="Delete" variant="ghost" tone="danger" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Warning tone">
        <div style={buttonRow}>
          <IconButton icon="trash-2" ariaLabel="Delete" variant="primary" tone="warning" />
          <IconButton icon="trash-2" ariaLabel="Delete" variant="secondary" tone="warning" />
          <IconButton icon="trash-2" ariaLabel="Delete" variant="ghost" tone="warning" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Toggle (text editor toolbar)">
        <div style={buttonRow}>
          <IconButton icon="bold" ariaLabel="Bold" variant="ghost" pressed={bold} onPressedChange={setBold} />
          <IconButton icon="italic" ariaLabel="Italic" variant="ghost" pressed={italic} onPressedChange={setItalic} />
          <IconButton icon="underline" ariaLabel="Underline" variant="ghost" pressed={underline} onPressedChange={setUnderline} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Toggle (secondary variant)">
        <div style={buttonRow}>
          <IconButton icon="map-pin" ariaLabel="Pin" variant="secondary" pressed={pinned} onPressedChange={setPinned} />
          <IconButton icon="star" ariaLabel="Favorite" variant="secondary" pressed={starred} onPressedChange={setStarred} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled and loading">
        <div style={buttonRow}>
          <IconButton icon="ban" ariaLabel="Disabled" disabled variant="secondary" />
          <IconButton icon="refresh-cw" ariaLabel="Loading" loading variant="secondary" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

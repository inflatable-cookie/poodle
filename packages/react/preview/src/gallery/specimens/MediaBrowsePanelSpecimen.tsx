import { useState, type CSSProperties } from "react";
import { Button, MediaBrowsePanel } from "@inflatable-cookie/poodle-react";
import type { MediaPickerItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const baseItems: MediaPickerItem[] = [
  { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
  { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
  { id: "3", label: "Podcast intro", kind: "audio", meta: "Audio" },
  { id: "4", label: "Quarterly report", kind: "document", meta: "Document" },
];

const actionsStyle: CSSProperties = { display: "flex", gap: "0.5rem", flexWrap: "wrap" };
const variantStyle: CSSProperties = { width: "min(100%, 40rem)" };

export function MediaBrowsePanelSpecimen() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [items, setItems] = useState<MediaPickerItem[]>([...baseItems]);
  const [lastAction, setLastAction] = useState("None");

  function reset() {
    setLoading(false);
    setError(null);
    setItems([...baseItems]);
    setLastAction("Reset");
  }

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantStyle}>
          <MediaBrowsePanel items={baseItems} hasMore size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={variantStyle}>
          <MediaBrowsePanel items={baseItems} hasMore density={density} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Browse panel" bare>
          <div style={actionsStyle}>
            <Button
              variant="secondary"
              sizeRole="chrome"
              onClick={() => {
                setLoading(true);
                setError(null);
                setLastAction("Loading");
              }}
            >
              Loading
            </Button>
            <Button
              variant="secondary"
              sizeRole="chrome"
              onClick={() => {
                setLoading(false);
                setError("Failed to load media");
                setLastAction("Error");
              }}
            >
              Error
            </Button>
            <Button
              variant="secondary"
              sizeRole="chrome"
              onClick={() => {
                setLoading(false);
                setError(null);
                setItems([]);
                setLastAction("Empty");
              }}
            >
              Empty
            </Button>
            <Button variant="secondary" sizeRole="chrome" onClick={reset}>
              Reset
            </Button>
          </div>
          <div style={variantStyle}>
            <MediaBrowsePanel
              loading={loading}
              error={error}
              items={items}
              hasMore={items.length > 0}
              onSelect={(item) => setLastAction(`Selected ${item.label}`)}
              onLoadMore={() => setLastAction("Load more")}
            />
          </div>
          <p style={{ margin: 0 }}>
            Last action: <strong>{lastAction}</strong>
          </p>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

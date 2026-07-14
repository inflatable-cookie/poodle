import type { CSSProperties } from "react";
import { MediaThumbnail } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const gridStyle: CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fit, minmax(7rem, 1fr))",
  gap: "0.75rem",
};

const placeholderThumb: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  width: "100%",
  height: "100%",
  background: "var(--poodle-color-bg-subtle)",
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.75rem",
  fontWeight: 600,
};

const containedImage: CSSProperties = {
  display: "grid",
  placeItems: "center",
  width: "100%",
  minHeight: "10rem",
  background: "color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)",
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.75rem",
  fontWeight: 600,
};

export function MediaThumbnailSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Image thumbnails">
        <div style={gridStyle}>
          <MediaThumbnail kind="image" title="Photo 1" badge="New" aspectRatio="square">
            <div style={placeholderThumb}>IMG</div>
          </MediaThumbnail>
          <MediaThumbnail kind="image" title="Photo 2" meta="2.4 MB" aspectRatio="square">
            <div style={placeholderThumb}>IMG</div>
          </MediaThumbnail>
          <MediaThumbnail kind="video" title="Clip" badge="HD" meta="1:24" aspectRatio="square">
            <div style={placeholderThumb}>VID</div>
          </MediaThumbnail>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Compact presentation">
        <div style={gridStyle}>
          <MediaThumbnail kind="document" title="Report.pdf" presentation="compact" aspectRatio="landscape">
            <div style={placeholderThumb}>PDF</div>
          </MediaThumbnail>
          <MediaThumbnail kind="audio" title="Interview.mp3" presentation="compact" aspectRatio="landscape">
            <div style={placeholderThumb}>MP3</div>
          </MediaThumbnail>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Loading state">
        <div style={{ width: "min(100%, 10rem)" }}>
          <MediaThumbnail kind="image" state="loading" aspectRatio="square" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Contained image">
        <div style={{ width: "min(100%, 28rem)" }}>
          <MediaThumbnail
            kind="image"
            aspectRatio="auto"
            fit="contain"
            frameMinHeight="10rem"
            frameMaxHeight="14rem"
            title="Question diagram"
          >
            <div style={containedImage}>Image</div>
          </MediaThumbnail>
        </div>
      </SpecimenGroup>
    </div>
  );
}

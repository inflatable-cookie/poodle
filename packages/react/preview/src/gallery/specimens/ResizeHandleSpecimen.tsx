import type { CSSProperties } from "react";
import { ResizeHandle } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const rowStyle: CSSProperties = {
  display: "flex",
  alignItems: "stretch",
  height: "6rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "hidden",
};

const colStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  height: "10rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "hidden",
};

const paneStyle: CSSProperties = {
  flex: 1,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
  background: "color-mix(in srgb, var(--poodle-color-background-panel) 50%, transparent)",
};

const horizontalHandleWrapper: CSSProperties = { height: "100%" };
const verticalHandleWrapper: CSSProperties = { width: "100%" };

export function ResizeHandleSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Horizontal split (vertical handle — drag left/right)">
        <div style={rowStyle}>
          <div style={paneStyle}>Left</div>
          <div style={horizontalHandleWrapper}>
            <ResizeHandle orientation="horizontal" ariaLabel="Resize horizontal" />
          </div>
          <div style={paneStyle}>Right</div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Vertical split (horizontal handle — drag up/down)">
        <div style={colStyle}>
          <div style={paneStyle}>Top</div>
          <div style={verticalHandleWrapper}>
            <ResizeHandle orientation="vertical" ariaLabel="Resize vertical" />
          </div>
          <div style={paneStyle}>Bottom</div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled (horizontal split)">
        <div style={rowStyle}>
          <div style={paneStyle}>Left</div>
          <div style={horizontalHandleWrapper}>
            <ResizeHandle orientation="horizontal" disabled ariaLabel="Disabled resize" />
          </div>
          <div style={paneStyle}>Right</div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled (vertical split)">
        <div style={colStyle}>
          <div style={paneStyle}>Top</div>
          <div style={verticalHandleWrapper}>
            <ResizeHandle orientation="vertical" disabled ariaLabel="Disabled resize vertical" />
          </div>
          <div style={paneStyle}>Bottom</div>
        </div>
      </SpecimenGroup>
    </div>
  );
}

import type { CSSProperties } from "react";
import { useState } from "react";
import { ResizeHandle } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const MIN_HORIZONTAL = 48;
const MAX_HORIZONTAL = 280;
const MIN_VERTICAL = 40;
const MAX_VERTICAL = 120;

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

const growPaneStyle: CSSProperties = {
  ...paneStyle,
  flex: 1,
  minWidth: 0,
  minHeight: 0,
};

const horizontalHandleWrapper: CSSProperties = { height: "100%" };
const verticalHandleWrapper: CSSProperties = { width: "100%" };

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

export function ResizeHandleSpecimen() {
  const [leftWidth, setLeftWidth] = useState(120);
  const [topHeight, setTopHeight] = useState(80);

  function applyHorizontalDelta(delta: number): void {
    setLeftWidth((current) => clamp(current + delta, MIN_HORIZONTAL, MAX_HORIZONTAL));
  }

  function applyVerticalDelta(delta: number): void {
    setTopHeight((current) => clamp(current + delta, MIN_VERTICAL, MAX_VERTICAL));
  }

  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Horizontal split (vertical handle — drag left/right)">
        <div style={rowStyle}>
          <div style={{ ...paneStyle, flex: `0 0 ${leftWidth}px` }}>Left</div>
          <div style={horizontalHandleWrapper}>
            <ResizeHandle
              orientation="horizontal"
              ariaLabel="Resize horizontal"
              ariaValueNow={leftWidth}
              ariaValueMin={MIN_HORIZONTAL}
              ariaValueMax={MAX_HORIZONTAL}
              onResizeMove={applyHorizontalDelta}
              onResizeStep={applyHorizontalDelta}
            />
          </div>
          <div style={growPaneStyle}>Right</div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Vertical split (horizontal handle — drag up/down)">
        <div style={colStyle}>
          <div style={{ ...paneStyle, flex: `0 0 ${topHeight}px` }}>Top</div>
          <div style={verticalHandleWrapper}>
            <ResizeHandle
              orientation="vertical"
              ariaLabel="Resize vertical"
              ariaValueNow={topHeight}
              ariaValueMin={MIN_VERTICAL}
              ariaValueMax={MAX_VERTICAL}
              onResizeMove={applyVerticalDelta}
              onResizeStep={applyVerticalDelta}
            />
          </div>
          <div style={growPaneStyle}>Bottom</div>
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

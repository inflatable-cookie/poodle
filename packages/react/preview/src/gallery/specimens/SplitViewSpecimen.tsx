import type { CSSProperties } from "react";
import { Region, SplitView } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

const frameStyle: CSSProperties = {
  height: "10rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "visible",
};

const frameTallStyle: CSSProperties = { ...frameStyle, height: "16rem" };

const fillStyle: CSSProperties = { width: "100%", height: "100%" };

export function SplitViewSpecimen() {
  return (
    <div className="poodle-specimen">
      {/* 1. Simple layout with Regions */}
      <SpecimenGroup label="Basic horizontal layout">
        <div style={frameStyle}>
          <SplitView
            orientation="horizontal"
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Sidebar" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Main content" color="green" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Basic vertical layout">
        <div style={frameTallStyle}>
          <SplitView
            orientation="vertical"
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Editor" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Terminal" color="purple" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      {/* 2. Horizontal with collapse toggles */}
      <SpecimenGroup label="Horizontal with collapse toggles (drag to edge to collapse)">
        <div style={frameStyle}>
          <SplitView
            orientation="horizontal"
            showCollapsePrimary
            showCollapseSecondary
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Primary" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Secondary" color="green" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      {/* 3. Vertical with collapse toggles */}
      <SpecimenGroup label="Vertical with collapse toggles">
        <div style={frameTallStyle}>
          <SplitView
            orientation="vertical"
            showCollapsePrimary
            showCollapseSecondary
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Top" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Bottom" color="purple" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      {/* 4. Hover-revealed toggles */}
      <SpecimenGroup label="Hover-revealed toggles (move the pointer onto the seam)">
        <div style={frameStyle}>
          <SplitView
            orientation="horizontal"
            toggleVisibility="hover"
            showCollapsePrimary
            showCollapseSecondary
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Primary" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Secondary" color="green" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      {/* 5. Nested splits (IDE-style layout) */}
      <SpecimenGroup label="Nested splits (IDE-style layout)">
        <div style={frameTallStyle}>
          <SplitView
            orientation="horizontal"
            ratio={0.25}
            showCollapsePrimary
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Explorer" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <SplitView
                  orientation="vertical"
                  ratio={0.65}
                  showCollapseSecondary
                  primary={
                    <div className="poodle-specimen__fill" style={fillStyle}>
                      <Region label="Editor" color="green" />
                    </div>
                  }
                  secondary={
                    <div className="poodle-specimen__fill" style={fillStyle}>
                      <Region label="Terminal" color="purple" />
                    </div>
                  }
                />
              </div>
            }
          />
        </div>
      </SpecimenGroup>

      {/* 6. Disabled */}
      <SpecimenGroup label="Disabled">
        <div style={frameStyle}>
          <SplitView
            orientation="horizontal"
            disabled
            showCollapsePrimary
            showCollapseSecondary
            primary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Left" color="blue" />
              </div>
            }
            secondary={
              <div className="poodle-specimen__fill" style={fillStyle}>
                <Region label="Right" color="green" />
              </div>
            }
          />
        </div>
      </SpecimenGroup>
    </div>
  );
}

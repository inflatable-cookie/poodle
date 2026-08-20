import type { CSSProperties } from "react";
import { Region, SplitView } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const frameStyle: CSSProperties = {
  height: "10rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "visible",
};

const frameTallStyle: CSSProperties = { ...frameStyle, height: "16rem" };

const fillStyle: CSSProperties = { width: "100%", height: "100%" };

function axisSplit(sizeOrDensity: Record<string, unknown>) {
  return (
    <div style={frameStyle}>
      <SplitView
        orientation="horizontal"
        {...sizeOrDensity}
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
  );
}

export function SplitViewSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => axisSplit({ size })}
      densities={(density) => axisSplit({ density })}
    >
      {/* 1. Simple layout with Regions */}
      <SpecimenGroup label="Horizontal split">
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

      <SpecimenGroup label="Vertical split">
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

      {/* 2. Collapse controls */}
      <SpecimenGroup label="Collapse controls">
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
      <SpecimenGroup label="Hover-revealed controls">
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
      <SpecimenGroup label="Nested workspace">
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
    </SpecimenLayout>
  );
}

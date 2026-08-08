import type { CSSProperties } from "react";
import { Spinner } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const spinnerSizes = ["xs", "sm", "md", "lg", "xl"] as const;

const rowStyle: CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  alignItems: "center",
  gap: "1rem",
};

const tonesRowStyle: CSSProperties = { ...rowStyle, gap: "0.75rem" };

const chipStyle: CSSProperties = {
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  minWidth: "2.5rem",
  minHeight: "2.5rem",
  padding: "0.5rem",
  border: "1px solid var(--poodle-color-border-default)",
  borderRadius: "var(--poodle-radius-control)",
  background: "var(--poodle-color-background-surface)",
  color: "var(--poodle-color-text-primary)",
};

const chipInverseStyle: CSSProperties = {
  ...chipStyle,
  background: "var(--poodle-color-text-primary)",
  color: "var(--poodle-color-text-inverse)",
};

export function SpinnerSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={rowStyle}>
          <Spinner variant="ring" size={size} />
          <Spinner variant="grid" size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={rowStyle}>
          <Spinner variant="ring" density={density} />
          <Spinner variant="grid" density={density} />
        </div>
      )}
    >
      <SpecimenGroup label="Ring">
        <div style={rowStyle}>
          {spinnerSizes.map((size) => (
            <Spinner key={size} variant="ring" size={size} />
          ))}
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="CLI grid">
        <div style={rowStyle}>
          {spinnerSizes.map((size, index) => (
            <Spinner
              key={size}
              variant="grid"
              size={size}
              tone={index === 0 ? "muted" : index === 2 ? "accent" : "current"}
            />
          ))}
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Context tones">
        <div style={tonesRowStyle}>
          <span style={chipInverseStyle}>
            <Spinner variant="ring" tone="current" />
          </span>
          <span style={chipStyle}>
            <Spinner variant="ring" tone="accent" />
          </span>
          <span style={chipStyle}>
            <Spinner variant="grid" tone="muted" />
          </span>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

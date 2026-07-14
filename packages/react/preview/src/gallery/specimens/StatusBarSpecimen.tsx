import type { CSSProperties } from "react";
import { StatusBar } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const statusItemStyle: CSSProperties = {
  fontSize: "inherit",
  color: "var(--poodle-color-text-secondary)",
  padding: "0 0.375rem",
};

const variantBlockStyle: CSSProperties = { width: "100%" };

export function StatusBarSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={variantBlockStyle}>
          <StatusBar
            chrome
            summary="Status bar"
            size={size}
            trailing={
              <>
                <span style={statusItemStyle}>UTF-8</span>
                <span style={statusItemStyle}>TypeScript</span>
              </>
            }
          />
        </div>
      )}
      densities={(density) => (
        <div style={variantBlockStyle}>
          <StatusBar
            chrome
            summary="Status bar"
            density={density}
            trailing={
              <>
                <span style={statusItemStyle}>UTF-8</span>
                <span style={statusItemStyle}>TypeScript</span>
              </>
            }
          />
        </div>
      )}
    >
      <SpecimenGroup label="Default (no chrome)">
        <StatusBar
          leading={
            <>
              <span style={statusItemStyle}>main</span>
              <span style={statusItemStyle}>0 errors</span>
            </>
          }
          trailing={
            <>
              <span style={statusItemStyle}>Ln 42, Col 18</span>
              <span style={statusItemStyle}>UTF-8</span>
              <span style={statusItemStyle}>TypeScript</span>
            </>
          }
        />
      </SpecimenGroup>

      <SpecimenGroup label="With chrome">
        <StatusBar
          chrome
          leading={
            <>
              <span style={statusItemStyle}>main</span>
              <span style={statusItemStyle}>0 errors</span>
            </>
          }
          trailing={
            <>
              <span style={statusItemStyle}>Ln 42, Col 18</span>
              <span style={statusItemStyle}>UTF-8</span>
              <span style={statusItemStyle}>TypeScript</span>
            </>
          }
        />
      </SpecimenGroup>

      <SpecimenGroup label="Summary only">
        <StatusBar summary="3 items selected" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

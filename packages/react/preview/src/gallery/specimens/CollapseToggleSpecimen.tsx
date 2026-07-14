import { useState, type CSSProperties } from "react";
import { CollapseToggle } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const labeledStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  gap: "0.375rem",
};

const labelTextStyle: CSSProperties = {
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

export function CollapseToggleSpecimen() {
  const [collapsedLeft, setCollapsedLeft] = useState(false);
  const [collapsedRight, setCollapsedRight] = useState(false);
  const [collapsedUp, setCollapsedUp] = useState(false);
  const [collapsedDown, setCollapsedDown] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => <CollapseToggle direction="left" size={size} />}
      densities={(density) => <CollapseToggle direction="left" density={density} />}
    >
      <SpecimenGroup label="Directions">
        <div className="poodle-specimen__row">
          <div style={labeledStyle}>
            <CollapseToggle direction="left" collapsed={collapsedLeft} onToggle={setCollapsedLeft} />
            <span style={labelTextStyle}>Left {collapsedLeft ? "(collapsed)" : "(expanded)"}</span>
          </div>
          <div style={labeledStyle}>
            <CollapseToggle direction="right" collapsed={collapsedRight} onToggle={setCollapsedRight} />
            <span style={labelTextStyle}>Right {collapsedRight ? "(collapsed)" : "(expanded)"}</span>
          </div>
          <div style={labeledStyle}>
            <CollapseToggle direction="up" collapsed={collapsedUp} onToggle={setCollapsedUp} />
            <span style={labelTextStyle}>Up {collapsedUp ? "(collapsed)" : "(expanded)"}</span>
          </div>
          <div style={labeledStyle}>
            <CollapseToggle direction="down" collapsed={collapsedDown} onToggle={setCollapsedDown} />
            <span style={labelTextStyle}>Down {collapsedDown ? "(collapsed)" : "(expanded)"}</span>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div className="poodle-specimen__row">
          <CollapseToggle direction="left" disabled />
          <CollapseToggle direction="right" disabled />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

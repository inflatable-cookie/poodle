import type { CSSProperties } from "react";

import { Pill } from "@inflatable-cookie/poodle-react";

import { SceneSpecimen } from "../SceneSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";

const dismissRowStyle: CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  alignItems: "center",
  gap: "0.75rem",
};

export function PillSpecimen({ slug = "pill" }: { slug?: string }) {
  return (
    <SceneSpecimen slug={slug}>
      <SpecimenGroup label="Dismissible">
        <div style={dismissRowStyle}>
          <Pill dismissible dismissLabel="Remove filter: Videos">
            Videos
          </Pill>
          <Pill dismissible>Audio</Pill>
          <Pill tone="info" appearance="subtle" dismissible dismissLabel="Remove filter: Published">
            Published
          </Pill>
        </div>
      </SpecimenGroup>
    </SceneSpecimen>
  );
}

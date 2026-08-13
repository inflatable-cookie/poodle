import type { CSSProperties } from "react";
import { UpdateCenter } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const offer = {
  status: { kind: "ready" },
  availability: {
    state: "offer",
    version: "1.4.0",
    reason: "staged",
    notes: "Faster renders, a rebuilt automation pass, and two crash fixes.",
  },
} as const;

const releaseNotes =
  "Faster renders across the board, a rebuilt automation pass, two crash fixes," +
  " and better memory use on large projects. Reboots required for the automation" +
  " changes to take effect.";

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };
const anchorStyle: CSSProperties = {
  display: "flex",
  justifyContent: "flex-end",
  width: "min(42rem, 100%)",
};

export function UpdateCenterSpecimen() {
  return (
    <SpecimenLayout showSizes={false} showDensities={false}>
      <div style={stackStyle}>
        <SpecimenGroup label="Attention">
          <div style={anchorStyle}>
            <UpdateCenter presence="attention" {...offer} defaultOpen />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Release notes">
          <div style={anchorStyle}>
            <UpdateCenter
              presence="attention"
              {...offer}
              availability={{ ...offer.availability, notes: releaseNotes }}
              defaultOpen
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Quiet (postponed offer)">
          <div style={anchorStyle}>
            <UpdateCenter
              presence="quiet"
              {...offer}
              deferral={{ version: "1.4.0", cause: { cause: "userPostponed" } }}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Quiet (work in flight)">
          <div style={anchorStyle}>
            <UpdateCenter
              presence="quiet"
              {...offer}
              progress={{ state: "downloading", fraction: 0.42 }}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Quiet (indeterminate download)">
          <div style={anchorStyle}>
            <UpdateCenter
              presence="quiet"
              {...offer}
              progress={{ state: "downloading", fraction: null }}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

import type { CSSProperties } from "react";
import { StatusIndicator } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const listStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.5rem",
};

const rowStyle: CSSProperties = {
  display: "flex",
  gap: "1rem",
  alignItems: "center",
};

const copyStyle: CSSProperties = {
  fontSize: "1rem",
  lineHeight: 1.6,
  color: "var(--poodle-color-text-primary)",
};

export function StatusIndicatorSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <StatusIndicator status="success" size={size} label={size.toUpperCase()} />}
      densities={(density) => <StatusIndicator status="success" density={density} label="Success" />}
    >
      <SpecimenGroup label="All statuses">
        <div style={listStyle}>
          <StatusIndicator status="neutral" label="Neutral" />
          <StatusIndicator status="info" label="Info" />
          <StatusIndicator status="success" label="Success" />
          <StatusIndicator status="warning" label="Warning" />
          <StatusIndicator status="danger" label="Danger" />
          <StatusIndicator status="pending" label="Pending" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Without labels (dot only)">
        <div style={rowStyle}>
          <StatusIndicator status="success" ariaLabel="Online" />
          <StatusIndicator status="info" ariaLabel="Active" />
          <StatusIndicator status="warning" ariaLabel="Away" />
          <StatusIndicator status="danger" ariaLabel="Offline" />
          <StatusIndicator status="neutral" ariaLabel="Unknown" />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Child content">
        <StatusIndicator status="success">Build passing</StatusIndicator>
      </SpecimenGroup>

      <SpecimenGroup label="Inherited typography">
        <p style={copyStyle}>
          Deploy status:{" "}
          <StatusIndicator status="success" label="Healthy" typography="inherit" />{" "}
          across the last 24 hours.
        </p>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

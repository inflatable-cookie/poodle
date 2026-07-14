import { Code, MetaBar, MetaItem, Pill } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function MetaBarSpecimen() {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
      <SpecimenGroup label="Header metadata">
        <MetaBar ariaLabel="Project metadata">
          <MetaItem label="ID">
            <Code inline source="proj_01JX9G9NVV1W3M4P6K8Q8T2D5A" showCopyButton />
          </MetaItem>
          <Pill tone="success">Active</Pill>
          <MetaItem label="Owner">Clay</MetaItem>
          <MetaItem label="Updated">2 hours ago</MetaItem>
        </MetaBar>
      </SpecimenGroup>

      <SpecimenGroup label="No separators">
        <MetaBar showSeparators={false}>
          <MetaItem label="Type">Media</MetaItem>
          <Pill tone="neutral">Public</Pill>
          <MetaItem>1920 × 1080</MetaItem>
        </MetaBar>
      </SpecimenGroup>

      <SpecimenGroup label="Inherited typography">
        <p
          style={{
            fontSize: "1rem",
            lineHeight: 1.6,
            color: "var(--poodle-color-text-primary)",
          }}
        >
          Release metadata:{" "}
          <MetaItem label="Status" typography="inherit">
            Stable
          </MetaItem>
          <MetaItem label="Version" typography="inherit">
            2.4.1
          </MetaItem>
        </p>
      </SpecimenGroup>
    </div>
  );
}

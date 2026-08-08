import type { CSSProperties } from "react";
import { IconButton, InlineListSection, Pill, Text } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const row: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "0.75rem",
  width: "100%",
  minWidth: 0,
};

const versions = [
  { id: "v3", name: "Version 3", meta: "Ready" },
  { id: "v2", name: "Version 2", meta: "Archived" },
  { id: "v1", name: "Version 1", meta: "Archived" },
];

export function InlineListSectionSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Framed related list">
        <InlineListSection
          title="Versions"
          items={versions}
          count={versions.length}
          actions={<IconButton icon="plus" ariaLabel="Add version" variant="secondary" />}
          item={(version) => (
            <div style={row}>
              <Text as="span" weight="medium">
                {version.name}
              </Text>
              <Pill size="sm" tone={version.meta === "Ready" ? "success" : "neutral"}>
                {version.meta}
              </Pill>
            </div>
          )}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
        <InlineListSection
          title="Aliases"
          items={[] as string[]}
          emptyMessage="No aliases yet."
          item={(alias) => <Text>{alias}</Text>}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Unframed">
        <InlineListSection
          title="References"
          items={versions.slice(0, 2)}
          framed={false}
          item={(version) => <Text as="span">{version.name}</Text>}
        />
      </SpecimenGroup>
    </div>
  );
}

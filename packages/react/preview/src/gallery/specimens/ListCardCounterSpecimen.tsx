import { useState } from "react";
import { Icon, ListCard, ListCardCounter } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

export function ListCardCounterSpecimen() {
  const [linkedAction, setLinkedAction] = useState("No linked counter clicked yet.");

  return (
    <>
      <SpecimenGroup label="Static footer counters">
        <ListCard
          title="Design system"
          subtitle="12 contributors"
          leadingShape="rounded-square"
          interactive
          leading={<Icon icon="folder" />}
          footer={
            <>
              <ListCardCounter icon="file-text" count={24} tooltip="24 documents" />
              <ListCardCounter icon="image" count={8} tooltip="8 images" />
            </>
          }
        />
      </SpecimenGroup>

      <SpecimenGroup label="Linked footer counter">
        <ListCard
          title="Brand guidelines"
          subtitle="Last updated 2 weeks ago"
          leadingShape="rounded-square"
          interactive
          leading={<Icon icon="folder" />}
          footer={
            <ListCardCounter
              icon="layers"
              count={3}
              tooltip="3 sub-folders"
              href="#sub-folders"
              onClick={() => setLinkedAction("Opened sub-folders")}
            />
          }
        />
        <p style={{ margin: "0.75rem 0 0", fontSize: "0.8125rem", color: "var(--poodle-color-text-secondary)" }}>
          {linkedAction}
        </p>
      </SpecimenGroup>
    </>
  );
}

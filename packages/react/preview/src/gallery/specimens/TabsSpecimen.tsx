import { useState } from "react";
import { Tabs } from "@inflatable-cookie/poodle-react";
import {
  projectCorpus,
  tabsCases,
  tabsInterface,
  type ProjectedInstance,
} from "@inflatable-cookie/poodle-core/conformance";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// g14.004: the shared corpus owns groups, fixtures, collection order, and axes.
const groups = projectCorpus(tabsCases, tabsInterface);

function propsOf(instance: ProjectedInstance): Record<string, unknown> {
  const props: Record<string, unknown> = { ...instance.props };
  for (const key of Object.keys(props)) {
    if (props[key] === null) delete props[key];
  }
  return props;
}

export function TabsSpecimen() {
  const [valueLog, setValueLog] = useState("No tab change yet.");

  return (
    <SpecimenLayout>
      {groups.map((group) => (
        <SpecimenGroup key={group.label} label={group.label}>
          {group.instances.map((instance) => (
            <div
              key={instance.caseId + instance.caption}
              style={{ display: "flex", alignItems: "center", gap: "0.75rem" }}
            >
              <span
                style={{
                  color: "var(--poodle-color-text-secondary, #c9d4e0)",
                  fontSize: "0.75rem",
                  minWidth: "12rem",
                }}
              >
                {instance.caption}
              </span>
              <Tabs
                {...(propsOf(instance) as never)}
                onValueChange={(value) => setValueLog(`${instance.caption}: ${value}`)}
              />
            </div>
          ))}
        </SpecimenGroup>
      ))}
      <SpecimenGroup label="Interaction" bare>
        <span style={{ fontSize: "0.75rem" }}>{valueLog}</span>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

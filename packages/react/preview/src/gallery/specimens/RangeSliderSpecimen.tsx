import { useState } from "react";
import { RangeSlider } from "@inflatable-cookie/poodle-react";
import {
  rangeSliderCases,
  rangeSliderInterface,
  projectCorpus,
  type ProjectedInstance,
} from "@inflatable-cookie/poodle-core/conformance";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const groups = projectCorpus(rangeSliderCases, rangeSliderInterface);

function propsOf(instance: ProjectedInstance): Record<string, unknown> {
  const props: Record<string, unknown> = { ...instance.props };
  for (const key of Object.keys(props)) {
    if (props[key] === null) delete props[key];
  }
  return props;
}

export function RangeSliderSpecimen() {
  const [valueLog, setValueLog] = useState("No range change yet.");

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
              <RangeSlider
                {...(propsOf(instance) as never)}
                onValueChange={(value) => {
                  setValueLog(`${instance.caption}: [${value[0]}, ${value[1]}]`);
                }}
              />
            </div>
          ))}
        </SpecimenGroup>
      ))}
      <SpecimenGroup label="Interaction" bare>
        <div style={{ display: "flex", alignItems: "center", gap: "0.75rem" }}>
          <span
            style={{
              color: "var(--poodle-color-text-secondary, #c9d4e0)",
              fontSize: "0.75rem",
              minWidth: "12rem",
            }}
          >
            {valueLog}
          </span>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

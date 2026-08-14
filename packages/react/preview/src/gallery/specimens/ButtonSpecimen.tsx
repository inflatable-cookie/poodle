import { useState } from "react";
import { Button } from "@inflatable-cookie/poodle-react";
import { buttonCases, buttonInterface, projectCorpus } from "@inflatable-cookie/poodle-core/conformance";
import type { ProjectedInstance } from "@inflatable-cookie/poodle-core/conformance";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

// g14.001: the specimen page is a projection of the conformance case
// corpus — groups, captions, axes, and fixtures come from
// packages/core/src/conformance/button-cases.ts, never restated here.
const groups = projectCorpus(buttonCases, buttonInterface);

function propsOf(instance: ProjectedInstance): Record<string, unknown> {
  const props: Record<string, unknown> = { ...instance.props };
  for (const key of Object.keys(props)) {
    if (props[key] === null) delete props[key];
  }
  props.leadingIcon = instance.leadingIcon ?? null;
  props.trailingIcon = instance.trailingIcon ?? null;
  return props;
}

export function ButtonSpecimen() {
  const [clickLog, setClickLog] = useState("No button clicked yet.");

  return (
    <SpecimenLayout>
      {groups.map((group) => (
        <SpecimenGroup key={group.label} label={group.label}>
          {group.instances.map((instance) => (
            <div
              key={`${instance.caseId}:${instance.caption}`}
              className="poodle-specimen__row"
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
              <Button
                {...(propsOf(instance) as never)}
                onClick={() => setClickLog(`Clicked: ${instance.caption}`)}
              >
                {instance.label}
              </Button>
            </div>
          ))}
        </SpecimenGroup>
      ))}

      <SpecimenGroup label="Interaction" bare>
        <div className="poodle-specimen__row">{clickLog}</div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

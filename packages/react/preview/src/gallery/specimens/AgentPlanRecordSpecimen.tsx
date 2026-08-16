import { AgentPlanRecord } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const plan = [
  "## Proposed plan",
  "",
  "1. Add the `AgentPlan` surface to the composer",
  "2. Wire the decision callbacks through the host",
  "3. Append the settled record to the transcript",
  "",
  "Then run the gates.",
].join("\n");

const longPlan = [
  "## Rollout",
  "",
  ...Array.from({ length: 12 }, (_, index) => `${index + 1}. Step ${index + 1} of the rollout, with enough prose to matter`),
].join("\n");

export function AgentPlanRecordSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <AgentPlanRecord plan={plan} status="accepted" size={size} />}
      densities={(density) => <AgentPlanRecord plan={plan} status="accepted" density={density} />}
    >
      <SpecimenGroup label="Accepted">
        <AgentPlanRecord plan={plan} status="accepted" />
      </SpecimenGroup>

      <SpecimenGroup label="Revised">
        <AgentPlanRecord plan={plan} status="revised" decisionLabel="Revised with operator feedback" />
      </SpecimenGroup>

      <SpecimenGroup label="Dismissed">
        <AgentPlanRecord plan={plan} status="dismissed" />
      </SpecimenGroup>

      <SpecimenGroup label="Expanded">
        <AgentPlanRecord plan={plan} status="accepted" expanded />
      </SpecimenGroup>

      <SpecimenGroup label="With provenance">
        <AgentPlanRecord plan={plan} status="accepted" decidedAt="2026-08-07 10:00" />
      </SpecimenGroup>

      <SpecimenGroup label="Long plan">
        <AgentPlanRecord plan={longPlan} status="accepted" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
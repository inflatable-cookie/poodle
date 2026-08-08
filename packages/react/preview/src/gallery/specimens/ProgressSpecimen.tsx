import { Progress } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function ProgressSpecimen() {
  return (
    <SpecimenLayout
      showSizes
      showDensities={false}
      sizes={(size) => (
        <div style={{ width: "min(100%, 20rem)" }}>
          <Progress value={60} ariaLabel={`Progress at ${size}`} size={size} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Determinate">
          <Progress value={0} ariaLabel="Empty progress" />
          <Progress value={35} ariaLabel="35% progress" />
          <Progress value={72} ariaLabel="72% progress" />
          <Progress value={100} ariaLabel="Complete progress" />
        </SpecimenGroup>

        <SpecimenGroup label="Indeterminate">
          <Progress indeterminate ariaLabel="Loading" />
        </SpecimenGroup>

        <SpecimenGroup label="Custom max">
          <Progress value={3} max={5} ariaLabel="3 of 5 steps complete" valueText="3 of 5 steps" />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}

import { useState } from "react";
import { Stepper, type StepperStep } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const wizardSteps: StepperStep[] = [
  { value: "state", label: "Current state", status: "complete" },
  { value: "recovery", label: "Recovery", status: "complete" },
  { value: "categories", label: "Categories", status: "pending" },
  { value: "apply", label: "Apply and verify", status: "pending", isDisabled: true },
];

const workingSteps: StepperStep[] = [
  { value: "read", label: "Read source", status: "complete" },
  { value: "extract", label: "Extract tokens", status: "running" },
  { value: "map", label: "Map to theme", status: "pending" },
];

const failedSteps: StepperStep[] = [
  { value: "read", label: "Read source", status: "complete" },
  { value: "gate", label: "Quality gate", status: "failed" },
  { value: "apply", label: "Apply changes", status: "pending" },
];

const mixedSteps: StepperStep[] = [
  { value: "read", label: "Read source", status: "complete" },
  { value: "gate", label: "Quality gate", status: "failed" },
  { value: "extract", label: "Extract tokens", status: "running" },
  { value: "apply", label: "Apply changes", status: "pending" },
];

const doneSteps: StepperStep[] = [
  { value: "scan", label: "Scan the tree", status: "complete" },
  { value: "plan", label: "Draft the lane plan", status: "complete" },
  { value: "review", label: "Review with the gate", status: "complete" },
  { value: "apply", label: "Apply the changes", status: "complete" },
  { value: "record", label: "Record architecture verdict and next lane", status: "complete" },
];

export function StepperSpecimen() {
  const [current, setCurrent] = useState("categories");
  const [lastRerun, setLastRerun] = useState("");
  const [collapsed, setCollapsed] = useState(true);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Stepper steps={wizardSteps} size={size} defaultValue="categories" ariaLabel={`${size} steps`} />
      )}
      densities={(density) => (
        <Stepper
          steps={wizardSteps}
          density={density}
          defaultValue="categories"
          ariaLabel={`${density} steps`}
        />
      )}
    >
      <SpecimenGroup label="Default">
        <Stepper
          steps={wizardSteps}
          value={current}
          ariaLabel="DAW sync steps"
          onValueChange={setCurrent}
        />
        <p>Current: <strong>{current}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Vertical">
        <div style={{ maxWidth: "20rem" }}>
          <Stepper
            steps={wizardSteps}
            orientation="vertical"
            value={current}
            ariaLabel="DAW sync steps, vertical"
            onValueChange={setCurrent}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Collapsed">
        <div style={{ maxWidth: "30rem" }}>
          <Stepper
            steps={doneSteps}
            orientation="vertical"
            collapsible
            collapsed={collapsed}
            defaultValue="record"
            ariaLabel="Lane progress"
            onCollapsedChange={setCollapsed}
          />
        </div>
        <p>Collapsed: <strong>{String(collapsed)}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Collapsed statuses">
        <div style={{ maxWidth: "30rem" }}>
          <Stepper
            steps={mixedSteps}
            orientation="vertical"
            collapsible
            defaultCollapsed
            defaultValue="extract"
            ariaLabel="Pipeline progress"
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Working">
        <Stepper steps={workingSteps} defaultValue="extract" ariaLabel="Import progress" />
      </SpecimenGroup>

      <SpecimenGroup label="Failed">
        <Stepper steps={failedSteps} defaultValue="gate" ariaLabel="Pipeline steps" />
      </SpecimenGroup>

      <SpecimenGroup label="Re-run">
        <Stepper
          steps={workingSteps.map((step) => ({ ...step, status: "complete" as const }))}
          defaultValue="read"
          ariaLabel="Completed pipeline"
          onRerun={setLastRerun}
        />
        <p>Last re-run: <strong>{lastRerun || "none"}</strong></p>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <Stepper steps={wizardSteps} disabled defaultValue="categories" ariaLabel="Disabled steps" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}

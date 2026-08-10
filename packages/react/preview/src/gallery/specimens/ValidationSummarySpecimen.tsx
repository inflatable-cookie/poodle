import { ValidationSummary } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const entries = [
  { fieldId: "project-name", label: "Project name", message: "Enter a project name.", validationState: "invalid" as const },
  { fieldId: "repository", label: "Repository", message: "Checking availability…", validationState: "pending" as const },
];

export function ValidationSummarySpecimen() {
  return <div className="poodle-specimen">
    <SpecimenGroup label="Blocking errors"><ValidationSummary title="Fix these fields" entries={entries} /></SpecimenGroup>
    <SpecimenGroup label="Including pending checks"><ValidationSummary title="Review before continuing" entries={entries} includePending /></SpecimenGroup>
  </div>;
}

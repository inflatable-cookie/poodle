import type { MouseEvent } from "react";
import { Field, FieldSet, TextInput, ValidationSummary } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";

const entries = [
  { fieldId: "project-name", label: "Project name", message: "Enter a project name.", validationState: "invalid" as const },
  { fieldId: "repository", label: "Repository", message: "Checking availability…", validationState: "pending" as const },
];

export function ValidationSummarySpecimen() {
  function handleSummaryClick(event: MouseEvent<HTMLDivElement>): void {
    const anchor = event.target instanceof Element
      ? event.target.closest<HTMLAnchorElement>(".poodle-validation-summary a")
      : null;
    const targetId = anchor?.getAttribute("href")?.slice(1);
    const target = targetId ? document.getElementById(targetId) : null;
    if (!anchor || !target) return;

    event.preventDefault();
    target.scrollIntoView?.({ block: "nearest" });
    target.focus({ preventScroll: true });
  }

  return <div className="poodle-specimen" onClickCapture={handleSummaryClick}>
    <SpecimenGroup label="Blocking errors">
      <ValidationSummary title="Fix these fields" entries={entries} />
      <FieldSet legend="Project details">
        <Field
          id="project-name"
          label="Project name"
          required
          validationState="invalid"
          error="Enter a project name."
          control={({ describedBy, validationState }) => (
            <TextInput
              id="project-name"
              placeholder="My project"
              ariaLabel="Project name"
              describedBy={describedBy}
              validationState={validationState}
            />
          )}
        />
        <Field
          id="repository"
          label="Repository"
          required
          validationState="pending"
          pendingMessage="Checking availability…"
          control={({ describedBy, validationState }) => (
            <TextInput
              id="repository"
              placeholder="owner/repository"
              ariaLabel="Repository"
              describedBy={describedBy}
              validationState={validationState}
            />
          )}
        />
      </FieldSet>
    </SpecimenGroup>
    <SpecimenGroup label="Including pending checks"><ValidationSummary title="Review before continuing" entries={entries} includePending /></SpecimenGroup>
  </div>;
}

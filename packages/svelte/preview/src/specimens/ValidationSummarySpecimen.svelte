<script lang="ts">
  import { ValidationSummary } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const entries = [
    { fieldId: "project-name", label: "Project name", message: "Enter a project name.", validationState: "invalid" as const },
    { fieldId: "repository", label: "Repository", message: "Checking availability…", validationState: "pending" as const },
  ];
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Blocking errors">
    <ValidationSummary title="Fix these fields" {entries} />
  </SpecimenGroup>
  <SpecimenGroup label="Including pending checks">
    <ValidationSummary title="Review before continuing" {entries} includePending />
  </SpecimenGroup>
</div>

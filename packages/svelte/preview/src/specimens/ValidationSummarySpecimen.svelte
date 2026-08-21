<script lang="ts">
  import { Field, FieldSet, TextInput, ValidationSummary } from "@inflatable-cookie/poodle-svelte";
  import { onMount } from "svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const entries = [
    { fieldId: "project-name", label: "Project name", message: "Enter a project name.", validationState: "invalid" as const },
    { fieldId: "repository", label: "Repository", message: "Checking availability…", validationState: "pending" as const },
  ];

  let specimenRoot: HTMLDivElement;

  onMount(() => {
    const handleSummaryClick = (event: MouseEvent) => {
      const anchor = event.target instanceof Element
        ? event.target.closest<HTMLAnchorElement>(".poodle-validation-summary a")
        : null;
      const targetId = anchor?.getAttribute("href")?.slice(1);
      const target = targetId ? document.getElementById(targetId) : null;
      if (!anchor || !target) return;

      event.preventDefault();
      if (typeof target.scrollIntoView === "function") {
        target.scrollIntoView({ block: "nearest" });
      }
      target.focus({ preventScroll: true });
    };

    specimenRoot.addEventListener("click", handleSummaryClick);
    return () => specimenRoot.removeEventListener("click", handleSummaryClick);
  });
</script>

<div class="poodle-specimen" bind:this={specimenRoot}>
  <SpecimenGroup label="Blocking errors">
    <ValidationSummary title="Fix these fields" {entries} />
    <FieldSet legend="Project details">
      <Field
        id="project-name"
        label="Project name"
        required
        validationState="invalid"
        error="Enter a project name."
      >
        {#snippet control({ describedBy, validationState })}
          <TextInput
            id="project-name"
            placeholder="My project"
            ariaLabel="Project name"
            {describedBy}
            {validationState}
          />
        {/snippet}
      </Field>
      <Field
        id="repository"
        label="Repository"
        required
        validationState="pending"
        pendingMessage="Checking availability…"
      >
        {#snippet control({ describedBy, validationState })}
          <TextInput
            id="repository"
            placeholder="owner/repository"
            ariaLabel="Repository"
            {describedBy}
            {validationState}
          />
        {/snippet}
      </Field>
    </FieldSet>
  </SpecimenGroup>
  <SpecimenGroup label="Including pending checks">
    <ValidationSummary title="Review before continuing" {entries} includePending />
  </SpecimenGroup>
</div>

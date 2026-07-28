<script lang="ts">
  import { Stepper, type StepperStep } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  // The Soundcheck arrangement this design came from: two done, one current,
  // one not yet reached.
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

  // The case position-derived state cannot express: a failed step *behind* the
  // current one. Deriving from `index < current` would render this as done.
  const failedSteps: StepperStep[] = [
    { value: "read", label: "Read source", status: "complete" },
    { value: "gate", label: "Quality gate", status: "failed" },
    { value: "apply", label: "Apply changes", status: "pending" },
  ];

  let current = "categories";
  let lastRerun = "";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Stepper
      steps={wizardSteps}
      value={current}
      ariaLabel="DAW sync steps"
      onValueChange={(value) => (current = value)}
    />
    <p>Current: <strong>{current}</strong></p>
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
      onRerun={(value) => (lastRerun = value)}
    />
    <p>Last re-run: <strong>{lastRerun || "none"}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <Stepper steps={wizardSteps} disabled defaultValue="categories" ariaLabel="Disabled steps" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Stepper steps={wizardSteps} {size} defaultValue="categories" ariaLabel={`${size} steps`} />
  {/snippet}

  {#snippet densities(density)}
    <Stepper steps={wizardSteps} {density} defaultValue="categories" ariaLabel={`${density} steps`} />
  {/snippet}
</SpecimenLayout>

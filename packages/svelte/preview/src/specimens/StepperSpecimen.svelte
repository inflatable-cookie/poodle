<script lang="ts">
  import { Stepper, type StepperStep } from "@inflatable-cookie/poodle-svelte";
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

  // All four statuses on one rail — the only arrangement where the collapsed
  // form's colour coding is legible at a glance.
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

  let current = $state("categories");
  let lastRerun = $state("");
  let collapsed = $state(true);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Guided workflow">
    <Stepper
      steps={wizardSteps}
      value={current}
      ariaLabel="DAW sync steps"
      onValueChange={(value) => (current = value)}
    />
    <div style="max-width: 20rem;">
      <Stepper
        steps={wizardSteps}
        orientation="vertical"
        value={current}
        ariaLabel="DAW sync steps, vertical"
        onValueChange={(value) => (current = value)}
      />
    </div>
    <p>Current: <strong>{current}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Collapsed progress">
    <div style="max-width: 30rem;">
      <Stepper
        steps={doneSteps}
        orientation="vertical"
        collapsible
        bind:collapsed
        defaultValue="record"
        ariaLabel="Lane progress"
      />
    </div>
    <p>Collapsed: <strong>{collapsed}</strong></p>
    <div style="max-width: 30rem;">
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

  <SpecimenGroup label="Running and failed states">
    <Stepper steps={workingSteps} defaultValue="extract" ariaLabel="Import progress" />
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

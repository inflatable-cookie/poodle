<script lang="ts">
  import { RadioGroup, type RadioGroupOption, type ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const planOptions: RadioGroupOption[] = [
    { value: "free", label: "Free" },
    { value: "pro", label: "Pro" },
    { value: "enterprise", label: "Enterprise" },
  ];

  const sizeOptions: RadioGroupOption[] = [
    { value: "sm", label: "Small" },
    { value: "md", label: "Medium" },
    { value: "lg", label: "Large" },
    { value: "xl", label: "Extra large" },
  ];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let selectedPlan = "pro";
  let selectedSize = "md";
</script>

<div class="specimen">
  <SpecimenGroup label="Vertical (default)">
    <RadioGroup
      options={planOptions}
      value={selectedPlan}
      ariaLabel="Select plan"
      on:valueChange={(e) => (selectedPlan = e.detail.value)}
    />
    <p>Selected: <strong>{selectedPlan}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Horizontal">
    <RadioGroup
      options={sizeOptions}
      value={selectedSize}
      orientation="horizontal"
      ariaLabel="Select size"
      on:valueChange={(e) => (selectedSize = e.detail.value)}
    />
    <p>Selected: <strong>{selectedSize}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__stack">
      {#each controlSizes as size}
        <RadioGroup
          options={planOptions}
          defaultValue="pro"
          ariaLabel={"Plan at " + size}
          {size}
        />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__stack">
          <span class="specimen__label">{density}</span>
          <RadioGroup
            options={planOptions}
            defaultValue="pro"
            ariaLabel={"Plan at " + density + " density"}
            {density}
          />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled">
    <RadioGroup options={planOptions} defaultValue="free" disabled ariaLabel="Disabled plan selector" />
  </SpecimenGroup>

  <SpecimenGroup label="Custom selected color">
    <RadioGroup
      options={planOptions}
      value={selectedPlan}
      selectedColor="#22c55e"
      ariaLabel="Select plan with custom selected color"
      on:valueChange={(e) => (selectedPlan = e.detail.value)}
    />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }
</style>

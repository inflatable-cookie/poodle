<script lang="ts">
  import { RadioGroup, type RadioGroupOption } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let selectedPlan = "pro";
  let selectedSize = "md";
</script>

<SpecimenLayout>
  <SpecimenGroup label="Vertical (default)">
    <RadioGroup
      options={planOptions}
      value={selectedPlan}
      ariaLabel="Select plan"
      onValueChange={(value) => (selectedPlan = value)}
    />
    <p>Selected: <strong>{selectedPlan}</strong></p>
  </SpecimenGroup>

  <SpecimenGroup label="Horizontal">
    <RadioGroup
      options={sizeOptions}
      value={selectedSize}
      orientation="horizontal"
      ariaLabel="Select size"
      onValueChange={(value) => (selectedSize = value)}
    />
    <p>Selected: <strong>{selectedSize}</strong></p>
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
      onValueChange={(value) => (selectedPlan = value)}
    />
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <RadioGroup
      options={planOptions}
      defaultValue="pro"
      orientation="horizontal"
      {size}
      ariaLabel={"Plan at " + size}
    />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <RadioGroup
      options={planOptions}
      defaultValue="pro"
      orientation="horizontal"
      {density}
      ariaLabel={"Plan at " + density + " density"}
    />
  </svelte:fragment>
</SpecimenLayout>

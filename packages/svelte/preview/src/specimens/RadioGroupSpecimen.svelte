<script lang="ts">
  import { RadioGroup, Eyebrow, type RadioGroupOption } from "@poodle/svelte-primitives";

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
  <div class="specimen__group">
    <Eyebrow>Vertical (default)</Eyebrow>
    <RadioGroup
      options={planOptions}
      value={selectedPlan}
      ariaLabel="Select plan"
      on:valueChange={(e) => (selectedPlan = e.detail.value)}
    />
    <p>Selected: <strong>{selectedPlan}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Horizontal</Eyebrow>
    <RadioGroup
      options={sizeOptions}
      value={selectedSize}
      orientation="horizontal"
      ariaLabel="Select size"
      on:valueChange={(e) => (selectedSize = e.detail.value)}
    />
    <p>Selected: <strong>{selectedSize}</strong></p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
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
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <RadioGroup options={planOptions} defaultValue="free" disabled ariaLabel="Disabled plan selector" />
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom selected color</Eyebrow>
    <RadioGroup
      options={planOptions}
      value={selectedPlan}
      selectedColor="#22c55e"
      ariaLabel="Select plan with custom selected color"
      on:valueChange={(e) => (selectedPlan = e.detail.value)}
    />
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>

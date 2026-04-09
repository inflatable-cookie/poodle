<script lang="ts">
  import { Checkbox } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let notifications = true;
  let marketing = false;
  let terms = false;
</script>

<div class="specimen">
  <SpecimenGroup label="Default">
    <Checkbox
      label="Enable email notifications"
      checked={notifications}
      on:checkedChange={(e) => (notifications = e.detail.checked)}
    />
    <Checkbox
      label="Subscribe to marketing emails"
      checked={marketing}
      on:checkedChange={(e) => (marketing = e.detail.checked)}
    />
    <Checkbox
      label="I agree to the terms and conditions"
      checked={terms}
      on:checkedChange={(e) => (terms = e.detail.checked)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="Sizes">
    <div class="specimen__row">
      {#each controlSizes as size}
        <Checkbox label="Accept terms" {size} />
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Densities">
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Checkbox id={"density-" + density} label="Option" {density} />
        </div>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="States">
    <Checkbox label="Disabled unchecked" disabled />
    <Checkbox label="Disabled checked" checked={true} disabled />
    <Checkbox label="Mixed / indeterminate" mixed />
    <Checkbox label="Read-only checked" checked={true} readOnly />
  </SpecimenGroup>

  <SpecimenGroup label="Custom selected color">
    <Checkbox label="Billable feature" checked={true} selectedColor="#22c55e" />
    <Checkbox label="Requires moderation" checked={true} selectedColor="#f59e0b" />
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
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

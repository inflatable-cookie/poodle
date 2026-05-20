<script lang="ts">
  import { FormActions, Button } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const dangerItems = [
    {
      label: "Discard draft",
      onSelect: () => console.log("discard"),
    },
  ];
</script>

<SpecimenLayout showSizes={false}>
  {#snippet children()}
    <div class="poodle-specimen">
      <SpecimenGroup label="End-aligned (default)">
        <FormActions>
          <Button variant="ghost">Cancel</Button>
          <Button variant="primary">Save changes</Button>
        </FormActions>
      </SpecimenGroup>

      <SpecimenGroup label="Start-aligned">
        <FormActions align="start">
          <Button variant="ghost">Back</Button>
          <Button variant="primary">Continue</Button>
        </FormActions>
      </SpecimenGroup>

      <SpecimenGroup label="Space between">
        <FormActions align="between">
          <Button variant="secondary" tone="danger">Delete</Button>
          <Button variant="primary">Save</Button>
        </FormActions>
      </SpecimenGroup>

      <SpecimenGroup label="Responsive danger actions">
        <div class="poodle-form-actions-specimen__responsive">
          <FormActions align="end" {dangerItems}>
            <Button variant="ghost">Back</Button>
            <Button variant="primary">Save changes</Button>
            {#snippet danger()}
              <Button variant="ghost" tone="danger">Discard draft</Button>
            {/snippet}
          </FormActions>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Bordered separation">
        <FormActions showTopBorder>
          <Button variant="ghost">Cancel</Button>
          <Button variant="primary">Save changes</Button>
        </FormActions>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-form-actions-specimen__variant-block">
      <div class="poodle-form-actions-specimen__label">{density.toUpperCase()}</div>
      <div class="poodle-form-actions-specimen__variant">
        <FormActions {density}>
          <Button variant="ghost">Cancel</Button>
          <Button variant="primary">Save changes</Button>
        </FormActions>
      </div>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-form-actions-specimen__responsive {
    max-width: 20rem;
  }

  .poodle-form-actions-specimen__variant-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: min(22rem, 100%);
  }

  .poodle-form-actions-specimen__variant {
    width: 100%;
  }

  .poodle-form-actions-specimen__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
</style>

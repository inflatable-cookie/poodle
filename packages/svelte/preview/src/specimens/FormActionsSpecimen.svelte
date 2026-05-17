<script lang="ts">
  import { FormActions, Button } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const dangerItems = [
    {
      label: "Discard draft",
      onSelect: () => console.log("discard"),
    },
  ];
</script>

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
    <FormActions align="end" {dangerItems}>
      <Button variant="ghost">Back</Button>
      <Button variant="primary">Save changes</Button>
      {#snippet danger()}
        <Button variant="ghost" tone="danger">Discard draft</Button>
      {/snippet}
    </FormActions>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>

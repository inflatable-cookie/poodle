<script lang="ts">
  import { FormActions, Button, Eyebrow } from "@poodle/svelte-primitives";

  const dangerItems = [
    {
      label: "Discard draft",
      onSelect: () => console.log("discard"),
    },
  ];
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>End-aligned (default)</Eyebrow>
    <FormActions>
      <Button variant="secondary">Cancel</Button>
      <Button>Save changes</Button>
    </FormActions>
  </div>

  <div class="specimen__group">
    <Eyebrow>Start-aligned</Eyebrow>
    <FormActions align="start">
      <Button variant="secondary">Back</Button>
      <Button>Continue</Button>
    </FormActions>
  </div>

  <div class="specimen__group">
    <Eyebrow>Space between</Eyebrow>
    <FormActions align="between">
      <Button tone="danger">Delete</Button>
      <Button>Save</Button>
    </FormActions>
  </div>

  <div class="specimen__group">
    <Eyebrow>Responsive danger actions</Eyebrow>
    <FormActions align="end" {dangerItems}>
      <Button variant="secondary">Back</Button>
      <Button>Save changes</Button>
      <svelte:fragment slot="danger">
        <Button variant="ghost" tone="danger">Discard draft</Button>
      </svelte:fragment>
    </FormActions>
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
</style>

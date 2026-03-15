<script lang="ts">
  import { FormActions, Button, Eyebrow } from "@pug/svelte-primitives";
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
      <Button variant="danger">Delete</Button>
      <Button>Save</Button>
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

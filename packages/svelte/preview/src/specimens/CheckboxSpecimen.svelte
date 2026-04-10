<script lang="ts">
  import { Checkbox } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let notifications = true;
  let marketing = false;
  let terms = false;
</script>

<SpecimenLayout>
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

  <svelte:fragment slot="sizes" let:size>
    <Checkbox label="Accept terms" {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Checkbox id={"density-" + density} label="Option" {density} />
  </svelte:fragment>
</SpecimenLayout>

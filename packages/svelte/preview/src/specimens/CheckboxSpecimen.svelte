<script lang="ts">
  import { Checkbox } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let notifications = $state(true);
  let marketing = $state(false);
  let terms = $state(false);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Checkbox
      label="Enable email notifications"
      checked={notifications}
      onCheckedChange={(checked) => (notifications = checked)}
    />
    <Checkbox
      label="Subscribe to marketing emails"
      checked={marketing}
      onCheckedChange={(checked) => (marketing = checked)}
    />
    <Checkbox
      label="I agree to the terms and conditions"
      checked={terms}
      onCheckedChange={(checked) => (terms = checked)}
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

  {#snippet sizes(size)}
    <Checkbox label="Accept terms" {size} />
  {/snippet}

  {#snippet densities(density)}
    <Checkbox id={"density-" + density} label="Option" {density} />
  {/snippet}
</SpecimenLayout>

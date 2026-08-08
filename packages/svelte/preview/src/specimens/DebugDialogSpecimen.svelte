<script lang="ts">
  import { DebugDialog } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const value = {
    id: "asset_42",
    status: "ready",
    checks: ["metadata", "thumbnail", "permissions"],
  };
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="With debug value">
    <DebugDialog {value} title="Asset payload" triggerLabel="Inspect payload" />
  </SpecimenGroup>

  <SpecimenGroup label="Custom trigger">
    <DebugDialog
      value={value}
      title="Compact payload"
      triggerLabel="Debug"
      triggerVariant="secondary"
      triggerSize="xs"
      maxHeight="18rem"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Hidden when null">
    <DebugDialog value={null} />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>

<script lang="ts">
  import { StatusIndicator } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="All statuses">
    <div class="status-list">
      <StatusIndicator status="neutral" label="Neutral" />
      <StatusIndicator status="info" label="Info" />
      <StatusIndicator status="success" label="Success" />
      <StatusIndicator status="warning" label="Warning" />
      <StatusIndicator status="danger" label="Danger" />
      <StatusIndicator status="pending" label="Pending" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Without labels (dot only)">
    <div class="status-row">
      <StatusIndicator status="success" ariaLabel="Online" />
      <StatusIndicator status="info" ariaLabel="Active" />
      <StatusIndicator status="warning" ariaLabel="Away" />
      <StatusIndicator status="danger" ariaLabel="Offline" />
      <StatusIndicator status="neutral" ariaLabel="Unknown" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Slot content">
    <StatusIndicator status="success">Build passing</StatusIndicator>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <StatusIndicator status="success" {size} label={size.toUpperCase()} />
  </svelte:fragment>
</SpecimenLayout>

<style>
  .status-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .status-row {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
</style>

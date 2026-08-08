<script lang="ts">
  import { StatusIndicator } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  <SpecimenGroup label="All statuses">
    <div class="poodle-status-list">
      <StatusIndicator status="neutral" label="Neutral" />
      <StatusIndicator status="info" label="Info" />
      <StatusIndicator status="success" label="Success" />
      <StatusIndicator status="warning" label="Warning" />
      <StatusIndicator status="danger" label="Danger" />
      <StatusIndicator status="pending" label="Pending" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Without labels (dot only)">
    <div class="poodle-status-row">
      <StatusIndicator status="success" ariaLabel="Online" />
      <StatusIndicator status="info" ariaLabel="Active" />
      <StatusIndicator status="warning" ariaLabel="Away" />
      <StatusIndicator status="danger" ariaLabel="Offline" />
      <StatusIndicator status="neutral" ariaLabel="Unknown" />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Child content">
    <StatusIndicator status="success">Build passing</StatusIndicator>
  </SpecimenGroup>

  <SpecimenGroup label="Inherited typography">
    <p class="poodle-status-copy">
      Deploy status:
      <StatusIndicator status="success" label="Healthy" typography="inherit" />
      across the last 24 hours.
    </p>
  </SpecimenGroup>

  {#snippet sizes(size)}
    <StatusIndicator status="success" {size} label={size.toUpperCase()} />
  {/snippet}

  {#snippet densities(density)}
    <StatusIndicator status="success" {density} label="Success" />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-status-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .poodle-status-row {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .poodle-status-copy {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--poodle-color-text-primary);
  }
</style>

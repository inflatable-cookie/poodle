<script lang="ts">
  import { StatusIndicator, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>All statuses</Eyebrow>
    <div class="status-list">
      <StatusIndicator status="neutral" label="Neutral" />
      <StatusIndicator status="info" label="Info" />
      <StatusIndicator status="success" label="Success" />
      <StatusIndicator status="warning" label="Warning" />
      <StatusIndicator status="danger" label="Danger" />
      <StatusIndicator status="pending" label="Pending" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Without labels (dot only)</Eyebrow>
    <div class="status-row">
      <StatusIndicator status="success" ariaLabel="Online" />
      <StatusIndicator status="warning" ariaLabel="Away" />
      <StatusIndicator status="danger" ariaLabel="Offline" />
      <StatusIndicator status="neutral" ariaLabel="Unknown" />
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Slot content</Eyebrow>
    <StatusIndicator status="success">Build passing</StatusIndicator>
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

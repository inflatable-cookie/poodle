<script lang="ts">
  import { DetailSection } from "@poodle/svelte-composites";
  import { Button, DetailRow, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With title and rows</Eyebrow>
    <DetailSection title="Project details" description="Core metadata for this project.">
      <DetailRow label="Name" value="Poodle Design System" />
      <DetailRow label="Owner" value="Clay + Aura" />
      <DetailRow label="Created" value="March 2025" />
      <DetailRow label="Status" value="Active" />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>With actions</Eyebrow>
    <DetailSection title="Billing">
      <svelte:fragment slot="actions">
        <Button variant="secondary" size="sm">Edit</Button>
      </svelte:fragment>
      <DetailRow label="Plan" value="Pro" />
      <DetailRow label="Billing cycle" value="Monthly" />
      <DetailRow label="Next invoice" value="April 1, 2026" />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>DetailRow with description</Eyebrow>
    <DetailSection title="Configuration">
      <DetailRow label="API endpoint" value="https://api.example.com/v2" description="The base URL for all API requests." truncateValue />
      <DetailRow label="Rate limit" value="1,000 req/min" description="Maximum requests per minute." />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>Two-column details</Eyebrow>
    <DetailSection
      title="Runtime summary"
      description="Compact layout for denser metadata surfaces."
      columns={2}
    >
      <DetailRow label="Route" value="local-brokered" layout="stacked" />
      <DetailRow label="Posture" value="aura-local-brokered" layout="stacked" />
      <DetailRow label="Authority" value="local" layout="stacked" />
      <DetailRow label="Displays" value="2" layout="stacked" />
    </DetailSection>
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

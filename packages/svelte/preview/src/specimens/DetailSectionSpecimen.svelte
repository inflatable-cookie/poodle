<script lang="ts">
  import { DetailSection } from "@poodle/svelte-composites";
  import { Button, DetailItem, Eyebrow } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>With title and rows</Eyebrow>
    <DetailSection title="Project details" description="Core metadata for this project.">
      <DetailItem label="Name" value="Poodle Design System" />
      <DetailItem label="Owner" value="Clay + Aura" />
      <DetailItem label="Created" value="March 2025" />
      <DetailItem label="Status" value="Active" />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>With actions</Eyebrow>
    <DetailSection title="Billing">
      <svelte:fragment slot="actions">
        <Button variant="secondary" size="sm">Edit</Button>
      </svelte:fragment>
      <DetailItem label="Plan" value="Pro" />
      <DetailItem label="Billing cycle" value="Monthly" />
      <DetailItem label="Next invoice" value="April 1, 2026" />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>DetailItem with description</Eyebrow>
    <DetailSection title="Configuration">
      <DetailItem label="API endpoint" value="https://api.example.com/v2" description="The base URL for all API requests." truncateValue />
      <DetailItem label="Rate limit" value="1,000 req/min" description="Maximum requests per minute." />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>Two-column details</Eyebrow>
    <DetailSection
      title="Runtime summary"
      description="Compact layout for denser metadata surfaces."
      columns={2}
    >
      <DetailItem label="Route" value="local-brokered" layout="stacked" />
      <DetailItem label="Posture" value="aura-local-brokered" layout="stacked" />
      <DetailItem label="Authority" value="local" layout="stacked" />
      <DetailItem label="Displays" value="2" layout="stacked" />
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

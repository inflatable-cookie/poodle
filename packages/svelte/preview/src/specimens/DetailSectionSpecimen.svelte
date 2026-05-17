<script lang="ts">
  import { DetailSection } from "@poodle/svelte";
  import { Button, DetailItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="With title and rows">
    <DetailSection title="Project details" description="Core metadata for this project.">
      <DetailItem label="Name" value="Poodle Design System" />
      <DetailItem label="Owner" value="Clay + Aura" />
      <DetailItem label="Created" value="March 2025" />
      <DetailItem label="Status" value="Active" />
    </DetailSection>
  </SpecimenGroup>

  <SpecimenGroup label="With actions">
    <DetailSection title="Billing">
      {#snippet actions()}
        <Button variant="secondary" size="sm">Edit</Button>
      {/snippet}
      <DetailItem label="Plan" value="Pro" />
      <DetailItem label="Billing cycle" value="Monthly" />
      <DetailItem label="Next invoice" value="April 1, 2026" />
    </DetailSection>
  </SpecimenGroup>

  <SpecimenGroup label="DetailItem with description">
    <DetailSection title="Configuration">
      <DetailItem label="API endpoint" value="https://api.example.com/v2" description="The base URL for all API requests." truncateValue />
      <DetailItem label="Rate limit" value="1,000 req/min" description="Maximum requests per minute." />
    </DetailSection>
  </SpecimenGroup>

  <SpecimenGroup label="Two-column details">
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
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>

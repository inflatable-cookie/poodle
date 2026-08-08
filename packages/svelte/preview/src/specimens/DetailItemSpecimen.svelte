<script lang="ts">
  import { DetailItem, Button, Pill } from "@inflatable-cookie/poodle-svelte";
  import { DetailSection } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout showSizes={false}>
  <div class="poodle-specimen">
    <SpecimenGroup label="Inline layout (default)">
      <DetailSection title="Package info">
        <DetailItem label="Name" value="Poodle Design System" />
        <DetailItem label="Version" value="2.1.0" />
        <DetailItem label="License" value="MIT" />
      </DetailSection>
    </SpecimenGroup>

    <SpecimenGroup label="With description">
      <DetailItem label="API endpoint" value="https://api.example.com/v2" description="Base URL for all API requests." />
    </SpecimenGroup>

    <SpecimenGroup label="With action slot">
      <DetailItem label="Email" value="clay@example.com">
        {#snippet action()}
          <Button variant="secondary" size="sm">Change</Button>
        {/snippet}
      </DetailItem>
    </SpecimenGroup>

    <SpecimenGroup label="With value slot">
      <DetailItem label="Status">
        {#snippet valueContent()}
          <Pill tone="success" appearance="badge">Active</Pill>
        {/snippet}
      </DetailItem>
    </SpecimenGroup>

    <SpecimenGroup label="Stacked layout">
      <DetailItem
        label="Arrangement"
        value="2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A..."
        truncateValue
        layout="stacked"
      />
    </SpecimenGroup>

    <SpecimenGroup label="Surface presentation">
      <DetailSection title="Account">
        <DetailItem label="Name" value="Alice Chen" presentation="surface" />
        <DetailItem label="Role" value="Engineer" presentation="surface" />
        <DetailItem label="Email" value="alice@example.com" presentation="surface">
          {#snippet action()}
            <Button variant="secondary" size="sm">Edit</Button>
          {/snippet}
        </DetailItem>
      </DetailSection>
    </SpecimenGroup>
  </div>

  {#snippet densities(density)}
    <div class="poodle-detail-item-density-demo">
      <DetailItem
        label="Storage"
        value="84.2 GB"
        description="Current usage for the active workspace."
        presentation="surface"
        {density}
      >
        {#snippet action()}
          <Button variant="secondary" size="sm">Manage</Button>
        {/snippet}
      </DetailItem>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-detail-item-density-demo {
    width: min(100%, 32rem);
  }

</style>

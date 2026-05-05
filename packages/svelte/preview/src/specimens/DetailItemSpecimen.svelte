<script lang="ts">
  import { DetailItem, Button, Pill } from "@poodle/svelte";
  import { DetailSection } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

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
      <svelte:fragment slot="action">
        <Button variant="secondary" size="sm">Change</Button>
      </svelte:fragment>
    </DetailItem>
  </SpecimenGroup>

  <SpecimenGroup label="With value slot">
    <DetailItem label="Status">
      <svelte:fragment slot="value">
        <Pill tone="success" appearance="badge">Active</Pill>
      </svelte:fragment>
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
        <svelte:fragment slot="action">
          <Button variant="secondary" size="sm">Edit</Button>
        </svelte:fragment>
      </DetailItem>
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

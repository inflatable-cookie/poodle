<script lang="ts">
  import { DetailItem, Eyebrow, Button, Pill } from "@poodle/svelte-primitives";
  import { DetailSection } from "@poodle/svelte-composites";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Inline layout (default)</Eyebrow>
    <DetailSection title="Package info">
      <DetailItem label="Name" value="Poodle Design System" />
      <DetailItem label="Version" value="2.1.0" />
      <DetailItem label="License" value="MIT" />
    </DetailSection>
  </div>

  <div class="specimen__group">
    <Eyebrow>With description</Eyebrow>
    <DetailItem label="API endpoint" value="https://api.example.com/v2" description="Base URL for all API requests." />
  </div>

  <div class="specimen__group">
    <Eyebrow>With action slot</Eyebrow>
    <DetailItem label="Email" value="clay@example.com">
      <svelte:fragment slot="action">
        <Button variant="secondary" size="sm">Change</Button>
      </svelte:fragment>
    </DetailItem>
  </div>

  <div class="specimen__group">
    <Eyebrow>With value slot</Eyebrow>
    <DetailItem label="Status">
      <svelte:fragment slot="value">
        <Pill tone="success" appearance="badge">Active</Pill>
      </svelte:fragment>
    </DetailItem>
  </div>

  <div class="specimen__group">
    <Eyebrow>Stacked layout</Eyebrow>
    <DetailItem
      label="Arrangement"
      value="2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A..."
      truncateValue
      layout="stacked"
    />
  </div>

  <div class="specimen__group">
    <Eyebrow>Surface presentation</Eyebrow>
    <DetailSection title="Account">
      <DetailItem label="Name" value="Alice Chen" presentation="surface" />
      <DetailItem label="Role" value="Engineer" presentation="surface" />
      <DetailItem label="Email" value="alice@example.com" presentation="surface">
        <svelte:fragment slot="action">
          <Button variant="secondary" size="sm">Edit</Button>
        </svelte:fragment>
      </DetailItem>
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
    gap: 0.375rem;
  }
</style>
